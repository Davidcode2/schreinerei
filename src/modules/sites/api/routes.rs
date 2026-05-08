use axum::{
    extract::{DefaultBodyLimit, Multipart, Path, Query, State},
    http::header,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{
    AssignmentRole, ProjectType, SiteId, SiteStatus, TimeEntryId, UserId, WorkType,
};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::sites::application::site_service::SiteService;
use crate::modules::sites::domain::{
    ActivityType, AssignUser, CreateActivity, CreateSite, CreateTimeEntry, UpdateSite,
    UpdateTimeEntry,
};
use crate::modules::sites::infrastructure::site_repository::DashboardSite;
use crate::AppState;

const MAX_UPLOAD_REQUEST_SIZE_BYTES: usize = 12 * 1024 * 1024;

/// Create the sites API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Sites
        .route("/api/v1/sites", get(list_sites).post(create_site))
        .route(
            "/api/v1/sites/history-report",
            get(list_site_history_report),
        )
        .route(
            "/api/v1/sites/{id}",
            get(get_site).patch(update_site).delete(delete_site),
        )
        .route("/api/v1/sites/{id}/summary", get(get_site_summary))
        .route(
            "/api/v1/sites/{id}/invoice-summary",
            get(get_site_invoice_summary),
        )
        // Assignments
        .route("/api/v1/sites/{id}/assign", post(assign_user))
        .route(
            "/api/v1/sites/{id}/assign/{user_id}",
            delete(remove_assignment),
        )
        .route("/api/v1/sites/{id}/assignments", get(list_assignments))
        // Time entries
        .route(
            "/api/v1/sites/{id}/time-entries",
            get(list_site_time_entries),
        )
        .route("/api/v1/time-entries", post(create_time_entry))
        .route("/api/v1/time-entries/my", get(list_my_time_entries))
        .route(
            "/api/v1/time-entries/{id}",
            get(get_time_entry)
                .patch(update_time_entry)
                .delete(delete_time_entry),
        )
        // Activities
        .route(
            "/api/v1/sites/{id}/activities",
            get(list_activities).post(create_activity),
        )
        .route(
            "/api/v1/sites/{id}/activities/{activity_id}",
            delete(delete_activity),
        )
        .route(
            "/api/v1/sites/{id}/attachments",
            post(upload_site_attachment)
                .layer(DefaultBodyLimit::max(MAX_UPLOAD_REQUEST_SIZE_BYTES)),
        )
        .route(
            "/api/v1/sites/{id}/attachments/photo",
            post(upload_site_photo_attachment)
                .layer(DefaultBodyLimit::max(MAX_UPLOAD_REQUEST_SIZE_BYTES)),
        )
        .route(
            "/api/v1/attachments/{attachment_id}",
            get(get_attachment_bytes),
        )
        .route(
            "/api/v1/attachments/{attachment_id}/thumbnail",
            get(get_attachment_thumbnail_bytes),
        )
        // Dashboard
        .route("/api/v1/dashboard/sites", get(get_dashboard))
}

