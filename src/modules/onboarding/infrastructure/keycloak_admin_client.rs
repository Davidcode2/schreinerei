use reqwest::Client;
use serde::Deserialize;

use crate::common::error::AppError;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct KeycloakAdminClient {
    base_url: String,
    realm: String,
    admin_realm: String,
    client_id: String,
    client_secret: String,
    client: Client,
}

impl KeycloakAdminClient {
    pub fn from_config(config: &AppConfig) -> Result<Self, AppError> {
        let client_id = required_config(
            config.keycloak_admin_client_id.as_deref(),
            "KEYCLOAK_ADMIN_CLIENT_ID",
        )?;
        let client_secret = required_config(
            config.keycloak_admin_client_secret.as_deref(),
            "KEYCLOAK_ADMIN_CLIENT_SECRET",
        )?;
        let admin_realm = config
            .keycloak_admin_realm
            .as_deref()
            .filter(|realm| !realm.trim().is_empty())
            .unwrap_or(&config.keycloak_realm)
            .to_string();

        Ok(Self {
            base_url: config.keycloak_url.trim_end_matches('/').to_string(),
            realm: config.keycloak_realm.clone(),
            admin_realm,
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            client: Client::new(),
        })
    }

    pub async fn invite_user_to_organization(
        &self,
        organization_id: &str,
        invite: &KeycloakOrganizationInvite,
    ) -> Result<(), AppError> {
        let token = self.admin_access_token().await?;
        let url = format!(
            "{}/admin/realms/{}/organizations/{}/members/invite-user",
            self.base_url, self.realm, organization_id
        );

        let mut form = vec![("email", invite.email.as_str())];
        if let Some(first_name) = invite.first_name.as_deref() {
            form.push(("firstName", first_name));
        }
        if let Some(last_name) = invite.last_name.as_deref() {
            form.push(("lastName", last_name));
        }

        let response = self
            .client
            .post(url)
            .bearer_auth(token)
            .form(&form)
            .send()
            .await
            .map_err(|error| AppError::Internal(format!("Keycloak invite failed: {error}")))?;

        if response.status().is_success() {
            return Ok(());
        }

        Err(AppError::Validation(format!(
            "Keycloak invite failed with status {}",
            response.status()
        )))
    }

    async fn admin_access_token(&self) -> Result<String, AppError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.admin_realm
        );
        let response = self
            .client
            .post(url)
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
            ])
            .send()
            .await
            .map_err(|error| {
                AppError::Internal(format!("Keycloak admin token request failed: {error}"))
            })?;

        if !response.status().is_success() {
            return Err(AppError::Validation(format!(
                "Keycloak admin token request failed with status {}",
                response.status()
            )));
        }

        response
            .json::<TokenResponse>()
            .await
            .map(|token| token.access_token)
            .map_err(|error| {
                AppError::Internal(format!("Failed to parse Keycloak admin token: {error}"))
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeycloakOrganizationInvite {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl KeycloakOrganizationInvite {
    pub fn new(email: String, name: Option<String>) -> Self {
        let (first_name, last_name) = split_name(name);
        Self {
            email,
            first_name,
            last_name,
        }
    }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
}

fn required_config<'a>(value: Option<&'a str>, name: &str) -> Result<&'a str, AppError> {
    value
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| AppError::Internal(format!("{name} is required for Keycloak invites")))
}

fn split_name(name: Option<String>) -> (Option<String>, Option<String>) {
    let Some(name) = name else {
        return (None, None);
    };
    let normalized = name.trim();
    if normalized.is_empty() {
        return (None, None);
    }

    let mut parts = normalized.splitn(2, char::is_whitespace);
    let first_name = parts.next().map(str::to_string);
    let last_name = parts
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);

    (first_name, last_name)
}

#[cfg(test)]
mod tests {
    use super::{split_name, KeycloakOrganizationInvite};

    #[test]
    fn keycloak_invite_splits_optional_display_name() {
        let invite = KeycloakOrganizationInvite::new(
            "ada@example.com".to_string(),
            Some("Ada Lovelace".to_string()),
        );

        assert_eq!(invite.first_name.as_deref(), Some("Ada"));
        assert_eq!(invite.last_name.as_deref(), Some("Lovelace"));
    }

    #[test]
    fn split_name_ignores_blank_values() {
        assert_eq!(split_name(Some("   ".to_string())), (None, None));
    }
}
