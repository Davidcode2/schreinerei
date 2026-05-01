use axum::{
    extract::{Multipart, Path, Query, State},
    http::header,
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
    routing::{get, post, delete},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::auth::extractor::AuthenticatedUser;
use crate::common::error::AppError;
use crate::common::types::{SiteId, UserId, SiteStatus, AssignmentRole, WorkType, TimeEntryId};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::sites::application::site_service::SiteService;
use crate::modules::sites::domain::{CreateSite, UpdateSite, CreateTimeEntry, UpdateTimeEntry, AssignUser, CreateActivity, ActivityType};
use crate::modules::sites::infrastructure::site_repository::DashboardSite;
use crate::AppState;

/// Create the sites API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Sites
        .route("/api/v1/sites", get(list_sites).post(create_site))
        .route("/api/v1/sites/{id}", get(get_site).patch(update_site).delete(delete_site))
        
        // Assignments
        .route("/api/v1/sites/{id}/assign", post(assign_user))
        .route("/api/v1/sites/{id}/assign/{user_id}", delete(remove_assignment))
        .route("/api/v1/sites/{id}/assignments", get(list_assignments))
        
        // Time entries
        .route("/api/v1/sites/{id}/time-entries", get(list_site_time_entries))
        .route("/api/v1/time-entries", post(create_time_entry))
        .route("/api/v1/time-entries/my", get(list_my_time_entries))
        .route("/api/v1/time-entries/{id}", get(get_time_entry).patch(update_time_entry).delete(delete_time_entry))
        
        // Activities
        .route("/api/v1/sites/{id}/activities", get(list_activities).post(create_activity))
        .route("/api/v1/sites/{id}/attachments", post(upload_site_attachment))
        .route("/api/v1/sites/{id}/attachments/photo", post(upload_site_attachment))
        .route("/api/v1/attachments/{attachment_id}", get(get_attachment_bytes))
        .route("/api/v1/attachments/{attachment_id}/thumbnail", get(get_attachment_thumbnail_bytes))
        
        // Dashboard
        .route("/api/v1/dashboard/sites", get(get_dashboard))
}

// === DTOs ===

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct SiteResponse {
    pub id: String,
    pub name: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub estimated_days: Option<i32>,
    pub created_at: String,
}

