use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::events::{DomainEvent, EventType};
use crate::common::types::{TenantId, VehicleId, ToolId, ResourceType};

/// Payload for VehicleCreated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleCreatedPayload {
    pub vehicle_id: VehicleId,
    pub name: String,
    pub vehicle_type: String,
}

impl VehicleCreatedPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::VehicleCreated,
            tenant_id,
            "Vehicle",
            self.vehicle_id.to_string(),
            json!(self),
        )
    }
}

/// Payload for ToolCreated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCreatedPayload {
    pub tool_id: ToolId,
    pub name: String,
    pub category: Option<String>,
}

impl ToolCreatedPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::ToolCreated,
            tenant_id,
            "Tool",
            self.tool_id.to_string(),
            json!(self),
        )
    }
}

/// Payload for ResourceStatusChanged event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStatusChangedPayload {
    pub resource_type: ResourceType,
    pub resource_id: String,
    pub old_status: String,
    pub new_status: String,
}

impl ResourceStatusChangedPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::ResourceStatusChanged,
            tenant_id,
            self.resource_type.to_string(),
            self.resource_id.clone(),
            json!(self),
        )
    }
}
