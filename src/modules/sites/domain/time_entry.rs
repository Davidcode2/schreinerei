use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{SiteId, TenantId, TimeEntryId, UserId, WorkType};

/// TimeEntry aggregate representing work time booking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: TimeEntryId,
    pub tenant_id: TenantId,
    pub site_id: Option<SiteId>, // NULL for workshop work
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

/// Command to update a time entry (partial update)
/// Option<Option<T>> distinguishes between "not provided" (None) and "set to null" (Some(None))
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTimeEntry {
    pub site_id: Option<Option<SiteId>>, // None = not provided, Some(None) = set to null
    pub work_type: Option<WorkType>,
    pub hours: Option<f64>,
    pub work_date: Option<NaiveDate>,
    pub notes: Option<Option<String>>,
}

impl UpdateTimeEntry {
    /// Validate the update time entry command
    pub fn validate(&self) -> Result<(), String> {
        if let Some(hours) = self.hours {
            if hours <= 0.0 {
                return Err("Hours must be positive".to_string());
            }
            if hours > 24.0 {
                return Err("Hours cannot exceed 24 per day".to_string());
            }
        }
        if let Some(work_date) = self.work_date {
            let today = chrono::Local::now().date_naive();
            if work_date > today {
                return Err("Work date cannot be in the future".to_string());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn today() -> NaiveDate {
        chrono::Local::now().date_naive()
    }

    fn yesterday() -> NaiveDate {
        today() - chrono::Duration::days(1)
    }

    fn tomorrow() -> NaiveDate {
        today() + chrono::Duration::days(1)
    }

    #[test]
    fn create_time_entry_validate_succeeds_with_valid_hours() {
        let cmd = CreateTimeEntry {
            site_id: None,
            work_type: WorkType::Site,
            hours: 8.0,
            work_date: today(),
            notes: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_time_entry_validate_fails_with_zero_hours() {
        let cmd = CreateTimeEntry {
            site_id: None,
            work_type: WorkType::Site,
            hours: 0.0,
            work_date: today(),
            notes: None,
        };
        assert_eq!(cmd.validate(), Err("Hours must be positive".to_string()));
    }

    #[test]
    fn create_time_entry_validate_fails_with_negative_hours() {
        let cmd = CreateTimeEntry {
            site_id: None,
            work_type: WorkType::Site,
            hours: -1.0,
            work_date: today(),
            notes: None,
        };
        assert_eq!(cmd.validate(), Err("Hours must be positive".to_string()));
    }

    #[test]
    fn create_time_entry_validate_fails_with_hours_exceeding_24() {
        let cmd = CreateTimeEntry {
            site_id: None,
            work_type: WorkType::Site,
            hours: 25.0,
            work_date: today(),
            notes: None,
        };
        assert_eq!(
            cmd.validate(),
            Err("Hours cannot exceed 24 per day".to_string())
        );
    }

    #[test]
    fn create_time_entry_validate_succeeds_with_exactly_24_hours() {
        let cmd = CreateTimeEntry {
            site_id: None,
            work_type: WorkType::Site,
            hours: 24.0,
            work_date: today(),
            notes: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_time_entry_validate_succeeds_with_yesterday_date() {
        let cmd = CreateTimeEntry {
            site_id: None,
            work_type: WorkType::Site,
            hours: 8.0,
            work_date: yesterday(),
            notes: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_time_entry_validate_fails_with_future_date() {
        let cmd = CreateTimeEntry {
            site_id: None,
            work_type: WorkType::Site,
            hours: 8.0,
            work_date: tomorrow(),
            notes: None,
        };
        assert_eq!(
            cmd.validate(),
            Err("Work date cannot be in the future".to_string())
        );
    }
}
