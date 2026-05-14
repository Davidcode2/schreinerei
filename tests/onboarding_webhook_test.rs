use async_trait::async_trait;
use schreinerei::common::error::AppError;
use schreinerei::modules::onboarding::application::{OnboardingService, PaymentProvider};
use schreinerei::modules::onboarding::infrastructure::onboarding_repository::OnboardingRepository;
use schreinerei::modules::onboarding::infrastructure::payment_provider::{
    CreateMolliePaymentRequest, ProviderCheckout, ProviderPayment,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn paid_mollie_webhook_confirms_session_and_replay_is_idempotent(pool: PgPool) {
    let payment_id = "tr_paid_replay";
    insert_onboarding_session(&pool, payment_id, "pending_payment", None).await;
    let service = webhook_service(pool.clone(), payment_id, "paid");

    let first = service
        .process_mollie_webhook(payment_id.to_string(), json!({ "id": payment_id }))
        .await
        .expect("first webhook should process");
    let replay = service
        .process_mollie_webhook(payment_id.to_string(), json!({ "id": payment_id }))
        .await
        .expect("webhook replay should process idempotently");

    assert!(first.event_inserted);
    assert!(!replay.event_inserted);
    assert_eq!(first.session_status.as_deref(), Some("payment_confirmed"));
    assert_eq!(replay.session_status.as_deref(), Some("payment_confirmed"));
    assert_eq!(session_status(&pool, payment_id).await, "payment_confirmed");
    assert_eq!(payment_event_count(&pool, payment_id).await, 1);
}

#[sqlx::test]
async fn pending_mollie_webhook_records_event_without_confirming_session(pool: PgPool) {
    let payment_id = "tr_pending_state";
    insert_onboarding_session(&pool, payment_id, "pending_payment", None).await;
    let service = webhook_service(pool.clone(), payment_id, "pending");

    let result = service
        .process_mollie_webhook(payment_id.to_string(), json!({ "id": payment_id }))
        .await
        .expect("pending webhook should process");

    assert!(result.event_inserted);
    assert_eq!(result.provider_status, "pending");
    assert_eq!(result.session_status.as_deref(), Some("pending_payment"));
    assert_eq!(session_status(&pool, payment_id).await, "pending_payment");
    assert_eq!(payment_event_count(&pool, payment_id).await, 1);
}

#[sqlx::test]
async fn failed_mollie_webhook_marks_session_failed(pool: PgPool) {
    let payment_id = "tr_failed_state";
    insert_onboarding_session(&pool, payment_id, "pending_payment", None).await;
    let service = webhook_service(pool.clone(), payment_id, "expired");

    let result = service
        .process_mollie_webhook(payment_id.to_string(), json!({ "id": payment_id }))
        .await
        .expect("failed webhook should process");

    assert_eq!(result.session_status.as_deref(), Some("payment_failed"));
    assert_eq!(session_status(&pool, payment_id).await, "payment_failed");
    assert_eq!(
        session_error_code(&pool, payment_id).await.as_deref(),
        Some("payment_not_confirmed")
    );
}

#[sqlx::test]
async fn paid_webhook_replay_does_not_reopen_completed_tenant_session(pool: PgPool) {
    let payment_id = "tr_completed_state";
    let tenant_id = insert_tenant(&pool).await;
    insert_onboarding_session(&pool, payment_id, "completed", Some(tenant_id)).await;
    let service = webhook_service(pool.clone(), payment_id, "paid");

    let result = service
        .process_mollie_webhook(payment_id.to_string(), json!({ "id": payment_id }))
        .await
        .expect("paid replay should be accepted for an already completed session");

    assert!(result.event_inserted);
    assert_eq!(result.session_status.as_deref(), Some("completed"));
    assert_eq!(session_status(&pool, payment_id).await, "completed");
    assert_eq!(session_tenant_id(&pool, payment_id).await, Some(tenant_id));
}

#[sqlx::test]
async fn public_session_lookup_returns_status_and_error_message(pool: PgPool) {
    let payment_id = "tr_lookup_state";
    insert_onboarding_session(&pool, payment_id, "keycloak_failed", None).await;
    sqlx::query(
        r#"
        UPDATE onboarding_sessions
        SET error_message = 'Provisioning failed in Keycloak'
        WHERE payment_provider = 'mollie' AND payment_id = $1
        "#,
    )
    .bind(payment_id)
    .execute(&pool)
    .await
    .expect("session error message should be updated");

    let session_id: Uuid = sqlx::query_scalar(
        "SELECT id FROM onboarding_sessions WHERE payment_provider = 'mollie' AND payment_id = $1",
    )
    .bind(payment_id)
    .fetch_one(&pool)
    .await
    .expect("session id should be fetched");

    let session = OnboardingRepository::new(pool)
        .find_public_session(session_id)
        .await
        .expect("public session lookup should succeed")
        .expect("session should exist");

    assert_eq!(session.id, session_id);
    assert_eq!(session.status.as_str(), "keycloak_failed");
    assert_eq!(
        session.error_message.as_deref(),
        Some("Provisioning failed in Keycloak")
    );
}

fn webhook_service(
    pool: PgPool,
    expected_payment_id: &str,
    provider_status: &str,
) -> OnboardingService<FakePaymentProvider> {
    OnboardingService::new(
        OnboardingRepository::new(pool),
        FakePaymentProvider {
            expected_payment_id: expected_payment_id.to_string(),
            provider_status: provider_status.to_string(),
        },
    )
}

#[derive(Clone)]
struct FakePaymentProvider {
    expected_payment_id: String,
    provider_status: String,
}

#[async_trait]
impl PaymentProvider for FakePaymentProvider {
    async fn create_checkout(
        &self,
        _request: CreateMolliePaymentRequest,
    ) -> Result<ProviderCheckout, AppError> {
        unreachable!("webhook tests do not create checkouts")
    }

    async fn fetch_payment(&self, payment_id: &str) -> Result<ProviderPayment, AppError> {
        assert_eq!(payment_id, self.expected_payment_id);
        Ok(ProviderPayment {
            id: payment_id.to_string(),
            status: self.provider_status.clone(),
        })
    }
}

async fn insert_onboarding_session(
    pool: &PgPool,
    payment_id: &str,
    status: &str,
    tenant_id: Option<Uuid>,
) {
    sqlx::query(
        r#"
        INSERT INTO onboarding_sessions (
            organization_name,
            organization_slug,
            admin_email,
            selected_plan,
            status,
            payment_provider,
            payment_id,
            checkout_url,
            tenant_id
        )
        VALUES ($1, $2, 'admin@example.com', 'starter', $3, 'mollie', $4, $5, $6)
        "#,
    )
    .bind(format!("Organization {payment_id}"))
    .bind(format!("organization-{payment_id}"))
    .bind(status)
    .bind(payment_id)
    .bind(format!("https://checkout.example.test/{payment_id}"))
    .bind(tenant_id)
    .execute(pool)
    .await
    .expect("onboarding session should be inserted");
}

async fn insert_tenant(pool: &PgPool) -> Uuid {
    let tenant_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO tenants (id, keycloak_realm, name, slug, keycloak_organization_alias)
        VALUES ($1, $2, 'Tenant', $3, $4)
        "#,
    )
    .bind(tenant_id)
    .bind(format!("realm-{tenant_id}"))
    .bind(format!("tenant-{tenant_id}"))
    .bind(format!("alias-{tenant_id}"))
    .execute(pool)
    .await
    .expect("tenant should be inserted");

    tenant_id
}

