use sqlx::{PgPool, Postgres, Transaction};

use crate::common::error::AppError;
use crate::common::types::{TenantId, UserId};

const CATEGORY_SEEDS: &[&str] = &[
    "onboarding-demo-category-plates",
    "onboarding-demo-category-hardware",
    "onboarding-demo-category-consumables",
    "onboarding-demo-category-timber",
    "onboarding-demo-category-edges",
    "onboarding-demo-category-finishes",
];
const MATERIAL_SEEDS: &[&str] = &[
    "onboarding-demo-material-multiplex",
    "onboarding-demo-material-hinge",
    "onboarding-demo-material-glue",
    "onboarding-demo-material-oak",
    "onboarding-demo-material-mdf",
    "onboarding-demo-material-edge-oak",
    "onboarding-demo-material-screws",
    "onboarding-demo-material-drawers",
    "onboarding-demo-material-oil",
    "onboarding-demo-material-silicone",
];
const SITE_SEEDS: &[&str] = &[
    "onboarding-demo-project-active",
    "onboarding-demo-project-planned",
    "onboarding-demo-project-library",
    "onboarding-demo-project-kindergarten",
    "onboarding-demo-project-farmshop",
    "onboarding-demo-project-showroom",
];
const USER_SEEDS: &[&str] = &[
    "onboarding-demo-user-lena",
    "onboarding-demo-user-moritz",
    "onboarding-demo-user-aylin",
    "onboarding-demo-user-felix",
    "onboarding-demo-user-mara",
];
const ASSET_SEEDS: &[&str] = &[
    "onboarding-demo-asset-vehicle",
    "onboarding-demo-asset-tool",
    "onboarding-demo-asset-vehicle-south",
    "onboarding-demo-asset-vehicle-pickup",
    "onboarding-demo-asset-vehicle-trailer",
    "onboarding-demo-asset-tool-lamello",
    "onboarding-demo-asset-tool-saw",
    "onboarding-demo-asset-tool-drill",
    "onboarding-demo-asset-tool-vacuum",
    "onboarding-demo-asset-tool-multitool",
    "onboarding-demo-asset-tool-router",
    "onboarding-demo-asset-tool-laser",
];

pub const COMPLETE_TEST_DATA_RECORD_COUNT: i64 = (CATEGORY_SEEDS.len()
    + MATERIAL_SEEDS.len()
    + SITE_SEEDS.len()
    + USER_SEEDS.len()
    + ASSET_SEEDS.len()) as i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestDataRemoval {
    pub removed_records: i64,
    pub retained_records: i64,
}

pub struct TestDataRepository {
    pool: PgPool,
}

impl TestDataRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn is_installed(&self, tenant_id: TenantId) -> Result<bool, AppError> {
        Ok(self.seeded_record_count(tenant_id).await? > 0)
    }

    pub async fn seeded_record_count(&self, tenant_id: TenantId) -> Result<i64, AppError> {
        let mut count = 0;
        for (table, names) in [
            ("categories", CATEGORY_SEEDS),
            ("materials", MATERIAL_SEEDS),
            ("sites", SITE_SEEDS),
            ("users", USER_SEEDS),
            ("assets", ASSET_SEEDS),
        ] {
            count += count_seeded_records(&self.pool, tenant_id, table, names).await?;
        }
        Ok(count)
    }

    pub async fn install(&self, tenant_id: TenantId, admin_id: UserId) -> Result<(), AppError> {
        let mut transaction = self.pool.begin().await.map_err(database_error)?;
        insert_categories(&mut transaction, tenant_id).await?;
        insert_materials(&mut transaction, tenant_id).await?;
        insert_material_batch(&mut transaction, tenant_id).await?;
        insert_sites(&mut transaction, tenant_id).await?;
        insert_assignment(&mut transaction, tenant_id, admin_id).await?;
        insert_assets(&mut transaction, tenant_id).await?;
        insert_asset_details(&mut transaction, tenant_id).await?;
        insert_rich_fixture(&mut transaction, tenant_id, admin_id).await?;
        transaction.commit().await.map_err(database_error)
    }

    pub async fn remove(&self, tenant_id: TenantId) -> Result<TestDataRemoval, AppError> {
        let before = self.seeded_record_count(tenant_id).await?;
        let mut transaction = self.pool.begin().await.map_err(database_error)?;
        delete_seeded_children(&mut transaction, tenant_id).await?;
        delete_seeded_assets(&mut transaction, tenant_id).await?;
        delete_seeded_inventory(&mut transaction, tenant_id).await?;
        delete_seeded_sites(&mut transaction, tenant_id).await?;
        delete_seeded_users(&mut transaction, tenant_id).await?;
        transaction.commit().await.map_err(database_error)?;
        let retained_records = self.seeded_record_count(tenant_id).await?;
        Ok(TestDataRemoval {
            removed_records: before - retained_records,
            retained_records,
        })
    }
}

