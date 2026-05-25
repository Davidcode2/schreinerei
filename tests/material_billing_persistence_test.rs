use schreinerei::common::types::{CategoryId, TenantId, Unit};
use schreinerei::modules::inventory::domain::{CreateMaterial, UpdateMaterial};
use schreinerei::modules::inventory::infrastructure::MaterialRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn update_material_persists_billing_fields(pool: PgPool) {
    let tenant_id = create_tenant(&pool).await;
    let category_id = create_category(&pool, tenant_id).await;
    let repo = MaterialRepository::new(pool.clone());

    let material = repo
        .create_material(
            &CreateMaterial {
                category_id,
                name: "Birke Multiplex".to_string(),
                description: None,
                unit: Unit::Piece,
                quantity: 10,
                min_quantity: 2,
                location: Some("Regal A1".to_string()),
                base_price_cents: None,
                price_markup_percentage: None,
                expires_on: None,
                batch_code: None,
            },
            tenant_id,
            false,
        )
        .await
        .expect("material should be created");

    let updated = repo
        .update_material(
            material.id,
            &UpdateMaterial {
                location: None,
                min_quantity: None,
                base_price_cents: Some(12_500),
                price_markup_percentage: Some(18),
                clear_location: None,
                clear_base_price_cents: None,
                clear_price_markup_percentage: None,
            },
            tenant_id,
        )
        .await
        .expect("material should update");

    assert_eq!(updated.base_price_cents, Some(12_500));
    assert_eq!(updated.price_markup_percentage, Some(18));

    let reloaded = repo
        .find_material_by_id(material.id, tenant_id)
        .await
        .expect("material lookup should succeed")
        .expect("material should exist");

    assert_eq!(reloaded.base_price_cents, Some(12_500));
    assert_eq!(reloaded.price_markup_percentage, Some(18));

    let row = sqlx::query(
        r#"
        SELECT base_price_cents, price_markup_percentage
        FROM materials
        WHERE id = $1 AND tenant_id = $2
        "#,
    )
    .bind(material.id.0)
    .bind(tenant_id.0)
    .fetch_one(&pool)
    .await
    .expect("material row should be readable");

    use sqlx::Row;

    assert_eq!(row.try_get::<Option<i64>, _>("base_price_cents").unwrap(), Some(12_500));
    assert_eq!(
        row.try_get::<Option<i32>, _>("price_markup_percentage").unwrap(),
        Some(18)
    );
}

async fn create_tenant(pool: &PgPool) -> TenantId {
    let id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO tenants (id, keycloak_realm, name, slug, keycloak_organization_alias)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(id)
    .bind(format!("realm-{id}"))
    .bind("Tenant")
    .bind(format!("slug-{id}"))
    .bind(format!("alias-{id}"))
    .execute(pool)
    .await
    .expect("tenant should be inserted");

    TenantId(id)
}

async fn create_category(pool: &PgPool, tenant_id: TenantId) -> CategoryId {
    let id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO categories (id, tenant_id, name, description, can_expire)
        VALUES ($1, $2, $3, $4, false)
        "#,
    )
    .bind(id)
    .bind(tenant_id.0)
    .bind("Holz")
    .bind("Platten")
    .execute(pool)
    .await
    .expect("category should be inserted");

    CategoryId(id)
}
