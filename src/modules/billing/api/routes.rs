use axum::{
    extract::{Path, State},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{InvoiceId, SiteId};
use crate::modules::billing::application::BillingService;
use crate::modules::billing::domain::{
    Invoice, InvoiceSnapshot, InvoiceSnapshotLineItem, InvoiceStatus, PdfArtifact,
};
use crate::modules::billing::infrastructure::InvoiceRepository;
use crate::modules::billing::pdf::{generate_invoice_pdf, invoice_pdf_filename};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::sites::api::routes::{
    ProjectLaborSummaryResponse, ProjectMaterialSummaryResponse, SiteInvoiceBillingResponse,
    SiteInvoiceProjectResponse,
};
use crate::modules::sites::application::site_service::{InvoiceSummary, ProjectSummary};
use crate::modules::sites::infrastructure::site_repository::SiteRepository;
use crate::AppState;

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
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct ProjectInvoiceDraftResponse {
    pub invoice: InvoiceResponse,
    pub project: SiteInvoiceProjectResponse,
    pub billing: SiteInvoiceBillingResponse,
    pub labor: ProjectLaborSummaryResponse,
    pub materials: ProjectMaterialSummaryResponse,
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
    let service = BillingService::new(InvoiceRepository::new(state.pool));
    let snapshot = invoice_snapshot(&invoice_summary);
    let draft = service
        .create_draft_invoice(
            ctx.tenant_id,
            crate::modules::billing::domain::CreateInvoiceDraft {
                site_id,
                sender_name: request.sender_name,
                sender_address: request.sender_address,
                snapshot,
                created_by: Some(ctx.user_id),
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

fn invoice_snapshot(summary: &InvoiceSummary) -> InvoiceSnapshot {
    InvoiceSnapshot {
        project_name: summary.site.name.clone(),
        customer_name: summary.site.customer_name.clone(),
        project_location: summary.site.location.clone(),
        billing_reference: summary.site.billing_reference.clone(),
        billing_notes: summary.site.billing_notes.clone(),
        quote_reference: summary.site.quote_reference.clone(),
        budget_amount_cents: summary.site.budget_amount_cents,
        labor_total_hours: summary.project.labor.total_hours,
        material_withdrawal_count: summary.project.materials.withdrawal_count,
        line_items: invoice_line_items(&summary.project)
            .into_iter()
            .map(InvoiceSnapshotLineItem::from)
            .collect(),
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
        }
    }
}

impl ProjectInvoiceDraftResponse {
    fn from_parts(invoice: Invoice, summary: InvoiceSummary) -> Self {
        let line_items = invoice_line_items(&summary.project);

        Self {
            invoice: InvoiceResponse::from(invoice),
            billing: SiteInvoiceBillingResponse::from(&summary.site),
            project: SiteInvoiceProjectResponse::from(summary.site),
            labor: ProjectLaborSummaryResponse::from(summary.project.labor),
            materials: ProjectMaterialSummaryResponse::from(summary.project.materials),
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

fn invoice_line_items(summary: &ProjectSummary) -> Vec<ProjectInvoiceLineItemResponse> {
    let mut lines = Vec::new();

    if summary.labor.site_hours > 0.0 {
        lines.push(ProjectInvoiceLineItemResponse {
            source: "labor_site".to_string(),
            description: "Baustellenarbeit".to_string(),
            quantity: summary.labor.site_hours,
            unit: "hours".to_string(),
            source_count: summary.labor.entry_count,
            priced: false,
        });
    }

    if summary.labor.workshop_hours > 0.0 {
        lines.push(ProjectInvoiceLineItemResponse {
            source: "labor_workshop".to_string(),
            description: "Werkstattarbeit".to_string(),
            quantity: summary.labor.workshop_hours,
            unit: "hours".to_string(),
            source_count: summary.labor.entry_count,
            priced: false,
        });
    }

    lines.extend(
        summary
            .materials
            .lines
            .iter()
            .map(|line| ProjectInvoiceLineItemResponse {
                source: "material".to_string(),
                description: format!("{} ({})", line.material_name, line.category_name),
                quantity: f64::from(line.total_withdrawn),
                unit: line.unit.clone(),
                source_count: line.withdrawal_count,
                priced: false,
            }),
    );

    lines
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
    use crate::modules::sites::domain::Site;
    use crate::modules::sites::infrastructure::site_repository::ProjectLaborSummary;

    #[test]
    fn invoice_line_items_use_actuals_without_prices() {
        let summary = ProjectSummary {
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
                    total_withdrawn: 4,
                    withdrawal_count: 2,
                    last_withdrawn_at: Utc::now(),
                }],
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
                billing_reference: site.billing_reference.clone(),
                billing_notes: site.billing_notes.clone(),
                quote_reference: site.quote_reference.clone(),
                budget_amount_cents: site.budget_amount_cents,
                labor_total_hours: 0.0,
                material_withdrawal_count: 0,
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
        assert_eq!(response.line_items.len(), 0);
    }

    #[sqlx::test]
    async fn create_project_invoice_rejects_non_admin(pool: PgPool) {
        let tenant_id = create_tenant(&pool, "Tenant A").await;
        let user_id = create_user(&pool, tenant_id, Role::Employee).await;
        let site_id = create_site(&pool, tenant_id, "Project A").await;

        let result = create_project_invoice(
            State(test_state(pool)),
            tenant_context(tenant_id, user_id, Role::Employee),
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
        let user_id = create_user(&pool, tenant_a, Role::Admin).await;
        let other_site_id = create_site(&pool, tenant_b, "Other Project").await;

        let result = create_project_invoice(
            State(test_state(pool.clone())),
            tenant_context(tenant_a, user_id, Role::Admin),
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
        let user_id = create_user(&pool, tenant_id, Role::Admin).await;
        let site_id = create_site(&pool, tenant_id, "Project A").await;
        create_time_entry(&pool, tenant_id, site_id, user_id, "site", 3.5).await;
        create_time_entry(&pool, tenant_id, site_id, user_id, "workshop", 2.0).await;
        create_material_withdrawal(&pool, tenant_id, site_id, user_id).await;

        let response = create_project_invoice(
            State(test_state(pool)),
            tenant_context(tenant_id, user_id, Role::Admin),
            Path(site_id.0),
            Json(CreateProjectInvoiceRequest {
                sender_name: Some(" Schreinerei ".to_string()),
                sender_address: Some("Werkstrasse 1".to_string()),
            }),
        )
        .await
        .expect("admin should create project invoice draft")
        .0;

        assert_eq!(response.invoice.site_id, site_id.0);
        assert_eq!(response.invoice.invoice_number_display, "RE-00001");
        assert_eq!(response.invoice.status, "generated");
        assert_eq!(response.invoice.created_by, Some(user_id.0));
        assert!(response.invoice.pdf_artifact.is_some());
        assert_eq!(response.project.name, "Project A");
        assert_eq!(response.labor.total_hours, 5.5);
        assert_eq!(response.materials.withdrawal_count, 1);
        assert_eq!(response.line_items.len(), 3);
        assert!(response.line_items.iter().all(|line| !line.priced));
    }

    #[sqlx::test]
    async fn download_invoice_pdf_returns_pdf_and_marks_generated(pool: PgPool) {
        let tenant_id = create_tenant(&pool, "Tenant A").await;
        let user_id = create_user(&pool, tenant_id, Role::Admin).await;
        let site_id = create_site(&pool, tenant_id, "Project A").await;

        let draft = create_project_invoice(
            State(test_state(pool.clone())),
            tenant_context(tenant_id, user_id, Role::Admin),
            Path(site_id.0),
            Json(empty_create_request()),
        )
        .await
        .expect("admin should create invoice")
        .0;

        let response = download_invoice_pdf(
            State(test_state(pool.clone())),
            tenant_context(tenant_id, user_id, Role::Admin),
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

        assert!(bytes.starts_with(b"%PDF-1.4"));
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
        let admin_id = create_user(&pool, tenant_id, Role::Admin).await;
        let employee_id = create_user(&pool, tenant_id, Role::Employee).await;
        let site_id = create_site(&pool, tenant_id, "Project A").await;
        let draft = create_project_invoice(
            State(test_state(pool.clone())),
            tenant_context(tenant_id, admin_id, Role::Admin),
            Path(site_id.0),
            Json(empty_create_request()),
        )
        .await
        .expect("admin should create invoice")
        .0;

        let result = download_invoice_pdf(
            State(test_state(pool)),
            tenant_context(tenant_id, employee_id, Role::Employee),
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
            tenant_context(tenant_a, admin_a, Role::Admin),
            Path(site_a.0),
            Json(empty_create_request()),
        )
        .await
        .expect("tenant a should create invoice")
        .0;

        let result = download_invoice_pdf(
            State(test_state(pool)),
            tenant_context(tenant_b, admin_b, Role::Admin),
            Path(draft.invoice.id),
        )
        .await;

        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    fn empty_create_request() -> CreateProjectInvoiceRequest {
        CreateProjectInvoiceRequest {
            sender_name: None,
            sender_address: None,
        }
    }

    fn tenant_context(tenant_id: TenantId, user_id: UserId, role: Role) -> TenantContext {
        TenantContext {
            tenant_id,
            user_id,
            email: "user@example.test".to_string(),
            roles: vec![role],
        }
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

    async fn create_user(pool: &PgPool, tenant_id: TenantId, role: Role) -> UserId {
        let id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO users (id, tenant_id, keycloak_user_id, email, role)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(format!("keycloak-{id}"))
        .bind(format!("{id}@example.test"))
        .bind(role.to_string())
        .execute(pool)
        .await
        .expect("user should be inserted");

        UserId(id)
    }

    async fn create_site(pool: &PgPool, tenant_id: TenantId, name: &str) -> SiteId {
        let id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO sites (
                id, tenant_id, project_type, name, customer_name, status,
                budget_amount_cents, billing_reference, billing_notes, quote_reference
            )
            VALUES ($1, $2, 'external_site', $3, 'Customer', 'active', 120000, 'BR-1', 'Billing note', 'Q-1')
            "#,
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(name)
        .execute(pool)
        .await
        .expect("site should be inserted");

        SiteId(id)
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
