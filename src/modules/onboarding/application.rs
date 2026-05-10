use async_trait::async_trait;
use chrono::{Duration, Utc};
use serde_json::Value;
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{Role, TenantId};
use crate::modules::onboarding::domain::{
    organization_slug, CreateOnboardingSession, InviteStatus, OnboardingSession, OnboardingStatus,
    ProviderPaymentStatus,
};
use crate::modules::onboarding::infrastructure::keycloak_admin_client::{
    KeycloakAdminClient, KeycloakOrganization, KeycloakOrganizationInvite,
};
use crate::modules::onboarding::infrastructure::onboarding_repository::{
    OnboardingRepository, SessionPaymentUpdate,
};
use crate::modules::onboarding::infrastructure::payment_provider::{
    CreateMolliePaymentRequest, MollieAmount, MolliePaymentMetadata, MolliePaymentProvider,
    ProviderCheckout, ProviderPayment,
};

const MOLLIE_PROVIDER: &str = "mollie";

#[async_trait]
pub trait PaymentProvider: Send + Sync {
    async fn create_checkout(
        &self,
        request: CreateMolliePaymentRequest,
    ) -> Result<ProviderCheckout, AppError>;

    async fn fetch_payment(&self, payment_id: &str) -> Result<ProviderPayment, AppError>;
}

#[async_trait]
impl PaymentProvider for MolliePaymentProvider {
    async fn create_checkout(
        &self,
        request: CreateMolliePaymentRequest,
    ) -> Result<ProviderCheckout, AppError> {
        self.create_checkout(request).await
    }

    async fn fetch_payment(&self, payment_id: &str) -> Result<ProviderPayment, AppError> {
        self.fetch_payment(payment_id).await
    }
}

#[async_trait]
pub trait OrganizationInviter: Send + Sync {
    async fn invite_user_to_organization(
        &self,
        organization_id: &str,
        invite: &KeycloakOrganizationInvite,
    ) -> Result<(), AppError>;
}

#[async_trait]
impl OrganizationInviter for KeycloakAdminClient {
    async fn invite_user_to_organization(
        &self,
        organization_id: &str,
        invite: &KeycloakOrganizationInvite,
    ) -> Result<(), AppError> {
        self.invite_user_to_organization(organization_id, invite)
            .await
    }
}

#[async_trait]
pub trait OrganizationProvisioner: Send + Sync {
    async fn create_organization(
        &self,
        name: &str,
        alias: &str,
    ) -> Result<KeycloakOrganization, AppError>;

    async fn invite_user_to_organization(
        &self,
        organization_id: &str,
        invite: &KeycloakOrganizationInvite,
    ) -> Result<(), AppError>;
}

#[async_trait]
impl OrganizationProvisioner for KeycloakAdminClient {
    async fn create_organization(
        &self,
        name: &str,
        alias: &str,
    ) -> Result<KeycloakOrganization, AppError> {
        self.create_organization(name, alias).await
    }

