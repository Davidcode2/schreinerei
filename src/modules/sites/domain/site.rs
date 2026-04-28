use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::common::types::{TenantId, SiteId, UserId, SiteStatus, AssignmentRole};

/// Site aggregate representing a construction site (Baustelle)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: SiteId,
    pub tenant_id: TenantId,
    pub name: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: SiteStatus,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub estimated_days: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Site {
    /// Check if status transition is valid
    pub fn can_transition_to(&self, new_status: SiteStatus) -> bool {
        match (&self.status, &new_status) {
            // Planned -> Active
            (SiteStatus::Planned, SiteStatus::Active) => true,
            // Active -> Completed
            (SiteStatus::Active, SiteStatus::Completed) => true,
            // Completed -> Archived
            (SiteStatus::Completed, SiteStatus::Archived) => true,
            // Any -> same (no change)
            _ if self.status == new_status => true,
            _ => false,
        }
    }
}

/// Site assignment linking a user to a site
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteAssignment {
    pub id: uuid::Uuid,
    pub tenant_id: TenantId,
    pub site_id: SiteId,
    pub user_id: UserId,
    pub role: AssignmentRole,
    pub created_at: DateTime<Utc>,
}

/// Command to create a new site
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSite {
    pub name: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub estimated_days: Option<i32>,
}

impl CreateSite {
    /// Validate the create site command
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Site name is required".to_string());
        }
        if self.customer_name.trim().is_empty() {
            return Err("Customer name is required".to_string());
        }
        // Validate date range if both dates are provided
        if let (Some(start), Some(end)) = (self.start_date, self.end_date) {
            if end < start {
                return Err("End date cannot be before start date".to_string());
            }
        }
        if let Some(days) = self.estimated_days {
            if days < 0 {
                return Err("Estimated days cannot be negative".to_string());
            }
        }
        Ok(())
    }
}

/// Command to update a site
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSite {
    pub name: Option<String>,
    pub customer_name: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: Option<SiteStatus>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub estimated_days: Option<i32>,
}

/// Command to assign a user to a site
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignUser {
    pub user_id: UserId,
    pub role: AssignmentRole,
}
