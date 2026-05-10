use schreinerei::modules::onboarding::domain::ProvisioningSession;
use schreinerei::modules::onboarding::infrastructure::onboarding_repository::OnboardingRepository;
use sqlx::PgPool;
use uuid::Uuid;

async fn create_paid_session(pool: &PgPool) -> ProvisioningSession {
    let session_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO onboarding_sessions (
            id,
            organization_name,
            organization_slug,
            admin_email,
            admin_name,
            selected_plan,
            status,
            payment_provider,
            payment_id
        )
        VALUES ($1, 'Acme Holzbau', 'acme-holzbau', 'admin@acme.test', 'Ada Admin', 'pro', 'payment_confirmed', 'mollie', 'tr_paid')
        "#,
    )
    .bind(session_id)
    .execute(pool)
    .await
    .expect("create paid session");

    ProvisioningSession {
        id: session_id,
        organization_name: "Acme Holzbau".to_string(),
        organization_slug: "acme-holzbau".to_string(),
        admin_email: "admin@acme.test".to_string(),
        admin_name: Some("Ada Admin".to_string()),
    }
}

#[sqlx::test]
async fn tenants_can_share_keycloak_realm_when_organization_alias_differs(pool: PgPool) {
    let realm = "schreinerei";
    sqlx::query(
        r#"
        INSERT INTO tenants (keycloak_realm, name, slug, keycloak_organization_id, keycloak_organization_alias)
        VALUES ($1, 'Tenant A', 'tenant-a', $2, 'tenant-a')
        "#,
    )
    .bind(realm)
    .bind(Uuid::new_v4())
    .execute(&pool)
    .await
    .expect("insert tenant A");

    sqlx::query(
        r#"
        INSERT INTO tenants (keycloak_realm, name, slug, keycloak_organization_id, keycloak_organization_alias)
        VALUES ($1, 'Tenant B', 'tenant-b', $2, 'tenant-b')
        "#,
    )
    .bind(realm)
    .bind(Uuid::new_v4())
    .execute(&pool)
    .await
    .expect("insert tenant B with same realm");
}

#[sqlx::test]
async fn complete_provisioning_creates_tenant_pending_admin_and_session_link(pool: PgPool) {
    let session = create_paid_session(&pool).await;
    let organization_id = Uuid::new_v4();
    let repo = OnboardingRepository::new(pool.clone());

    let claimed = repo
        .claim_session_for_provisioning("mollie", "tr_paid")
        .await
        .expect("claim session")
        .expect("session claimed");
    assert_eq!(claimed.id, session.id);

    let tenant_id = repo
        .complete_provisioning(&claimed, organization_id, "acme-holzbau", "schreinerei")
        .await
        .expect("complete provisioning");

    let status: String = sqlx::query_scalar("SELECT status FROM onboarding_sessions WHERE id = $1")
        .bind(session.id)
        .fetch_one(&pool)
        .await
        .expect("load session status");
    assert_eq!(status, "completed");

    let admin: (String, String) =
        sqlx::query_as("SELECT keycloak_user_id, role FROM users WHERE tenant_id = $1")
            .bind(tenant_id.0)
            .fetch_one(&pool)
            .await
            .expect("load pending admin");
    assert_eq!(admin.0, format!("pending-admin-{}", session.id));
    assert_eq!(admin.1, "admin");

    let preferences_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM user_preferences WHERE tenant_id = $1")
            .bind(tenant_id.0)
            .fetch_one(&pool)
            .await
            .expect("count preferences");
    assert_eq!(preferences_count, 1);
}
