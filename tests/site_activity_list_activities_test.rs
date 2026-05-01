use chrono::{Duration, Utc};
use schreinerei::common::types::{Role, SiteId, TenantId, UserId};
use schreinerei::modules::iam::application::user_service::TenantContext;
use schreinerei::modules::sites::application::site_service::SiteService;
use schreinerei::modules::sites::infrastructure::site_repository::SiteRepository;
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
    .bind(format!("realm-{slug}"))
    .bind(name)
    .bind(slug)
    .execute(pool)
    .await
    .expect("create tenant");

    id
}

async fn create_test_site(pool: &PgPool, tenant_id: Uuid, name: &str) -> Uuid {
    let id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO sites (id, tenant_id, name, customer_name, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'planned', NOW(), NOW())
        "#,
    )
    .bind(id)
    .bind(tenant_id)
    .bind(name)
    .bind("Kunde")
    .execute(pool)
    .await
    .expect("create site");

    id
}

async fn create_test_user(
    pool: &PgPool,
    tenant_id: Uuid,
    email: &str,
    name: Option<&str>,
) -> Uuid {
    let id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO users (id, tenant_id, keycloak_user_id, email, name, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 'employee', NOW(), NOW())
        "#,
    )
    .bind(id)
    .bind(tenant_id)
    .bind(format!("kc-{id}"))
    .bind(email)
    .bind(name)
    .execute(pool)
    .await
    .expect("create user");

    id
}

async fn create_test_activity(
    pool: &PgPool,
    tenant_id: Uuid,
    site_id: Uuid,
    user_id: Uuid,
    content: Option<&str>,
    photo_url: Option<&str>,
    created_at: chrono::DateTime<Utc>,
) -> Uuid {
    let id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO site_activities (id, tenant_id, site_id, user_id, activity_type, content, photo_url, created_at)
        VALUES ($1, $2, $3, $4, 'note', $5, $6, $7)
        "#,
    )
    .bind(id)
    .bind(tenant_id)
    .bind(site_id)
    .bind(user_id)
    .bind(content)
    .bind(photo_url)
    .bind(created_at)
    .execute(pool)
    .await
    .expect("create activity");

    id
}

async fn create_test_attachment(
    pool: &PgPool,
    tenant_id: Uuid,
    site_id: Uuid,
    activity_id: Uuid,
    filename: &str,
    mime_type: &str,
) -> Uuid {
    let id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO site_activity_attachments (
            id, tenant_id, activity_id, site_id, storage_key, thumbnail_key,
            original_filename, mime_type, size_bytes, original_bytes, thumbnail_bytes, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 42, $9, $10, NOW())
        "#,
    )
    .bind(id)
    .bind(tenant_id)
    .bind(activity_id)
    .bind(site_id)
    .bind(format!("{id}.bin"))
    .bind(Some(format!("{id}-thumb.bin")))
    .bind(filename)
    .bind(mime_type)
    .bind(Some(vec![1_u8, 2, 3]))
    .bind(Some(vec![4_u8, 5, 6]))
    .execute(pool)
    .await
    .expect("create attachment");

    id
}

fn tenant_context(tenant_id: Uuid, user_id: Uuid) -> TenantContext {
    TenantContext {
        tenant_id: TenantId(tenant_id),
        user_id: UserId(user_id),
        email: "viewer@test.invalid".to_string(),
        roles: vec![Role::Employee],
    }
}

#[sqlx::test]
async fn list_activities_uses_creator_name_display_value(pool: PgPool) {
    let tenant_id = create_test_tenant(&pool, "Viewer Tenant").await;
    let site_id = create_test_site(&pool, tenant_id, "Innenausbau").await;
    let user_id = create_test_user(&pool, tenant_id, "anna@test.invalid", Some("Anna Tischler")).await;

    create_test_activity(
        &pool,
        tenant_id,
        site_id,
        user_id,
        Some("Wand montiert"),
        None,
        Utc::now(),
    )
    .await;

    let service = SiteService::new(SiteRepository::new(pool.clone()));
    let activities = service
        .list_activities(SiteId(site_id), 10, &tenant_context(tenant_id, user_id))
        .await
        .expect("list activities");

    assert_eq!(activities.len(), 1);
    assert_eq!(activities[0].creator_name, "Anna Tischler");
}

