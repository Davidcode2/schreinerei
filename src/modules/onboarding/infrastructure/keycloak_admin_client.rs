use reqwest::{header, Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::common::error::AppError;
use crate::common::types::Role;
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

    pub async fn create_organization(
        &self,
        name: &str,
        alias: &str,
    ) -> Result<KeycloakOrganization, AppError> {
        let token = self.admin_access_token().await?;
        let url = format!(
            "{}/admin/realms/{}/organizations",
            self.base_url, self.realm
        );
        let request = KeycloakOrganizationRequest {
            name,
            alias,
            enabled: true,
        };

        let response = self
            .client
            .post(url)
            .bearer_auth(token.as_str())
            .json(&request)
            .send()
            .await
            .map_err(|error| {
                AppError::Internal(format!("Keycloak organization creation failed: {error}"))
            })?;

        if response.status() == StatusCode::CONFLICT {
            return self.find_organization_by_alias(alias, token.as_str()).await;
        }

        if !response.status().is_success() {
            return Err(AppError::Validation(format!(
                "Keycloak organization creation failed with status {}",
                response.status()
            )));
        }

        let created_id = response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok())
            .and_then(location_id);

        if let Some(id) = created_id {
            return Ok(KeycloakOrganization {
                id,
                alias: alias.to_string(),
            });
        }

        self.find_organization_by_alias(alias, token.as_str()).await
    }

    pub async fn assign_realm_role(&self, user_id: &str, role: Role) -> Result<(), AppError> {
        let token = self.admin_access_token().await?;
        let role_name = role.to_string();
        let role_url = format!(
            "{}/admin/realms/{}/roles/{}",
            self.base_url, self.realm, role_name
        );

        let role_response = self
            .client
            .get(role_url)
            .bearer_auth(token.as_str())
            .send()
            .await
            .map_err(|error| AppError::Internal(format!("Keycloak role lookup failed: {error}")))?;

        if !role_response.status().is_success() {
            return Err(AppError::Validation(format!(
                "Keycloak role lookup failed with status {}",
                role_response.status()
            )));
        }

        let role_representation = role_response
            .json::<KeycloakRoleRepresentation>()
            .await
            .map_err(|error| {
                AppError::Internal(format!("Failed to parse Keycloak role: {error}"))
            })?;

        let mapping_url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/realm",
            self.base_url, self.realm, user_id
        );

        let mapping_response = self
            .client
            .post(mapping_url)
            .bearer_auth(token)
            .json(&vec![role_representation])
            .send()
            .await
            .map_err(|error| {
                AppError::Internal(format!("Keycloak role assignment failed: {error}"))
            })?;

        if mapping_response.status().is_success() {
            return Ok(());
        }

        Err(AppError::Validation(format!(
            "Keycloak role assignment failed with status {}",
            mapping_response.status()
        )))
    }

    async fn find_organization_by_alias(
        &self,
        alias: &str,
        token: &str,
    ) -> Result<KeycloakOrganization, AppError> {
        let url = format!(
            "{}/admin/realms/{}/organizations",
            self.base_url, self.realm
        );
        let organizations = self
            .client
            .get(url)
            .bearer_auth(token)
            .query(&[
                ("search", alias),
                ("exact", "true"),
                ("briefRepresentation", "true"),
                ("max", "20"),
            ])
            .send()
            .await
            .map_err(|error| {
                AppError::Internal(format!("Keycloak organization lookup failed: {error}"))
            })?;

        if !organizations.status().is_success() {
            return Err(AppError::Validation(format!(
                "Keycloak organization lookup failed with status {}",
                organizations.status()
            )));
        }

        organizations
            .json::<Vec<KeycloakOrganizationResponse>>()
            .await
            .map_err(|error| {
                AppError::Internal(format!("Failed to parse Keycloak organizations: {error}"))
            })?
            .into_iter()
            .find(|organization| organization.alias.as_deref() == Some(alias))
            .and_then(|organization| organization.into_domain())
            .ok_or_else(|| {
                AppError::NotFound(format!("Keycloak organization alias not found: {alias}"))
            })
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeycloakOrganization {
    pub id: String,
    pub alias: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct KeycloakOrganizationRequest<'a> {
    name: &'a str,
    alias: &'a str,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
struct KeycloakOrganizationResponse {
    id: Option<String>,
    alias: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct KeycloakRoleRepresentation {
    id: String,
    name: String,
}

impl KeycloakOrganizationResponse {
    fn into_domain(self) -> Option<KeycloakOrganization> {
        Some(KeycloakOrganization {
            id: self.id?,
            alias: self.alias?,
        })
    }
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

fn location_id(location: &str) -> Option<String> {
    location
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::{location_id, split_name, KeycloakOrganizationInvite};

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

    #[test]
    fn location_id_uses_last_path_segment() {
        assert_eq!(
            location_id("https://auth.example/admin/realms/demo/organizations/org-123").as_deref(),
            Some("org-123")
        );
    }
}
