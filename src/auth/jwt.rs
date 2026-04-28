use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use jsonwebtoken::jwk::JwkSet;
use serde::{Deserialize, Serialize};

use crate::common::error::AppError;

/// Keycloak JWT claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (User ID)
    pub sub: String,
    /// Email address
    pub email: String,
    /// Preferred username
    pub preferred_username: Option<String>,
    /// Tenant ID (custom mapper in Keycloak)
    pub tenant_id: String,
    /// Realm access roles
    pub realm_access: RealmAccess,
    /// Expiration timestamp
    pub exp: usize,
    /// Issued at timestamp
    pub iat: usize,
}

/// Realm access from Keycloak token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

/// Validate a JWT token against the JWKS
pub fn validate_jwt(
    token: &str,
    jwks: &JwkSet,
    issuer: &str,
) -> Result<Claims, AppError> {
    // Extract header to get kid
    let header = jsonwebtoken::decode_header(token)
        .map_err(|e| AppError::Auth(format!("Failed to decode JWT header: {}", e)))?;

    let kid = header.kid.ok_or_else(|| {
        AppError::Auth("JWT header missing 'kid' field".to_string())
    })?;

    // Find matching key in JWKS
    let jwk = jwks.keys.iter()
        .find(|k| k.common.key_id.as_deref() == Some(kid.as_str()))
        .ok_or_else(|| AppError::Auth("No matching key found in JWKS".to_string()))?;

    // Create decoding key from JWK
    let decoding_key = DecodingKey::from_jwk(jwk)
        .map_err(|e| AppError::Auth(format!("Failed to create decoding key: {}", e)))?;

    // Set up validation
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[issuer]);
    validation.set_required_spec_claims(&["exp", "iat", "sub"]);

    // Decode and validate
    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| AppError::Auth(format!("JWT validation failed: {}", e)))?;

    Ok(token_data.claims)
}
