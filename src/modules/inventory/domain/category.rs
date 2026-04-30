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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_category_validate_succeeds_with_valid_name() {
        let cmd = CreateCategory {
            name: "Platten".to_string(),
            description: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_category_validate_fails_with_empty_name() {
        let cmd = CreateCategory {
            name: "".to_string(),
            description: None,
        };
        assert_eq!(cmd.validate(), Err("Category name is required".to_string()));
    }

    #[test]
    fn create_category_validate_fails_with_whitespace_name() {
        let cmd = CreateCategory {
            name: "   ".to_string(),
            description: None,
        };
        assert_eq!(cmd.validate(), Err("Category name is required".to_string()));
    }

    #[test]
    fn create_category_validate_fails_with_too_long_name() {
        let cmd = CreateCategory {
            name: "a".repeat(101),
            description: None,
        };
        assert_eq!(cmd.validate(), Err("Category name too long (max 100 chars)".to_string()));
    }
}
