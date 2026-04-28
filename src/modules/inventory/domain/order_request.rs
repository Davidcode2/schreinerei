use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{TenantId, MaterialId, UserId, OrderRequestId};

/// Order request for restocking materials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub id: OrderRequestId,
    pub tenant_id: TenantId,
    pub material_id: MaterialId,
    pub quantity: i32,
    pub requested_by: UserId,
    pub status: OrderStatus,
    pub reason: Option<String>,
    pub approved_by: Option<UserId>,
    pub approved_at: Option<DateTime<Utc>>,
    pub fulfilled_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Approved,
    Ordered,
    Fulfilled,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Pending => "pending",
            OrderStatus::Approved => "approved",
            OrderStatus::Ordered => "ordered",
            OrderStatus::Fulfilled => "fulfilled",
            OrderStatus::Cancelled => "cancelled",
        }
    }
}

impl std::str::FromStr for OrderStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(OrderStatus::Pending),
            "approved" => Ok(OrderStatus::Approved),
            "ordered" => Ok(OrderStatus::Ordered),
            "fulfilled" => Ok(OrderStatus::Fulfilled),
            "cancelled" => Ok(OrderStatus::Cancelled),
            _ => Err(format!("Unknown order status: {}", s)),
        }
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub material_id: MaterialId,
    pub quantity: i32,
    pub reason: Option<String>,
}

impl CreateOrderRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.quantity <= 0 {
            return Err("Quantity must be positive".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveOrderRequest {
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillOrderRequest {
    pub actual_quantity: i32,
    pub notes: Option<String>,
}
