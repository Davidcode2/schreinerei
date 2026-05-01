use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{TenantId, MaterialId, UserId, SiteId};

/// Stock entry record representing a stock change (withdrawal or adjustment)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockEntry {
    pub id: uuid::Uuid,
    pub tenant_id: TenantId,
    pub material_id: MaterialId,
    pub user_id: UserId,
    pub quantity_change: i32,
    pub quantity_after: i32,
    pub notes: Option<String>,
    pub site_id: Option<SiteId>,
    pub created_at: DateTime<Utc>,
}

/// Stock entry with optional site name for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockEntryWithSite {
    pub id: uuid::Uuid,
    pub tenant_id: TenantId,
    pub material_id: MaterialId,
    pub user_id: UserId,
    pub quantity_change: i32,
    pub quantity_after: i32,
    pub notes: Option<String>,
    pub site_id: Option<SiteId>,
    pub site_name: Option<String>,  // Resolved site name
    pub created_at: DateTime<Utc>,
}

/// Site-scoped stock history projection with material and user context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteStockHistoryEntry {
    pub id: uuid::Uuid,
    pub tenant_id: TenantId,
    pub material_id: MaterialId,
    pub material_name: String,
    pub category_name: String,
    pub user_id: UserId,
    pub extracted_by: String,
    pub quantity_change: i32,
    pub quantity_after: i32,
    pub notes: Option<String>,
    pub site_id: Option<SiteId>,
    pub site_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl StockEntryWithSite {
    /// Check if this is a withdrawal (negative change)
    pub fn is_withdrawal(&self) -> bool {
        self.quantity_change < 0
    }

    /// Get absolute quantity withdrawn
    pub fn withdrawn_quantity(&self) -> i32 {
        self.quantity_change.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn test_entry_with_site(quantity_change: i32, site_name: Option<&str>) -> StockEntryWithSite {
        StockEntryWithSite {
            id: uuid::Uuid::new_v4(),
            tenant_id: TenantId::new(),
            material_id: MaterialId::new(),
            user_id: UserId::new(),
            quantity_change,
            quantity_after: 10,
            notes: None,
            site_id: site_name.map(|_| SiteId::new()),
            site_name: site_name.map(String::from),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn stock_entry_is_withdrawal_returns_true_for_negative_change() {
        let entry = test_entry_with_site(-5, None);
        assert!(entry.is_withdrawal());
    }

    #[test]
    fn stock_entry_is_withdrawal_returns_false_for_positive_change() {
        let entry = test_entry_with_site(5, None);
        assert!(!entry.is_withdrawal());
    }

    #[test]
    fn stock_entry_is_withdrawal_returns_false_for_zero_change() {
        let entry = test_entry_with_site(0, None);
        assert!(!entry.is_withdrawal());
    }

    #[test]
    fn stock_entry_withdrawn_quantity_returns_absolute_value() {
        let entry = test_entry_with_site(-7, None);
        assert_eq!(entry.withdrawn_quantity(), 7);
    }

    #[test]
    fn stock_entry_site_name_is_some_when_set() {
        let entry = test_entry_with_site(-5, Some("Baustelle Müller"));
        assert_eq!(entry.site_name, Some("Baustelle Müller".to_string()));
    }

    #[test]
    fn stock_entry_site_name_is_none_when_not_set() {
        let entry = test_entry_with_site(-5, None);
        assert!(entry.site_name.is_none());
    }
}