impl From<crate::modules::sites::domain::Site> for SiteResponse {
    fn from(site: crate::modules::sites::domain::Site) -> Self {
        Self {
            id: site.id.to_string(),
            name: site.name,
            customer_name: site.customer_name,
            location: site.location,
            description: site.description,
            status: site.status.to_string(),
            start_date: site.start_date.map(|d| d.to_string()),
            end_date: site.end_date.map(|d| d.to_string()),
            estimated_days: site.estimated_days,
            created_at: site.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct CreateSiteRequest {
    pub name: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub estimated_days: Option<i32>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct UpdateSiteRequest {
    pub name: Option<String>,
    pub customer_name: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub estimated_days: Option<i32>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct ListSitesQuery {
    pub status: Option<String>,
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
    auth: AuthenticatedUser,
    Query(query): Query<ListSitesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let sites = service.list_sites(query.status, &ctx).await?;
    let response: Vec<SiteResponse> = sites.into_iter().map(SiteResponse::from).collect();
    
    Ok(Json(response))
}

pub async fn create_site(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<CreateSiteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let create = CreateSite {
        name: request.name,
        customer_name: request.customer_name,
        location: request.location,
        description: request.description,
        start_date: request.start_date.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        end_date: request.end_date.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        estimated_days: request.estimated_days,
    };
    
    let site = service.create_site(create, &ctx).await?;
    
    Ok((StatusCode::CREATED, Json(SiteResponse::from(site))))
}

pub async fn get_site(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    
    let site = service.get_site(site_id, &ctx).await?;
    
    Ok(Json(SiteResponse::from(site)))
}

pub async fn update_site(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<UpdateSiteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    
    let status = request.status
        .map(|s| s.parse::<SiteStatus>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;
    
    let update = UpdateSite {
        name: request.name,
        customer_name: request.customer_name,
        location: request.location,
        description: request.description,
        status,
        start_date: request.start_date.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        end_date: request.end_date.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        estimated_days: request.estimated_days,
    };
    
    let site = service.update_site(site_id, update, &ctx).await?;
    
    Ok(Json(SiteResponse::from(site)))
}

pub async fn delete_site(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    
    service.delete_site(site_id, &ctx).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

pub async fn assign_user(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<AssignUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    
    let user_id = Uuid::parse_str(&request.user_id)
        .map(UserId)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;
    
    let role = request.role
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
    auth: AuthenticatedUser,
    Path((id, user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
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
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    
    let assignments = service.list_assignments(site_id, &ctx).await?;
    let response: Vec<AssignmentResponse> = assignments.into_iter().map(AssignmentResponse::from).collect();
    
    Ok(Json(response))
}

pub async fn list_site_time_entries(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    
    let entries = service.list_time_entries(Some(site_id), None, &ctx).await?;
    let response: Vec<TimeEntryResponse> = entries.into_iter().map(TimeEntryResponse::from).collect();
    
    Ok(Json(response))
}

pub async fn create_time_entry(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<CreateTimeEntryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let site_id = request.site_id
        .map(|s| Uuid::parse_str(&s).map(SiteId))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    
    let work_type = request.work_type.parse::<WorkType>()
        .map_err(|e: String| AppError::Validation(e))?;
    
    let work_date = NaiveDate::parse_from_str(&request.work_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid work date format (expected YYYY-MM-DD)".to_string()))?;
    
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
    auth: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let entries = service.list_my_time_entries(&ctx).await?;
    let response: Vec<TimeEntryResponse> = entries.into_iter().map(TimeEntryResponse::from).collect();
    
    Ok(Json(response))
}

pub async fn get_time_entry(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let entry_id = Uuid::parse_str(&id)
        .map(TimeEntryId)
        .map_err(|_| AppError::Validation("Invalid time entry ID".to_string()))?;
    
    let entry = service.get_time_entry(entry_id, &ctx).await?;
    
    Ok(Json(TimeEntryResponse::from(entry)))
}

pub async fn update_time_entry(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<UpdateTimeEntryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let entry_id = Uuid::parse_str(&id)
        .map(TimeEntryId)
        .map_err(|_| AppError::Validation("Invalid time entry ID".to_string()))?;
    
    let site_id = request.site_id
        .map(|s| {
            if s.is_empty() {
                Ok(None)
            } else {
                Uuid::parse_str(&s).map(|u| Some(SiteId(u)))
            }
        })
        .transpose()
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    
    let work_type = request.work_type
        .map(|s| s.parse::<WorkType>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;
    
    let work_date = request.work_date
        .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d"))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid work date format (expected YYYY-MM-DD)".to_string()))?;
    
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
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
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
            activity_type: activity.activity_type.as_str().to_string(),
            content: activity.content,
            photo_url: activity.photo_url,
            attachments: activity.attachments.into_iter().map(SiteActivityAttachmentResponse::from).collect(),
            created_at: activity.created_at.to_rfc3339(),
        }
    }
}

impl From<crate::modules::sites::domain::ActivityAttachmentMetadata> for SiteActivityAttachmentResponse {
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
    pub activity_type: String,  // "photo" or "note"
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
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Query(query): Query<ActivityQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    
    let limit = query.limit.unwrap_or(50).min(100);
    
    let activities = service.list_activities(site_id, limit, &ctx).await?;
    let response: Vec<ActivityResponse> = activities.into_iter().map(ActivityResponse::from).collect();
    
    Ok(Json(response))
}

pub async fn create_activity(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<CreateActivityRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    
    let activity_type = match request.activity_type.as_str() {
        "photo" => ActivityType::Photo,
        "note" => ActivityType::Note,
        _ => return Err(AppError::Validation("Invalid activity type (expected 'photo' or 'note')".to_string())),
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

pub async fn upload_site_attachment(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let mut image_part: Option<(String, Vec<u8>)> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(format!("Invalid multipart payload: {e}")))?
    {
        if field.name() != Some("photo") {
            continue;
        }

        let mime_type = field
            .content_type()
            .ok_or_else(|| AppError::Validation("Photo MIME type is required".to_string()))?
            .to_string();
        let bytes = field
            .bytes()
            .await
            .map_err(|e| AppError::Validation(format!("Unable to read upload bytes: {e}")))?
            .to_vec();
        image_part = Some((mime_type, bytes));
        break;
    }

    let (mime_type, original_bytes) =
        image_part.ok_or_else(|| AppError::Validation("Multipart field 'photo' is required".to_string()))?;

    let result = service
        .upload_photo_attachment(
            crate::modules::sites::application::site_service::UploadPhotoCommand {
                site_id,
                mime_type,
                original_bytes,
                original_filename: "attachment".to_string(),
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
    auth: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let sites = service.get_dashboard(&ctx).await?;
    let response: Vec<DashboardSiteResponse> = sites.into_iter().map(DashboardSiteResponse::from).collect();
    
    Ok(Json(response))
}

pub async fn get_attachment_bytes(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(attachment_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let ctx = TenantContext::from_auth(&auth);
    let repo = crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool);

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
    auth: AuthenticatedUser,
    Path(attachment_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let ctx = TenantContext::from_auth(&auth);
    let repo = crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool);

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
    use super::{SiteActivityAttachmentResponse, UploadPhotoAttachmentResponse};

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
}
