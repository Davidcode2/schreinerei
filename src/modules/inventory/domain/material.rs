use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{TenantId, MaterialId, CategoryId, Unit};

/// Material aggregate with stock information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub id: MaterialId,
    pub tenant_id: TenantId,
    pub category_id: CategoryId,
    pub name: String,
    pub description: Option<String>,
    pub unit: Unit,
    pub quantity: i32,
    pub min_quantity: i32,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Material {
    /// Check if stock is at or below minimum threshold
    pub fn is_low_stock(&self) -> bool {
        self.quantity <= self.min_quantity
    }

    /// Check if this is the last unit (for warning)
    pub fn is_last_unit(&self) -> bool {
        self.quantity == 1
    }

    /// Check if can withdraw the specified amount
    pub fn can_withdraw(&self, amount: i32) -> bool {
        self.quantity >= amount
    }
}

/// Command to create a new material
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMaterial {
    pub category_id: CategoryId,
    pub name: String,
    pub description: Option<String>,
    pub unit: Unit,
    pub quantity: i32,
    pub min_quantity: i32,
    pub location: Option<String>,
}

impl CreateMaterial {
    /// Validate the create material command
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Material name is required".to_string());
        }
        if self.quantity < 0 {
            return Err("Quantity cannot be negative".to_string());
        }
        if self.min_quantity < 0 {
            return Err("Minimum quantity cannot be negative".to_string());
        }
        Ok(())
    }
}

/// Command to withdraw material from stock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawMaterial {
    pub material_id: MaterialId,
    pub quantity: i32,
    pub notes: Option<String>,
}

impl WithdrawMaterial {
    /// Validate the withdraw command
    pub fn validate(&self) -> Result<(), String> {
        if self.quantity <= 0 {
            return Err("Withdrawal quantity must be positive".to_string());
        }
        Ok(())
    }
}

/// Command to adjust stock (add or remove)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustStock {
    pub material_id: MaterialId,
    pub quantity: i32,
    pub reason: String,
}

impl AdjustStock {
    /// Validate the adjust stock command
    pub fn validate(&self) -> Result<(), String> {
        if self.reason.trim().is_empty() {
            return Err("Reason is required for stock adjustment".to_string());
        }
        Ok(())
    }
}
