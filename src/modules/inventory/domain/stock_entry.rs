use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use ts_rs::TS;

use crate::common::types::{TenantId, MaterialId, UserId, SiteId};

/// Entry type for stock history entries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub enum EntryType {
    Withdrawn,
    Adjusted,
    MaterialAdded,
    LocationChanged,
    MinQuantityChanged,
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntryType::Withdrawn => write!(f, "withdrawn"),
            EntryType::Adjusted => write!(f, "adjusted"),
            EntryType::MaterialAdded => write!(f, "material_added"),
            EntryType::LocationChanged => write!(f, "location_changed"),
            EntryType::MinQuantityChanged => write!(f, "min_quantity_changed"),
        }
    }
}

impl FromStr for EntryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "withdrawn" => Ok(EntryType::Withdrawn),
            "adjusted" => Ok(EntryType::Adjusted),
            "material_added" => Ok(EntryType::MaterialAdded),
            "location_changed" => Ok(EntryType::LocationChanged),
            "min_quantity_changed" => Ok(EntryType::MinQuantityChanged),
            _ => Err(format!("Unknown entry type: {}", s)),
        }
    }
}

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
    pub entry_type: EntryType,
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

/// Enriched stock entry with resolved names for display in history feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedStockEntry {
    pub id: uuid::Uuid,
    pub tenant_id: TenantId,
    pub material_id: MaterialId,
    pub user_id: UserId,
    pub user_name: String,
    pub entry_type: EntryType,
    pub quantity_change: i32,
    pub quantity_after: i32,
    pub notes: Option<String>,
    pub site_id: Option<SiteId>,
    pub site_name: Option<String>,
    pub category_name: String,
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
            entry_type: if quantity_change < 0 { EntryType::Withdrawn } else { EntryType::Adjusted },
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

    // === EntryType tests ===

    #[test]
    fn entry_type_display_roundtrip_withdrawn() {
        let et = EntryType::Withdrawn;
        assert_eq!(et.to_string(), "withdrawn");
        assert_eq!(EntryType::from_str("withdrawn").unwrap(), EntryType::Withdrawn);
    }

    #[test]
    fn entry_type_display_roundtrip_adjusted() {
        let et = EntryType::Adjusted;
        assert_eq!(et.to_string(), "adjusted");
        assert_eq!(EntryType::from_str("adjusted").unwrap(), EntryType::Adjusted);
    }

    #[test]
    fn entry_type_display_roundtrip_material_added() {
        let et = EntryType::MaterialAdded;
        assert_eq!(et.to_string(), "material_added");
        assert_eq!(EntryType::from_str("material_added").unwrap(), EntryType::MaterialAdded);
    }

    #[test]
    fn entry_type_display_roundtrip_location_changed() {
        let et = EntryType::LocationChanged;
        assert_eq!(et.to_string(), "location_changed");
        assert_eq!(EntryType::from_str("location_changed").unwrap(), EntryType::LocationChanged);
    }

    #[test]
    fn entry_type_display_roundtrip_min_quantity_changed() {
        let et = EntryType::MinQuantityChanged;
        assert_eq!(et.to_string(), "min_quantity_changed");
        assert_eq!(EntryType::from_str("min_quantity_changed").unwrap(), EntryType::MinQuantityChanged);
    }

    #[test]
    fn entry_type_from_str_rejects_unknown() {
        assert!(EntryType::from_str("unknown").is_err());
        assert!(EntryType::from_str("").is_err());
    }

    // === EnrichedStockEntry tests ===

    #[test]
    fn enriched_stock_entry_construction_with_all_fields() {
        let entry = EnrichedStockEntry {
            id: uuid::Uuid::new_v4(),
            tenant_id: TenantId::new(),
            material_id: MaterialId::new(),
            user_id: UserId::new(),
            user_name: "Max Mustermann".to_string(),
            entry_type: EntryType::MaterialAdded,
            quantity_change: 10,
            quantity_after: 25,
            notes: Some("Delivered".to_string()),
            site_id: Some(SiteId::new()),
            site_name: Some("Baustelle Müller".to_string()),
            category_name: "Platten".to_string(),
            created_at: Utc::now(),
        };
        assert_eq!(entry.user_name, "Max Mustermann");
        assert_eq!(entry.entry_type, EntryType::MaterialAdded);
        assert_eq!(entry.category_name, "Platten");
        assert_eq!(entry.quantity_change, 10);
        assert_eq!(entry.quantity_after, 25);
    }
}
