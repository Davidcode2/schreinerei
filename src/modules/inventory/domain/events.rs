use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::events::{DomainEvent, EventType};
use crate::common::types::{TenantId, MaterialId, UserId};

/// Payload for StockLow event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockLowPayload {
    pub material_id: MaterialId,
    pub material_name: String,
    pub current_quantity: i32,
    pub min_quantity: i32,
    pub triggered_by_user_id: Option<UserId>,
}

impl StockLowPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::StockLow,
            tenant_id,
            "Material",
            self.material_id.to_string(),
            json!(self),
        )
    }
}

/// Payload for StockWithdrawn event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockWithdrawnPayload {
    pub material_id: MaterialId,
    pub material_name: String,
    pub quantity_withdrawn: i32,
    pub quantity_after: i32,
    pub withdrawn_by: UserId,
    pub notes: Option<String>,
    pub is_last_unit: bool,
}

impl StockWithdrawnPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::StockWithdrawn,
            tenant_id,
            "Material",
            self.material_id.to_string(),
            json!(self),
        )
    }
}

/// Payload for MaterialCreated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialCreatedPayload {
    pub material_id: MaterialId,
    pub material_name: String,
    pub category_id: String,
    pub initial_quantity: i32,
    pub created_by: UserId,
}

impl MaterialCreatedPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::MaterialCreated,
            tenant_id,
            "Material",
            self.material_id.to_string(),
            json!(self),
        )
    }
}

/// Payload for StockAdjusted event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAdjustedPayload {
    pub material_id: MaterialId,
    pub material_name: String,
    pub quantity_change: i32,
    pub quantity_after: i32,
    pub reason: String,
    pub adjusted_by: UserId,
}

impl StockAdjustedPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::StockAdjusted,
            tenant_id,
            "Material",
            self.material_id.to_string(),
            json!(self),
        )
    }
}

/// Payload for OrderRequestCreated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequestCreatedPayload {
    pub order_request_id: String,
    pub material_id: MaterialId,
    pub material_name: String,
    pub quantity_requested: i32,
    pub requested_by: UserId,
    pub reason: Option<String>,
}

impl OrderRequestCreatedPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        let order_request_id = self.order_request_id.clone();
        DomainEvent::new(
            EventType::OrderRequestCreated,
            tenant_id,
            "OrderRequest",
            order_request_id,
            json!(self),
        )
    }
}
