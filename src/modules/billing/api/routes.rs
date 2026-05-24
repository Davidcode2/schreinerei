use axum::{
    extract::{Path, State},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use ts_rs::TS;
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{InvoiceId, SiteId};
use crate::modules::billing::application::BillingService;
use crate::modules::billing::domain::{
    BillingTaxMode, Invoice, InvoiceSnapshot, InvoiceSnapshotLineItem, InvoiceStatus, PdfArtifact,
};
use crate::modules::billing::infrastructure::InvoiceRepository;
use crate::modules::billing::pdf::{generate_invoice_pdf, invoice_pdf_filename};
use crate::modules::iam::application::user_service::{TenantContext, UserService};
use crate::modules::iam::infrastructure::user_repository::UserRepository;
use crate::modules::onboarding::infrastructure::keycloak_admin_client::KeycloakAdminClient;
use crate::modules::sites::api::routes::{
    ProjectLaborSummaryResponse, ProjectMaterialSummaryResponse, SiteInvoiceBillingResponse,
    SiteInvoiceProjectResponse,
};
use crate::modules::sites::application::site_service::InvoiceSummary;
use crate::modules::sites::domain::InvoicePricingMode;
use crate::modules::sites::infrastructure::site_repository::SiteRepository;
use crate::AppState;

#[derive(Debug, Clone, Copy)]
struct ResolvedInvoicePricing {
    mode: Option<InvoicePricingMode>,
    hourly_rate_cents: Option<i64>,
    fixed_price_cents: Option<i64>,
}

#[derive(Debug, Clone)]
struct TenantBillingProfile {
    sender_name: String,
    sender_address: Option<String>,
    billing_tax_mode: BillingTaxMode,
}

#[derive(Debug, Clone, Copy)]
struct InvoiceTotals {
    subtotal_amount_cents: Option<i64>,
    vat_rate_percent: Option<i32>,
    vat_amount_cents: Option<i64>,
    gross_amount_cents: Option<i64>,
    total_amount_cents: Option<i64>,
}

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/billing/invoices/{id}", get(get_invoice))
        .route(
            "/api/v1/billing/invoices/{id}/pdf",
            get(download_invoice_pdf),
        )
        .route(
            "/api/v1/billing/projects/{site_id}/invoices",
            get(list_project_invoices).post(create_project_invoice),
        )
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct CreateProjectInvoiceRequest {
    pub sender_name: Option<String>,
    pub sender_address: Option<String>,
    pub invoice_pricing_mode: Option<String>,
    pub hourly_rate_cents: Option<i64>,
    pub fixed_price_cents: Option<i64>,
    pub material_overrides: Option<Vec<ProjectInvoiceMaterialOverrideRequest>>,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct ProjectInvoiceMaterialOverrideRequest {
    pub material_id: Uuid,
    pub price_markup_percentage: i32,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct InvoiceResponse {
    pub id: Uuid,
    pub site_id: Uuid,
    pub invoice_number: i64,
    pub invoice_number_display: String,
    pub status: String,
    pub sender_name: Option<String>,
    pub sender_address: Option<String>,
    pub issued_at: Option<DateTime<Utc>>,
    pub due_on: Option<NaiveDate>,
    pub voided_at: Option<DateTime<Utc>>,
    pub pdf_artifact: Option<PdfArtifactResponse>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ProjectInvoiceLineItemResponse {
    pub source: String,
    pub description: String,
    pub quantity: f64,
    pub unit: String,
    pub source_count: i64,
    pub priced: bool,
    pub unit_price_cents: Option<i64>,
    pub line_total_cents: Option<i64>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct ProjectInvoiceDraftResponse {
    pub invoice: InvoiceResponse,
    pub project: SiteInvoiceProjectResponse,
    pub billing: SiteInvoiceBillingResponse,
    pub labor: ProjectLaborSummaryResponse,
    pub materials: ProjectMaterialSummaryResponse,
    pub billing_tax_mode: Option<String>,
    pub subtotal_amount_cents: Option<i64>,
    pub vat_rate_percent: Option<i32>,
    pub vat_amount_cents: Option<i64>,
    pub gross_amount_cents: Option<i64>,
    pub tax_note: Option<String>,
    pub total_amount_cents: Option<i64>,
    pub line_items: Vec<ProjectInvoiceLineItemResponse>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct PdfArtifactResponse {
    pub storage_path: String,
    pub sha256_hash: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
}

async fn get_invoice(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let service = BillingService::new(InvoiceRepository::new(state.pool));
    let invoice = service
        .find_invoice(ctx.tenant_id, InvoiceId(id))
        .await?
        .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;

    Ok(Json(invoice.into()))
}

async fn list_project_invoices(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(site_id): Path<Uuid>,
) -> Result<Json<Vec<InvoiceResponse>>, AppError> {
    let service = BillingService::new(InvoiceRepository::new(state.pool));
    let invoices = service
        .list_project_invoices(ctx.tenant_id, SiteId(site_id))
        .await?;

    Ok(Json(
        invoices.into_iter().map(InvoiceResponse::from).collect(),
    ))
}

async fn create_project_invoice(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(site_id): Path<Uuid>,
    Json(request): Json<CreateProjectInvoiceRequest>,
) -> Result<Json<ProjectInvoiceDraftResponse>, AppError> {
    if !ctx.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let site_id = SiteId(site_id);
    let invoice_summary = load_invoice_summary(state.pool.clone(), site_id, &ctx).await?;
    let invoice_summary = apply_invoice_overrides(invoice_summary, &request)?;
    let billing_profile = load_tenant_billing_profile(state.pool.clone(), &ctx).await?;
    let user_service = match KeycloakAdminClient::from_config(&state.config) {
        Ok(client) => UserService::new_with_role_assigner(
            UserRepository::new(state.pool.clone()),
            Arc::new(client),
        ),
        Err(_) => UserService::new(UserRepository::new(state.pool.clone())),
    };
    let created_by = user_service.get_or_create_user_id_from_ctx(&ctx).await?;
    let service = BillingService::new(InvoiceRepository::new(state.pool));
    let snapshot = invoice_snapshot(&invoice_summary, &billing_profile);
    let draft = service
        .create_draft_invoice(
            ctx.tenant_id,
            crate::modules::billing::domain::CreateInvoiceDraft {
                site_id,
                sender_name: Some(resolve_sender_name(&request, &billing_profile)),
                sender_address: resolve_sender_address(&request, &billing_profile),
                snapshot,
                created_by: Some(created_by),
            },
        )
        .await?;
    let generated = generate_invoice_pdf(&draft).map_err(AppError::Internal)?;
    let invoice = service
        .attach_pdf(ctx.tenant_id, generated.metadata)
        .await?;

    Ok(Json(ProjectInvoiceDraftResponse::from_parts(
        invoice,
        invoice_summary,
    )))
}

async fn download_invoice_pdf(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    if !ctx.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let service = BillingService::new(InvoiceRepository::new(state.pool));
    let invoice = service
        .find_invoice(ctx.tenant_id, InvoiceId(id))
        .await?
        .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;
    let generated = generate_invoice_pdf(&invoice).map_err(AppError::Internal)?;
    let filename = invoice_pdf_filename(&invoice);

    if invoice.pdf_artifact.is_none() {
        service
            .attach_pdf(ctx.tenant_id, generated.metadata.clone())
            .await?;
    }

    let disposition = format!("attachment; filename=\"{filename}\"");
    let disposition = HeaderValue::from_str(&disposition)
        .map_err(|error| AppError::Internal(format!("Invalid PDF filename: {error}")))?;

    Ok((
        StatusCode::OK,
        [
            (
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/pdf"),
            ),
            (header::CONTENT_DISPOSITION, disposition),
        ],
        generated.bytes,
    ))
}

async fn load_invoice_summary(
    pool: sqlx::PgPool,
    site_id: SiteId,
    ctx: &TenantContext,
) -> Result<InvoiceSummary, AppError> {
    let service = crate::modules::sites::application::site_service::SiteService::new(
        SiteRepository::new(pool),
    );
    service.get_invoice_summary(site_id, ctx).await
}

async fn load_tenant_billing_profile(
    pool: sqlx::PgPool,
    ctx: &TenantContext,
) -> Result<TenantBillingProfile, AppError> {
    let row = sqlx::query(
        r#"
        SELECT name, billing_sender_name, billing_sender_address, billing_tax_mode
        FROM tenants
        WHERE id = $1
        "#,
    )
    .bind(ctx.tenant_id.0)
    .fetch_one(&pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))?;

    let tenant_name: String = row
        .try_get("name")
        .map_err(|error| AppError::Database(error.to_string()))?;
    let sender_name = normalize_optional_text(
        row.try_get::<Option<String>, _>("billing_sender_name")
            .map_err(|error| AppError::Database(error.to_string()))?
            .as_deref(),
    )
    .unwrap_or(tenant_name);
    let sender_address = normalize_optional_text(
        row.try_get::<Option<String>, _>("billing_sender_address")
            .map_err(|error| AppError::Database(error.to_string()))?
            .as_deref(),
    );
    let tax_mode = row
        .try_get::<String, _>("billing_tax_mode")
        .map_err(|error| AppError::Database(error.to_string()))?
        .parse::<BillingTaxMode>()
        .map_err(AppError::Database)?;

    Ok(TenantBillingProfile {
        sender_name,
        sender_address,
        billing_tax_mode: tax_mode,
    })
}

fn invoice_snapshot(
    summary: &InvoiceSummary,
    billing_profile: &TenantBillingProfile,
) -> InvoiceSnapshot {
    let line_items = invoice_line_items(summary);
    let totals = invoice_totals(&line_items, billing_profile.billing_tax_mode);
    InvoiceSnapshot {
        project_name: summary.site.name.clone(),
        customer_name: summary.site.customer_name.clone(),
        project_location: summary.site.location.clone(),
        invoice_pricing_mode: summary.site.invoice_pricing_mode,
        hourly_rate_cents: summary.site.hourly_rate_cents,
        fixed_price_cents: summary.site.fixed_price_cents,
        billing_reference: summary.site.billing_reference.clone(),
        billing_notes: summary.site.billing_notes.clone(),
        quote_reference: summary.site.quote_reference.clone(),
        budget_amount_cents: summary.site.budget_amount_cents,
        labor_total_hours: summary.project.labor.total_hours,
        material_withdrawal_count: summary.project.materials.withdrawal_count,
        billing_tax_mode: Some(billing_profile.billing_tax_mode),
        subtotal_amount_cents: totals.subtotal_amount_cents,
        vat_rate_percent: totals.vat_rate_percent,
        vat_amount_cents: totals.vat_amount_cents,
        gross_amount_cents: totals.gross_amount_cents,
        tax_note: tax_note_for_mode(billing_profile.billing_tax_mode),
        total_amount_cents: totals.total_amount_cents,
        line_items: line_items
            .into_iter()
            .map(InvoiceSnapshotLineItem::from)
            .collect(),
    }
}

fn apply_invoice_overrides(
    mut summary: InvoiceSummary,
    request: &CreateProjectInvoiceRequest,
) -> Result<InvoiceSummary, AppError> {
    let requested_mode = request
        .invoice_pricing_mode
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(InvoicePricingMode::from_str)
        .transpose()
        .map_err(AppError::Validation)?;
    let pricing = resolve_invoice_pricing(
        requested_mode,
        request.hourly_rate_cents,
        request.fixed_price_cents,
        summary.site.invoice_pricing_mode,
        summary.site.hourly_rate_cents,
        summary.site.fixed_price_cents,
    )?;

    summary.site.invoice_pricing_mode = pricing.mode;
    summary.site.hourly_rate_cents = pricing.hourly_rate_cents;
    summary.site.fixed_price_cents = pricing.fixed_price_cents;
    apply_material_overrides(&mut summary, request.material_overrides.as_deref())?;

    Ok(summary)
}

fn apply_material_overrides(
    summary: &mut InvoiceSummary,
    overrides: Option<&[ProjectInvoiceMaterialOverrideRequest]>,
) -> Result<(), AppError> {
    let Some(overrides) = overrides else {
        return Ok(());
    };

    let known_material_ids: HashSet<Uuid> = summary
        .project
        .materials
        .lines
        .iter()
        .map(|line| line.material_id.0)
        .collect();
    let mut overrides_by_material = HashMap::new();

    for override_entry in overrides {
        if override_entry.price_markup_percentage < 0 {
            return Err(AppError::Validation(
                "Material markup cannot be negative".to_string(),
            ));
        }
        if !known_material_ids.contains(&override_entry.material_id) {
            return Err(AppError::Validation(
                "Material override references an unknown project material".to_string(),
            ));
        }
        if overrides_by_material
            .insert(
                override_entry.material_id,
                override_entry.price_markup_percentage,
            )
            .is_some()
        {
            return Err(AppError::Validation(
                "Material override contains duplicate materials".to_string(),
            ));
        }
    }

    for line in &mut summary.project.materials.lines {
        if let Some(markup_percentage) = overrides_by_material.get(&line.material_id.0) {
            line.price_markup_percentage = Some(*markup_percentage);
        }
    }

    Ok(())
}

fn resolve_invoice_pricing(
    requested_mode: Option<InvoicePricingMode>,
    requested_hourly_rate_cents: Option<i64>,
    requested_fixed_price_cents: Option<i64>,
    current_mode: Option<InvoicePricingMode>,
    current_hourly_rate_cents: Option<i64>,
    current_fixed_price_cents: Option<i64>,
) -> Result<ResolvedInvoicePricing, AppError> {
    validate_non_negative("Hourly rate", requested_hourly_rate_cents)?;
    validate_non_negative("Fixed price", requested_fixed_price_cents)?;

    let hourly_rate_cents = requested_hourly_rate_cents.or(current_hourly_rate_cents);
    let fixed_price_cents = requested_fixed_price_cents.or(current_fixed_price_cents);
    let mode = infer_invoice_pricing_mode(
        requested_mode.or(current_mode),
        hourly_rate_cents,
        fixed_price_cents,
    );

    match mode {
        Some(InvoicePricingMode::HourlyRate) if hourly_rate_cents.is_none() => Err(
            AppError::Validation("Hourly pricing requires an hourly rate".to_string()),
        ),
        Some(InvoicePricingMode::FixedPrice) if fixed_price_cents.is_none() => Err(
            AppError::Validation("Fixed-price billing requires a fixed price".to_string()),
        ),
        _ => Ok(ResolvedInvoicePricing {
            mode,
            hourly_rate_cents,
            fixed_price_cents,
        }),
    }
}

fn infer_invoice_pricing_mode(
    mode: Option<InvoicePricingMode>,
    hourly_rate_cents: Option<i64>,
    fixed_price_cents: Option<i64>,
) -> Option<InvoicePricingMode> {
    match mode {
        Some(mode) => Some(mode),
        None if fixed_price_cents.is_some() => Some(InvoicePricingMode::FixedPrice),
        None if hourly_rate_cents.is_some() => Some(InvoicePricingMode::HourlyRate),
        None => None,
    }
}

fn validate_non_negative(label: &str, value: Option<i64>) -> Result<(), AppError> {
    if let Some(value) = value {
        if value < 0 {
            return Err(AppError::Validation(format!("{label} cannot be negative")));
        }
    }

    Ok(())
}

fn normalize_optional_text(value: Option<&str>) -> Option<String> {
    value.and_then(|entry| {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn resolve_sender_name(
    request: &CreateProjectInvoiceRequest,
    billing_profile: &TenantBillingProfile,
) -> String {
    normalize_optional_text(request.sender_name.as_deref())
        .unwrap_or_else(|| billing_profile.sender_name.clone())
}

fn resolve_sender_address(
    request: &CreateProjectInvoiceRequest,
    billing_profile: &TenantBillingProfile,
) -> Option<String> {
    normalize_optional_text(request.sender_address.as_deref())
        .or_else(|| billing_profile.sender_address.clone())
}

fn invoice_totals(
    line_items: &[ProjectInvoiceLineItemResponse],
    billing_tax_mode: BillingTaxMode,
) -> InvoiceTotals {
    let subtotal_amount_cents = priced_subtotal_amount_cents(line_items);

    match (billing_tax_mode, subtotal_amount_cents) {
        (_, None) => InvoiceTotals {
            subtotal_amount_cents: None,
            vat_rate_percent: None,
            vat_amount_cents: None,
            gross_amount_cents: None,
            total_amount_cents: None,
        },
        (BillingTaxMode::Standard, Some(subtotal_amount_cents)) => {
            let vat_amount_cents = ((subtotal_amount_cents * 19) + 50) / 100;
            let gross_amount_cents = subtotal_amount_cents + vat_amount_cents;

            InvoiceTotals {
                subtotal_amount_cents: Some(subtotal_amount_cents),
                vat_rate_percent: Some(19),
                vat_amount_cents: Some(vat_amount_cents),
                gross_amount_cents: Some(gross_amount_cents),
                total_amount_cents: Some(gross_amount_cents),
            }
        }
        (BillingTaxMode::Kleinunternehmer, Some(subtotal_amount_cents)) => InvoiceTotals {
            subtotal_amount_cents: Some(subtotal_amount_cents),
            vat_rate_percent: None,
            vat_amount_cents: None,
            gross_amount_cents: Some(subtotal_amount_cents),
            total_amount_cents: Some(subtotal_amount_cents),
        },
    }
}

fn priced_subtotal_amount_cents(line_items: &[ProjectInvoiceLineItemResponse]) -> Option<i64> {
    let mut has_priced_lines = false;
    let subtotal: i64 = line_items
        .iter()
        .filter_map(|line| {
            if line.line_total_cents.is_some() {
                has_priced_lines = true;
            }
            line.line_total_cents
        })
        .sum();

    has_priced_lines.then_some(subtotal)
}

fn tax_note_for_mode(mode: BillingTaxMode) -> Option<String> {
    match mode {
        BillingTaxMode::Standard => None,
        BillingTaxMode::Kleinunternehmer => {
            Some("Gemäß § 19 UStG wird keine Umsatzsteuer berechnet.".to_string())
        }
    }
}

impl From<Invoice> for InvoiceResponse {
    fn from(invoice: Invoice) -> Self {
        Self {
            id: invoice.id.0,
            site_id: invoice.site_id.0,
            invoice_number: invoice.invoice_number,
            invoice_number_display: invoice.invoice_number_display,
            status: status_label(invoice.status),
            sender_name: invoice.sender_name,
            sender_address: invoice.sender_address,
            issued_at: invoice.issued_at,
            due_on: invoice.due_on,
            voided_at: invoice.voided_at,
            pdf_artifact: invoice.pdf_artifact.map(PdfArtifactResponse::from),
            created_by: invoice.created_by.map(|id| id.0),
            created_at: invoice.created_at,
            updated_at: invoice.updated_at,
        }
    }
}

impl From<ProjectInvoiceLineItemResponse> for InvoiceSnapshotLineItem {
    fn from(line: ProjectInvoiceLineItemResponse) -> Self {
        Self {
            source: line.source,
            description: line.description,
            quantity: line.quantity,
            unit: line.unit,
            source_count: line.source_count,
            priced: line.priced,
            unit_price_cents: line.unit_price_cents,
            line_total_cents: line.line_total_cents,
        }
    }
}

impl ProjectInvoiceDraftResponse {
    fn from_parts(invoice: Invoice, summary: InvoiceSummary) -> Self {
        let line_items = invoice_line_items(&summary);
        let snapshot = invoice.snapshot.clone();
        let snapshot = snapshot.as_ref();

        Self {
            invoice: InvoiceResponse::from(invoice),
            billing: SiteInvoiceBillingResponse::from(&summary.site),
            project: SiteInvoiceProjectResponse::from(summary.site),
            labor: ProjectLaborSummaryResponse::from(summary.project.labor),
            materials: ProjectMaterialSummaryResponse::from(summary.project.materials),
            billing_tax_mode: snapshot
                .and_then(|snapshot| snapshot.billing_tax_mode)
                .map(|mode| mode.to_string()),
            subtotal_amount_cents: snapshot.and_then(|snapshot| snapshot.subtotal_amount_cents),
            vat_rate_percent: snapshot.and_then(|snapshot| snapshot.vat_rate_percent),
            vat_amount_cents: snapshot.and_then(|snapshot| snapshot.vat_amount_cents),
            gross_amount_cents: snapshot.and_then(|snapshot| snapshot.gross_amount_cents),
            tax_note: snapshot.and_then(|snapshot| snapshot.tax_note.clone()),
            total_amount_cents: snapshot.and_then(|snapshot| snapshot.total_amount_cents),
            line_items,
        }
    }
}

impl From<PdfArtifact> for PdfArtifactResponse {
    fn from(artifact: PdfArtifact) -> Self {
        Self {
            storage_path: artifact.storage_path,
            sha256_hash: artifact.sha256_hash,
            content_type: artifact.content_type,
            size_bytes: artifact.size_bytes,
            created_at: artifact.created_at,
        }
    }
}

fn status_label(status: InvoiceStatus) -> String {
    status.as_str().to_string()
}

fn invoice_line_items(summary: &InvoiceSummary) -> Vec<ProjectInvoiceLineItemResponse> {
    let mut lines = Vec::new();
    let project = &summary.project;
    let pricing_mode = summary.site.invoice_pricing_mode;
    let hourly_rate_cents = summary.site.hourly_rate_cents;
    let fixed_price_cents = summary.site.fixed_price_cents;

    if pricing_mode == Some(InvoicePricingMode::FixedPrice) {
        if let Some(fixed_price_cents) = fixed_price_cents {
            lines.push(ProjectInvoiceLineItemResponse {
                source: "fixed_price".to_string(),
                description: format!("Pauschalpreis {}", summary.site.name),
                quantity: 1.0,
                unit: "project".to_string(),
                source_count: project.labor.entry_count,
                priced: true,
                unit_price_cents: Some(fixed_price_cents),
                line_total_cents: Some(fixed_price_cents),
            });
        }
    } else {
        let site_hours_priced =
            pricing_mode == Some(InvoicePricingMode::HourlyRate) && hourly_rate_cents.is_some();
        let workshop_hours_priced = site_hours_priced;

        if project.labor.site_hours > 0.0 {
            let line_total_cents = if site_hours_priced {
                hourly_rate_cents
                    .map(|rate| euro_cents_from_quantity(project.labor.site_hours, rate))
            } else {
                None
            };
            lines.push(ProjectInvoiceLineItemResponse {
                source: "labor_site".to_string(),
                description: "Baustellenarbeit".to_string(),
                quantity: project.labor.site_hours,
                unit: "hours".to_string(),
                source_count: project.labor.entry_count,
                priced: site_hours_priced,
                unit_price_cents: hourly_rate_cents.filter(|_| site_hours_priced),
                line_total_cents,
            });
        }

        if project.labor.workshop_hours > 0.0 {
            let line_total_cents = if workshop_hours_priced {
                hourly_rate_cents
                    .map(|rate| euro_cents_from_quantity(project.labor.workshop_hours, rate))
            } else {
                None
            };
            lines.push(ProjectInvoiceLineItemResponse {
                source: "labor_workshop".to_string(),
                description: "Werkstattarbeit".to_string(),
                quantity: project.labor.workshop_hours,
                unit: "hours".to_string(),
                source_count: project.labor.entry_count,
                priced: workshop_hours_priced,
                unit_price_cents: hourly_rate_cents.filter(|_| workshop_hours_priced),
                line_total_cents,
            });
        }
    }

    lines.extend(project.materials.lines.iter().map(|line| {
        let unit_price_cents =
            resolved_material_unit_price_cents(line.base_price_cents, line.price_markup_percentage);

        ProjectInvoiceLineItemResponse {
            source: "material".to_string(),
            description: format!("{} ({})", line.material_name, line.category_name),
            quantity: f64::from(line.total_withdrawn),
            unit: line.unit.clone(),
            source_count: line.withdrawal_count,
            priced: unit_price_cents.is_some(),
            unit_price_cents,
            line_total_cents: unit_price_cents.map(|unit_price_cents| {
                euro_cents_from_quantity(f64::from(line.total_withdrawn), unit_price_cents)
            }),
        }
    }));

    lines
}

fn euro_cents_from_quantity(quantity: f64, unit_price_cents: i64) -> i64 {
    ((quantity * (unit_price_cents as f64)).round()) as i64
}

fn resolved_material_unit_price_cents(
    base_price_cents: Option<i64>,
    price_markup_percentage: Option<i32>,
) -> Option<i64> {
    base_price_cents.map(|base_price_cents| {
        let markup_percentage = i64::from(price_markup_percentage.unwrap_or(0));
        ((base_price_cents * (100 + markup_percentage)) + 50) / 100
    })
}

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;
    use chrono::NaiveDate;
    use http::HeaderMap;
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::*;
    use crate::auth::jwks::JwksClient;
    use crate::common::types::{MaterialId, ProjectType, Role, SiteStatus, TenantId, UserId};
    use crate::config::AppConfig;
    use crate::modules::inventory::infrastructure::material_repository::{
        ProjectMaterialSummary, ProjectMaterialUsageLine,
    };
    use crate::modules::sites::application::site_service::ProjectSummary;
    use crate::modules::sites::domain::InvoicePricingMode;
    use crate::modules::sites::domain::Site;
    use crate::modules::sites::infrastructure::site_repository::ProjectLaborSummary;

    #[test]
    fn invoice_line_items_use_actuals_without_prices() {
        let summary = InvoiceSummary {
            site: test_site(None, None, None),
            project: ProjectSummary {
                labor: ProjectLaborSummary {
                    total_hours: 7.5,
                    entry_count: 3,
                    site_hours: 5.0,
                    workshop_hours: 2.5,
                    last_work_date: Some(NaiveDate::from_ymd_opt(2026, 5, 10).unwrap()),
                },
                materials: ProjectMaterialSummary {
                    distinct_material_count: 1,
                    withdrawal_count: 2,
                    lines: vec![ProjectMaterialUsageLine {
                        material_id: MaterialId(Uuid::new_v4()),
                        material_name: "Eiche Leimholz".to_string(),
                        category_name: "Platten".to_string(),
                        unit: "Stk".to_string(),
                        base_price_cents: None,
                        price_markup_percentage: None,
                        total_withdrawn: 4,
                        withdrawal_count: 2,
                        last_withdrawn_at: Utc::now(),
                    }],
                },
            },
        };

        let lines = invoice_line_items(&summary);

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].source, "labor_site");
        assert_eq!(lines[0].quantity, 5.0);
        assert_eq!(lines[1].source, "labor_workshop");
        assert_eq!(lines[2].source, "material");
        assert_eq!(lines[2].description, "Eiche Leimholz (Platten)");
        assert!(lines.iter().all(|line| !line.priced));
        assert!(lines.iter().all(|line| line.line_total_cents.is_none()));
    }

    #[test]
    fn invoice_line_items_price_materials_from_base_price_and_markup() {
        let summary = InvoiceSummary {
            site: test_site(None, None, None),
            project: ProjectSummary {
                labor: ProjectLaborSummary {
                    total_hours: 0.0,
                    entry_count: 0,
                    site_hours: 0.0,
                    workshop_hours: 0.0,
                    last_work_date: None,
                },
                materials: ProjectMaterialSummary {
                    distinct_material_count: 1,
                    withdrawal_count: 1,
                    lines: vec![ProjectMaterialUsageLine {
                        material_id: MaterialId(Uuid::new_v4()),
                        material_name: "Birke".to_string(),
                        category_name: "Platten".to_string(),
                        unit: "Stk".to_string(),
                        base_price_cents: Some(10_000),
                        price_markup_percentage: Some(15),
                        total_withdrawn: 2,
                        withdrawal_count: 1,
                        last_withdrawn_at: Utc::now(),
                    }],
                },
            },
        };

        let lines = invoice_line_items(&summary);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].unit_price_cents, Some(11_500));
        assert_eq!(lines[0].line_total_cents, Some(23_000));
        assert!(lines[0].priced);
    }

    #[test]
    fn invoice_line_items_price_labor_when_hourly_rate_mode_is_set() {
        let summary = InvoiceSummary {
            site: test_site(Some(InvoicePricingMode::HourlyRate), Some(8_500), None),
            project: ProjectSummary {
                labor: ProjectLaborSummary {
                    total_hours: 5.5,
                    entry_count: 2,
                    site_hours: 3.5,
                    workshop_hours: 2.0,
                    last_work_date: Some(NaiveDate::from_ymd_opt(2026, 5, 10).unwrap()),
                },
                materials: ProjectMaterialSummary {
                    distinct_material_count: 0,
                    withdrawal_count: 0,
                    lines: Vec::new(),
                },
            },
        };

        let lines = invoice_line_items(&summary);
        let totals = invoice_totals(&lines, BillingTaxMode::Standard);

        assert_eq!(lines.len(), 2);
        assert!(lines.iter().all(|line| line.priced));
        assert_eq!(lines[0].unit_price_cents, Some(8_500));
        assert_eq!(lines[0].line_total_cents, Some(29_750));
        assert_eq!(lines[1].line_total_cents, Some(17_000));
        assert_eq!(totals.subtotal_amount_cents, Some(46_750));
        assert_eq!(totals.vat_amount_cents, Some(8_883));
        assert_eq!(totals.total_amount_cents, Some(55_633));
    }

    #[test]
    fn invoice_line_items_use_fixed_price_line_when_mode_is_fixed_price() {
        let summary = InvoiceSummary {
            site: test_site(
                Some(InvoicePricingMode::FixedPrice),
                Some(8_500),
                Some(120_000),
            ),
            project: ProjectSummary {
                labor: ProjectLaborSummary {
                    total_hours: 5.5,
                    entry_count: 2,
                    site_hours: 3.5,
                    workshop_hours: 2.0,
                    last_work_date: Some(NaiveDate::from_ymd_opt(2026, 5, 10).unwrap()),
                },
                materials: ProjectMaterialSummary {
                    distinct_material_count: 0,
                    withdrawal_count: 0,
                    lines: Vec::new(),
                },
            },
        };

        let lines = invoice_line_items(&summary);
        let totals = invoice_totals(&lines, BillingTaxMode::Kleinunternehmer);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].source, "fixed_price");
        assert_eq!(lines[0].quantity, 1.0);
        assert_eq!(lines[0].unit_price_cents, Some(120_000));
        assert_eq!(lines[0].line_total_cents, Some(120_000));
        assert_eq!(totals.subtotal_amount_cents, Some(120_000));
        assert_eq!(totals.vat_amount_cents, None);
        assert_eq!(totals.total_amount_cents, Some(120_000));
    }

    #[test]
    fn draft_response_keeps_project_and_billing_context() {
        let site = Site {
            id: SiteId(Uuid::new_v4()),
            tenant_id: TenantId(Uuid::new_v4()),
            project_type: ProjectType::ExternalSite,
            name: "Kueche Meyer".to_string(),
            customer_name: "Meyer".to_string(),
            location: Some("Berlin".to_string()),
            description: None,
            status: SiteStatus::Active,
            start_date: None,
            end_date: None,
            estimated_days: Some(5),
            budget_amount_cents: Some(120_000),
            invoice_pricing_mode: Some(InvoicePricingMode::HourlyRate),
            hourly_rate_cents: Some(8_500),
            fixed_price_cents: None,
            billing_reference: Some("AB-42".to_string()),
            billing_notes: Some("Nach Aufwand pruefen".to_string()),
            quote_reference: Some("Q-42".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let invoice = Invoice {
            id: InvoiceId(Uuid::new_v4()),
            tenant_id: site.tenant_id,
            site_id: site.id,
            invoice_number: 1,
            invoice_number_display: "RE-00001".to_string(),
            status: InvoiceStatus::Draft,
            sender_name: None,
            sender_address: None,
            issued_at: None,
            due_on: None,
            voided_at: None,
            snapshot: Some(InvoiceSnapshot {
                project_name: site.name.clone(),
                customer_name: site.customer_name.clone(),
                project_location: site.location.clone(),
                invoice_pricing_mode: site.invoice_pricing_mode,
                hourly_rate_cents: site.hourly_rate_cents,
                fixed_price_cents: site.fixed_price_cents,
                billing_reference: site.billing_reference.clone(),
                billing_notes: site.billing_notes.clone(),
                quote_reference: site.quote_reference.clone(),
                budget_amount_cents: site.budget_amount_cents,
                labor_total_hours: 0.0,
                material_withdrawal_count: 0,
                billing_tax_mode: Some(BillingTaxMode::Standard),
                subtotal_amount_cents: None,
                vat_rate_percent: Some(19),
                vat_amount_cents: None,
                gross_amount_cents: None,
                tax_note: None,
                total_amount_cents: None,
                line_items: Vec::new(),
            }),
            pdf_artifact: None,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let summary = InvoiceSummary {
            site,
            project: ProjectSummary {
                labor: ProjectLaborSummary {
                    total_hours: 0.0,
                    entry_count: 0,
                    site_hours: 0.0,
                    workshop_hours: 0.0,
                    last_work_date: None,
                },
                materials: ProjectMaterialSummary {
                    distinct_material_count: 0,
                    withdrawal_count: 0,
                    lines: Vec::new(),
                },
            },
        };

        let response = ProjectInvoiceDraftResponse::from_parts(invoice, summary);

        assert_eq!(response.invoice.status, "draft");
        assert_eq!(response.project.name, "Kueche Meyer");
        assert_eq!(response.billing.budget_amount_cents, Some(120_000));
        assert_eq!(
            response.billing.invoice_pricing_mode.as_deref(),
            Some("hourly_rate")
        );
        assert_eq!(response.billing_tax_mode.as_deref(), Some("standard"));
        assert_eq!(response.total_amount_cents, None);
        assert_eq!(response.line_items.len(), 0);
    }

    #[sqlx::test]
    async fn create_project_invoice_rejects_non_admin(pool: PgPool) {
        let tenant_id = create_tenant(&pool, "Tenant A").await;
        let user = create_user(&pool, tenant_id, Role::Employee).await;
        let site_id = create_site(&pool, tenant_id, "Project A").await;

        let result = create_project_invoice(
            State(test_state(pool)),
            tenant_context(tenant_id, user.auth_user_id, &user.email, Role::Employee),
            Path(site_id.0),
            Json(empty_create_request()),
        )
        .await;

        assert!(matches!(result, Err(AppError::Forbidden(_))));
    }

    #[sqlx::test]
    async fn create_project_invoice_rejects_cross_tenant_site(pool: PgPool) {
        let tenant_a = create_tenant(&pool, "Tenant A").await;
        let tenant_b = create_tenant(&pool, "Tenant B").await;
        let user = create_user(&pool, tenant_a, Role::Admin).await;
        let other_site_id = create_site(&pool, tenant_b, "Other Project").await;

        let result = create_project_invoice(
            State(test_state(pool.clone())),
            tenant_context(tenant_a, user.auth_user_id, &user.email, Role::Admin),
            Path(other_site_id.0),
            Json(empty_create_request()),
        )
        .await;
        let invoice_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM invoices")
            .fetch_one(&pool)
            .await
            .expect("invoice count should be readable");

        assert!(matches!(result, Err(AppError::NotFound(_))));
        assert_eq!(invoice_count, 0);
    }

    #[sqlx::test]
    async fn create_project_invoice_returns_draft_with_actual_lines(pool: PgPool) {
        let tenant_id = create_tenant(&pool, "Tenant A").await;
        let user = create_user(&pool, tenant_id, Role::Admin).await;
        let site_id = create_site(&pool, tenant_id, "Project A").await;
        create_time_entry(&pool, tenant_id, site_id, user.local_id, "site", 3.5).await;
        create_time_entry(&pool, tenant_id, site_id, user.local_id, "workshop", 2.0).await;
        create_material_withdrawal(&pool, tenant_id, site_id, user.local_id).await;

        let response = create_project_invoice(
            State(test_state(pool)),
            tenant_context(tenant_id, user.auth_user_id, &user.email, Role::Admin),
            Path(site_id.0),
            Json(CreateProjectInvoiceRequest {
                sender_name: Some(" Schreinerei ".to_string()),
                sender_address: Some("Werkstrasse 1".to_string()),
                invoice_pricing_mode: None,
                hourly_rate_cents: None,
                fixed_price_cents: None,
                material_overrides: None,
            }),
        )
        .await
        .expect("admin should create project invoice draft")
        .0;

        assert_eq!(response.invoice.site_id, site_id.0);
        assert_eq!(response.invoice.invoice_number_display, "RE-00001");
        assert_eq!(response.invoice.status, "generated");
        assert_eq!(response.invoice.created_by, Some(user.local_id.0));
        assert!(response.invoice.pdf_artifact.is_some());
        assert_eq!(response.project.name, "Project A");
        assert_eq!(response.labor.total_hours, 5.5);
        assert_eq!(response.materials.withdrawal_count, 1);
        assert_eq!(response.line_items.len(), 3);
        assert!(response.line_items.iter().all(|line| !line.priced));
        assert_eq!(response.billing_tax_mode.as_deref(), Some("standard"));
        assert_eq!(response.total_amount_cents, None);
    }

    #[sqlx::test]
    async fn create_project_invoice_prices_labor_when_project_uses_hourly_rate(pool: PgPool) {
        let tenant_id = create_tenant(&pool, "Tenant A").await;
        let user = create_user(&pool, tenant_id, Role::Admin).await;
        let site_id = create_site_with_billing(
            &pool,
            tenant_id,
            "Project A",
            Some("hourly_rate"),
            Some(8_500),
            None,
        )
        .await;
        create_time_entry(&pool, tenant_id, site_id, user.local_id, "site", 3.5).await;
        create_time_entry(&pool, tenant_id, site_id, user.local_id, "workshop", 2.0).await;

        let response = create_project_invoice(
            State(test_state(pool)),
            tenant_context(tenant_id, user.auth_user_id, &user.email, Role::Admin),
            Path(site_id.0),
            Json(empty_create_request()),
        )
        .await
        .expect("admin should create project invoice draft")
        .0;

        assert_eq!(
            response.billing.invoice_pricing_mode.as_deref(),
            Some("hourly_rate")
        );
        assert_eq!(response.subtotal_amount_cents, Some(46_750));
        assert_eq!(response.vat_amount_cents, Some(8_883));
        assert_eq!(response.gross_amount_cents, Some(55_633));
        assert_eq!(response.total_amount_cents, Some(55_633));
        assert_eq!(response.line_items[0].unit_price_cents, Some(8_500));
        assert_eq!(response.line_items[0].line_total_cents, Some(29_750));
        assert_eq!(response.line_items[1].line_total_cents, Some(17_000));
    }

    #[sqlx::test]
    async fn create_project_invoice_uses_fixed_price_line_when_project_is_fixed_price(
        pool: PgPool,
    ) {
        let tenant_id = create_tenant(&pool, "Tenant A").await;
        let user = create_user(&pool, tenant_id, Role::Admin).await;
        let site_id = create_site_with_billing(
            &pool,
            tenant_id,
            "Project A",
            Some("fixed_price"),
            Some(8_500),
            Some(120_000),
        )
        .await;
        create_time_entry(&pool, tenant_id, site_id, user.local_id, "site", 3.5).await;

        let response = create_project_invoice(
            State(test_state(pool)),
            tenant_context(tenant_id, user.auth_user_id, &user.email, Role::Admin),
            Path(site_id.0),
            Json(empty_create_request()),
        )
        .await
        .expect("admin should create project invoice draft")
        .0;

        assert_eq!(response.billing_tax_mode.as_deref(), Some("standard"));
        assert_eq!(response.subtotal_amount_cents, Some(120_000));
        assert_eq!(response.vat_amount_cents, Some(22_800));
        assert_eq!(response.total_amount_cents, Some(142_800));
        assert_eq!(response.line_items[0].source, "fixed_price");
        assert_eq!(response.line_items[0].unit_price_cents, Some(120_000));
        assert_eq!(response.line_items[0].line_total_cents, Some(120_000));
    }

    #[sqlx::test]
    async fn create_project_invoice_allows_one_off_pricing_override_without_mutating_site(
        pool: PgPool,
    ) {
        let tenant_id = create_tenant(&pool, "Tenant A").await;
        let user = create_user(&pool, tenant_id, Role::Admin).await;
        let site_id = create_site_with_billing(
            &pool,
            tenant_id,
            "Project A",
            Some("hourly_rate"),
            Some(8_500),
            None,
        )
        .await;
        create_time_entry(&pool, tenant_id, site_id, user.local_id, "site", 3.5).await;

        let response = create_project_invoice(
            State(test_state(pool.clone())),
            tenant_context(tenant_id, user.auth_user_id, &user.email, Role::Admin),
            Path(site_id.0),
            Json(CreateProjectInvoiceRequest {
                sender_name: None,
                sender_address: None,
                invoice_pricing_mode: Some("fixed_price".to_string()),
                hourly_rate_cents: None,
                fixed_price_cents: Some(150_000),
                material_overrides: None,
            }),
        )
        .await
        .expect("override should create invoice draft")
        .0;

        let stored_mode: Option<String> =
            sqlx::query_scalar("SELECT invoice_pricing_mode FROM sites WHERE id = $1")
                .bind(site_id.0)
                .fetch_one(&pool)
                .await
                .expect("site pricing mode should be readable");

        assert_eq!(
            response.billing.invoice_pricing_mode.as_deref(),
            Some("fixed_price")
        );
        assert_eq!(response.subtotal_amount_cents, Some(150_000));
        assert_eq!(response.vat_amount_cents, Some(28_500));
        assert_eq!(response.total_amount_cents, Some(178_500));
        assert_eq!(stored_mode.as_deref(), Some("hourly_rate"));
    }

    #[sqlx::test]
    async fn create_project_invoice_rejects_invalid_override_combinations(pool: PgPool) {
        let tenant_id = create_tenant(&pool, "Tenant A").await;
        let user = create_user(&pool, tenant_id, Role::Admin).await;
        let site_id = create_site(&pool, tenant_id, "Project A").await;

        let result = create_project_invoice(
            State(test_state(pool)),
            tenant_context(tenant_id, user.auth_user_id, &user.email, Role::Admin),
            Path(site_id.0),
            Json(CreateProjectInvoiceRequest {
                sender_name: None,
                sender_address: None,
                invoice_pricing_mode: Some("fixed_price".to_string()),
                hourly_rate_cents: None,
                fixed_price_cents: None,
                material_overrides: None,
            }),
        )
        .await;

        assert!(
            matches!(result, Err(AppError::Validation(message)) if message == "Fixed-price billing requires a fixed price")
        );
    }

    #[sqlx::test]
    async fn download_invoice_pdf_returns_pdf_and_marks_generated(pool: PgPool) {
        let tenant_id = create_tenant(&pool, "Tenant A").await;
        let user = create_user(&pool, tenant_id, Role::Admin).await;
        let site_id = create_site(&pool, tenant_id, "Project A").await;

        let draft = create_project_invoice(
            State(test_state(pool.clone())),
            tenant_context(tenant_id, user.auth_user_id, &user.email, Role::Admin),
            Path(site_id.0),
            Json(empty_create_request()),
        )
        .await
        .expect("admin should create invoice")
        .0;

        let response = download_invoice_pdf(
            State(test_state(pool.clone())),
            tenant_context(tenant_id, user.auth_user_id, &user.email, Role::Admin),
            Path(draft.invoice.id),
        )
        .await
        .expect("admin should download invoice pdf")
        .into_response();
        let headers = response.headers().clone();
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("pdf body should be readable");
        let status: String = sqlx::query_scalar("SELECT status FROM invoices WHERE id = $1")
            .bind(draft.invoice.id)
            .fetch_one(&pool)
            .await
            .expect("invoice status should be readable");
        let pdf_size: i64 = sqlx::query_scalar("SELECT pdf_size_bytes FROM invoices WHERE id = $1")
            .bind(draft.invoice.id)
            .fetch_one(&pool)
            .await
            .expect("pdf metadata should be readable");

        assert!(bytes.starts_with(b"%PDF-"));
        assert_eq!(
            header_value(&headers, header::CONTENT_TYPE),
            "application/pdf"
        );
        assert_eq!(
            header_value(&headers, header::CONTENT_DISPOSITION),
            "attachment; filename=\"RE-00001.pdf\""
        );
        assert_eq!(status, "generated");
        assert_eq!(pdf_size, bytes.len() as i64);
    }

    #[sqlx::test]
    async fn download_invoice_pdf_rejects_non_admin(pool: PgPool) {
        let tenant_id = create_tenant(&pool, "Tenant A").await;
        let admin = create_user(&pool, tenant_id, Role::Admin).await;
        let employee = create_user(&pool, tenant_id, Role::Employee).await;
        let site_id = create_site(&pool, tenant_id, "Project A").await;
        let draft = create_project_invoice(
            State(test_state(pool.clone())),
            tenant_context(tenant_id, admin.auth_user_id, &admin.email, Role::Admin),
            Path(site_id.0),
            Json(empty_create_request()),
        )
        .await
        .expect("admin should create invoice")
        .0;

        let result = download_invoice_pdf(
            State(test_state(pool)),
            tenant_context(
                tenant_id,
                employee.auth_user_id,
                &employee.email,
                Role::Employee,
            ),
            Path(draft.invoice.id),
        )
        .await;

        assert!(matches!(result, Err(AppError::Forbidden(_))));
    }

    #[sqlx::test]
    async fn download_invoice_pdf_rejects_cross_tenant_invoice(pool: PgPool) {
        let tenant_a = create_tenant(&pool, "Tenant A").await;
        let tenant_b = create_tenant(&pool, "Tenant B").await;
        let admin_a = create_user(&pool, tenant_a, Role::Admin).await;
        let admin_b = create_user(&pool, tenant_b, Role::Admin).await;
        let site_a = create_site(&pool, tenant_a, "Project A").await;
        let draft = create_project_invoice(
            State(test_state(pool.clone())),
            tenant_context(tenant_a, admin_a.auth_user_id, &admin_a.email, Role::Admin),
            Path(site_a.0),
            Json(empty_create_request()),
        )
        .await
        .expect("tenant a should create invoice")
        .0;

        let result = download_invoice_pdf(
            State(test_state(pool)),
            tenant_context(tenant_b, admin_b.auth_user_id, &admin_b.email, Role::Admin),
            Path(draft.invoice.id),
        )
        .await;

        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    fn empty_create_request() -> CreateProjectInvoiceRequest {
        CreateProjectInvoiceRequest {
            sender_name: None,
            sender_address: None,
            invoice_pricing_mode: None,
            hourly_rate_cents: None,
            fixed_price_cents: None,
            material_overrides: None,
        }
    }

    fn tenant_context(
        tenant_id: TenantId,
        user_id: UserId,
        email: &str,
        role: Role,
    ) -> TenantContext {
        TenantContext {
            tenant_id,
            user_id,
            email: email.to_string(),
            roles: vec![role],
        }
    }

    struct TestUser {
        local_id: UserId,
        auth_user_id: UserId,
        email: String,
    }

    fn test_state(pool: PgPool) -> AppState {
        AppState {
            config: AppConfig {
                database_url: String::new(),
                keycloak_url: "http://localhost".to_string(),
                keycloak_realm: "schreinerei".to_string(),
                jwt_issuer: "http://localhost/realms/schreinerei".to_string(),
                run_migrations: false,
                host: "127.0.0.1".to_string(),
                port: 0,
                mollie_api_key: None,
                mollie_api_base_url: "http://localhost".to_string(),
                mollie_onboarding_amount_value: "29.00".to_string(),
                mollie_onboarding_amount_currency: "EUR".to_string(),
                app_public_url: "http://localhost:3000".to_string(),
                frontend_public_url: "http://localhost:5173".to_string(),
                keycloak_admin_client_id: None,
                keycloak_admin_client_secret: None,
                keycloak_admin_realm: None,
                keycloak_organization_invite_ttl_seconds: 604800,
            },
            pool,
            jwks_client: JwksClient::new("http://localhost", "schreinerei"),
        }
    }

    fn header_value(headers: &HeaderMap, name: header::HeaderName) -> &str {
        headers
            .get(name)
            .expect("header should exist")
            .to_str()
            .expect("header should be valid text")
    }

    async fn create_tenant(pool: &PgPool, name: &str) -> TenantId {
        let id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO tenants (id, keycloak_realm, name, slug, keycloak_organization_alias)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(id)
        .bind(format!("realm-{id}"))
        .bind(name)
        .bind(format!("slug-{id}"))
        .bind(format!("alias-{id}"))
        .execute(pool)
        .await
        .expect("tenant should be inserted");

        TenantId(id)
    }

    async fn create_user(pool: &PgPool, tenant_id: TenantId, role: Role) -> TestUser {
        let local_id = UserId(Uuid::new_v4());
        let auth_user_id = UserId(Uuid::new_v4());
        let email = format!("{auth_user_id}@example.test");
        sqlx::query(
            r#"
            INSERT INTO users (id, tenant_id, keycloak_user_id, email, role)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(local_id.0)
        .bind(tenant_id.0)
        .bind(auth_user_id.to_string())
        .bind(&email)
        .bind(role.to_string())
        .execute(pool)
        .await
        .expect("user should be inserted");

        TestUser {
            local_id,
            auth_user_id,
            email,
        }
    }

    async fn create_site(pool: &PgPool, tenant_id: TenantId, name: &str) -> SiteId {
        create_site_with_billing(pool, tenant_id, name, None, None, None).await
    }

    async fn create_site_with_billing(
        pool: &PgPool,
        tenant_id: TenantId,
        name: &str,
        invoice_pricing_mode: Option<&str>,
        hourly_rate_cents: Option<i64>,
        fixed_price_cents: Option<i64>,
    ) -> SiteId {
        let id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO sites (
                id, tenant_id, project_type, name, customer_name, status,
                budget_amount_cents, invoice_pricing_mode, hourly_rate_cents, fixed_price_cents,
                billing_reference, billing_notes, quote_reference
            )
            VALUES ($1, $2, 'external_site', $3, 'Customer', 'active', $4, $5, $6, $7, 'BR-1', 'Billing note', 'Q-1')
            "#,
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(name)
        .bind(120000_i64)
        .bind(invoice_pricing_mode)
        .bind(hourly_rate_cents)
        .bind(fixed_price_cents)
        .execute(pool)
        .await
        .expect("site should be inserted");

        SiteId(id)
    }

    fn test_site(
        invoice_pricing_mode: Option<InvoicePricingMode>,
        hourly_rate_cents: Option<i64>,
        fixed_price_cents: Option<i64>,
    ) -> Site {
        Site {
            id: SiteId(Uuid::new_v4()),
            tenant_id: TenantId(Uuid::new_v4()),
            project_type: ProjectType::ExternalSite,
            name: "Project A".to_string(),
            customer_name: "Customer".to_string(),
            location: Some("Berlin".to_string()),
            description: None,
            status: SiteStatus::Active,
            start_date: None,
            end_date: None,
            estimated_days: Some(5),
            budget_amount_cents: Some(120_000),
            invoice_pricing_mode,
            hourly_rate_cents,
            fixed_price_cents,
            billing_reference: Some("BR-1".to_string()),
            billing_notes: Some("Billing note".to_string()),
            quote_reference: Some("Q-1".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    async fn create_time_entry(
        pool: &PgPool,
        tenant_id: TenantId,
        site_id: SiteId,
        user_id: UserId,
        work_type: &str,
        hours: f64,
    ) {
        sqlx::query(
            r#"
            INSERT INTO time_entries (id, tenant_id, site_id, user_id, work_type, hours, work_date)
            VALUES ($1, $2, $3, $4, $5, $6, '2026-05-10')
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id.0)
        .bind(site_id.0)
        .bind(user_id.0)
        .bind(work_type)
        .bind(hours)
        .execute(pool)
        .await
        .expect("time entry should be inserted");
    }

    async fn create_material_withdrawal(
        pool: &PgPool,
        tenant_id: TenantId,
        site_id: SiteId,
        user_id: UserId,
    ) {
        let category_id = Uuid::new_v4();
        let material_id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO categories (id, tenant_id, name)
            VALUES ($1, $2, 'Platten')
            "#,
        )
        .bind(category_id)
        .bind(tenant_id.0)
        .execute(pool)
        .await
        .expect("category should be inserted");

        sqlx::query(
            r#"
            INSERT INTO materials (id, tenant_id, category_id, name, unit, quantity, min_quantity)
            VALUES ($1, $2, $3, 'Eiche Leimholz', 'Stk', 10, 1)
            "#,
        )
        .bind(material_id)
        .bind(tenant_id.0)
        .bind(category_id)
        .execute(pool)
        .await
        .expect("material should be inserted");

        sqlx::query(
            r#"
            INSERT INTO stock_entries (
                id, tenant_id, material_id, user_id, quantity_change,
                quantity_after, site_id, entry_type
            )
            VALUES ($1, $2, $3, $4, -4, 6, $5, 'withdrawn')
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id.0)
        .bind(material_id)
        .bind(user_id.0)
        .bind(site_id.0)
        .execute(pool)
        .await
        .expect("stock entry should be inserted");
    }
}
