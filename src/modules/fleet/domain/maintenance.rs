use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{
    AssetId, MaintenanceDueId, MaintenanceDueStatus, MaintenanceScheduleId, MaintenanceSeverity,
    ResourceType, TenantId, UserId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    pub id: MaintenanceScheduleId,
    pub tenant_id: TenantId,
    pub asset_id: AssetId,
    pub task_description: String,
    pub interval_days: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceDue {
    pub id: MaintenanceDueId,
    pub tenant_id: TenantId,
    pub schedule_id: MaintenanceScheduleId,
    pub asset_id: AssetId,
    pub resource_type: ResourceType,
    pub resource_name: String,
    pub task_description: String,
    pub due_date: NaiveDate,
    pub status: MaintenanceDueStatus,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<UserId>,
    pub resolution_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MaintenanceDue {
    pub fn severity(&self, today: NaiveDate) -> MaintenanceSeverity {
        if self.due_date < today {
            MaintenanceSeverity::Overdue
        } else {
            MaintenanceSeverity::Due
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMaintenanceSchedule {
    pub asset_id: AssetId,
    pub task_description: String,
    pub interval_days: i32,
    pub next_due_date: NaiveDate,
}

impl CreateMaintenanceSchedule {
    pub fn validate(&self) -> Result<(), String> {
        if self.task_description.trim().is_empty() {
            return Err("Task description is required".to_string());
        }

        if self.interval_days <= 0 {
            return Err("Interval must be greater than zero days".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveMaintenanceDue {
    pub resolution_notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_maintenance_schedule_validate_rejects_empty_task() {
        let create = CreateMaintenanceSchedule {
            asset_id: AssetId::new(),
            task_description: "  ".to_string(),
            interval_days: 30,
            next_due_date: Utc::now().date_naive(),
        };

        assert_eq!(
            create.validate(),
            Err("Task description is required".to_string())
        );
    }

    #[test]
    fn create_maintenance_schedule_validate_rejects_non_positive_interval() {
        let create = CreateMaintenanceSchedule {
            asset_id: AssetId::new(),
            task_description: "Sicherheitsprüfung".to_string(),
            interval_days: 0,
            next_due_date: Utc::now().date_naive(),
        };

        assert_eq!(
            create.validate(),
            Err("Interval must be greater than zero days".to_string())
        );
    }

    #[test]
    fn due_severity_is_overdue_only_before_today() {
        let today = Utc::now().date_naive();
        let due = MaintenanceDue {
            id: MaintenanceDueId::new(),
            tenant_id: TenantId::new(),
            schedule_id: MaintenanceScheduleId::new(),
            asset_id: AssetId::new(),
            resource_type: ResourceType::Vehicle,
            resource_name: "Bulli".to_string(),
            task_description: "Ölwechsel".to_string(),
            due_date: today,
            status: MaintenanceDueStatus::Open,
            resolved_at: None,
            resolved_by: None,
            resolution_notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(due.severity(today), MaintenanceSeverity::Due);

        let overdue = MaintenanceDue {
            due_date: today - chrono::Duration::days(1),
            ..due
        };
        assert_eq!(overdue.severity(today), MaintenanceSeverity::Overdue);
    }
}