async fn insert_rich_fixture(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
    admin_id: UserId,
) -> Result<(), AppError> {
    let statement = include_str!("test_data_repository/rich_seed.sql")
        .replace("{{tenant_id}}", &tenant_id.to_string())
        .replace("{{admin_id}}", &admin_id.to_string())
        .replace(
            "{{kitchen_image}}",
            &hex::encode(include_bytes!(
                "test_data_repository/kitchen_measurement.png"
            )),
        )
        .replace(
            "{{kitchen_thumb}}",
            &hex::encode(include_bytes!(
                "test_data_repository/kitchen_measurement-thumb.png"
            )),
        )
        .replace(
            "{{workshop_image}}",
            &hex::encode(include_bytes!("test_data_repository/workshop_progress.png")),
        )
        .replace(
            "{{workshop_thumb}}",
            &hex::encode(include_bytes!(
                "test_data_repository/workshop_progress-thumb.png"
            )),
        )
        .replace(
            "{{reception_image}}",
            &hex::encode(include_bytes!(
                "test_data_repository/reception_installation.png"
            )),
        )
        .replace(
            "{{reception_thumb}}",
            &hex::encode(include_bytes!(
                "test_data_repository/reception_installation-thumb.png"
            )),
        );
    for seed_statement in statement.split(';').filter(|part| !part.trim().is_empty()) {
        sqlx::query(seed_statement)
            .execute(&mut **transaction)
            .await
            .map_err(database_error)?;
    }
    Ok(())
}

async fn insert_categories(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO categories (id, tenant_id, name, description, can_expire)
        VALUES
            (uuid_generate_v5($1, 'onboarding-demo-category-plates'), $1, 'Plattenwerkstoffe', 'Startbestand für Korpus- und Montagearbeiten.', FALSE),
            (uuid_generate_v5($1, 'onboarding-demo-category-hardware'), $1, 'Beschläge', 'Häufig benötigte Beschläge für erste Beispielprojekte.', FALSE),
            (uuid_generate_v5($1, 'onboarding-demo-category-consumables'), $1, 'Verbrauchsmaterial', 'Leim, Schrauben und Montagekleinteile.', TRUE)
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(tenant_id.0)
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(database_error)
}

async fn insert_materials(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO materials
            (id, tenant_id, category_id, name, description, unit, quantity, legacy_quantity, min_quantity, location, qr_code)
        VALUES
            (uuid_generate_v5($1, 'onboarding-demo-material-multiplex'), $1, uuid_generate_v5($1, 'onboarding-demo-category-plates'), 'Birke Multiplex 18 mm', 'Demo-Material für erste Entnahmen und Bestandswarnungen.', 'Stück', 12, 12, 5, 'Plattenlager A', 'DEMO-MAT-001-' || $1::text),
            (uuid_generate_v5($1, 'onboarding-demo-material-hinge'), $1, uuid_generate_v5($1, 'onboarding-demo-category-hardware'), 'Topfscharnier 110 Grad', 'Demo-Beschlag für Montage- und Projektbeispiele.', 'Stück', 80, 80, 20, 'Beschlagschrank 1', 'DEMO-MAT-002-' || $1::text),
            (uuid_generate_v5($1, 'onboarding-demo-material-glue'), $1, uuid_generate_v5($1, 'onboarding-demo-category-consumables'), 'D4 Leim 500 g', 'Demo-Verbrauchsmaterial mit Ablaufwarnung.', 'Stück', 8, 0, 4, 'Chemieschrank', 'DEMO-MAT-003-' || $1::text)
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(tenant_id.0)
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(database_error)
}

async fn insert_material_batch(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO material_batches
            (id, tenant_id, material_id, expires_on, initial_quantity, remaining_quantity, batch_code)
        VALUES
            (uuid_generate_v5($1, 'onboarding-demo-material-glue-batch'), $1, uuid_generate_v5($1, 'onboarding-demo-material-glue'), CURRENT_DATE + 90, 8, 8, 'DEMO-LEIM-' || LEFT($1::text, 8))
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(tenant_id.0)
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(database_error)
}

