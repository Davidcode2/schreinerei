use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::common::types::{TenantId, SiteId, TimeEntryId, UserId, WorkType};

/// TimeEntry aggregate representing work time booking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: TimeEntryId,
    pub tenant_id: TenantId,
    pub site_id: Option<SiteId>,  // NULL for workshop work
    pub user_id: UserId,
    pub work_type: WorkType,
    pub hours: f64,
    pub work_date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl TimeEntry {
    /// Get work type as string for database storage
    pub fn work_type_str(&self) -> &'static str {
        self.work_type.as_str()
    }
}

/// Command to create a time entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimeEntry {
    pub site_id: Option<SiteId>,
    pub work_type: WorkType,
    pub hours: f64,
    pub work_date: NaiveDate,
    pub notes: Option<String>,
}

impl CreateTimeEntry {
    /// Validate the create time entry command
    pub fn validate(&self) -> Result<(), String> {
        if self.hours <= 0.0 {
            return Err("Hours must be positive".to_string());
        }
        if self.hours > 24.0 {
            return Err("Hours cannot exceed 24 per day".to_string());
        }
        // Validate work date is not in the future
        let today = chrono::Local::now().date_naive();
        if self.work_date > today {
            return Err("Work date cannot be in the future".to_string());
        }
        Ok(())
    }
}
