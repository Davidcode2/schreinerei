use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::common::types::TenantId;

/// Tenant aggregate representing a customer organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: TenantId,
    pub keycloak_realm: String,
    pub name: String,
    pub slug: TenantSlug,
    /// Keycloak Organization UUID for organization-based tenancy
    pub keycloak_organization_id: Option<uuid::Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Tenant slug value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TenantSlug(String);

impl TenantSlug {
    /// Create a new tenant slug with validation
    pub fn new(slug: String) -> Result<Self, String> {
        if slug.is_empty() {
            return Err("Slug cannot be empty".to_string());
        }
        if slug.len() > 100 {
            return Err("Slug cannot exceed 100 characters".to_string());
        }
        // Only allow alphanumeric and hyphens
        if !slug.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return Err("Slug can only contain alphanumeric characters and hyphens".to_string());
        }
        // Cannot start or end with hyphen
        if slug.starts_with('-') || slug.ends_with('-') {
            return Err("Slug cannot start or end with hyphen".to_string());
        }
        Ok(Self(slug))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TenantSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for TenantSlug {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tenant_slug_new_succeeds_with_valid_slug() {
        let result = TenantSlug::new("my-tenant".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "my-tenant");
    }

    #[test]
    fn tenant_slug_new_fails_with_empty_string() {
        let result = TenantSlug::new("".to_string());
        assert_eq!(result, Err("Slug cannot be empty".to_string()));
    }

    #[test]
    fn tenant_slug_new_fails_with_too_long_slug() {
        let long_slug = "a".repeat(101);
        let result = TenantSlug::new(long_slug);
        assert_eq!(result, Err("Slug cannot exceed 100 characters".to_string()));
    }

    #[test]
    fn tenant_slug_new_fails_with_invalid_characters() {
        let result = TenantSlug::new("invalid_slug!".to_string());
        assert_eq!(
            result,
            Err("Slug can only contain alphanumeric characters and hyphens".to_string())
        );
    }

    #[test]
    fn tenant_slug_new_fails_when_starts_with_hyphen() {
        let result = TenantSlug::new("-starts-hyphen".to_string());
        assert_eq!(
            result,
            Err("Slug cannot start or end with hyphen".to_string())
        );
    }

    #[test]
    fn tenant_slug_new_fails_when_ends_with_hyphen() {
        let result = TenantSlug::new("ends-hyphen-".to_string());
        assert_eq!(
            result,
            Err("Slug cannot start or end with hyphen".to_string())
        );
    }

    #[test]
    fn tenant_slug_from_str_succeeds_with_valid_string() {
        let result: Result<TenantSlug, String> = "valid-slug".parse();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "valid-slug");
    }

    #[test]
    fn tenant_slug_display_formats_correctly() {
        let slug = TenantSlug::new("my-tenant".to_string()).unwrap();
        assert_eq!(format!("{}", slug), "my-tenant");
    }
}
