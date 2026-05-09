use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{AssetId, AssetKind, ResourceStatus, ResourceType, TenantId};

/// Core asset identity shared by vehicles, tools, and machines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: AssetId,
    pub tenant_id: TenantId,
    pub kind: AssetKind,
    pub name: String,
    pub description: Option<String>,
    pub status: ResourceStatus,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Asset {
    pub fn resource_type(&self) -> ResourceType {
        self.kind.into()
    }

    pub fn can_be_reserved(&self) -> bool {
        matches!(
            self.kind,
            AssetKind::Vehicle | AssetKind::Tool | AssetKind::Machine
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_asset(kind: AssetKind) -> Asset {
        Asset {
            id: AssetId::new(),
            tenant_id: TenantId::new(),
            kind,
            name: "Test Asset".to_string(),
            description: None,
            status: ResourceStatus::Available,
            location: None,
            qr_code: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn machine_assets_are_reservable() {
        let asset = test_asset(AssetKind::Machine);

        assert!(asset.can_be_reserved());
        assert_eq!(asset.resource_type(), ResourceType::Machine);
    }
}
