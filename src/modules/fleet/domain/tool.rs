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
