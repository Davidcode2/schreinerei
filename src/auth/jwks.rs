use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use jsonwebtoken::jwk::JwkSet;
use reqwest::Client;

use crate::common::error::AppError;

/// JWKS client that fetches and caches keys from Keycloak
#[derive(Debug, Clone)]
pub struct JwksClient {
    client: Client,
    jwks_url: String,
    cache: Arc<RwLock<Option<JwkSet>>>,
}

impl JwksClient {
    /// Create a new JWKS client
    pub fn new(keycloak_url: &str, realm: &str) -> Self {
        let jwks_url = format!(
            "{}/realms/{}/protocol/openid-connect/certs",
            keycloak_url, realm
        );

        Self {
            client: Client::new(),
            jwks_url,
            cache: Arc::new(RwLock::new(None)),
        }
    }

    /// Fetch JWKS from Keycloak
    pub async fn fetch_jwks(&self) -> Result<JwkSet, AppError> {
        let response = self.client
            .get(&self.jwks_url)
            .send()
            .await
            .map_err(|e| AppError::Auth(format!("Failed to fetch JWKS: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Auth(format!(
                "JWKS endpoint returned status: {}",
                response.status()
            )));
        }

        let jwks: JwkSet = response
            .json()
            .await
            .map_err(|e| AppError::Auth(format!("Failed to parse JWKS: {}", e)))?;

        // Update cache
        let mut cache = self.cache.write().await;
        *cache = Some(jwks.clone());

        tracing::debug!("Fetched and cached {} JWKS keys", jwks.keys.len());

        Ok(jwks)
    }

    /// Get cached JWKS, fetching if not present
    pub async fn get_jwks(&self) -> Result<JwkSet, AppError> {
        let cache = self.cache.read().await;
        if let Some(jwks) = cache.as_ref() {
            return Ok(jwks.clone());
        }
        drop(cache);

        // Not cached, fetch it
        self.fetch_jwks().await
    }

    /// Start periodic refresh of JWKS (every hour)
    pub fn refresh_periodically(self) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(3600));
            loop {
                interval.tick().await;
                match self.fetch_jwks().await {
                    Ok(jwks) => {
                        tracing::debug!("Refreshed JWKS cache with {} keys", jwks.keys.len());
                    }
                    Err(e) => {
                        tracing::warn!("Failed to refresh JWKS: {}", e);
                    }
                }
            }
        });
    }
}
