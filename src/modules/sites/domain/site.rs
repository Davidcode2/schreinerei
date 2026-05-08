use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{AssignmentRole, ProjectType, SiteId, SiteStatus, TenantId, UserId};

/// Site aggregate representing a construction site (Baustelle)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: SiteId,
    pub tenant_id: TenantId,
    pub project_type: ProjectType,
    pub name: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: SiteStatus,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub estimated_days: Option<i32>,
    pub budget_amount_cents: Option<i64>,
    pub billing_reference: Option<String>,
    pub billing_notes: Option<String>,
    pub quote_reference: Option<String>,
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
    pub project_type: ProjectType,
    pub name: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub estimated_days: Option<i32>,
    pub budget_amount_cents: Option<i64>,
    pub billing_reference: Option<String>,
    pub billing_notes: Option<String>,
    pub quote_reference: Option<String>,
}

impl CreateSite {
    /// Validate the create site command
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Site name is required".to_string());
        }
        if self.project_type == ProjectType::ExternalSite && self.customer_name.trim().is_empty() {
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
        if let Some(amount) = self.budget_amount_cents {
            if amount < 0 {
                return Err("Budget amount cannot be negative".to_string());
            }
        }
        Ok(())
    }
}

/// Command to update a site
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSite {
    pub project_type: Option<ProjectType>,
    pub name: Option<String>,
    pub customer_name: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: Option<SiteStatus>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub estimated_days: Option<i32>,
    pub budget_amount_cents: Option<i64>,
    pub billing_reference: Option<String>,
    pub billing_notes: Option<String>,
    pub quote_reference: Option<String>,
    pub clear_budget_amount: bool,
    pub clear_billing_reference: bool,
    pub clear_billing_notes: bool,
    pub clear_quote_reference: bool,
}

impl UpdateSite {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(name) = &self.name {
            if name.trim().is_empty() {
                return Err("Site name cannot be empty".to_string());
            }
        }
        if let Some(customer_name) = &self.customer_name {
            if customer_name.trim().is_empty() {
                return Err("Customer name cannot be empty".to_string());
            }
        }
        if let Some(days) = self.estimated_days {
            if days < 0 {
                return Err("Estimated days cannot be negative".to_string());
            }
        }
        if let Some(amount) = self.budget_amount_cents {
            if amount < 0 {
                return Err("Budget amount cannot be negative".to_string());
            }
        }
        Ok(())
    }
}