async fn insert_sites(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO sites
            (id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, budget_amount_cents, billing_reference, billing_notes, quote_reference)
        VALUES
            (uuid_generate_v5($1, 'onboarding-demo-project-active'), $1, 'external_site', 'Demo Einbauschrank', 'Familie Beispiel', 'Musterstraße 12', 'Aktives Demo-Projekt für Dashboard, Zeitbuchung und Materialentnahme.', 'active', CURRENT_DATE - 2, CURRENT_DATE + 3, 4, 450000, 'DEMO-2026-001', 'Demo-Daten für die erste Orientierung.', 'ANG-DEMO-001'),
            (uuid_generate_v5($1, 'onboarding-demo-project-planned'), $1, 'internal_workshop', 'Demo Werkstattauftrag', 'Intern', 'Werkstatt', 'Geplanter interner Auftrag für den Projektfilter.', 'planned', CURRENT_DATE + 5, CURRENT_DATE + 7, 2, NULL, NULL, NULL, NULL)
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(tenant_id.0)
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(database_error)
}

async fn insert_assignment(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
    admin_id: UserId,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO site_assignments (id, tenant_id, site_id, user_id, role)
        VALUES (uuid_generate_v5($1, 'onboarding-demo-assignment-admin-active'), $1, uuid_generate_v5($1, 'onboarding-demo-project-active'), $2, 'lead')
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(tenant_id.0)
    .bind(admin_id.0)
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(database_error)
}

async fn insert_assets(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO assets (id, tenant_id, asset_kind, name, description, status, location, qr_code)
        VALUES
            (uuid_generate_v5($1, 'onboarding-demo-asset-vehicle'), $1, 'vehicle', 'Demo Montagebus', 'Reservierbares Demo-Fahrzeug für die Fuhrparkansicht.', 'available', 'Hof', 'DEMO-FLT-001-' || $1::text),
            (uuid_generate_v5($1, 'onboarding-demo-asset-tool'), $1, 'tool', 'Demo Tauchsäge', 'Reservierbares Demo-Werkzeug für die Werkzeugansicht.', 'available', 'Werkzeugausgabe', 'DEMO-TL-001-' || $1::text)
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(tenant_id.0)
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(database_error)
}

async fn insert_asset_details(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    execute_seed_statement(
        transaction,
        tenant_id,
        r#"
        INSERT INTO vehicle_details (asset_id, tenant_id, license_plate, vehicle_type)
        VALUES (uuid_generate_v5($1, 'onboarding-demo-asset-vehicle'), $1, 'DE MO 1', 'van')
        ON CONFLICT (asset_id) DO NOTHING
        "#,
    )
    .await?;
    execute_seed_statement(
        transaction,
        tenant_id,
        r#"
        INSERT INTO vehicle_display_colors (asset_id, tenant_id, display_color)
        VALUES (uuid_generate_v5($1, 'onboarding-demo-asset-vehicle'), $1, '#2563eb')
        ON CONFLICT (asset_id) DO NOTHING
        "#,
    )
    .await?;
    execute_seed_statement(
        transaction,
        tenant_id,
        r#"
        INSERT INTO tool_details (asset_id, tenant_id, category)
        VALUES (uuid_generate_v5($1, 'onboarding-demo-asset-tool'), $1, 'Sägetechnik')
        ON CONFLICT (asset_id) DO NOTHING
        "#,
    )
    .await
}