    async fn invite_user_to_organization(
        &self,
        organization_id: &str,
        invite: &KeycloakOrganizationInvite,
    ) -> Result<(), AppError> {
        self.invite_user_to_organization(organization_id, invite)
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSessionOptions {
    pub amount_value: String,
    pub amount_currency: String,
    pub app_public_url: String,
    pub frontend_public_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebhookProcessingResult {
    pub payment_id: String,
    pub event_inserted: bool,
    pub provider_status: String,
    pub session_status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedInvite {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub status: InviteStatus,
    pub invite_url: String,
    pub organization_alias: String,
    pub expires_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicInvite {
    pub email: String,
    pub role: String,
    pub status: InviteStatus,
    pub expires_at: chrono::DateTime<Utc>,
}

pub struct TenantProvisioningService<K> {
    repository: OnboardingRepository,
    keycloak: K,
    keycloak_realm: String,
}

impl<K> TenantProvisioningService<K>
where
    K: OrganizationProvisioner,
{
    pub fn new(repository: OnboardingRepository, keycloak: K, keycloak_realm: String) -> Self {
        Self {
            repository,
            keycloak,
            keycloak_realm,
        }
    }

    pub async fn provision_for_payment(
        &self,
        provider: &str,
        payment_id: &str,
    ) -> Result<Option<String>, AppError> {
        let Some(session) = self
            .repository
            .claim_session_for_provisioning(provider, payment_id)
            .await?
        else {
            return self
                .repository
                .find_session_status_by_payment(provider, payment_id)
                .await;
        };

        match self.provision_claimed_session(&session).await {
            Ok(status) => Ok(Some(status)),
            Err(error) => {
                let message = error.to_string();
                self.repository
                    .mark_keycloak_failed(session.id, "keycloak_provisioning_failed", &message)
                    .await
                    .map(Some)
            }
        }
    }

    async fn provision_claimed_session(
        &self,
        session: &crate::modules::onboarding::infrastructure::onboarding_repository::ProvisioningSession,
    ) -> Result<String, AppError> {
        let tenant_id = match session.tenant_id {
            Some(tenant_id) => tenant_id,
            None => {
                self.repository
                    .ensure_tenant_for_session(
                        session.id,
                        &self.keycloak_realm,
                        &session.organization_name,
                        &session.organization_slug,
                        &session.admin_email,
                        session.admin_name.as_deref(),
                    )
                    .await?
            }
        };

        let organization = match (
            session.keycloak_organization_id.as_ref(),
            session.keycloak_organization_alias.as_ref(),
        ) {
            (Some(id), Some(alias)) => KeycloakOrganization {
                id: id.clone(),
                alias: alias.clone(),
            },
            _ => {
                let organization = self
                    .keycloak
                    .create_organization(&session.organization_name, &session.organization_slug)
                    .await?;
                self.repository
                    .save_keycloak_organization(
                        session.id,
                        tenant_id,
                        &organization.id,
                        &organization.alias,
                    )
                    .await?;
                organization
            }
        };

        let invite = KeycloakOrganizationInvite::new(
            session.admin_email.clone(),
            session.admin_name.clone(),
        );
        self.keycloak
            .invite_user_to_organization(&organization.id, &invite)
            .await?;

        self.repository.complete_provisioning(session.id).await
    }
}

pub struct OnboardingService<P> {
    repository: OnboardingRepository,
    payment_provider: P,
}

impl<P> OnboardingService<P>
where
    P: PaymentProvider,
{
    pub fn new(repository: OnboardingRepository, payment_provider: P) -> Self {
        Self {
            repository,
            payment_provider,
        }
    }

    pub async fn create_session(
        &self,
        command: CreateOnboardingSession,
        options: CreateSessionOptions,
    ) -> Result<OnboardingSession, AppError> {
        command.validate().map_err(AppError::Validation)?;

        let slug = organization_slug(&command.organization_name);
        let session = match self.repository.find_session_by_slug(&slug).await? {
            Some(existing) if can_reuse_pending_session(&existing, &command) => existing,
            Some(_) => {
                return Err(AppError::Conflict(
                    "An onboarding session already exists for this organization".to_string(),
                ))
            }
            None => self.repository.insert_session(&command, &slug).await?,
        };

        if session.checkout_url.is_some() {
            return Ok(session);
        }

        let checkout = self
            .payment_provider
            .create_checkout(CreateMolliePaymentRequest {
                amount: MollieAmount {
                    currency: options.amount_currency,
                    value: options.amount_value,
                },
                description: format!(
                    "Schreinerei onboarding for {}",
                    command.normalized_organization_name()
                ),
                redirect_url: onboarding_complete_url(&options.frontend_public_url, session.id),
                webhook_url: mollie_webhook_url(&options.app_public_url),
                metadata: MolliePaymentMetadata {
                    onboarding_session_id: session.id.to_string(),
                    organization_slug: session.organization_slug.clone(),
                },
            })
            .await?;

        self.repository
            .update_session_checkout(
                session.id,
                MOLLIE_PROVIDER,
                &checkout.payment_id,
                &checkout.checkout_url,
            )
            .await
    }

    pub async fn process_mollie_webhook(
        &self,
        payment_id: String,
        raw_payload: Value,
    ) -> Result<WebhookProcessingResult, AppError> {
        validate_mollie_payment_id(&payment_id)?;

        let payment = self.payment_provider.fetch_payment(&payment_id).await?;
        let provider_status = payment.status.clone();
        let event_inserted = self
            .repository
            .record_payment_event(
                MOLLIE_PROVIDER,
                &payment.id,
                &payment.id,
                &raw_payload,
                Some(&provider_status),
            )
            .await?;

        let payment_status = payment.payment_status();
        let session_status = self
            .update_session_from_payment(&payment, payment_status)
            .await?;

        Ok(WebhookProcessingResult {
            payment_id: payment.id,
            event_inserted,
            provider_status,
            session_status,
        })
    }

    async fn update_session_from_payment(
        &self,
        payment: &ProviderPayment,
        status: ProviderPaymentStatus,
    ) -> Result<Option<String>, AppError> {
        let update = if status.should_confirm_payment() {
            SessionPaymentUpdate::confirmed()
        } else if status.should_fail_payment() {
            SessionPaymentUpdate::failed(&payment.status)
        } else {
            return self
                .repository
                .find_session_status_by_payment(MOLLIE_PROVIDER, &payment.id)
                .await;
        };

        self.repository
            .update_session_payment_status(MOLLIE_PROVIDER, &payment.id, update)
            .await
    }
}

pub struct OrganizationInviteService<K> {
    repository: OnboardingRepository,
    keycloak: K,
    frontend_public_url: String,
    ttl_seconds: i64,
}

impl<K> OrganizationInviteService<K>
where
    K: OrganizationInviter,
{
    pub fn new(
        repository: OnboardingRepository,
        keycloak: K,
        frontend_public_url: String,
        ttl_seconds: i64,
    ) -> Self {
        Self {
            repository,
            keycloak,
            frontend_public_url,
            ttl_seconds,
        }
    }

    pub async fn generate_invite(
        &self,
        tenant_id: TenantId,
        email: String,
        name: Option<String>,
        role: Role,
    ) -> Result<GeneratedInvite, AppError> {
        let email = normalize_invite_email(&email)?;
        let tenant = self
            .repository
            .find_tenant_invite_context(tenant_id)
            .await?;
        let token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::seconds(self.ttl_seconds.max(60));
        let invite = self
            .repository
            .upsert_pending_invite(tenant_id, &email, &role.to_string(), &token, expires_at)
            .await?;

        let keycloak_invite = KeycloakOrganizationInvite::new(email, name);
        if let Err(error) = self
            .keycloak
            .invite_user_to_organization(&tenant.keycloak_organization_id, &keycloak_invite)
            .await
        {
            self.repository.mark_invite_failed(invite.id).await?;
            return Err(error);
        }

        self.repository
            .mark_invite_keycloak_sent(invite.id, &invite.token)
            .await?;

        Ok(GeneratedInvite {
            id: invite.id,
            email: invite.email,
            role: invite.role,
            status: invite.status,
            invite_url: invite_url(&self.frontend_public_url, &invite.token),
            organization_alias: tenant.keycloak_organization_alias,
            expires_at: invite.expires_at,
        })
    }
}

pub struct PublicInviteService {
    repository: OnboardingRepository,
}

impl PublicInviteService {
    pub fn new(repository: OnboardingRepository) -> Self {
        Self { repository }
    }

    pub async fn find_public_invite(&self, token: String) -> Result<PublicInvite, AppError> {
        if token.trim().is_empty() {
            return Err(AppError::Validation("Invite token is required".to_string()));
        }

        let invite = self
            .repository
            .find_invite_by_token(token.trim())
            .await?
            .ok_or_else(|| AppError::NotFound("Invite not found".to_string()))?;

        Ok(PublicInvite {
            email: invite.email,
            role: invite.role,
            status: invite.status,
            expires_at: invite.expires_at,
        })
    }
}

fn can_reuse_pending_session(
    session: &OnboardingSession,
    command: &CreateOnboardingSession,
) -> bool {
    session.status == OnboardingStatus::PendingPayment
        && session.admin_email == command.normalized_admin_email()
        && session.selected_plan == command.normalized_selected_plan()
}

fn onboarding_complete_url(frontend_public_url: &str, session_id: uuid::Uuid) -> String {
    format!(
        "{}/onboarding/complete?session={}",
        frontend_public_url.trim_end_matches('/'),
        session_id
    )
}

fn mollie_webhook_url(app_public_url: &str) -> String {
    format!(
        "{}/api/v1/onboarding/webhooks/mollie",
        app_public_url.trim_end_matches('/')
    )
}

fn normalize_invite_email(email: &str) -> Result<String, AppError> {
    let email = email.trim().to_lowercase();
    if email.is_empty() || !email.contains('@') {
        return Err(AppError::Validation(
            "A valid invite email is required".to_string(),
        ));
    }

    Ok(email)
}

fn invite_url(frontend_public_url: &str, token: &str) -> String {
    format!(
        "{}/signup?invite={}",
        frontend_public_url.trim_end_matches('/'),
        token
    )
}

fn validate_mollie_payment_id(payment_id: &str) -> Result<(), AppError> {
    if payment_id.trim().is_empty() {
        return Err(AppError::Validation(
            "Mollie webhook payment id is required".to_string(),
        ));
    }

    if !payment_id.starts_with("tr_") {
        return Err(AppError::Validation(
            "Mollie webhook payment id is invalid".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        invite_url, mollie_webhook_url, normalize_invite_email, onboarding_complete_url,
        validate_mollie_payment_id,
    };
    use uuid::Uuid;

    #[test]
    fn mollie_payment_id_must_be_present() {
        assert!(validate_mollie_payment_id("").is_err());
        assert!(validate_mollie_payment_id("   ").is_err());
    }

    #[test]
    fn mollie_payment_id_must_use_transaction_prefix() {
        assert!(validate_mollie_payment_id("ord_123").is_err());
        assert!(validate_mollie_payment_id("tr_123").is_ok());
    }

    #[test]
    fn onboarding_urls_are_derived_from_public_base_urls() {
        let session_id = Uuid::nil();

        assert_eq!(
            onboarding_complete_url("https://app.example.test/", session_id),
            "https://app.example.test/onboarding/complete?session=00000000-0000-0000-0000-000000000000"
        );
        assert_eq!(
            mollie_webhook_url("https://api.example.test/"),
            "https://api.example.test/api/v1/onboarding/webhooks/mollie"
        );
    }

    #[test]
    fn invite_email_is_normalized() {
        assert_eq!(
            normalize_invite_email(" ADA@EXAMPLE.COM ").unwrap(),
            "ada@example.com"
        );
        assert!(normalize_invite_email("not-an-email").is_err());
    }

    #[test]
    fn invite_url_points_to_signup_landing() {
        assert_eq!(
            invite_url("https://app.example.com/", "token-1"),
            "https://app.example.com/signup?invite=token-1"
        );
    }
}