#[sqlx::test]
async fn creator_name_falls_back_to_email_then_user_id_text(pool: PgPool) {
    let tenant_id = create_test_tenant(&pool, "Viewer Tenant").await;
    let other_tenant_id = create_test_tenant(&pool, "Other Tenant").await;
    let site_id = create_test_site(&pool, tenant_id, "Innenausbau").await;
    let viewer_user_id = create_test_user(&pool, tenant_id, "viewer@test.invalid", Some("Viewer"))
        .await;
    let email_fallback_user_id =
        create_test_user(&pool, tenant_id, "fallback@test.invalid", Some("")).await;
    let cross_tenant_user_id =
        create_test_user(&pool, other_tenant_id, "secret@test.invalid", Some("Secret User")).await;

    create_test_activity(
        &pool,
        tenant_id,
        site_id,
        email_fallback_user_id,
        Some("Email fallback"),
        None,
        Utc::now(),
    )
    .await;
    create_test_activity(
        &pool,
        tenant_id,
        site_id,
        cross_tenant_user_id,
        Some("UUID fallback"),
        None,
        Utc::now() - Duration::minutes(1),
    )
    .await;

    let service = SiteService::new(SiteRepository::new(pool.clone()));
    let activities = service
        .list_activities(SiteId(site_id), 10, &tenant_context(tenant_id, viewer_user_id))
        .await
        .expect("list activities");

    assert_eq!(activities[0].creator_name, "fallback@test.invalid");
    assert_eq!(activities[1].creator_name, cross_tenant_user_id.to_string());
}

#[sqlx::test]
async fn list_activities_preserves_order_and_attachment_hydration(pool: PgPool) {
    let tenant_id = create_test_tenant(&pool, "Viewer Tenant").await;
    let site_id = create_test_site(&pool, tenant_id, "Innenausbau").await;
    let user_id = create_test_user(&pool, tenant_id, "anna@test.invalid", Some("Anna Tischler")).await;

    let newest_activity_id = create_test_activity(
        &pool,
        tenant_id,
        site_id,
        user_id,
        Some("Neueste Aktivität"),
        Some("/api/v1/attachments/photo-id"),
        Utc::now(),
    )
    .await;
    let oldest_activity_id = create_test_activity(
        &pool,
        tenant_id,
        site_id,
        user_id,
        Some("Ältere Aktivität"),
        None,
        Utc::now() - Duration::minutes(5),
    )
    .await;

    let attachment_id = create_test_attachment(
        &pool,
        tenant_id,
        site_id,
        oldest_activity_id,
        "plan.pdf",
        "application/pdf",
    )
    .await;

    let service = SiteService::new(SiteRepository::new(pool.clone()));
    let activities = service
        .list_activities(SiteId(site_id), 10, &tenant_context(tenant_id, user_id))
        .await
        .expect("list activities");

    assert_eq!(activities.len(), 2);
    assert_eq!(activities[0].id.0, newest_activity_id);
    assert_eq!(activities[0].photo_url.as_deref(), Some("/api/v1/attachments/photo-id"));
    assert_eq!(activities[1].id.0, oldest_activity_id);
    assert_eq!(activities[1].attachments.len(), 1);
    assert_eq!(activities[1].attachments[0].id, attachment_id);
    assert_eq!(activities[1].attachments[0].filename, "plan.pdf");
    assert_eq!(activities[1].attachments[0].url, format!("/api/v1/attachments/{attachment_id}"));
}
