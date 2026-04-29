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
