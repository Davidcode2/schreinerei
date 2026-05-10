use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::common::types::{InvoiceId, SiteId, TenantId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InvoiceStatus {
    Draft,
    Generated,
    Void,
}

impl InvoiceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InvoiceStatus::Draft => "draft",
            InvoiceStatus::Generated => "generated",
            InvoiceStatus::Void => "void",
        }
    }
}

impl fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for InvoiceStatus {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "draft" => Ok(InvoiceStatus::Draft),
            "generated" => Ok(InvoiceStatus::Generated),
            "void" => Ok(InvoiceStatus::Void),
            _ => Err(format!("Invalid invoice status: {}", value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PdfArtifact {
    pub storage_path: String,
    pub sha256_hash: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Invoice {
    pub id: InvoiceId,
    pub tenant_id: TenantId,
    pub site_id: SiteId,
    pub invoice_number: i64,
    pub invoice_number_display: String,
    pub status: InvoiceStatus,
    pub sender_name: Option<String>,
    pub sender_address: Option<String>,
    pub issued_at: Option<DateTime<Utc>>,
    pub due_on: Option<NaiveDate>,
    pub voided_at: Option<DateTime<Utc>>,
    pub pdf_artifact: Option<PdfArtifact>,
    pub created_by: Option<UserId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateInvoiceDraft {
    pub site_id: SiteId,
    pub sender_name: Option<String>,
    pub sender_address: Option<String>,
    pub created_by: Option<UserId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachInvoicePdf {
    pub invoice_id: InvoiceId,
    pub storage_path: String,
    pub sha256_hash: String,
    pub content_type: String,
    pub size_bytes: i64,
}
