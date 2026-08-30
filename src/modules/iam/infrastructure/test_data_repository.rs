use sqlx::{PgPool, Postgres, Transaction};

use crate::common::error::AppError;
use crate::common::types::{TenantId, UserId};

pub struct TestDataRepository {
    pool: PgPool,
}

impl TestDataRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn is_installed(&self, tenant_id: TenantId) -> Result<bool, AppError> {
        sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM sites WHERE id = uuid_generate_v5($1, 'onboarding-demo-project-active') AND tenant_id = $1)",
        )
        .bind(tenant_id.0)
        .fetch_one(&self.pool)
        .await
        .map_err(database_error)
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
        transaction.commit().await.map_err(database_error)
    }

    pub async fn remove(&self, tenant_id: TenantId) -> Result<(), AppError> {
        let mut transaction = self.pool.begin().await.map_err(database_error)?;
        if has_linked_custom_data(&mut transaction, tenant_id).await? {
            return Err(AppError::Conflict(
                "Test data is still referenced by custom organization data".to_string(),
            ));
        }
        delete_seeded_children(&mut transaction, tenant_id).await?;
        delete_seeded_assets(&mut transaction, tenant_id).await?;
        delete_seeded_inventory(&mut transaction, tenant_id).await?;
        delete_seeded_sites(&mut transaction, tenant_id).await?;
        transaction.commit().await.map_err(database_error)
    }
}

async fn has_linked_custom_data(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<bool, AppError> {
    sqlx::query_scalar(
        r#"
        WITH seeded AS (
            SELECT
                ARRAY[
                    uuid_generate_v5($1, 'onboarding-demo-project-active'),
                    uuid_generate_v5($1, 'onboarding-demo-project-planned')
                ] AS site_ids,
                ARRAY[
                    uuid_generate_v5($1, 'onboarding-demo-material-multiplex'),
                    uuid_generate_v5($1, 'onboarding-demo-material-hinge'),
                    uuid_generate_v5($1, 'onboarding-demo-material-glue')
                ] AS material_ids,
                ARRAY[
                    uuid_generate_v5($1, 'onboarding-demo-asset-vehicle'),
                    uuid_generate_v5($1, 'onboarding-demo-asset-tool')
                ] AS asset_ids
        )
        SELECT EXISTS (
            SELECT 1 FROM site_assignments, seeded
            WHERE tenant_id = $1 AND site_id = ANY(seeded.site_ids)
              AND id <> uuid_generate_v5($1, 'onboarding-demo-assignment-admin-active')
            UNION ALL SELECT 1 FROM site_activities, seeded WHERE tenant_id = $1 AND site_id = ANY(seeded.site_ids)
            UNION ALL SELECT 1 FROM site_activity_attachments, seeded WHERE tenant_id = $1 AND site_id = ANY(seeded.site_ids)
            UNION ALL SELECT 1 FROM site_appointments, seeded WHERE tenant_id = $1 AND site_id = ANY(seeded.site_ids)
            UNION ALL SELECT 1 FROM time_entries, seeded WHERE tenant_id = $1 AND site_id = ANY(seeded.site_ids)
            UNION ALL SELECT 1 FROM invoices, seeded WHERE tenant_id = $1 AND site_id = ANY(seeded.site_ids)
            UNION ALL SELECT 1 FROM stock_entries, seeded WHERE tenant_id = $1 AND (site_id = ANY(seeded.site_ids) OR material_id = ANY(seeded.material_ids))
            UNION ALL SELECT 1 FROM order_requests, seeded WHERE tenant_id = $1 AND material_id = ANY(seeded.material_ids)
            UNION ALL SELECT 1 FROM goods_receipt_lines, seeded WHERE tenant_id = $1 AND material_id = ANY(seeded.material_ids)
            UNION ALL SELECT 1 FROM material_batches, seeded
                WHERE tenant_id = $1 AND material_id = ANY(seeded.material_ids)
                  AND id <> uuid_generate_v5($1, 'onboarding-demo-material-glue-batch')
            UNION ALL SELECT 1 FROM reservations, seeded
                WHERE tenant_id = $1 AND (site_id = ANY(seeded.site_ids) OR project_id = ANY(seeded.site_ids) OR asset_id = ANY(seeded.asset_ids))
            UNION ALL SELECT 1 FROM maintenance_schedules, seeded WHERE tenant_id = $1 AND asset_id = ANY(seeded.asset_ids)
        )
        "#,
    )
    .bind(tenant_id.0)
    .fetch_one(&mut **transaction)
    .await
    .map_err(database_error)
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
    delete_seeded_child(
        transaction,
        tenant_id,
        "material_batches",
        "id",
        "onboarding-demo-material-glue-batch",
    )
    .await?;
    delete_seeded_child(
        transaction,
        tenant_id,
        "site_assignments",
        "id",
        "onboarding-demo-assignment-admin-active",
    )
    .await?;
    delete_seeded_child(
        transaction,
        tenant_id,
        "vehicle_display_colors",
        "asset_id",
        "onboarding-demo-asset-vehicle",
    )
    .await?;
    delete_seeded_child(
        transaction,
        tenant_id,
        "vehicle_details",
        "asset_id",
        "onboarding-demo-asset-vehicle",
    )
    .await?;
    delete_seeded_child(
        transaction,
        tenant_id,
        "tool_details",
        "asset_id",
        "onboarding-demo-asset-tool",
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

async fn delete_seeded_child(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
    table: &str,
    id_column: &str,
    name: &str,
) -> Result<(), AppError> {
    let statement = format!(
        "DELETE FROM {table} WHERE tenant_id = $1 AND {id_column} = uuid_generate_v5($1, $2)"
    );
    sqlx::query(&statement)
        .bind(tenant_id.0)
        .bind(name)
        .execute(&mut **transaction)
        .await
        .map(|_| ())
        .map_err(database_error)
}

async fn delete_seeded_assets(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    delete_ids(
        transaction,
        tenant_id,
        "assets",
        &[
            "onboarding-demo-asset-vehicle",
            "onboarding-demo-asset-tool",
        ],
    )
    .await
}

async fn delete_seeded_inventory(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    delete_ids(
        transaction,
        tenant_id,
        "materials",
        &[
            "onboarding-demo-material-multiplex",
            "onboarding-demo-material-hinge",
            "onboarding-demo-material-glue",
        ],
    )
    .await?;
    delete_ids(
        transaction,
        tenant_id,
        "categories",
        &[
            "onboarding-demo-category-plates",
            "onboarding-demo-category-hardware",
            "onboarding-demo-category-consumables",
        ],
    )
    .await
}

async fn delete_seeded_sites(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    delete_ids(
        transaction,
        tenant_id,
        "sites",
        &[
            "onboarding-demo-project-active",
            "onboarding-demo-project-planned",
        ],
    )
    .await
}

async fn delete_ids(
    transaction: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
    table: &str,
    names: &[&str],
) -> Result<(), AppError> {
    for name in names {
        let statement =
            format!("DELETE FROM {table} WHERE tenant_id = $1 AND id = uuid_generate_v5($1, $2)");
        sqlx::query(&statement)
            .bind(tenant_id.0)
            .bind(name)
            .execute(&mut **transaction)
            .await
            .map_err(database_error)?;
    }
    Ok(())
}

fn database_error(error: sqlx::Error) -> AppError {
    AppError::Database(error.to_string())
}