async fn delete_seeded_children(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    delete_seeded_records(
        transaction,
        tenant_id,
        "site_activity_attachments",
        "id",
        &[
            "onboarding-demo-attachment-kitchen",
            "onboarding-demo-attachment-workshop",
            "onboarding-demo-attachment-reception",
        ],
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "site_activities",
        "id",
        &[
            "onboarding-demo-activity-winter-start",
            "onboarding-demo-activity-winter-measure",
            "onboarding-demo-activity-winter-progress",
            "onboarding-demo-activity-library-start",
            "onboarding-demo-activity-library-note",
            "onboarding-demo-activity-practice-measure",
            "onboarding-demo-activity-kita-complete",
            "onboarding-demo-activity-kita-note",
            "onboarding-demo-activity-showroom-progress",
            "onboarding-demo-activity-farmshop-complete",
            "onboarding-demo-activity-farmshop-note",
            "onboarding-demo-activity-practice-visual",
        ],
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "site_appointments",
        "id",
        &[
            "onboarding-demo-appointment-winter",
            "onboarding-demo-appointment-library",
            "onboarding-demo-appointment-practice",
            "onboarding-demo-appointment-showroom",
        ],
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "stock_entries",
        "id",
        &[
            "onboarding-demo-stock-multiplex-in",
            "onboarding-demo-stock-multiplex-out",
            "onboarding-demo-stock-hinge-in",
            "onboarding-demo-stock-hinge-out",
            "onboarding-demo-stock-oak-out",
            "onboarding-demo-stock-mdf-out",
            "onboarding-demo-stock-edge-out",
            "onboarding-demo-stock-screws-out",
            "onboarding-demo-stock-drawers-out",
            "onboarding-demo-stock-oil-out",
            "onboarding-demo-stock-silicone-out",
            "onboarding-demo-stock-mdf-location",
        ],
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "order_requests",
        "id",
        &["onboarding-demo-order-hinges"],
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "reservations",
        "id",
        &[
            "onboarding-demo-reservation-lamello",
            "onboarding-demo-reservation-bus-south",
            "onboarding-demo-reservation-drill",
            "onboarding-demo-reservation-pickup",
            "onboarding-demo-reservation-router",
            "onboarding-demo-reservation-vacuum",
            "onboarding-demo-reservation-laser",
            "onboarding-demo-reservation-multitool",
        ],
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "maintenance_due",
        "id",
        &[
            "onboarding-demo-maintenance-due-saw",
            "onboarding-demo-maintenance-due-bus-resolved",
            "onboarding-demo-maintenance-due-bus-open",
            "onboarding-demo-maintenance-due-vacuum",
            "onboarding-demo-maintenance-due-trailer",
            "onboarding-demo-maintenance-due-router-resolved",
            "onboarding-demo-maintenance-due-router-open",
        ],
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "maintenance_schedules",
        "id",
        &[
            "onboarding-demo-maintenance-saw",
            "onboarding-demo-maintenance-bus",
            "onboarding-demo-maintenance-vacuum",
            "onboarding-demo-maintenance-trailer",
            "onboarding-demo-maintenance-router",
        ],
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "time_entries",
        "id",
        &[
            "onboarding-demo-time-01",
            "onboarding-demo-time-02",
            "onboarding-demo-time-03",
            "onboarding-demo-time-04",
            "onboarding-demo-time-05",
            "onboarding-demo-time-06",
            "onboarding-demo-time-07",
            "onboarding-demo-time-08",
            "onboarding-demo-time-09",
            "onboarding-demo-time-10",
            "onboarding-demo-time-11",
            "onboarding-demo-time-12",
            "onboarding-demo-time-13",
            "onboarding-demo-time-14",
            "onboarding-demo-time-15",
            "onboarding-demo-time-16",
            "onboarding-demo-time-17",
            "onboarding-demo-time-18",
            "onboarding-demo-time-19",
            "onboarding-demo-time-20",
        ],
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "site_assignments",
        "id",
        &[
            "onboarding-demo-assignment-admin-active",
            "onboarding-demo-assignment-winter-lena",
            "onboarding-demo-assignment-winter-moritz",
            "onboarding-demo-assignment-library-aylin",
            "onboarding-demo-assignment-library-felix",
            "onboarding-demo-assignment-practice-mara",
            "onboarding-demo-assignment-kita-moritz",
            "onboarding-demo-assignment-farmshop-felix",
            "onboarding-demo-assignment-showroom-aylin",
            "onboarding-demo-assignment-showroom-mara",
        ],
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "material_batches",
        "id",
        &[
            "onboarding-demo-material-glue-batch",
            "onboarding-demo-material-oil-batch",
            "onboarding-demo-material-silicone-batch",
        ],
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "vehicle_display_colors",
        "asset_id",
        ASSET_SEEDS,
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "vehicle_details",
        "asset_id",
        ASSET_SEEDS,
    )
    .await?;
    delete_seeded_records(
        transaction,
        tenant_id,
        "tool_details",
        "asset_id",
        ASSET_SEEDS,
    )
    .await
}

async fn execute_seed_statement(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
    statement: &str,
) -> Result<(), AppError> {
    sqlx::query(statement)
        .bind(tenant_id.0)
        .execute(&mut **transaction)
        .await
        .map(|_| ())
        .map_err(database_error)
}

async fn delete_seeded_records(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
    table: &str,
    id_column: &str,
    names: &[&str],
) -> Result<(), AppError> {
    let statement = format!(
        "DELETE FROM {table} WHERE tenant_id = $1 AND {id_column} IN (SELECT uuid_generate_v5($1, seed_name) FROM unnest($2::text[]) AS seeds(seed_name))"
    );
    sqlx::query(&statement)
        .bind(tenant_id.0)
        .bind(names)
        .execute(&mut **transaction)
        .await
        .map(|_| ())
        .map_err(database_error)
}

