use async_trait::async_trait;
use schreinerei::common::error::AppError;
use schreinerei::common::types::{Role, TenantId, UserId};
use schreinerei::modules::iam::application::user_service::{
    RealmRoleAssigner, TenantContext, UserService,
};
use schreinerei::modules::iam::infrastructure::user_repository::UserRepository;
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
    Arc, Mutex,
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

    let (admin_role, is_original_admin): (String, bool) = sqlx::query_as(
        "SELECT role, is_original_admin FROM users WHERE tenant_id = $1 AND email = 'admin@example.com'",
    )
    .bind(row.get::<Uuid, _>("tenant_id"))
    .fetch_one(&pool)
    .await
    .expect("pending onboarding admin should be created");
    assert_eq!(admin_role, "admin");
    assert!(is_original_admin);
}

#[sqlx::test]
async fn passes_success_url_as_organization_redirect_url(pool: PgPool) {
    let _session_id = insert_confirmed_session(&pool, PAYMENT_ID).await;
    let organization_id = Uuid::new_v4().to_string();
    let keycloak = FakeKeycloak::succeeding(&organization_id, "schreinerei-beispiel");
    let service = provisioning_service(pool.clone(), keycloak.clone());

    service
        .provision_for_payment(PAYMENT_PROVIDER, PAYMENT_ID)
        .await
        .expect("provisioning should succeed");

    assert_eq!(
        keycloak.captured_redirect_url.lock().unwrap().as_deref(),
        Some(ONBOARDING_SUCCESS_URL)
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

#[sqlx::test]
async fn first_login_claims_pending_onboarding_admin(pool: PgPool) {
    let session_id = insert_confirmed_session(&pool, "tr_admin_claim").await;
    let organization_id = Uuid::new_v4().to_string();
    let keycloak = FakeKeycloak::succeeding(&organization_id, "schreinerei-beispiel");
    let service = provisioning_service(pool.clone(), keycloak);
    service
        .provision_for_payment(PAYMENT_PROVIDER, "tr_admin_claim")
        .await
        .expect("provisioning should succeed");

    let tenant_id: Uuid =
        sqlx::query_scalar("SELECT tenant_id FROM onboarding_sessions WHERE id = $1")
            .bind(session_id)
            .fetch_one(&pool)
            .await
            .expect("tenant should be attached");

    sqlx::query(
        r#"
        INSERT INTO users (tenant_id, keycloak_user_id, email, name, role, created_at, updated_at)
        VALUES (
            $1,
            'pending-invite-same-email',
            'admin@example.com',
            'Pending Employee',
            'employee',
            NOW() - INTERVAL '1 hour',
            NOW() - INTERVAL '1 hour'
        )
        "#,
    )
    .bind(tenant_id)
    .execute(&pool)
    .await
    .expect("older same-email pending invite should be insertable");

    let keycloak_user_id = UserId(Uuid::new_v4());
    let user_service = UserService::new(UserRepository::new(pool.clone()));
    let ctx = TenantContext {
        tenant_id: TenantId(tenant_id),
        user_id: keycloak_user_id,
        email: "ADMIN@example.com".to_string(),
        roles: vec![Role::Employee],
        token_roles: vec![Role::Employee],
    };

    let user = user_service
        .get_or_create_from_ctx(&ctx)
        .await
        .expect("pending admin should be claimed");

    assert_eq!(user.role, Role::Admin);
    assert_eq!(user.keycloak_user_id, keycloak_user_id.to_string());
    let user_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE tenant_id = $1 AND keycloak_user_id = $2",
    )
    .bind(tenant_id)
    .bind(keycloak_user_id.to_string())
    .fetch_one(&pool)
    .await
    .expect("claimed user should be countable");
    assert_eq!(user_count, 1);

    let pending_invite_role: String = sqlx::query_scalar(
        "SELECT role FROM users WHERE tenant_id = $1 AND keycloak_user_id = 'pending-invite-same-email'",
    )
        .bind(tenant_id)
        .fetch_one(&pool)
        .await
        .expect("non-onboarding pending invite should remain unclaimed");
    assert_eq!(pending_invite_role, "employee");
}

#[sqlx::test]
async fn first_login_assigns_keycloak_admin_role_for_claimed_onboarding_admin(pool: PgPool) {
    let session_id = insert_confirmed_session(&pool, "tr_admin_role_sync").await;
    let organization_id = Uuid::new_v4().to_string();
    let keycloak = FakeKeycloak::succeeding(&organization_id, "schreinerei-beispiel");
    let service = provisioning_service(pool.clone(), keycloak);
    service
        .provision_for_payment(PAYMENT_PROVIDER, "tr_admin_role_sync")
        .await
        .expect("provisioning should succeed");

    let tenant_id: Uuid =
        sqlx::query_scalar("SELECT tenant_id FROM onboarding_sessions WHERE id = $1")
            .bind(session_id)
            .fetch_one(&pool)
            .await
            .expect("tenant should be attached");

    let keycloak_user_id = UserId(Uuid::new_v4());
    let assigner = Arc::new(FakeRealmRoleAssigner::default());
    let user_service =
        UserService::new_with_role_assigner(UserRepository::new(pool.clone()), assigner.clone());
    let ctx = TenantContext {
        tenant_id: TenantId(tenant_id),
        user_id: keycloak_user_id,
        email: "admin@example.com".to_string(),
        roles: vec![Role::Admin],
        token_roles: vec![],
    };

    let user = user_service
        .get_or_create_from_ctx(&ctx)
        .await
        .expect("pending admin should be claimed");

    assert_eq!(user.role, Role::Admin);
    assert!(assigner.called.load(Ordering::SeqCst));
}

#[sqlx::test]
async fn first_login_skips_keycloak_role_sync_when_token_already_admin(pool: PgPool) {
    let session_id = insert_confirmed_session(&pool, "tr_admin_role_synced").await;
    let organization_id = Uuid::new_v4().to_string();
    let keycloak = FakeKeycloak::succeeding(&organization_id, "schreinerei-beispiel");
    let service = provisioning_service(pool.clone(), keycloak);
    service
        .provision_for_payment(PAYMENT_PROVIDER, "tr_admin_role_synced")
        .await
        .expect("provisioning should succeed");

    let tenant_id: Uuid =
        sqlx::query_scalar("SELECT tenant_id FROM onboarding_sessions WHERE id = $1")
            .bind(session_id)
            .fetch_one(&pool)
            .await
            .expect("tenant should be attached");

    let keycloak_user_id = UserId(Uuid::new_v4());
    let assigner = Arc::new(FakeRealmRoleAssigner::default());
    let user_service =
        UserService::new_with_role_assigner(UserRepository::new(pool.clone()), assigner.clone());
    let ctx = TenantContext {
        tenant_id: TenantId(tenant_id),
        user_id: keycloak_user_id,
        email: "admin@example.com".to_string(),
        roles: vec![Role::Admin],
        token_roles: vec![Role::Admin],
    };

    let user = user_service
        .get_or_create_from_ctx(&ctx)
        .await
        .expect("pending admin should be claimed");

    assert_eq!(user.role, Role::Admin);
    assert!(!assigner.called.load(Ordering::SeqCst));
}

fn provisioning_service(
    pool: PgPool,
    keycloak: FakeKeycloak,
) -> TenantProvisioningService<FakeKeycloak> {
    TenantProvisioningService::new(
        OnboardingRepository::new(pool),
        keycloak,
        "schreinerei".to_string(),
        FRONTEND_PUBLIC_URL.to_string(),
    )
}

const FRONTEND_PUBLIC_URL: &str = "https://schreinerei.jakob-lingel.dev";
const ONBOARDING_SUCCESS_URL: &str = "https://schreinerei.jakob-lingel.dev/onboarding/success";

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
    captured_redirect_url: Arc<Mutex<Option<String>>>,
}

