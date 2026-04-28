use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::events::{DomainEvent, EventType};
use crate::common::types::{TenantId, SiteId, UserId, SiteStatus};

/// Payload for SiteCreated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteCreatedPayload {
    pub site_id: SiteId,
    pub name: String,
    pub customer_name: String,
}

impl SiteCreatedPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::SiteCreated,
            tenant_id,
            "Site",
            self.site_id.to_string(),
            json!(self),
        )
    }
}

/// Payload for SiteStatusChanged event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteStatusChangedPayload {
    pub site_id: SiteId,
    pub old_status: String,
    pub new_status: String,
    pub changed_by: UserId,
}

impl SiteStatusChangedPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::SiteStatusChanged,
            tenant_id,
            "Site",
            self.site_id.to_string(),
            json!(self),
        )
    }
}

/// Payload for UserAssignedToSite event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAssignedToSitePayload {
    pub site_id: SiteId,
    pub user_id: UserId,
    pub role: String,
    pub assigned_by: UserId,
}

impl UserAssignedToSitePayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::UserAssignedToSite,
            tenant_id,
            "Site",
            self.site_id.to_string(),
            json!(self),
        )
    }
}

/// Payload for TimeEntryCreated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryCreatedPayload {
    pub site_id: Option<SiteId>,
    pub user_id: UserId,
    pub hours: f64,
    pub work_type: String,
    pub work_date: String,
}

impl TimeEntryCreatedPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::TimeEntryCreated,
            tenant_id,
            "TimeEntry",
            tenant_id.to_string(), // aggregate_id is tenant for time entries
            json!(self),
        )
    }
}