async fn delete_seeded_assets(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        DELETE FROM assets
        WHERE tenant_id = $1
          AND id IN (SELECT uuid_generate_v5($1, seed_name) FROM unnest($2::text[]) AS seeds(seed_name))
          AND NOT EXISTS (SELECT 1 FROM reservations WHERE asset_id = assets.id)
          AND NOT EXISTS (SELECT 1 FROM maintenance_schedules WHERE asset_id = assets.id)
          AND NOT EXISTS (SELECT 1 FROM maintenance_due WHERE asset_id = assets.id)
        "#,
    )
    .bind(tenant_id.0)
    .bind(ASSET_SEEDS)
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(database_error)
}

async fn delete_seeded_inventory(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        DELETE FROM materials
        WHERE tenant_id = $1
          AND id IN (SELECT uuid_generate_v5($1, seed_name) FROM unnest($2::text[]) AS seeds(seed_name))
          AND NOT EXISTS (SELECT 1 FROM stock_entries WHERE material_id = materials.id)
          AND NOT EXISTS (SELECT 1 FROM order_requests WHERE material_id = materials.id)
          AND NOT EXISTS (SELECT 1 FROM goods_receipt_lines WHERE material_id = materials.id)
          AND NOT EXISTS (SELECT 1 FROM material_batches WHERE material_id = materials.id)
        "#,
    )
    .bind(tenant_id.0)
    .bind(MATERIAL_SEEDS)
    .execute(&mut **transaction)
    .await
    .map_err(database_error)?;
    sqlx::query(
        r#"
        DELETE FROM categories
        WHERE tenant_id = $1
          AND id IN (SELECT uuid_generate_v5($1, seed_name) FROM unnest($2::text[]) AS seeds(seed_name))
          AND NOT EXISTS (SELECT 1 FROM materials WHERE category_id = categories.id)
        "#,
    )
    .bind(tenant_id.0)
    .bind(CATEGORY_SEEDS)
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(database_error)
}

async fn delete_seeded_sites(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        DELETE FROM sites
        WHERE tenant_id = $1
          AND id IN (SELECT uuid_generate_v5($1, seed_name) FROM unnest($2::text[]) AS seeds(seed_name))
          AND NOT EXISTS (SELECT 1 FROM site_assignments WHERE site_id = sites.id)
          AND NOT EXISTS (SELECT 1 FROM site_activities WHERE site_id = sites.id)
          AND NOT EXISTS (SELECT 1 FROM site_activity_attachments WHERE site_id = sites.id)
          AND NOT EXISTS (SELECT 1 FROM site_appointments WHERE site_id = sites.id)
          AND NOT EXISTS (SELECT 1 FROM time_entries WHERE site_id = sites.id)
          AND NOT EXISTS (SELECT 1 FROM invoices WHERE site_id = sites.id)
          AND NOT EXISTS (SELECT 1 FROM stock_entries WHERE site_id = sites.id)
          AND NOT EXISTS (SELECT 1 FROM reservations WHERE site_id = sites.id OR project_id = sites.id)
        "#,
    )
    .bind(tenant_id.0)
    .bind(SITE_SEEDS)
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(database_error)
}

async fn delete_seeded_users(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        DELETE FROM users
        WHERE tenant_id = $1
          AND id IN (SELECT uuid_generate_v5($1, seed_name) FROM unnest($2::text[]) AS seeds(seed_name))
          AND NOT EXISTS (SELECT 1 FROM site_assignments WHERE user_id = users.id)
          AND NOT EXISTS (SELECT 1 FROM site_activities WHERE user_id = users.id)
          AND NOT EXISTS (SELECT 1 FROM time_entries WHERE user_id = users.id)
          AND NOT EXISTS (SELECT 1 FROM stock_entries WHERE user_id = users.id)
          AND NOT EXISTS (SELECT 1 FROM order_requests WHERE requested_by = users.id OR approved_by = users.id)
          AND NOT EXISTS (SELECT 1 FROM reservations WHERE user_id = users.id)
          AND NOT EXISTS (SELECT 1 FROM maintenance_due WHERE resolved_by = users.id)
        "#,
    )
    .bind(tenant_id.0)
    .bind(USER_SEEDS)
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(database_error)
}

async fn count_seeded_records(
    pool: &PgPool,
    tenant_id: TenantId,
    table: &str,
    names: &[&str],
) -> Result<i64, AppError> {
    let statement = format!(
        "SELECT COUNT(*) FROM {table} WHERE tenant_id = $1 AND id IN (SELECT uuid_generate_v5($1, seed_name) FROM unnest($2::text[]) AS seeds(seed_name))"
    );
    sqlx::query_scalar(&statement)
        .bind(tenant_id.0)
        .bind(names)
        .fetch_one(pool)
        .await
        .map_err(database_error)
}

fn database_error(error: sqlx::Error) -> AppError {
    AppError::Database(error.to_string())
}
