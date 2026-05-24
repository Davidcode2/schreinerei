use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::common::types::{InvoiceId, SiteId, TenantId, UserId};
use crate::modules::sites::domain::InvoicePricingMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingTaxMode {
    Standard,
    Kleinunternehmer,
}

impl BillingTaxMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            BillingTaxMode::Standard => "standard",
            BillingTaxMode::Kleinunternehmer => "kleinunternehmer",
        }
    }
}

impl fmt::Display for BillingTaxMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for BillingTaxMode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "standard" => Ok(BillingTaxMode::Standard),
            "kleinunternehmer" => Ok(BillingTaxMode::Kleinunternehmer),
            _ => Err(format!("Invalid billing tax mode: {value}")),
        }
    }
}

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub snapshot: Option<InvoiceSnapshot>,
    pub pdf_artifact: Option<PdfArtifact>,
    pub created_by: Option<UserId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateInvoiceDraft {
    pub site_id: SiteId,
    pub sender_name: Option<String>,
    pub sender_address: Option<String>,
    pub snapshot: InvoiceSnapshot,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceSnapshot {
    pub project_name: String,
    pub customer_name: String,
    pub project_location: Option<String>,
    #[serde(default)]
    pub invoice_pricing_mode: Option<InvoicePricingMode>,
    #[serde(default)]
    pub hourly_rate_cents: Option<i64>,
    #[serde(default)]
    pub fixed_price_cents: Option<i64>,
    pub billing_reference: Option<String>,
    pub billing_notes: Option<String>,
    pub quote_reference: Option<String>,
    pub budget_amount_cents: Option<i64>,
    pub labor_total_hours: f64,
    pub material_withdrawal_count: i64,
    #[serde(default)]
    pub billing_tax_mode: Option<BillingTaxMode>,
    #[serde(default)]
    pub subtotal_amount_cents: Option<i64>,
    #[serde(default)]
    pub vat_rate_percent: Option<i32>,
    #[serde(default)]
    pub vat_amount_cents: Option<i64>,
    #[serde(default)]
    pub gross_amount_cents: Option<i64>,
    #[serde(default)]
    pub tax_note: Option<String>,
    #[serde(default)]
    pub total_amount_cents: Option<i64>,
    pub line_items: Vec<InvoiceSnapshotLineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceSnapshotLineItem {
    pub source: String,
    pub description: String,
    pub quantity: f64,
    pub unit: String,
    pub source_count: i64,
    pub priced: bool,
    #[serde(default)]
    pub unit_price_cents: Option<i64>,
    #[serde(default)]
    pub line_total_cents: Option<i64>,
}
