use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use uuid::Uuid;

use crate::common::error::AppError;
use crate::modules::onboarding::application::{
    CreateSessionOptions, OnboardingService, PublicInviteService, TenantProvisioningService,
};
use crate::modules::onboarding::domain::{CreateOnboardingSession, OnboardingStatus};
use crate::modules::onboarding::infrastructure::keycloak_admin_client::KeycloakAdminClient;
use crate::modules::onboarding::infrastructure::onboarding_repository::OnboardingRepository;
use crate::modules::onboarding::infrastructure::payment_provider::MolliePaymentProvider;
use crate::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/onboarding/sessions",
            post(create_onboarding_session),
        )
        .route(
            "/api/v1/onboarding/sessions/{session_id}",
            get(get_onboarding_session),
        )
        .route("/api/v1/onboarding/invites/{token}", get(get_public_invite))
        .route("/api/v1/onboarding/webhooks/mollie", post(mollie_webhook))
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct CreateOnboardingSessionRequest {
    pub organization_name: String,
    pub admin_email: String,
    pub admin_name: Option<String>,
    pub selected_plan: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct OnboardingSessionResponse {
    pub session_id: Uuid,
    pub organization_slug: String,
    pub status: String,
    pub payment_provider: Option<String>,
    pub payment_id: Option<String>,
    pub checkout_url: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct OnboardingSessionStatusResponse {
    pub session_id: Uuid,
    pub organization_slug: String,
    pub status: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct MollieWebhookRequest {
    pub id: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct MollieWebhookResponse {
    pub status: String,
    pub payment_id: String,
    pub event_inserted: bool,
    pub provider_status: String,
    pub session_status: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct PublicInviteResponse {
    pub email: String,
    pub role: String,
    pub status: String,
    pub expires_at: String,
}

async fn create_onboarding_session(
    State(state): State<AppState>,
    Json(payload): Json<CreateOnboardingSessionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let api_key = state.config.mollie_api_key.clone().ok_or_else(|| {
        AppError::Internal("MOLLIE_API_KEY is required for onboarding checkout".to_string())
    })?;

    let service = OnboardingService::new(
        OnboardingRepository::new(state.pool.clone()),
        MolliePaymentProvider::new(api_key, state.config.mollie_api_base_url.clone()),
    );

    let session = service
        .create_session(
            CreateOnboardingSession {
                organization_name: payload.organization_name,
                admin_email: payload.admin_email,
                admin_name: payload.admin_name,
                selected_plan: payload.selected_plan,
            },
            CreateSessionOptions {
                amount_value: state.config.mollie_onboarding_amount_value.clone(),
                amount_currency: state.config.mollie_onboarding_amount_currency.clone(),
                app_public_url: state.config.app_public_url.clone(),
                frontend_public_url: state.config.frontend_public_url.clone(),
            },
        )
        .await?;

    let checkout_url = session.checkout_url.clone().ok_or_else(|| {
        AppError::Internal("Onboarding checkout URL was not generated".to_string())
    })?;

    Ok((
        StatusCode::CREATED,
        Json(OnboardingSessionResponse {
            session_id: session.id,
            organization_slug: session.organization_slug,
            status: session.status.to_string(),
            payment_provider: session.payment_provider,
            payment_id: session.payment_id,
            checkout_url,
        }),
    ))
}

async fn mollie_webhook(
    State(state): State<AppState>,
    Form(payload): Form<MollieWebhookRequest>,
) -> Result<impl IntoResponse, AppError> {
    let api_key = state.config.mollie_api_key.clone().ok_or_else(|| {
        AppError::Internal("MOLLIE_API_KEY is required for Mollie webhook processing".to_string())
    })?;

    let service = OnboardingService::new(
        OnboardingRepository::new(state.pool.clone()),
        MolliePaymentProvider::new(api_key, state.config.mollie_api_base_url.clone()),
    );

    let raw_payload = json!({ "id": payload.id });
    let result = service
        .process_mollie_webhook(payload.id, raw_payload)
        .await?;
    let session_status = if result.session_status.as_deref() == Some("payment_confirmed") {
        let keycloak = KeycloakAdminClient::from_config(&state.config)?;
        let provisioning = TenantProvisioningService::new(
            OnboardingRepository::new(state.pool.clone()),
            keycloak,
            state.config.keycloak_realm.clone(),
        );

        provisioning
            .provision_for_payment("mollie", &result.payment_id)
            .await?
    } else {
        result.session_status.clone()
    };

    Ok((
        StatusCode::OK,
        Json(MollieWebhookResponse {
            status: "processed".to_string(),
            payment_id: result.payment_id,
            event_inserted: result.event_inserted,
            provider_status: result.provider_status,
            session_status,
        }),
    ))
}

async fn get_onboarding_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let repository = OnboardingRepository::new(state.pool.clone());
    if let Some(session) = repository.find_session(session_id).await? {
        refresh_onboarding_session(&state, &repository, session).await?;
    }

    let session = repository
        .find_public_session(session_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Onboarding session not found".to_string()))?;

    Ok(Json(OnboardingSessionStatusResponse {
        session_id: session.id,
        organization_slug: session.organization_slug,
        status: session.status.to_string(),
        error_message: session.error_message,
    }))
}

async fn refresh_onboarding_session(
    state: &AppState,
    repository: &OnboardingRepository,
    session: crate::modules::onboarding::domain::OnboardingSession,
) -> Result<(), AppError> {
    let Some(payment_id) = session.payment_id.clone() else {
        return Ok(());
    };

    if session.payment_provider.as_deref() != Some("mollie") {
        return Ok(());
    }

    if matches!(
        session.status,
        OnboardingStatus::Completed | OnboardingStatus::PaymentFailed
    ) {
        return Ok(());
    }

    let api_key = state.config.mollie_api_key.clone().ok_or_else(|| {
        AppError::Internal("MOLLIE_API_KEY is required for onboarding status refresh".to_string())
    })?;
    let service = OnboardingService::new(
        repository.clone(),
        MolliePaymentProvider::new(api_key, state.config.mollie_api_base_url.clone()),
    );
    let result = service
        .process_mollie_webhook(payment_id.clone(), json!({ "id": payment_id }))
        .await?;
    let should_provision = result.session_status.as_deref() == Some("payment_confirmed")
        || matches!(session.status, OnboardingStatus::PaymentConfirmed);

    if should_provision {
        let keycloak = KeycloakAdminClient::from_config(&state.config)?;
        let provisioning = TenantProvisioningService::new(
            repository.clone(),
            keycloak,
            state.config.keycloak_realm.clone(),
        );

        provisioning
            .provision_for_payment("mollie", &result.payment_id)
            .await?;
    }

    Ok(())
}

async fn get_public_invite(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = PublicInviteService::new(OnboardingRepository::new(state.pool));
    let invite = service.find_public_invite(token).await?;

    Ok(Json(PublicInviteResponse {
        email: invite.email,
        role: invite.role,
        status: invite.status.to_string(),
        expires_at: invite.expires_at.to_rfc3339(),
    }))
}
