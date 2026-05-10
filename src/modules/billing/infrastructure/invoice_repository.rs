use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{InvoiceId, SiteId, TenantId, UserId};
use crate::modules::billing::domain::{AttachInvoicePdf, CreateInvoiceDraft, Invoice, PdfArtifact};

pub struct InvoiceRepository {
    pool: PgPool,
}

impl InvoiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_draft(
        &self,
        tenant_id: TenantId,
        create: CreateInvoiceDraft,
    ) -> Result<Invoice, AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|error| AppError::Database(error.to_string()))?;
        let (invoice_number, invoice_number_display) =
            allocate_invoice_number(&mut tx, tenant_id).await?;
        let invoice_id = Uuid::new_v4();
        let now = Utc::now();

        let row = sqlx::query_as::<_, InvoiceRow>(
            r#"
            INSERT INTO invoices (
                id, tenant_id, site_id, invoice_number, invoice_number_display, status,
                sender_name, sender_address, created_by, created_at, updated_at
            )
            SELECT $1, $2, $3, $4, $5, 'draft', $6, $7, $8, $9, $10
            WHERE EXISTS (
                SELECT 1
                FROM sites
                WHERE id = $3 AND tenant_id = $2 AND deleted_at IS NULL
            )
            RETURNING
                id, tenant_id, site_id, invoice_number, invoice_number_display, status,
                sender_name, sender_address, issued_at, due_on, voided_at,
                pdf_storage_path, pdf_sha256_hash, pdf_content_type, pdf_size_bytes,
                pdf_created_at, created_by, created_at, updated_at
            "#,
        )
        .bind(invoice_id)
        .bind(tenant_id.0)
        .bind(create.site_id.0)
        .bind(invoice_number)
        .bind(&invoice_number_display)
        .bind(normalize_optional_text(create.sender_name.as_deref()))
        .bind(normalize_optional_text(create.sender_address.as_deref()))
        .bind(create.created_by.map(|id| id.0))
        .bind(now)
        .bind(now)
        .fetch_optional(tx.as_mut())
        .await
        .map_err(map_invoice_write_error)?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        tx.commit()
            .await
            .map_err(|error| AppError::Database(error.to_string()))?;

        row.try_into_invoice()
    }

    pub async fn find_by_id(
        &self,
        tenant_id: TenantId,
        invoice_id: InvoiceId,
    ) -> Result<Option<Invoice>, AppError> {
        let row = sqlx::query_as::<_, InvoiceRow>(
            r#"
            SELECT
                id, tenant_id, site_id, invoice_number, invoice_number_display, status,
                sender_name, sender_address, issued_at, due_on, voided_at,
                pdf_storage_path, pdf_sha256_hash, pdf_content_type, pdf_size_bytes,
                pdf_created_at, created_by, created_at, updated_at
            FROM invoices
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(invoice_id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))?;

        row.map(InvoiceRow::try_into_invoice).transpose()
    }

    pub async fn list_for_site(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
    ) -> Result<Vec<Invoice>, AppError> {
        let rows = sqlx::query_as::<_, InvoiceRow>(
            r#"
            SELECT
                id, tenant_id, site_id, invoice_number, invoice_number_display, status,
                sender_name, sender_address, issued_at, due_on, voided_at,
                pdf_storage_path, pdf_sha256_hash, pdf_content_type, pdf_size_bytes,
                pdf_created_at, created_by, created_at, updated_at
            FROM invoices
            WHERE tenant_id = $1 AND site_id = $2
            ORDER BY invoice_number DESC
            "#,
        )
        .bind(tenant_id.0)
        .bind(site_id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))?;

        rows.into_iter().map(InvoiceRow::try_into_invoice).collect()
    }

    pub async fn attach_pdf(
        &self,
        tenant_id: TenantId,
        attach: AttachInvoicePdf,
    ) -> Result<Invoice, AppError> {
        let now = Utc::now();
        let row = sqlx::query_as::<_, InvoiceRow>(
            r#"
            UPDATE invoices
            SET
                status = 'generated',
                pdf_storage_path = $3,
                pdf_sha256_hash = $4,
                pdf_content_type = $5,
                pdf_size_bytes = $6,
                pdf_created_at = $7,
                issued_at = COALESCE(issued_at, $7),
                updated_at = $7
            WHERE id = $1 AND tenant_id = $2
            RETURNING
                id, tenant_id, site_id, invoice_number, invoice_number_display, status,
                sender_name, sender_address, issued_at, due_on, voided_at,
                pdf_storage_path, pdf_sha256_hash, pdf_content_type, pdf_size_bytes,
                pdf_created_at, created_by, created_at, updated_at
            "#,
        )
        .bind(attach.invoice_id.0)
        .bind(tenant_id.0)
        .bind(normalize_required_text(&attach.storage_path, "PDF path")?)
        .bind(normalize_required_text(&attach.sha256_hash, "PDF hash")?)
        .bind(normalize_required_text(
            &attach.content_type,
            "PDF content type",
        )?)
        .bind(attach.size_bytes)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_invoice_write_error)?
        .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;

        row.try_into_invoice()
    }
}

