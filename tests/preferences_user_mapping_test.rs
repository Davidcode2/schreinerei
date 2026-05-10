use schreinerei::auth::extractor::AuthenticatedUser;
use schreinerei::common::types::{Role, TenantId, UserId};
use schreinerei::modules::iam::application::user_preferences_service::UserPreferencesService;
use schreinerei::modules::iam::application::user_service::UserService;
use schreinerei::modules::iam::infrastructure::user_repository::UserRepository;
use sqlx::PgPool;
use uuid::Uuid;

async fn create_test_tenant(pool: &PgPool, name: &str) -> Uuid {
    let id = Uuid::new_v4();
    let slug = format!("{}-{}", name.to_lowercase().replace(' ', "-"), id);

    sqlx::query(
        r#"
        INSERT INTO tenants (id, keycloak_realm, name, slug)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(id)
    .bind(format!("realm-{}", slug))
    .bind(name)
    .bind(slug)
    .execute(pool)
    .await
    .expect("create tenant");

    id
}

fn auth_user(keycloak_subject: Uuid, tenant_id: Uuid, email: &str) -> AuthenticatedUser {
    AuthenticatedUser {
        user_id: UserId(keycloak_subject),
        tenant_id: TenantId(tenant_id),
        email: email.to_string(),
        roles: vec![Role::Employee],
    }
}

async fn create_pending_admin(pool: &PgPool, tenant_id: Uuid, email: &str) -> Uuid {
    let id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO users (id, tenant_id, keycloak_user_id, email, role)
        VALUES ($1, $2, $3, $4, 'admin')
        "#,
    )
    .bind(id)
    .bind(tenant_id)
    .bind(format!("pending-admin-{id}"))
    .bind(email)
    .execute(pool)
    .await
    .expect("create pending admin");

    id
}

#[sqlx::test]
async fn preferences_mapping_uses_tenant_local_user_id(pool: PgPool) {
    // Regression for Phase 23 UAT gap: preferences writes must target local users.id.
    let tenant_id = create_test_tenant(&pool, "Tenant Mapping").await;
    let keycloak_subject = Uuid::new_v4();
    let auth = auth_user(keycloak_subject, tenant_id, "mapping@test.com");

    let user_service = UserService::new(UserRepository::new(pool.clone()));
    let prefs_service = UserPreferencesService::new(pool.clone());

    let user = user_service
        .get_or_create_from_auth(&auth)
        .await
        .expect("resolve local user");

    let updated = prefs_service
        .clear_active_site(user.id, TenantId(tenant_id))
        .await
        .expect("clear active site should upsert preferences");

    let stored_user_id: Uuid = sqlx::query_scalar(
        "SELECT user_id FROM user_preferences WHERE tenant_id = $1 AND user_id = $2",
    )
    .bind(tenant_id)
    .bind(user.id.0)
    .fetch_one(&pool)
    .await
    .expect("load stored preference user id");

    assert_eq!(stored_user_id, user.id.0);
    assert_eq!(updated.preferences.active_site_id, None);
    assert_ne!(stored_user_id, keycloak_subject);
}

#[sqlx::test]
async fn preferences_clear_updates_same_row_without_fk_violation(pool: PgPool) {
    let tenant_id = create_test_tenant(&pool, "Tenant Clear").await;
    let auth = auth_user(Uuid::new_v4(), tenant_id, "clear@test.com");

    let user_service = UserService::new(UserRepository::new(pool.clone()));
    let prefs_service = UserPreferencesService::new(pool.clone());
    let user = user_service
        .get_or_create_from_auth(&auth)
        .await
        .expect("resolve local user");

    prefs_service
        .clear_active_site(user.id, TenantId(tenant_id))
        .await
        .expect("first clear should create row");
    prefs_service
        .clear_active_site(user.id, TenantId(tenant_id))
        .await
        .expect("second clear should update same row");

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM user_preferences WHERE tenant_id = $1 AND user_id = $2",
    )
    .bind(tenant_id)
    .bind(user.id.0)
    .fetch_one(&pool)
    .await
    .expect("count preference rows");

    assert_eq!(count, 1);
}

#[sqlx::test]
async fn same_keycloak_subject_stays_tenant_isolated(pool: PgPool) {
    let subject = Uuid::new_v4();
    let tenant_a = create_test_tenant(&pool, "Tenant A").await;
    let tenant_b = create_test_tenant(&pool, "Tenant B").await;

    let service = UserService::new(UserRepository::new(pool.clone()));
    let auth_a = auth_user(subject, tenant_a, "shared@test.com");
    let auth_b = auth_user(subject, tenant_b, "shared@test.com");

    let user_a = service
        .get_or_create_from_auth(&auth_a)
        .await
        .expect("resolve user in tenant A");
    let user_b = service
        .get_or_create_from_auth(&auth_b)
        .await
        .expect("resolve user in tenant B");

    assert_ne!(user_a.id, user_b.id);

    let count_a: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE tenant_id = $1 AND keycloak_user_id = $2",
    )
    .bind(tenant_a)
    .bind(subject.to_string())
    .fetch_one(&pool)
    .await
    .expect("count users tenant A");

    let count_b: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE tenant_id = $1 AND keycloak_user_id = $2",
    )
    .bind(tenant_b)
    .bind(subject.to_string())
    .fetch_one(&pool)
    .await
    .expect("count users tenant B");

    assert_eq!(count_a, 1);
    assert_eq!(count_b, 1);
}

#[sqlx::test]
async fn first_login_claims_tenant_local_pending_admin_by_email(pool: PgPool) {
    let tenant_id = create_test_tenant(&pool, "Onboarding Tenant").await;
    let pending_id = create_pending_admin(&pool, tenant_id, "admin@test.invalid").await;
    let subject = Uuid::new_v4();
    let auth = auth_user(subject, tenant_id, "ADMIN@test.invalid");

    let service = UserService::new(UserRepository::new(pool.clone()));
    let user = service
        .get_or_create_from_auth(&auth)
        .await
        .expect("claim pending admin");

    assert_eq!(user.id.0, pending_id);
    assert_eq!(user.role, Role::Admin);
    assert_eq!(user.keycloak_user_id, subject.to_string());

    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE tenant_id = $1")
        .bind(tenant_id)
        .fetch_one(&pool)
        .await
        .expect("count users");
    assert_eq!(user_count, 1);
}

#[sqlx::test]
async fn pending_admin_email_claim_is_tenant_scoped(pool: PgPool) {
    let tenant_a = create_test_tenant(&pool, "Tenant Pending A").await;
    let tenant_b = create_test_tenant(&pool, "Tenant Pending B").await;
    let pending_a = create_pending_admin(&pool, tenant_a, "shared@test.invalid").await;
    let subject = Uuid::new_v4();
    let auth = auth_user(subject, tenant_b, "shared@test.invalid");

    let service = UserService::new(UserRepository::new(pool.clone()));
    let user_b = service
        .get_or_create_from_auth(&auth)
        .await
        .expect("create tenant B user");

    assert_ne!(user_b.id.0, pending_a);
    assert_eq!(user_b.role, Role::Employee);

    let pending_keycloak_id: String =
        sqlx::query_scalar("SELECT keycloak_user_id FROM users WHERE id = $1")
            .bind(pending_a)
            .fetch_one(&pool)
            .await
            .expect("load pending user");
    assert!(pending_keycloak_id.starts_with("pending-admin-"));
}