/// Command to assign a user to a site
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignUser {
    pub user_id: UserId,
    pub role: AssignmentRole,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_site(status: SiteStatus) -> Site {
        Site {
            id: SiteId::new(),
            tenant_id: TenantId::new(),
            project_type: ProjectType::ExternalSite,
            name: "Test Site".to_string(),
            customer_name: "Test Customer".to_string(),
            location: None,
            description: None,
            status,
            start_date: None,
            end_date: None,
            estimated_days: None,
            budget_amount_cents: None,
            billing_reference: None,
            billing_notes: None,
            quote_reference: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // State machine tests
    #[test]
    fn site_can_transition_from_planned_to_active() {
        let site = test_site(SiteStatus::Planned);
        assert!(site.can_transition_to(SiteStatus::Active));
    }

    #[test]
    fn site_cannot_transition_from_planned_to_completed() {
        let site = test_site(SiteStatus::Planned);
        assert!(!site.can_transition_to(SiteStatus::Completed));
    }

    #[test]
    fn site_cannot_transition_from_planned_to_archived() {
        let site = test_site(SiteStatus::Planned);
        assert!(!site.can_transition_to(SiteStatus::Archived));
    }

    #[test]
    fn site_can_transition_from_active_to_completed() {
        let site = test_site(SiteStatus::Active);
        assert!(site.can_transition_to(SiteStatus::Completed));
    }

    #[test]
    fn site_cannot_transition_from_active_to_planned() {
        let site = test_site(SiteStatus::Active);
        assert!(!site.can_transition_to(SiteStatus::Planned));
    }

    #[test]
    fn site_can_transition_from_completed_to_archived() {
        let site = test_site(SiteStatus::Completed);
        assert!(site.can_transition_to(SiteStatus::Archived));
    }

    #[test]
    fn site_cannot_transition_from_completed_to_active() {
        let site = test_site(SiteStatus::Completed);
        assert!(!site.can_transition_to(SiteStatus::Active));
    }

    #[test]
    fn site_can_transition_to_same_status() {
        let site = test_site(SiteStatus::Planned);
        assert!(site.can_transition_to(SiteStatus::Planned));
    }

    // CreateSite validation tests
    #[test]
    fn create_site_validate_succeeds_with_valid_data() {
        let cmd = CreateSite {
            project_type: ProjectType::ExternalSite,
            name: "New Site".to_string(),
            customer_name: "Customer".to_string(),
            location: None,
            description: None,
            start_date: None,
            end_date: None,
            estimated_days: None,
            budget_amount_cents: None,
            billing_reference: None,
            billing_notes: None,
            quote_reference: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_site_validate_fails_with_empty_name() {
        let cmd = CreateSite {
            project_type: ProjectType::ExternalSite,
            name: "".to_string(),
            customer_name: "Customer".to_string(),
            location: None,
            description: None,
            start_date: None,
            end_date: None,
            estimated_days: None,
            budget_amount_cents: None,
            billing_reference: None,
            billing_notes: None,
            quote_reference: None,
        };
        assert_eq!(cmd.validate(), Err("Site name is required".to_string()));
    }

    #[test]
    fn create_site_validate_fails_with_empty_customer_name() {
        let cmd = CreateSite {
            project_type: ProjectType::ExternalSite,
            name: "Site".to_string(),
            customer_name: "".to_string(),
            location: None,
            description: None,
            start_date: None,
            end_date: None,
            estimated_days: None,
            budget_amount_cents: None,
            billing_reference: None,
            billing_notes: None,
            quote_reference: None,
        };
        assert_eq!(cmd.validate(), Err("Customer name is required".to_string()));
    }

    #[test]
    fn create_site_validate_fails_with_end_date_before_start_date() {
        let cmd = CreateSite {
            project_type: ProjectType::ExternalSite,
            name: "Site".to_string(),
            customer_name: "Customer".to_string(),
            location: None,
            description: None,
            start_date: Some(NaiveDate::from_ymd_opt(2024, 12, 15).unwrap()),
            end_date: Some(NaiveDate::from_ymd_opt(2024, 12, 1).unwrap()),
            estimated_days: None,
            budget_amount_cents: None,
            billing_reference: None,
            billing_notes: None,
            quote_reference: None,
        };
        assert_eq!(
            cmd.validate(),
            Err("End date cannot be before start date".to_string())
        );
    }

    #[test]
    fn create_site_validate_fails_with_negative_estimated_days() {
        let cmd = CreateSite {
            project_type: ProjectType::ExternalSite,
            name: "Site".to_string(),
            customer_name: "Customer".to_string(),
            location: None,
            description: None,
            start_date: None,
            end_date: None,
            estimated_days: Some(-5),
            budget_amount_cents: None,
            billing_reference: None,
            billing_notes: None,
            quote_reference: None,
        };
        assert_eq!(
            cmd.validate(),
            Err("Estimated days cannot be negative".to_string())
        );
    }

    #[test]
    fn create_site_validate_allows_internal_workshop_without_location() {
        let cmd = CreateSite {
            project_type: ProjectType::InternalWorkshop,
            name: "Werkstattauftrag".to_string(),
            customer_name: "".to_string(),
            location: None,
            description: Some("Vorbereitung".to_string()),
            start_date: None,
            end_date: None,
            estimated_days: Some(1),
            budget_amount_cents: Some(250000),
            billing_reference: Some("BR-1".to_string()),
            billing_notes: Some("Teilrechnung".to_string()),
            quote_reference: Some("ANG-2026-01".to_string()),
        };

        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_site_validate_fails_with_negative_budget_amount() {
        let cmd = CreateSite {
            project_type: ProjectType::ExternalSite,
            name: "Site".to_string(),
            customer_name: "Customer".to_string(),
            location: None,
            description: None,
            start_date: None,
            end_date: None,
            estimated_days: None,
            budget_amount_cents: Some(-1),
            billing_reference: None,
            billing_notes: None,
            quote_reference: None,
        };

        assert_eq!(
            cmd.validate(),
            Err("Budget amount cannot be negative".to_string())
        );
    }

    #[test]
    fn project_type_roundtrips() {
        let parsed = "internal_workshop".parse::<ProjectType>().unwrap();
        assert_eq!(parsed, ProjectType::InternalWorkshop);
        assert_eq!(parsed.to_string(), "internal_workshop");
        assert!("invalid".parse::<ProjectType>().is_err());
    }
}
