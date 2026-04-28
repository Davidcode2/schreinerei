use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
    routing::{get, post, delete, patch},
};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::auth::extractor::AuthenticatedUser;
use crate::common::error::AppError;
use crate::common::types::{SiteId, UserId, SiteStatus, AssignmentRole, WorkType};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::sites::application::site_service::SiteService;
use crate::modules::sites::domain::{CreateSite, UpdateSite, CreateTimeEntry, AssignUser};
use crate::AppState;

/// Create the sites API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Sites
        .route("/api/v1/sites", get(list_sites).post(create_site))
        .route("/api/v1/sites/{id}", get(get_site).patch(update_site))
        
        // Assignments
        .route("/api/v1/sites/{id}/assign", post(assign_user))
        .route("/api/v1/sites/{id}/assign/{user_id}", delete(remove_assignment))
        .route("/api/v1/sites/{id}/assignments", get(list_assignments))
        
        // Time entries
        .route("/api/v1/sites/{id}/time-entries", get(list_site_time_entries))
        .route("/api/v1/time-entries", post(create_time_entry))
        .route("/api/v1/time-entries/my", get(list_my_time_entries))
}

// === DTOs ===

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct CreateSiteRequest {
    pub name: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub estimated_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct ListSitesQuery {
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct AssignUserRequest {
    pub user_id: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct CreateTimeEntryRequest {
    pub site_id: Option<String>,
    pub work_type: String,
    pub hours: f64,
    pub work_date: String,
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
