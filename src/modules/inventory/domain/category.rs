use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{CategoryId, TenantId};

/// Material category (Platten, Kanten, Lacke, Schrauben, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: CategoryId,
    pub tenant_id: TenantId,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Command to create a new category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub description: Option<String>,
}

impl CreateCategory {
    /// Validate the create category command
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Category name is required".to_string());
        }
        if self.name.len() > 100 {
            return Err("Category name too long (max 100 chars)".to_string());
        }
        Ok(())
    }
}
