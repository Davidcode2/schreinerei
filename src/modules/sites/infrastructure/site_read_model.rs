use std::collections::HashMap;

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{SiteId, TenantId};

/// Multi-site labor aggregation row
#[derive(Debug, Clone, FromRow)]
struct LaborSummaryRow {
    site_id: Uuid,
    total_hours: f64,
    worker_count: i64,
}

/// Multi-site material withdrawal totals row
#[derive(Debug, Clone, FromRow)]
struct MaterialTotalsRow {
    site_id: Uuid,
    distinct_material_count: i64,
    withdrawal_count: i64,
}

/// Labor summary per site (lightweight, no work-type split)
#[derive(Debug, Clone, Copy, Default)]
pub struct SiteLaborSummary {
    pub total_hours: f64,
    pub worker_count: i64,
}

/// Material withdrawal totals per site
#[derive(Debug, Clone, Copy, Default)]
pub struct SiteMaterialWithdrawalTotals {
    pub distinct_material_count: i64,
    pub withdrawal_count: i64,
}

/// Shared repository for derived site read models.
pub struct SiteReadModelRepository {
    pool: PgPool,
}

impl SiteReadModelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_labor_summaries(
        &self,
        tenant_id: TenantId,
        site_ids: Option<&[SiteId]>,
    ) -> Result<HashMap<SiteId, SiteLaborSummary>, AppError> {
        let ids: Vec<Uuid> = site_ids
            .map(|ids| ids.iter().map(|id| id.0).collect())
            .unwrap_or_default();

        let rows = if let Some(_ids) = site_ids {
            sqlx::query_as::<_, LaborSummaryRow>(
                r#"
                SELECT
                    te.site_id,
                    COALESCE(SUM(te.hours), 0)::FLOAT8 AS total_hours,
                    COUNT(DISTINCT te.user_id)::INT8 AS worker_count
                FROM time_entries te
                WHERE te.tenant_id = $1 AND te.site_id = ANY($2::uuid[])
                GROUP BY te.site_id
                "#,
            )
            .bind(tenant_id.0)
            .bind(&ids)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
        } else {
            sqlx::query_as::<_, LaborSummaryRow>(
                r#"
                SELECT
                    te.site_id,
                    COALESCE(SUM(te.hours), 0)::FLOAT8 AS total_hours,
                    COUNT(DISTINCT te.user_id)::INT8 AS worker_count
                FROM time_entries te
                WHERE te.tenant_id = $1
                GROUP BY te.site_id
                "#,
            )
            .bind(tenant_id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
        };

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    SiteId(row.site_id),
                    SiteLaborSummary {
                        total_hours: row.total_hours,
                        worker_count: row.worker_count,
                    },
                )
            })
            .collect())
    }

    pub async fn get_material_withdrawal_totals(
        &self,
        tenant_id: TenantId,
        site_ids: Option<&[SiteId]>,
    ) -> Result<HashMap<SiteId, SiteMaterialWithdrawalTotals>, AppError> {
        let ids: Vec<Uuid> = site_ids
            .map(|ids| ids.iter().map(|id| id.0).collect())
            .unwrap_or_default();

        let rows = if let Some(_ids) = site_ids {
            sqlx::query_as::<_, MaterialTotalsRow>(
                r#"
                SELECT
                    se.site_id,
                    COUNT(DISTINCT se.material_id)::INT8 AS distinct_material_count,
                    COUNT(*)::INT8 AS withdrawal_count
                FROM stock_entries se
                WHERE se.tenant_id = $1 AND se.entry_type = 'withdrawn' AND se.site_id = ANY($2::uuid[])
                GROUP BY se.site_id
                "#,
            )
            .bind(tenant_id.0)
            .bind(&ids)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
        } else {
            sqlx::query_as::<_, MaterialTotalsRow>(
                r#"
                SELECT
                    se.site_id,
                    COUNT(DISTINCT se.material_id)::INT8 AS distinct_material_count,
                    COUNT(*)::INT8 AS withdrawal_count
                FROM stock_entries se
                WHERE se.tenant_id = $1 AND se.entry_type = 'withdrawn'
                GROUP BY se.site_id
                "#,
            )
            .bind(tenant_id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
        };

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    SiteId(row.site_id),
                    SiteMaterialWithdrawalTotals {
                        distinct_material_count: row.distinct_material_count,
                        withdrawal_count: row.withdrawal_count,
                    },
                )
            })
            .collect())
    }
}

/// Pure function: classify cost basis from site metadata and actuals.
/// Single source of truth — replaces copy-pasted SQL CASE expressions.
pub fn classify_cost_basis(
    budget_amount_cents: Option<i64>,
    billing_reference: Option<&str>,
    quote_reference: Option<&str>,
    total_hours: f64,
    withdrawal_count: i64,
) -> &'static str {
    let has_billing_ref = billing_reference.is_some() || quote_reference.is_some();
    let has_actuals = total_hours > 0.0 || withdrawal_count > 0;

    if budget_amount_cents.is_some() && has_billing_ref {
        "invoice_ready"
    } else if budget_amount_cents.is_some() && has_actuals {
        "budget_vs_actual"
    } else if budget_amount_cents.is_some() {
        "budget_only"
    } else if has_actuals {
        "actuals_only"
    } else {
        "none"
    }
}