#[derive(Default)]
struct FakeRealmRoleAssigner {
    called: AtomicBool,
}

#[async_trait]
impl RealmRoleAssigner for FakeRealmRoleAssigner {
    async fn assign_realm_role(&self, _user_id: &str, role: Role) -> Result<(), AppError> {
        assert_eq!(role, Role::Admin);
        self.called.store(true, Ordering::SeqCst);
        Ok(())
    }
}

impl FakeKeycloak {
    fn succeeding(organization_id: &str, organization_alias: &str) -> Self {
        Self {
            organization_id: organization_id.to_string(),
            organization_alias: organization_alias.to_string(),
            fail_create: Arc::new(AtomicBool::new(false)),
            captured_redirect_url: Arc::new(Mutex::new(None)),
        }
    }

    fn failing_once() -> Self {
        Self {
            organization_id: Uuid::new_v4().to_string(),
            organization_alias: "schreinerei-beispiel".to_string(),
            fail_create: Arc::new(AtomicBool::new(true)),
            captured_redirect_url: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl OrganizationProvisioner for FakeKeycloak {
    async fn create_organization(
        &self,
        _name: &str,
        _alias: &str,
        redirect_url: &str,
    ) -> Result<KeycloakOrganization, AppError> {
        if self.fail_create.swap(false, Ordering::SeqCst) {
            return Err(AppError::Internal("temporary keycloak failure".to_string()));
        }

        *self.captured_redirect_url.lock().unwrap() = Some(redirect_url.to_string());

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