async fn allocate_invoice_number(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    tenant_id: TenantId,
) -> Result<(i64, String), AppError> {
    sqlx::query(
        r#"
        INSERT INTO invoice_number_sequences (tenant_id)
        VALUES ($1)
        ON CONFLICT (tenant_id) DO NOTHING
        "#,
    )
    .bind(tenant_id.0)
    .execute(tx.as_mut())
    .await
    .map_err(|error| AppError::Database(error.to_string()))?;

    let row = sqlx::query(
        r#"
        SELECT next_number, prefix
        FROM invoice_number_sequences
        WHERE tenant_id = $1
        FOR UPDATE
        "#,
    )
    .bind(tenant_id.0)
    .fetch_one(tx.as_mut())
    .await
    .map_err(|error| AppError::Database(error.to_string()))?;

    let invoice_number: i64 = row.try_get("next_number").map_err(map_row_error)?;
    let prefix: String = row.try_get("prefix").map_err(map_row_error)?;
    let display = format!("{}-{:05}", prefix, invoice_number);

    sqlx::query(
        r#"
        UPDATE invoice_number_sequences
        SET next_number = next_number + 1, updated_at = NOW()
        WHERE tenant_id = $1
        "#,
    )
    .bind(tenant_id.0)
    .execute(tx.as_mut())
    .await
    .map_err(|error| AppError::Database(error.to_string()))?;

    Ok((invoice_number, display))
}

#[derive(Debug, FromRow)]
struct InvoiceRow {
    id: Uuid,
    tenant_id: Uuid,
    site_id: Uuid,
    invoice_number: i64,
    invoice_number_display: String,
    status: String,
    sender_name: Option<String>,
    sender_address: Option<String>,
    issued_at: Option<DateTime<Utc>>,
    due_on: Option<NaiveDate>,
    voided_at: Option<DateTime<Utc>>,
    pdf_storage_path: Option<String>,
    pdf_sha256_hash: Option<String>,
    pdf_content_type: Option<String>,
    pdf_size_bytes: Option<i64>,
    pdf_created_at: Option<DateTime<Utc>>,
    created_by: Option<Uuid>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl InvoiceRow {
    fn try_into_invoice(self) -> Result<Invoice, AppError> {
        let status = self.status.parse().map_err(AppError::Database)?;
        let pdf_artifact = self.try_pdf_artifact()?;

        Ok(Invoice {
            id: InvoiceId(self.id),
            tenant_id: TenantId(self.tenant_id),
            site_id: SiteId(self.site_id),
            invoice_number: self.invoice_number,
            invoice_number_display: self.invoice_number_display,
            status,
            sender_name: self.sender_name,
            sender_address: self.sender_address,
            issued_at: self.issued_at,
            due_on: self.due_on,
            voided_at: self.voided_at,
            pdf_artifact,
            created_by: self.created_by.map(UserId),
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }

    fn try_pdf_artifact(&self) -> Result<Option<PdfArtifact>, AppError> {
        match (
            &self.pdf_storage_path,
            &self.pdf_sha256_hash,
            &self.pdf_content_type,
            self.pdf_size_bytes,
            self.pdf_created_at,
        ) {
            (None, None, None, None, None) => Ok(None),
            (
                Some(storage_path),
                Some(sha256_hash),
                Some(content_type),
                Some(size_bytes),
                Some(created_at),
            ) => Ok(Some(PdfArtifact {
                storage_path: storage_path.clone(),
                sha256_hash: sha256_hash.clone(),
                content_type: content_type.clone(),
                size_bytes,
                created_at,
            })),
            _ => Err(AppError::Database(
                "Invoice PDF metadata is incomplete".to_string(),
            )),
        }
    }
}

fn normalize_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|trimmed| !trimmed.is_empty())
        .map(ToOwned::to_owned)
}

fn normalize_required_text(value: &str, label: &str) -> Result<String, AppError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AppError::Validation(format!("{} must not be empty", label)))
    } else {
        Ok(trimmed.to_string())
    }
}

fn map_invoice_write_error(error: sqlx::Error) -> AppError {
    let message = error.to_string();
    if message.contains("invoices_pdf_size_bytes_check") {
        AppError::Validation("PDF size must be greater than zero".to_string())
    } else if message.contains("invoices_status_check") {
        AppError::Validation("Invalid invoice status".to_string())
    } else {
        AppError::Database(message)
    }
}

fn map_row_error(error: sqlx::Error) -> AppError {
    AppError::Database(error.to_string())
}
