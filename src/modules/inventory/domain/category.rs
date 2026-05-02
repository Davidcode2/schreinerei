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

/// Command to update a category (PATCH semantics)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl UpdateCategory {
    /// Validate the update category command
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("Category name cannot be empty".to_string());
            }
            if name.len() > 100 {
                return Err("Category name too long (max 100 chars)".to_string());
            }
        }
        // description: Some("") clears the description, None keeps it unchanged
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
        assert_eq!(
            cmd.validate(),
            Err("Category name too long (max 100 chars)".to_string())
        );
    }

    #[test]
    fn create_category_validate_succeeds_with_description() {
        let cmd = CreateCategory {
            name: "Beschläge".to_string(),
            description: Some("Schubladenauszüge".to_string()),
        };
        assert!(cmd.validate().is_ok());
    }

    // === UpdateCategory tests ===

    #[test]
    fn update_category_validate_succeeds_with_valid_name() {
        let cmd = UpdateCategory {
            name: Some("New Name".to_string()),
            description: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn update_category_validate_succeeds_with_description_only() {
        let cmd = UpdateCategory {
            name: None,
            description: Some("New description".to_string()),
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn update_category_validate_succeeds_with_clear_description() {
        let cmd = UpdateCategory {
            name: None,
            description: Some("".to_string()),
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn update_category_validate_succeeds_with_name_and_description() {
        let cmd = UpdateCategory {
            name: Some("Plattenwerkstoffe".to_string()),
            description: Some("Lager Nord".to_string()),
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn update_category_validate_fails_with_empty_name() {
        let cmd = UpdateCategory {
            name: Some("".to_string()),
            description: None,
        };
        assert_eq!(
            cmd.validate(),
            Err("Category name cannot be empty".to_string())
        );
    }

    #[test]
    fn update_category_validate_fails_with_whitespace_name() {
        let cmd = UpdateCategory {
            name: Some("   ".to_string()),
            description: None,
        };
        assert_eq!(
            cmd.validate(),
            Err("Category name cannot be empty".to_string())
        );
    }

    #[test]
    fn update_category_validate_fails_with_too_long_name() {
        let cmd = UpdateCategory {
            name: Some("a".repeat(101)),
            description: None,
        };
        assert_eq!(
            cmd.validate(),
            Err("Category name too long (max 100 chars)".to_string())
        );
    }

    #[test]
    fn update_category_validate_succeeds_with_no_changes() {
        let cmd = UpdateCategory {
            name: None,
            description: None,
        };
        assert!(cmd.validate().is_ok());
    }
}