async fn session_status(pool: &PgPool, payment_id: &str) -> String {
    sqlx::query_scalar(
        "SELECT status FROM onboarding_sessions WHERE payment_provider = 'mollie' AND payment_id = $1",
    )
    .bind(payment_id)
    .fetch_one(pool)
    .await
    .expect("session status should be fetched")
}

async fn session_error_code(pool: &PgPool, payment_id: &str) -> Option<String> {
    sqlx::query_scalar(
        "SELECT error_code FROM onboarding_sessions WHERE payment_provider = 'mollie' AND payment_id = $1",
    )
    .bind(payment_id)
    .fetch_one(pool)
    .await
    .expect("session error code should be fetched")
}

async fn session_tenant_id(pool: &PgPool, payment_id: &str) -> Option<Uuid> {
    sqlx::query_scalar(
        "SELECT tenant_id FROM onboarding_sessions WHERE payment_provider = 'mollie' AND payment_id = $1",
    )
    .bind(payment_id)
    .fetch_one(pool)
    .await
    .expect("session tenant id should be fetched")
}

async fn payment_event_count(pool: &PgPool, payment_id: &str) -> i64 {
    sqlx::query_scalar(
        "SELECT COUNT(*) FROM payment_events WHERE provider = 'mollie' AND payment_id = $1",
    )
    .bind(payment_id)
    .fetch_one(pool)
    .await
    .expect("payment event count should be fetched")
}
