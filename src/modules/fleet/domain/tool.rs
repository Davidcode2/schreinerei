use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{TenantId, ToolId, ResourceStatus};

/// Tool aggregate representing a tool (Werkzeug)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: ToolId,
    pub tenant_id: TenantId,
    pub name: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub status: ResourceStatus,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tool {
    /// Check if status transition is valid
    /// Same transitions as Vehicle: Available → Reserved → InUse → Available, any → Maintenance
    pub fn can_transition_to(&self, new_status: ResourceStatus) -> bool {
        match (&self.status, &new_status) {
            // Available → Reserved
            (ResourceStatus::Available, ResourceStatus::Reserved) => true,
            // Available → InUse (direct use without reservation)
            (ResourceStatus::Available, ResourceStatus::InUse) => true,
            // Reserved → InUse
            (ResourceStatus::Reserved, ResourceStatus::InUse) => true,
            // Reserved → Available (cancellation)
            (ResourceStatus::Reserved, ResourceStatus::Available) => true,
            // InUse → Available
            (ResourceStatus::InUse, ResourceStatus::Available) => true,
            // Any → Maintenance
            (_, ResourceStatus::Maintenance) => true,
            // Maintenance → Available
            (ResourceStatus::Maintenance, ResourceStatus::Available) => true,
            // Same status (no change)
            _ if self.status == new_status => true,
            _ => false,
        }
    }
}

/// Command to create a new tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTool {
    pub name: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

impl CreateTool {
    /// Validate the create tool command
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Tool name is required".to_string());
        }
        Ok(())
    }
}

/// Command to update a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTool {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub status: Option<ResourceStatus>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tool(status: ResourceStatus) -> Tool {
        Tool {
            id: ToolId::new(),
            tenant_id: TenantId::new(),
            name: "Test Tool".to_string(),
            category: Some("Power Tools".to_string()),
            description: None,
            status,
            location: None,
            qr_code: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // State machine tests
    #[test]
    fn tool_can_transition_from_available_to_reserved() {
        let t = test_tool(ResourceStatus::Available);
        assert!(t.can_transition_to(ResourceStatus::Reserved));
    }

    #[test]
    fn tool_can_transition_from_available_to_in_use() {
        let t = test_tool(ResourceStatus::Available);
        assert!(t.can_transition_to(ResourceStatus::InUse));
    }

    #[test]
    fn tool_can_transition_from_available_to_maintenance() {
        let t = test_tool(ResourceStatus::Available);
        assert!(t.can_transition_to(ResourceStatus::Maintenance));
    }

    #[test]
    fn tool_can_transition_from_reserved_to_in_use() {
        let t = test_tool(ResourceStatus::Reserved);
        assert!(t.can_transition_to(ResourceStatus::InUse));
    }

    #[test]
    fn tool_can_transition_from_reserved_to_available() {
        let t = test_tool(ResourceStatus::Reserved);
        assert!(t.can_transition_to(ResourceStatus::Available));
    }

    #[test]
    fn tool_can_transition_from_in_use_to_available() {
        let t = test_tool(ResourceStatus::InUse);
        assert!(t.can_transition_to(ResourceStatus::Available));
    }

    #[test]
    fn tool_can_transition_from_maintenance_to_available() {
        let t = test_tool(ResourceStatus::Maintenance);
        assert!(t.can_transition_to(ResourceStatus::Available));
    }

    #[test]
    fn tool_cannot_transition_from_maintenance_to_in_use() {
        let t = test_tool(ResourceStatus::Maintenance);
        assert!(!t.can_transition_to(ResourceStatus::InUse));
    }

    #[test]
    fn tool_can_transition_to_same_status() {
        let t = test_tool(ResourceStatus::Available);
        assert!(t.can_transition_to(ResourceStatus::Available));
    }

    // CreateTool validation tests
    #[test]
    fn create_tool_validate_succeeds_with_valid_data() {
        let cmd = CreateTool {
            name: "Circular Saw".to_string(),
            category: Some("Power Tools".to_string()),
            description: None,
            location: None,
            qr_code: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_tool_validate_fails_with_empty_name() {
        let cmd = CreateTool {
            name: "".to_string(),
            category: None,
            description: None,
            location: None,
            qr_code: None,
        };
        assert_eq!(cmd.validate(), Err("Tool name is required".to_string()));
    }
}
