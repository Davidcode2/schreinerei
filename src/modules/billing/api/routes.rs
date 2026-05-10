use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use ts_rs::TS;
use uuid::Uuid;

use crate::auth::extractor::AuthenticatedUser;
use crate::common::error::AppError;
use crate::common::types::{InvoiceId, SiteId};
use crate::modules::billing::application::BillingService;
use crate::modules::billing::domain::{Invoice, InvoiceStatus, PdfArtifact};
use crate::modules::billing::infrastructure::InvoiceRepository;
use crate::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/billing/invoices/{id}", get(get_invoice))
        .route(
            "/api/v1/billing/projects/{site_id}/invoices",
            get(list_project_invoices),
        )
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct InvoiceResponse {
    pub id: Uuid,
    pub site_id: Uuid,
    pub invoice_number: i64,
    pub invoice_number_display: String,
    pub status: String,
    pub sender_name: Option<String>,
    pub sender_address: Option<String>,
    pub issued_at: Option<DateTime<Utc>>,
    pub due_on: Option<NaiveDate>,
    pub voided_at: Option<DateTime<Utc>>,
    pub pdf_artifact: Option<PdfArtifactResponse>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct PdfArtifactResponse {
    pub storage_path: String,
    pub sha256_hash: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
}

async fn get_invoice(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let service = BillingService::new(InvoiceRepository::new(state.pool));
    let invoice = service
        .find_invoice(user.tenant_id, InvoiceId(id))
        .await?
        .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;

    Ok(Json(invoice.into()))
}

async fn list_project_invoices(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(site_id): Path<Uuid>,
) -> Result<Json<Vec<InvoiceResponse>>, AppError> {
    let service = BillingService::new(InvoiceRepository::new(state.pool));
    let invoices = service
        .list_project_invoices(user.tenant_id, SiteId(site_id))
        .await?;

    Ok(Json(
        invoices.into_iter().map(InvoiceResponse::from).collect(),
    ))
}

impl From<Invoice> for InvoiceResponse {
    fn from(invoice: Invoice) -> Self {
        Self {
            id: invoice.id.0,
            site_id: invoice.site_id.0,
            invoice_number: invoice.invoice_number,
            invoice_number_display: invoice.invoice_number_display,
            status: status_label(invoice.status),
            sender_name: invoice.sender_name,
            sender_address: invoice.sender_address,
            issued_at: invoice.issued_at,
            due_on: invoice.due_on,
            voided_at: invoice.voided_at,
            pdf_artifact: invoice.pdf_artifact.map(PdfArtifactResponse::from),
            created_by: invoice.created_by.map(|id| id.0),
            created_at: invoice.created_at,
            updated_at: invoice.updated_at,
        }
    }
}

impl From<PdfArtifact> for PdfArtifactResponse {
    fn from(artifact: PdfArtifact) -> Self {
        Self {
            storage_path: artifact.storage_path,
            sha256_hash: artifact.sha256_hash,
            content_type: artifact.content_type,
            size_bytes: artifact.size_bytes,
            created_at: artifact.created_at,
        }
    }
}

fn status_label(status: InvoiceStatus) -> String {
    status.as_str().to_string()
}
