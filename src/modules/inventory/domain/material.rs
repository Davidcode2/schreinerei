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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn test_material(quantity: i32, min_quantity: i32) -> Material {
        Material {
            id: MaterialId::new(),
            tenant_id: TenantId::new(),
            category_id: CategoryId::new(),
            name: "Test Material".to_string(),
            description: None,
            unit: Unit::Piece,
            quantity,
            min_quantity,
            location: None,
            qr_code: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn material_is_low_stock_returns_true_when_at_or_below_minimum() {
        let material = test_material(5, 10);
        assert!(material.is_low_stock());
    }

    #[test]
    fn material_is_low_stock_returns_false_when_above_minimum() {
        let material = test_material(15, 10);
        assert!(!material.is_low_stock());
    }

    #[test]
    fn material_is_last_unit_returns_true_when_quantity_is_one() {
        let material = test_material(1, 0);
        assert!(material.is_last_unit());
    }

    #[test]
    fn material_is_last_unit_returns_false_when_quantity_not_one() {
        let material = test_material(5, 0);
        assert!(!material.is_last_unit());
    }

    #[test]
    fn material_can_withdraw_returns_true_when_sufficient_stock() {
        let material = test_material(10, 0);
        assert!(material.can_withdraw(5));
    }

    #[test]
    fn material_can_withdraw_returns_true_when_exact_amount() {
        let material = test_material(10, 0);
        assert!(material.can_withdraw(10));
    }

    #[test]
    fn material_can_withdraw_returns_false_when_insufficient_stock() {
        let material = test_material(10, 0);
        assert!(!material.can_withdraw(11));
    }

    #[test]
    fn create_material_validate_succeeds_with_valid_data() {
        let cmd = CreateMaterial {
            category_id: CategoryId::new(),
            name: "Wood".to_string(),
            description: None,
            unit: Unit::Piece,
            quantity: 10,
            min_quantity: 5,
            location: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_material_validate_fails_with_empty_name() {
        let cmd = CreateMaterial {
            category_id: CategoryId::new(),
            name: "".to_string(),
            description: None,
            unit: Unit::Piece,
            quantity: 10,
            min_quantity: 5,
            location: None,
        };
        assert_eq!(cmd.validate(), Err("Material name is required".to_string()));
    }

    #[test]
    fn create_material_validate_fails_with_negative_quantity() {
        let cmd = CreateMaterial {
            category_id: CategoryId::new(),
            name: "Wood".to_string(),
            description: None,
            unit: Unit::Piece,
            quantity: -1,
            min_quantity: 5,
            location: None,
        };
        assert_eq!(cmd.validate(), Err("Quantity cannot be negative".to_string()));
    }

    #[test]
    fn create_material_validate_fails_with_negative_min_quantity() {
        let cmd = CreateMaterial {
            category_id: CategoryId::new(),
            name: "Wood".to_string(),
            description: None,
            unit: Unit::Piece,
            quantity: 10,
            min_quantity: -1,
            location: None,
        };
        assert_eq!(cmd.validate(), Err("Minimum quantity cannot be negative".to_string()));
    }

    #[test]
    fn withdraw_material_validate_succeeds_with_positive_quantity() {
        let cmd = WithdrawMaterial {
            material_id: MaterialId::new(),
            quantity: 1,
            notes: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn withdraw_material_validate_fails_with_zero_quantity() {
        let cmd = WithdrawMaterial {
            material_id: MaterialId::new(),
            quantity: 0,
            notes: None,
        };
        assert_eq!(cmd.validate(), Err("Withdrawal quantity must be positive".to_string()));
    }

    #[test]
    fn withdraw_material_validate_fails_with_negative_quantity() {
        let cmd = WithdrawMaterial {
            material_id: MaterialId::new(),
            quantity: -1,
            notes: None,
        };
        assert_eq!(cmd.validate(), Err("Withdrawal quantity must be positive".to_string()));
    }

    #[test]
    fn adjust_stock_validate_succeeds_with_reason() {
        let cmd = AdjustStock {
            material_id: MaterialId::new(),
            quantity: 5,
            reason: "Found more stock".to_string(),
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn adjust_stock_validate_fails_with_empty_reason() {
        let cmd = AdjustStock {
            material_id: MaterialId::new(),
            quantity: 5,
            reason: "".to_string(),
        };
        assert_eq!(cmd.validate(), Err("Reason is required for stock adjustment".to_string()));
    }
}
