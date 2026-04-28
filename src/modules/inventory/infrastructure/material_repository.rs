use sqlx::PgPool;

use crate::common::error::AppError;

/// Repository for material data access with tenant isolation
pub struct MaterialRepository {
    pool: PgPool,
}

impl MaterialRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
