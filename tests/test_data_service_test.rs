use schreinerei::common::types::{Role, TenantId, UserId};
use schreinerei::modules::iam::application::test_data_service::TestDataService;
use schreinerei::modules::iam::application::user_service::{TenantContext, UserService};
use schreinerei::modules::iam::infrastructure::{
    test_data_repository::TestDataRepository, user_repository::UserRepository,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[sqlx::test]
async fn admin_imports_test_data_with_current_domain_values(pool: PgPool) {
    let context = insert_admin(&pool).await;
    let service = test_data_service(&pool);

    let status = service.install(&context).await.expect("install test data");

    assert!(status.installed);
    let assignment_user_id: Uuid = sqlx::query_scalar(
        "SELECT user_id FROM site_assignments WHERE tenant_id = $1 AND role = 'lead'",
    )
    .bind(context.tenant_id.0)
    .fetch_one(&pool)
    .await
    .expect("demo assignment user");
    assert_eq!(assignment_user_id, local_user_id(&pool, &context).await.0);

    let project_type: String = sqlx::query_scalar(
        "SELECT project_type FROM sites WHERE tenant_id = $1 AND name = 'Demo Werkstattauftrag'",
    )
    .bind(context.tenant_id.0)
    .fetch_one(&pool)
    .await
    .expect("demo project");
    assert_eq!(project_type, "internal_workshop");

    let material = sqlx::query(
        r#"
        SELECT m.unit, m.quantity, COALESCE(SUM(b.remaining_quantity), 0) AS batch_quantity
        FROM materials m
        LEFT JOIN material_batches b ON b.material_id = m.id AND b.tenant_id = m.tenant_id
        WHERE m.tenant_id = $1 AND m.name = 'D4 Leim 500 g'
        GROUP BY m.id
        "#,
    )
    .bind(context.tenant_id.0)
    .fetch_one(&pool)
    .await
    .expect("expiring demo material");
    assert_eq!(material.get::<String, _>("unit"), "Stück");
    assert_eq!(material.get::<i32, _>("quantity"), 8);
    assert_eq!(material.get::<i64, _>("batch_quantity"), 8);

    let vehicle_color: String = sqlx::query_scalar(
        r#"
        SELECT c.display_color
        FROM assets a
        JOIN vehicle_display_colors c ON c.asset_id = a.id AND c.tenant_id = a.tenant_id
        WHERE a.tenant_id = $1 AND a.name = 'Demo Montagebus'
        "#,
    )
    .bind(context.tenant_id.0)
    .fetch_one(&pool)
    .await
    .expect("visible demo vehicle");
    assert_eq!(vehicle_color, "#2563eb");
}

#[sqlx::test]
async fn removing_test_data_preserves_custom_tenant_data(pool: PgPool) {
    let context = insert_admin(&pool).await;
    let service = test_data_service(&pool);
    service.install(&context).await.expect("install test data");
    let custom_category_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO categories (id, tenant_id, name, can_expire) VALUES ($1, $2, 'Eigene Kategorie', FALSE)",
    )
    .bind(custom_category_id)
    .bind(context.tenant_id.0)
    .execute(&pool)
    .await
    .expect("custom category");

    let status = service.remove(&context).await.expect("remove test data");

    assert!(!status.installed);
    let seeded_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sites WHERE tenant_id = $1 AND name LIKE 'Demo %'",
    )
    .bind(context.tenant_id.0)
    .fetch_one(&pool)
    .await
    .expect("seeded site count");
    assert_eq!(seeded_count, 0);
    let custom_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM categories WHERE id = $1 AND tenant_id = $2)",
    )
    .bind(custom_category_id)
    .bind(context.tenant_id.0)
    .fetch_one(&pool)
    .await
    .expect("custom category remains");
    assert!(custom_exists);
}

#[sqlx::test]
async fn regular_user_cannot_install_or_remove_test_data(pool: PgPool) {
    let mut context = insert_admin(&pool).await;
    context.roles = vec![Role::Employee];
    let service = test_data_service(&pool);

    let install_error = service
        .install(&context)
        .await
        .expect_err("install forbidden");
    let remove_error = service
        .remove(&context)
        .await
        .expect_err("remove forbidden");

    assert!(matches!(
        install_error,
        schreinerei::common::error::AppError::Forbidden(_)
    ));
    assert!(matches!(
        remove_error,
        schreinerei::common::error::AppError::Forbidden(_)
    ));
    let seeded_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sites WHERE tenant_id = $1")
        .bind(context.tenant_id.0)
        .fetch_one(&pool)
        .await
        .expect("site count");
    assert_eq!(seeded_count, 0);
}

#[sqlx::test]
async fn removal_refuses_to_delete_custom_data_linked_to_test_data(pool: PgPool) {
    let context = insert_admin(&pool).await;
    let service = test_data_service(&pool);
    service.install(&context).await.expect("install test data");
    let local_user_id = local_user_id(&pool, &context).await;
    let activity_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO site_activities (id, tenant_id, site_id, user_id, activity_type, content)
        VALUES ($1, $2, uuid_generate_v5($2, 'onboarding-demo-project-active'), $3, 'note', 'Eigene Notiz')
        "#,
    )
    .bind(activity_id)
    .bind(context.tenant_id.0)
    .bind(local_user_id.0)
    .execute(&pool)
    .await
    .expect("custom activity");

    let error = service
        .remove(&context)
        .await
        .expect_err("linked custom data must be preserved");

    assert!(matches!(
        error,
        schreinerei::common::error::AppError::Conflict(_)
    ));
    let activity_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM site_activities WHERE id = $1)")
            .bind(activity_id)
            .fetch_one(&pool)
            .await
            .expect("activity remains");
    assert!(activity_exists);
}

fn test_data_service(pool: &PgPool) -> TestDataService {
    TestDataService::new(
        TestDataRepository::new(pool.clone()),
        UserService::new(UserRepository::new(pool.clone())),
    )
}

async fn local_user_id(pool: &PgPool, context: &TenantContext) -> UserId {
    UserId(
        sqlx::query_scalar("SELECT id FROM users WHERE tenant_id = $1 AND keycloak_user_id = $2")
            .bind(context.tenant_id.0)
            .bind(context.user_id.to_string())
            .fetch_one(pool)
            .await
            .expect("local user"),
    )
}

async fn insert_admin(pool: &PgPool) -> TenantContext {
    let tenant_id = TenantId::new();
    let local_user_id = UserId::new();
    let authenticated_user_id = UserId::new();
    sqlx::query(
        r#"
        INSERT INTO tenants (id, keycloak_realm, name, slug, keycloak_organization_alias)
        VALUES ($1, 'schreinerei', 'Test Tenant', $2, $2)
        "#,
    )
    .bind(tenant_id.0)
    .bind(format!("test-{}", tenant_id.0))
    .execute(pool)
    .await
    .expect("tenant");
    sqlx::query(
        r#"
        INSERT INTO users (id, tenant_id, keycloak_user_id, email, name, role)
        VALUES ($1, $2, $3, 'admin@example.com', 'Admin', 'admin')
        "#,
    )
    .bind(local_user_id.0)
    .bind(tenant_id.0)
    .bind(authenticated_user_id.to_string())
    .execute(pool)
    .await
    .expect("admin");

    TenantContext {
        tenant_id,
        user_id: authenticated_user_id,
        email: "admin@example.com".to_string(),
        roles: vec![Role::Admin],
    }
}