// === DTOs ===

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct SiteResponse {
    pub id: String,
    pub project_type: String,
    pub name: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub estimated_days: Option<i32>,
    pub budget_amount_cents: Option<i64>,
    pub billing_reference: Option<String>,
    pub billing_notes: Option<String>,
    pub quote_reference: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct ProjectMaterialUsageLineResponse {
    pub material_id: String,
    pub material_name: String,
    pub category_name: String,
    pub unit: String,
    pub total_withdrawn: i32,
    pub withdrawal_count: i64,
    pub last_withdrawn_at: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct ProjectMaterialSummaryResponse {
    pub distinct_material_count: i64,
    pub withdrawal_count: i64,
    pub lines: Vec<ProjectMaterialUsageLineResponse>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct ProjectLaborSummaryResponse {
    pub total_hours: f64,
    pub entry_count: i64,
    pub site_hours: f64,
    pub workshop_hours: f64,
    pub last_work_date: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct SiteProjectSummaryResponse {
    pub labor: ProjectLaborSummaryResponse,
    pub materials: ProjectMaterialSummaryResponse,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct SiteInvoiceProjectResponse {
    pub id: String,
    pub name: String,
    pub project_type: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub status: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub estimated_days: Option<i32>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct SiteInvoiceBillingResponse {
    pub budget_amount_cents: Option<i64>,
    pub quote_reference: Option<String>,
    pub billing_reference: Option<String>,
    pub billing_notes: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct SiteInvoiceSummaryResponse {
    pub export_version: String,
    pub generated_at: String,
    pub project: SiteInvoiceProjectResponse,
    pub billing: SiteInvoiceBillingResponse,
    pub labor: ProjectLaborSummaryResponse,
    pub materials: ProjectMaterialSummaryResponse,
}

impl From<crate::modules::sites::domain::Site> for SiteResponse {
    fn from(site: crate::modules::sites::domain::Site) -> Self {
        Self {
            id: site.id.to_string(),
            project_type: site.project_type.to_string(),
            name: site.name,
            customer_name: site.customer_name,
            location: site.location,
            description: site.description,
            status: site.status.to_string(),
            start_date: site.start_date.map(|d| d.to_string()),
            end_date: site.end_date.map(|d| d.to_string()),
            estimated_days: site.estimated_days,
            budget_amount_cents: site.budget_amount_cents,
            billing_reference: site.billing_reference,
            billing_notes: site.billing_notes,
            quote_reference: site.quote_reference,
            created_at: site.created_at.to_rfc3339(),
        }
    }
}

impl From<crate::modules::sites::infrastructure::site_repository::SiteHistoryReportRow>
    for SiteHistoryReportRowResponse
{
    fn from(
        row: crate::modules::sites::infrastructure::site_repository::SiteHistoryReportRow,
    ) -> Self {
        Self {
            site_id: row.site_id.to_string(),
            project_type: row.project_type.to_string(),
            name: row.name,
            customer_name: row.customer_name,
            status: row.status,
            start_date: row.start_date.map(|value| value.to_string()),
            end_date: row.end_date.map(|value| value.to_string()),
            estimated_days: row.estimated_days,
            budget_amount_cents: row.budget_amount_cents,
            billing_reference: row.billing_reference,
            quote_reference: row.quote_reference,
            total_hours: row.total_hours,
            worker_count: row.worker_count,
            distinct_material_count: row.distinct_material_count,
            withdrawal_count: row.withdrawal_count,
            cost_basis: row.cost_basis,
        }
    }
}

impl From<crate::modules::inventory::infrastructure::material_repository::ProjectMaterialUsageLine>
    for ProjectMaterialUsageLineResponse
{
    fn from(
        line: crate::modules::inventory::infrastructure::material_repository::ProjectMaterialUsageLine,
    ) -> Self {
        Self {
            material_id: line.material_id.to_string(),
            material_name: line.material_name,
            category_name: line.category_name,
            unit: line.unit,
            total_withdrawn: line.total_withdrawn,
            withdrawal_count: line.withdrawal_count,
            last_withdrawn_at: line.last_withdrawn_at.to_rfc3339(),
        }
    }
}

impl From<crate::modules::inventory::infrastructure::material_repository::ProjectMaterialSummary>
    for ProjectMaterialSummaryResponse
{
    fn from(
        summary: crate::modules::inventory::infrastructure::material_repository::ProjectMaterialSummary,
    ) -> Self {
        Self {
            distinct_material_count: summary.distinct_material_count,
            withdrawal_count: summary.withdrawal_count,
            lines: summary
                .lines
                .into_iter()
                .map(ProjectMaterialUsageLineResponse::from)
                .collect(),
        }
    }
}

impl From<crate::modules::sites::infrastructure::site_repository::ProjectLaborSummary>
    for ProjectLaborSummaryResponse
{
    fn from(
        summary: crate::modules::sites::infrastructure::site_repository::ProjectLaborSummary,
    ) -> Self {
        Self {
            total_hours: summary.total_hours,
            entry_count: summary.entry_count,
            site_hours: summary.site_hours,
            workshop_hours: summary.workshop_hours,
            last_work_date: summary.last_work_date.map(|value| value.to_string()),
        }
    }
}

impl From<crate::modules::sites::application::site_service::ProjectSummary>
    for SiteProjectSummaryResponse
{
    fn from(summary: crate::modules::sites::application::site_service::ProjectSummary) -> Self {
        Self {
            labor: ProjectLaborSummaryResponse::from(summary.labor),
            materials: ProjectMaterialSummaryResponse::from(summary.materials),
        }
    }
}

impl From<crate::modules::sites::domain::Site> for SiteInvoiceProjectResponse {
    fn from(site: crate::modules::sites::domain::Site) -> Self {
        Self {
            id: site.id.to_string(),
            name: site.name,
            project_type: site.project_type.to_string(),
            customer_name: site.customer_name,
            location: site.location,
            status: site.status.to_string(),
            start_date: site.start_date.map(|value| value.to_string()),
            end_date: site.end_date.map(|value| value.to_string()),
            estimated_days: site.estimated_days,
        }
    }
}

impl From<&crate::modules::sites::domain::Site> for SiteInvoiceBillingResponse {
    fn from(site: &crate::modules::sites::domain::Site) -> Self {
        Self {
            budget_amount_cents: site.budget_amount_cents,
            quote_reference: site.quote_reference.clone(),
            billing_reference: site.billing_reference.clone(),
            billing_notes: site.billing_notes.clone(),
        }
    }
}

impl From<crate::modules::sites::application::site_service::InvoiceSummary>
    for SiteInvoiceSummaryResponse
{
    fn from(summary: crate::modules::sites::application::site_service::InvoiceSummary) -> Self {
        let generated_at = chrono::Utc::now().to_rfc3339();
        Self {
            export_version: "v1".to_string(),
            generated_at,
            billing: SiteInvoiceBillingResponse::from(&summary.site),
            project: SiteInvoiceProjectResponse::from(summary.site),
            labor: ProjectLaborSummaryResponse::from(summary.project.labor),
            materials: ProjectMaterialSummaryResponse::from(summary.project.materials),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct CreateSiteRequest {
    pub project_type: String,
    pub name: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub estimated_days: Option<i32>,
    pub budget_amount_cents: Option<i64>,
    pub billing_reference: Option<String>,
    pub billing_notes: Option<String>,
    pub quote_reference: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct UpdateSiteRequest {
    pub project_type: Option<String>,
    pub name: Option<String>,
    pub customer_name: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub estimated_days: Option<i32>,
    pub budget_amount_cents: Option<i64>,
    pub billing_reference: Option<String>,
    pub billing_notes: Option<String>,
    pub quote_reference: Option<String>,
    pub clear_budget_amount: Option<bool>,
    pub clear_billing_reference: Option<bool>,
    pub clear_billing_notes: Option<bool>,
    pub clear_quote_reference: Option<bool>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct ListSitesQuery {
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct SiteHistoryReportQuery {
    pub customer: Option<String>,
    pub project_type: Option<String>,
    pub worker_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub duration_min_hours: Option<f64>,
    pub duration_max_hours: Option<f64>,
    pub cost_basis: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct SiteHistoryReportRowResponse {
    pub site_id: String,
    pub project_type: String,
    pub name: String,
    pub customer_name: String,
    pub status: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub estimated_days: Option<i32>,
    pub budget_amount_cents: Option<i64>,
    pub billing_reference: Option<String>,
    pub quote_reference: Option<String>,
    pub total_hours: f64,
    pub worker_count: i64,
    pub distinct_material_count: i64,
    pub withdrawal_count: i64,
    pub cost_basis: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct AssignmentResponse {
    pub id: String,
    pub site_id: String,
    pub user_id: String,
    pub role: String,
    pub created_at: String,
}

impl From<crate::modules::sites::domain::SiteAssignment> for AssignmentResponse {
    fn from(assignment: crate::modules::sites::domain::SiteAssignment) -> Self {
        Self {
            id: assignment.id.to_string(),
            site_id: assignment.site_id.to_string(),
            user_id: assignment.user_id.to_string(),
            role: assignment.role.to_string(),
            created_at: assignment.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct AssignUserRequest {
    pub user_id: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct TimeEntryResponse {
    pub id: String,
    pub site_id: Option<String>,
    pub user_id: String,
    pub work_type: String,
    pub hours: f64,
    pub work_date: String,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct SiteActivityAttachmentResponse {
    pub attachment_id: String,
    pub filename: String,
    pub mime_type: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct UploadSiteAttachmentResponse {
    pub attachment_id: String,
    pub filename: String,
    pub mime_type: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct UploadPhotoAttachmentResponse {
    pub attachment_id: String,
    pub photo_url: String,
    pub thumbnail_url: String,
}

impl From<crate::modules::sites::domain::TimeEntry> for TimeEntryResponse {
    fn from(entry: crate::modules::sites::domain::TimeEntry) -> Self {
        Self {
            id: entry.id.to_string(),
            site_id: entry.site_id.map(|s| s.to_string()),
            user_id: entry.user_id.to_string(),
            work_type: entry.work_type.to_string(),
            hours: entry.hours,
            work_date: entry.work_date.to_string(),
            notes: entry.notes,
            created_at: entry.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct CreateTimeEntryRequest {
    pub site_id: Option<String>,
    pub work_type: String,
    pub hours: f64,
    pub work_date: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct UpdateTimeEntryRequest {
    pub site_id: Option<String>,
    pub work_type: Option<String>,
    pub hours: Option<f64>,
    pub work_date: Option<String>,
    pub notes: Option<String>,
}

// === Handlers ===

pub async fn list_sites(
    State(state): State<AppState>,
    ctx: TenantContext,
    Query(query): Query<ListSitesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let sites = service.list_sites(query.status, &ctx).await?;
    let response: Vec<SiteResponse> = sites.into_iter().map(SiteResponse::from).collect();

    Ok(Json(response))
}

pub async fn list_site_history_report(
    State(state): State<AppState>,
    ctx: TenantContext,
    Query(query): Query<SiteHistoryReportQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );

    let project_type = query
        .project_type
        .map(|value| value.parse::<ProjectType>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;

    let worker_id = query
        .worker_id
        .map(|value| Uuid::parse_str(&value))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid worker ID".to_string()))?
        .map(UserId);

    let filter = crate::modules::sites::infrastructure::site_repository::SiteHistoryReportFilter {
        customer: query.customer,
        project_type,
        worker_id,
        date_from: query
            .date_from
            .and_then(|value| NaiveDate::parse_from_str(&value, "%Y-%m-%d").ok()),
        date_to: query
            .date_to
            .and_then(|value| NaiveDate::parse_from_str(&value, "%Y-%m-%d").ok()),
        duration_min_hours: query.duration_min_hours,
        duration_max_hours: query.duration_max_hours,
        cost_basis: query.cost_basis,
    };

    let rows = service.list_site_history_report(filter, &ctx).await?;
    let response: Vec<SiteHistoryReportRowResponse> = rows
        .into_iter()
        .map(SiteHistoryReportRowResponse::from)
        .collect();

    Ok(Json(response))
}

pub async fn create_site(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(request): Json<CreateSiteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let create = CreateSite {
        project_type: request
            .project_type
            .parse::<ProjectType>()
            .map_err(|e: String| AppError::Validation(e))?,
        name: request.name,
        customer_name: request.customer_name,
        location: request.location,
        description: request.description,
        start_date: request
            .start_date
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        end_date: request
            .end_date
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        estimated_days: request.estimated_days,
        budget_amount_cents: request.budget_amount_cents,
        billing_reference: request.billing_reference,
        billing_notes: request.billing_notes,
        quote_reference: request.quote_reference,
    };

    let site = service.create_site(create, &ctx).await?;

    Ok((StatusCode::CREATED, Json(SiteResponse::from(site))))
}

pub async fn get_site(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let site = service.get_site(site_id, &ctx).await?;

    Ok(Json(SiteResponse::from(site)))
}

pub async fn get_site_summary(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let summary = service.get_project_summary(site_id, &ctx).await?;

    Ok(Json(SiteProjectSummaryResponse::from(summary)))
}

pub async fn get_site_invoice_summary(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let summary = service.get_invoice_summary(site_id, &ctx).await?;

    Ok(Json(SiteInvoiceSummaryResponse::from(summary)))
}

pub async fn update_site(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Json(request): Json<UpdateSiteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let status = request
        .status
        .map(|s| s.parse::<SiteStatus>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;

    let update = UpdateSite {
        project_type: request
            .project_type
            .map(|s| s.parse::<ProjectType>())
            .transpose()
            .map_err(|e: String| AppError::Validation(e))?,
        name: request.name,
        customer_name: request.customer_name,
        location: request.location,
        description: request.description,
        status,
        start_date: request
            .start_date
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        end_date: request
            .end_date
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        estimated_days: request.estimated_days,
        budget_amount_cents: request.budget_amount_cents,
        billing_reference: request.billing_reference,
        billing_notes: request.billing_notes,
        quote_reference: request.quote_reference,
        clear_budget_amount: request.clear_budget_amount.unwrap_or(false),
        clear_billing_reference: request.clear_billing_reference.unwrap_or(false),
        clear_billing_notes: request.clear_billing_notes.unwrap_or(false),
        clear_quote_reference: request.clear_quote_reference.unwrap_or(false),
    };

    let site = service.update_site(site_id, update, &ctx).await?;

    Ok(Json(SiteResponse::from(site)))
}

pub async fn delete_site(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    service.delete_site(site_id, &ctx).await?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

pub async fn assign_user(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Json(request): Json<AssignUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let user_id = Uuid::parse_str(&request.user_id)
        .map(UserId)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;

    let role = request
        .role
        .map(|s| s.parse::<AssignmentRole>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?
        .unwrap_or(AssignmentRole::Worker);

    let assign = AssignUser { user_id, role };

    service.assign_user(site_id, assign, &ctx).await?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

pub async fn remove_assignment(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path((id, user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let user_id = Uuid::parse_str(&user_id)
        .map(UserId)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;

    service.remove_assignment(site_id, user_id, &ctx).await?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

pub async fn list_assignments(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let assignments = service.list_assignments(site_id, &ctx).await?;
    let response: Vec<AssignmentResponse> = assignments
        .into_iter()
        .map(AssignmentResponse::from)
        .collect();

    Ok(Json(response))
}

pub async fn list_site_time_entries(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let entries = service.list_time_entries(Some(site_id), None, &ctx).await?;
    let response: Vec<TimeEntryResponse> =
        entries.into_iter().map(TimeEntryResponse::from).collect();

    Ok(Json(response))
}

pub async fn create_time_entry(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(request): Json<CreateTimeEntryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = request
        .site_id
        .map(|s| Uuid::parse_str(&s).map(SiteId))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let work_type = request
        .work_type
        .parse::<WorkType>()
        .map_err(|e: String| AppError::Validation(e))?;

    let work_date = NaiveDate::parse_from_str(&request.work_date, "%Y-%m-%d").map_err(|_| {
        AppError::Validation("Invalid work date format (expected YYYY-MM-DD)".to_string())
    })?;

    let create = CreateTimeEntry {
        site_id,
        work_type,
        hours: request.hours,
        work_date,
        notes: request.notes,
    };

    let entry = service.create_time_entry(create, &ctx).await?;

    Ok((StatusCode::CREATED, Json(TimeEntryResponse::from(entry))))
}

pub async fn list_my_time_entries(
    State(state): State<AppState>,
    ctx: TenantContext,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let entries = service.list_my_time_entries(&ctx).await?;
    let response: Vec<TimeEntryResponse> =
        entries.into_iter().map(TimeEntryResponse::from).collect();

    Ok(Json(response))
}

pub async fn get_time_entry(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let entry_id = Uuid::parse_str(&id)
        .map(TimeEntryId)
        .map_err(|_| AppError::Validation("Invalid time entry ID".to_string()))?;

    let entry = service.get_time_entry(entry_id, &ctx).await?;

    Ok(Json(TimeEntryResponse::from(entry)))
}

pub async fn update_time_entry(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Json(request): Json<UpdateTimeEntryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let entry_id = Uuid::parse_str(&id)
        .map(TimeEntryId)
        .map_err(|_| AppError::Validation("Invalid time entry ID".to_string()))?;

    let site_id = request
        .site_id
        .map(|s| {
            if s.is_empty() {
                Ok(None)
            } else {
                Uuid::parse_str(&s).map(|u| Some(SiteId(u)))
            }
        })
        .transpose()
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let work_type = request
        .work_type
        .map(|s| s.parse::<WorkType>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;

    let work_date = request
        .work_date
        .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d"))
        .transpose()
        .map_err(|_| {
            AppError::Validation("Invalid work date format (expected YYYY-MM-DD)".to_string())
        })?;

    let update = UpdateTimeEntry {
        site_id,
        work_type,
        hours: request.hours,
        work_date,
        notes: request.notes.map(Some),
    };

    let entry = service.update_time_entry(entry_id, update, &ctx).await?;

    Ok(Json(TimeEntryResponse::from(entry)))
}

pub async fn delete_time_entry(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let entry_id = Uuid::parse_str(&id)
        .map(TimeEntryId)
        .map_err(|_| AppError::Validation("Invalid time entry ID".to_string()))?;

    service.delete_time_entry(entry_id, &ctx).await?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

// === Activity DTOs ===

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct ActivityResponse {
    pub id: String,
    pub site_id: String,
    pub user_id: String,
    pub creator_name: String,
    pub can_delete: bool,
    pub activity_type: String,
    pub content: Option<String>,
    pub photo_url: Option<String>,
    pub attachments: Vec<SiteActivityAttachmentResponse>,
    pub created_at: String,
}

impl From<crate::modules::sites::domain::Activity> for ActivityResponse {
    fn from(activity: crate::modules::sites::domain::Activity) -> Self {
        Self {
            id: activity.id.to_string(),
            site_id: activity.site_id.to_string(),
            user_id: activity.user_id.to_string(),
            creator_name: activity.creator_name,
            can_delete: activity.can_delete,
            activity_type: activity.activity_type.as_str().to_string(),
            content: activity.content,
            photo_url: activity.photo_url,
            attachments: activity
                .attachments
                .into_iter()
                .map(SiteActivityAttachmentResponse::from)
                .collect(),
            created_at: activity.created_at.to_rfc3339(),
        }
    }
}

impl From<crate::modules::sites::domain::ActivityAttachmentMetadata>
    for SiteActivityAttachmentResponse
{
    fn from(attachment: crate::modules::sites::domain::ActivityAttachmentMetadata) -> Self {
        Self {
            attachment_id: attachment.id.to_string(),
            filename: attachment.filename,
            mime_type: attachment.mime_type,
            url: attachment.url,
            thumbnail_url: attachment.thumbnail_url,
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct CreateActivityRequest {
    pub activity_type: String, // "photo" or "note"
    pub content: Option<String>,
    pub photo_url: Option<String>,
    #[serde(default)]
    pub attachment_ids: Vec<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct ActivityQuery {
    pub limit: Option<i32>,
}

// === Dashboard DTOs ===

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct DashboardSiteResponse {
    pub id: String,
    pub project_type: String,
    pub name: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub status: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub estimated_days: Option<i32>,
    pub assigned_users: i64,
    pub total_hours: f64,
}

impl From<DashboardSite> for DashboardSiteResponse {
    fn from(site: DashboardSite) -> Self {
        Self {
            id: site.id.to_string(),
            project_type: site.project_type.to_string(),
            name: site.name,
            customer_name: site.customer_name,
            location: site.location,
            status: site.status,
            start_date: site.start_date.map(|d| d.to_string()),
            end_date: site.end_date.map(|d| d.to_string()),
            estimated_days: site.estimated_days,
            assigned_users: site.assigned_users,
            total_hours: site.total_hours,
        }
    }
}

// === Activity Handlers ===

pub async fn list_activities(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Query(query): Query<ActivityQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let limit = query.limit.unwrap_or(50).min(100);

    let activities = service.list_activities(site_id, limit, &ctx).await?;
    let response: Vec<ActivityResponse> =
        activities.into_iter().map(ActivityResponse::from).collect();

    Ok(Json(response))
}

pub async fn create_activity(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Json(request): Json<CreateActivityRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let activity_type = match request.activity_type.as_str() {
        "photo" => ActivityType::Photo,
        "note" => ActivityType::Note,
        _ => {
            return Err(AppError::Validation(
                "Invalid activity type (expected 'photo' or 'note')".to_string(),
            ))
        }
    };

    let create = CreateActivity {
        site_id,
        activity_type,
        content: request.content,
        photo_url: request.photo_url,
        attachment_ids: request
            .attachment_ids
            .into_iter()
            .map(|attachment_id| {
                Uuid::parse_str(&attachment_id)
                    .map_err(|_| AppError::Validation("Invalid attachment ID".to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?,
    };

    let activity = service.create_activity(create, &ctx).await?;

    Ok((StatusCode::CREATED, Json(ActivityResponse::from(activity))))
}

pub async fn delete_activity(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path((site_id, activity_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&site_id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    let activity_id = Uuid::parse_str(&activity_id)
        .map(crate::common::types::ActivityId)
        .map_err(|_| AppError::Validation("Invalid activity ID".to_string()))?;

    service.delete_activity(site_id, activity_id, &ctx).await?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

pub async fn upload_site_attachment(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let mut attachment_part: Option<(String, String, Vec<u8>)> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(format!("Invalid multipart payload: {e}")))?
    {
        if field.name() != Some("attachment") {
            continue;
        }

        let file_name = field.file_name().unwrap_or("attachment").to_string();

        let mime_type = field
            .content_type()
            .ok_or_else(|| AppError::Validation("Attachment MIME type is required".to_string()))?
            .to_string();
        let bytes = field
            .bytes()
            .await
            .map_err(|e| AppError::Validation(format!("Unable to read upload bytes: {e}")))?
            .to_vec();
        attachment_part = Some((file_name, mime_type, bytes));
        break;
    }

    let (original_filename, mime_type, original_bytes) = attachment_part.ok_or_else(|| {
        AppError::Validation("Multipart field 'attachment' is required".to_string())
    })?;

    let result = service
        .upload_site_attachment(
            crate::modules::sites::application::site_service::UploadPhotoCommand {
                site_id,
                mime_type,
                original_bytes,
                original_filename,
            },
            &ctx,
        )
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(UploadSiteAttachmentResponse {
            attachment_id: result.attachment_id.to_string(),
            filename: result.filename,
            mime_type: result.mime_type,
            url: result.photo_url,
            thumbnail_url: result.thumbnail_url,
        }),
    ))
}

pub async fn upload_site_photo_attachment(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let mut image_part: Option<(String, String, Vec<u8>)> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(format!("Invalid multipart payload: {e}")))?
    {
        if field.name() != Some("photo") {
            continue;
        }

        let file_name = field.file_name().unwrap_or("photo").to_string();
        let mime_type = field
            .content_type()
            .ok_or_else(|| AppError::Validation("Photo MIME type is required".to_string()))?
            .to_string();
        let bytes = field
            .bytes()
            .await
            .map_err(|e| AppError::Validation(format!("Unable to read upload bytes: {e}")))?
            .to_vec();
        image_part = Some((file_name, mime_type, bytes));
        break;
    }

    let (original_filename, mime_type, original_bytes) = image_part
        .ok_or_else(|| AppError::Validation("Multipart field 'photo' is required".to_string()))?;

    let result = service
        .upload_photo_attachment(
            crate::modules::sites::application::site_service::UploadPhotoCommand {
                site_id,
                mime_type,
                original_bytes,
                original_filename,
            },
            &ctx,
        )
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(UploadPhotoAttachmentResponse {
            attachment_id: result.attachment_id.to_string(),
            photo_url: result.photo_url,
            thumbnail_url: result.thumbnail_url.unwrap_or_default(),
        }),
    ))
}

// === Dashboard Handler ===

pub async fn get_dashboard(
    State(state): State<AppState>,
    ctx: TenantContext,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let sites = service.get_dashboard(&ctx).await?;
    let response: Vec<DashboardSiteResponse> =
        sites.into_iter().map(DashboardSiteResponse::from).collect();

    Ok(Json(response))
}

pub async fn get_attachment_bytes(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(attachment_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let repo =
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool);

    let attachment_uuid = Uuid::parse_str(&attachment_id)
        .map_err(|_| AppError::Validation("Invalid attachment ID".to_string()))?;

    let attachment = repo
        .find_attachment_by_id(attachment_uuid, ctx.tenant_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Attachment not found".to_string()))?;

    let bytes = attachment
        .original_bytes
        .ok_or_else(|| AppError::NotFound("Attachment content not found".to_string()))?;

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, attachment.mime_type),
            (header::CACHE_CONTROL, "private, max-age=300".to_string()),
        ],
        bytes,
    ))
}

pub async fn get_attachment_thumbnail_bytes(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(attachment_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let repo =
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool);

    let attachment_uuid = Uuid::parse_str(&attachment_id)
        .map_err(|_| AppError::Validation("Invalid attachment ID".to_string()))?;

    let attachment = repo
        .find_attachment_by_id(attachment_uuid, ctx.tenant_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Attachment not found".to_string()))?;

    let bytes = attachment
        .thumbnail_bytes
        .ok_or_else(|| AppError::NotFound("Attachment thumbnail not found".to_string()))?;

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, attachment.mime_type),
            (header::CACHE_CONTROL, "private, max-age=300".to_string()),
        ],
        bytes,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        ActivityResponse, DashboardSiteResponse, SiteActivityAttachmentResponse, SiteResponse,
        UploadPhotoAttachmentResponse, UploadSiteAttachmentResponse,
    };
    use crate::common::types::{ActivityId, ProjectType, SiteId, SiteStatus, TenantId, UserId};
    use crate::modules::sites::domain::{Activity, ActivityType};
    use crate::modules::sites::infrastructure::site_repository::DashboardSite;
    use chrono::Utc;

    #[test]
    fn activity_response_can_delete_maps_server_permission_bit() {
        let response = ActivityResponse::from(Activity {
            id: ActivityId::new(),
            tenant_id: TenantId::new(),
            site_id: SiteId::new(),
            user_id: UserId::new(),
            creator_name: "Max Mustermann".to_string(),
            can_delete: true,
            activity_type: ActivityType::Note,
            content: Some("Notiz".to_string()),
            photo_url: None,
            attachments: Vec::new(),
            created_at: Utc::now(),
        });

        assert!(response.can_delete);
    }

    #[test]
    fn upload_response_contains_required_contract_fields() {
        let dto = UploadPhotoAttachmentResponse {
            attachment_id: uuid::Uuid::new_v4().to_string(),
            photo_url: "/api/v1/attachments/1".to_string(),
            thumbnail_url: "/api/v1/attachments/1/thumbnail".to_string(),
        };

        assert!(!dto.attachment_id.is_empty());
        assert!(dto.photo_url.contains("/attachments/"));
        assert!(dto.thumbnail_url.ends_with("/thumbnail"));
    }

    #[test]
    fn generic_attachment_response_contains_required_contract_fields() {
        let dto = SiteActivityAttachmentResponse {
            attachment_id: uuid::Uuid::new_v4().to_string(),
            filename: "lieferschein.pdf".to_string(),
            mime_type: "application/pdf".to_string(),
            url: "/api/v1/attachments/1".to_string(),
            thumbnail_url: None,
        };

        assert_eq!(dto.filename, "lieferschein.pdf");
        assert_eq!(dto.mime_type, "application/pdf");
        assert!(dto.url.contains("/attachments/"));
        assert!(dto.thumbnail_url.is_none());
    }

    #[test]
    fn upload_site_attachment_response_preserves_filename_and_mime_type() {
        let dto = UploadSiteAttachmentResponse {
            attachment_id: uuid::Uuid::new_v4().to_string(),
            filename: "plan.pdf".to_string(),
            mime_type: "application/pdf".to_string(),
            url: "/api/v1/attachments/1".to_string(),
            thumbnail_url: None,
        };

        assert_eq!(dto.filename, "plan.pdf");
        assert_eq!(dto.mime_type, "application/pdf");
    }

    #[test]
    fn site_response_includes_project_type() {
        let response = SiteResponse::from(crate::modules::sites::domain::Site {
            id: SiteId::new(),
            tenant_id: TenantId::new(),
            project_type: ProjectType::InternalWorkshop,
            name: "Werkstattprojekt".to_string(),
            customer_name: "".to_string(),
            location: None,
            description: Some("Test".to_string()),
            status: SiteStatus::Planned,
            start_date: None,
            end_date: None,
            estimated_days: Some(1),
            budget_amount_cents: None,
            billing_reference: None,
            billing_notes: None,
            quote_reference: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        assert_eq!(response.project_type, "internal_workshop");
    }

    #[test]
    fn dashboard_site_includes_project_type() {
        let response = DashboardSiteResponse::from(DashboardSite {
            id: SiteId::new(),
            tenant_id: TenantId::new(),
            project_type: ProjectType::ExternalSite,
            name: "Kueche".to_string(),
            customer_name: "Mustermann".to_string(),
            location: Some("Leipzig".to_string()),
            status: "planned".to_string(),
            start_date: None,
            end_date: None,
            estimated_days: Some(3),
            assigned_users: 2,
            total_hours: 8.0,
        });

        assert_eq!(response.project_type, "external_site");
    }
}
