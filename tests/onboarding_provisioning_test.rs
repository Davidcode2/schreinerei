use async_trait::async_trait;
use schreinerei::common::error::AppError;
use schreinerei::modules::onboarding::application::{
    OrganizationProvisioner, TenantProvisioningService,
};
use schreinerei::modules::onboarding::infrastructure::keycloak_admin_client::{
    KeycloakOrganization, KeycloakOrganizationInvite,
};
use schreinerei::modules::onboarding::infrastructure::onboarding_repository::OnboardingRepository;
use sqlx::{PgPool, Row};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use uuid::Uuid;

const PAYMENT_PROVIDER: &str = "mollie";
const PAYMENT_ID: &str = "tr_paid_provisioning";

#[sqlx::test]
async fn provisions_paid_session_once(pool: PgPool) {
    let session_id = insert_confirmed_session(&pool, PAYMENT_ID).await;
    let organization_id = Uuid::new_v4().to_string();
    let keycloak = FakeKeycloak::succeeding(&organization_id, "schreinerei-beispiel");
    let service = provisioning_service(pool.clone(), keycloak);

    let status = service
        .provision_for_payment(PAYMENT_PROVIDER, PAYMENT_ID)
        .await
        .expect("provisioning should succeed");

    assert_eq!(status.as_deref(), Some("completed"));

    let row = sqlx::query(
        r#"
        SELECT os.status, os.tenant_id, os.keycloak_organization_id, os.keycloak_organization_alias,
               t.keycloak_realm, t.keycloak_organization_id::text AS tenant_keycloak_organization_id,
               t.keycloak_organization_alias AS tenant_keycloak_organization_alias
        FROM onboarding_sessions os
        JOIN tenants t ON t.id = os.tenant_id
        WHERE os.id = $1
        "#,
    )
    .bind(session_id)
    .fetch_one(&pool)
    .await
    .expect("provisioned session should be queryable");

    assert_eq!(row.get::<String, _>("status"), "completed");
    assert_eq!(
        row.get::<String, _>("keycloak_organization_id"),
        organization_id
    );
    assert_eq!(
        row.get::<String, _>("keycloak_organization_alias"),
        "schreinerei-beispiel"
    );
    assert_eq!(row.get::<String, _>("keycloak_realm"), "schreinerei");
    assert_eq!(
        row.get::<String, _>("tenant_keycloak_organization_id"),
        organization_id
    );
    assert_eq!(
        row.get::<String, _>("tenant_keycloak_organization_alias"),
        "schreinerei-beispiel"
    );
}

#[sqlx::test]
async fn keycloak_failure_keeps_tenant_for_retry(pool: PgPool) {
    let session_id = insert_confirmed_session(&pool, "tr_retry_provisioning").await;
    let failing = FakeKeycloak::failing_once();
    let service = provisioning_service(pool.clone(), failing);

    let status = service
        .provision_for_payment(PAYMENT_PROVIDER, "tr_retry_provisioning")
        .await
        .expect("failure should be recorded as retryable state");

    assert_eq!(status.as_deref(), Some("keycloak_failed"));
    let first_tenant_id: Uuid =
        sqlx::query_scalar("SELECT tenant_id FROM onboarding_sessions WHERE id = $1")
            .bind(session_id)
            .fetch_one(&pool)
            .await
            .expect("tenant should remain attached after failure");

    let organization_id = Uuid::new_v4().to_string();
    let retrying = FakeKeycloak::succeeding(&organization_id, "schreinerei-beispiel");
    let retry_service = provisioning_service(pool.clone(), retrying);
    let retry_status = retry_service
        .provision_for_payment(PAYMENT_PROVIDER, "tr_retry_provisioning")
        .await
        .expect("retry should complete provisioning");

    assert_eq!(retry_status.as_deref(), Some("completed"));
    let retry_tenant_id: Uuid =
        sqlx::query_scalar("SELECT tenant_id FROM onboarding_sessions WHERE id = $1")
            .bind(session_id)
            .fetch_one(&pool)
            .await
            .expect("tenant should still be attached");
    assert_eq!(retry_tenant_id, first_tenant_id);
}

fn provisioning_service(
    pool: PgPool,
    keycloak: FakeKeycloak,
) -> TenantProvisioningService<FakeKeycloak> {
    TenantProvisioningService::new(
        OnboardingRepository::new(pool),
        keycloak,
        "schreinerei".to_string(),
    )
}

async fn insert_confirmed_session(pool: &PgPool, payment_id: &str) -> Uuid {
    sqlx::query_scalar(
        r#"
        INSERT INTO onboarding_sessions (
            organization_name,
            organization_slug,
            admin_email,
            admin_name,
            selected_plan,
            status,
            payment_provider,
            payment_id
        )
        VALUES (
            'Schreinerei Beispiel',
            'schreinerei-beispiel',
            'admin@example.com',
            'Ada Admin',
            'starter',
            'payment_confirmed',
            $1,
            $2
        )
        RETURNING id
        "#,
    )
    .bind(PAYMENT_PROVIDER)
    .bind(payment_id)
    .fetch_one(pool)
    .await
    .expect("confirmed onboarding session should be inserted")
}

#[derive(Clone)]
struct FakeKeycloak {
    organization_id: String,
    organization_alias: String,
    fail_create: Arc<AtomicBool>,
}

impl FakeKeycloak {
    fn succeeding(organization_id: &str, organization_alias: &str) -> Self {
        Self {
            organization_id: organization_id.to_string(),
            organization_alias: organization_alias.to_string(),
            fail_create: Arc::new(AtomicBool::new(false)),
        }
    }

    fn failing_once() -> Self {
        Self {
            organization_id: Uuid::new_v4().to_string(),
            organization_alias: "schreinerei-beispiel".to_string(),
            fail_create: Arc::new(AtomicBool::new(true)),
        }
    }
}

#[async_trait]
impl OrganizationProvisioner for FakeKeycloak {
    async fn create_organization(
        &self,
        _name: &str,
        _alias: &str,
    ) -> Result<KeycloakOrganization, AppError> {
        if self.fail_create.swap(false, Ordering::SeqCst) {
            return Err(AppError::Internal("temporary keycloak failure".to_string()));
        }

        Ok(KeycloakOrganization {
            id: self.organization_id.clone(),
            alias: self.organization_alias.clone(),
        })
    }

    async fn invite_user_to_organization(
        &self,
        _organization_id: &str,
        _invite: &KeycloakOrganizationInvite,
    ) -> Result<(), AppError> {
        Ok(())
    }
}
