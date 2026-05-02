use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{MaterialId, OrderRequestId, TenantId, UserId};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_order_request_validate_succeeds_with_positive_quantity() {
        let cmd = CreateOrderRequest {
            material_id: MaterialId::new(),
            quantity: 10,
            reason: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_order_request_validate_fails_with_zero_quantity() {
        let cmd = CreateOrderRequest {
            material_id: MaterialId::new(),
            quantity: 0,
            reason: None,
        };
        assert_eq!(cmd.validate(), Err("Quantity must be positive".to_string()));
    }

    #[test]
    fn create_order_request_validate_fails_with_negative_quantity() {
        let cmd = CreateOrderRequest {
            material_id: MaterialId::new(),
            quantity: -5,
            reason: None,
        };
        assert_eq!(cmd.validate(), Err("Quantity must be positive".to_string()));
    }

    #[test]
    fn order_status_as_str_returns_correct_strings() {
        assert_eq!(OrderStatus::Pending.as_str(), "pending");
        assert_eq!(OrderStatus::Approved.as_str(), "approved");
        assert_eq!(OrderStatus::Ordered.as_str(), "ordered");
        assert_eq!(OrderStatus::Fulfilled.as_str(), "fulfilled");
        assert_eq!(OrderStatus::Cancelled.as_str(), "cancelled");
    }

    #[test]
    fn order_status_from_str_parses_valid_strings() {
        let pending: Result<OrderStatus, String> = "pending".parse();
        assert_eq!(pending, Ok(OrderStatus::Pending));

        let approved: Result<OrderStatus, String> = "approved".parse();
        assert_eq!(approved, Ok(OrderStatus::Approved));

        let ordered: Result<OrderStatus, String> = "ordered".parse();
        assert_eq!(ordered, Ok(OrderStatus::Ordered));

        let fulfilled: Result<OrderStatus, String> = "fulfilled".parse();
        assert_eq!(fulfilled, Ok(OrderStatus::Fulfilled));

        let cancelled: Result<OrderStatus, String> = "cancelled".parse();
        assert_eq!(cancelled, Ok(OrderStatus::Cancelled));
    }

    #[test]
    fn order_status_from_str_fails_with_invalid_string() {
        let result: Result<OrderStatus, String> = "invalid".parse();
        assert!(result.is_err());
    }

    #[test]
    fn order_status_display_matches_as_str() {
        assert_eq!(OrderStatus::Pending.to_string(), "pending");
        assert_eq!(OrderStatus::Approved.to_string(), "approved");
        assert_eq!(OrderStatus::Ordered.to_string(), "ordered");
        assert_eq!(OrderStatus::Fulfilled.to_string(), "fulfilled");
        assert_eq!(OrderStatus::Cancelled.to_string(), "cancelled");
    }
}
