use std::sync::LazyLock;

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use sha2::{Digest, Sha256};
use typst::foundations::{Dict, IntoValue};
use typst::layout::PagedDocument;
use typst_as_lib::typst_kit_options::TypstKitFontOptions;
use typst_as_lib::TypstEngine;

use crate::modules::billing::domain::{
    AttachInvoicePdf, Invoice, InvoiceSnapshot, InvoiceSnapshotLineItem,
};

const CONTENT_TYPE: &str = "application/pdf";
const TEMPLATE: &str = include_str!("invoice.typ");

static INVOICE_ENGINE: LazyLock<TypstEngine<typst_as_lib::TypstTemplateMainFile>> =
    LazyLock::new(|| {
        TypstEngine::builder()
            .main_file(TEMPLATE)
            .search_fonts_with(
                TypstKitFontOptions::new()
                    .include_system_fonts(false)
                    .include_embedded_fonts(true),
            )
            .build()
    });

pub struct GeneratedInvoicePdf {
    pub bytes: Vec<u8>,
    pub metadata: AttachInvoicePdf,
}

pub fn generate_invoice_pdf(invoice: &Invoice) -> Result<GeneratedInvoicePdf, String> {
    let snapshot = invoice
        .snapshot
        .as_ref()
        .ok_or_else(|| "Invoice snapshot is missing".to_string())?;
    let document: PagedDocument = INVOICE_ENGINE
        .compile_with_input(invoice_typst_input(invoice, snapshot))
        .output
        .map_err(|error| format!("Failed to compile Typst invoice: {error}"))?;
    let bytes = typst_pdf::pdf(&document, &typst_pdf::PdfOptions::default())
        .map_err(|error| format!("Failed to export Typst invoice PDF: {error:?}"))?;
    let hash = Sha256::digest(&bytes);
    let storage_path = format!(
        "invoices/{}/{}.pdf",
        invoice.tenant_id, invoice.invoice_number_display
    );
    let size_bytes = i64::try_from(bytes.len()).map_err(|_| "PDF is too large".to_string())?;

    Ok(GeneratedInvoicePdf {
        bytes,
        metadata: AttachInvoicePdf {
            invoice_id: invoice.id,
            storage_path,
            sha256_hash: hex::encode(hash),
            content_type: CONTENT_TYPE.to_string(),
            size_bytes,
        },
    })
}

pub fn invoice_pdf_filename(invoice: &Invoice) -> String {
    format!("{}.pdf", invoice.invoice_number_display)
}

fn invoice_typst_input(invoice: &Invoice, snapshot: &InvoiceSnapshot) -> Dict {
    let mut invoice_dict = Dict::new();
    invoice_dict.insert(
        "invoice_number".into(),
        invoice.invoice_number_display.clone().into_value(),
    );
    invoice_dict.insert(
        "issued_at".into(),
        format_datetime(invoice.issued_at.unwrap_or(invoice.created_at)).into_value(),
    );
    invoice_dict.insert(
        "due_on".into(),
        invoice
            .due_on
            .map(format_date)
            .unwrap_or_default()
            .into_value(),
    );
    invoice_dict.insert(
        "sender_name".into(),
        invoice
            .sender_name
            .clone()
            .unwrap_or_else(|| "Schreinerei".to_string())
            .into_value(),
    );
    invoice_dict.insert(
        "sender_address_lines".into(),
        sender_address_lines(invoice).into_value(),
    );
    invoice_dict.insert(
        "sender_compact".into(),
        sender_compact(invoice).into_value(),
    );
    invoice_dict.insert(
        "customer_name".into(),
        snapshot.customer_name.clone().into_value(),
    );
    invoice_dict.insert(
        "project_name".into(),
        snapshot.project_name.clone().into_value(),
    );
    invoice_dict.insert(
        "project_location".into(),
        snapshot
            .project_location
            .clone()
            .unwrap_or_default()
            .into_value(),
    );
    invoice_dict.insert(
        "billing_reference".into(),
        snapshot
            .billing_reference
            .clone()
            .unwrap_or_default()
            .into_value(),
    );
    invoice_dict.insert(
        "quote_reference".into(),
        snapshot
            .quote_reference
            .clone()
            .unwrap_or_default()
            .into_value(),
    );
    invoice_dict.insert(
        "budget_amount".into(),
        snapshot
            .budget_amount_cents
            .map(format_euro_cents)
            .unwrap_or_default()
            .into_value(),
    );
    invoice_dict.insert(
        "labor_total_hours".into(),
        format_hours(snapshot.labor_total_hours).into_value(),
    );
    invoice_dict.insert(
        "material_withdrawal_count".into(),
        count_label(snapshot.material_withdrawal_count, "Bewegung", "Bewegungen").into_value(),
    );
    invoice_dict.insert(
        "billing_notes".into(),
        snapshot
            .billing_notes
            .clone()
            .unwrap_or_default()
            .into_value(),
    );
    invoice_dict.insert(
        "line_items".into(),
        invoice_line_items(snapshot.line_items.as_slice()).into_value(),
    );

    let mut inputs = Dict::new();
    inputs.insert("invoice".into(), invoice_dict.into_value());
    inputs
}

fn sender_address_lines(invoice: &Invoice) -> Vec<String> {
    invoice
        .sender_address
        .as_deref()
        .map(split_non_empty_lines)
        .unwrap_or_default()
}

fn sender_compact(invoice: &Invoice) -> String {
    let mut parts = Vec::new();

    if let Some(name) = invoice
        .sender_name
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        parts.push(name.trim().to_string());
    }

    parts.extend(sender_address_lines(invoice));
    parts.join(" • ")
}

fn invoice_line_items(line_items: &[InvoiceSnapshotLineItem]) -> Vec<Dict> {
    let items = if line_items.is_empty() {
        vec![fallback_line_item()]
    } else {
        line_items.iter().map(build_line_item).collect()
    };

    items
}

fn build_line_item(line_item: &InvoiceSnapshotLineItem) -> Dict {
    let mut item = Dict::new();
    item.insert(
        "description".into(),
        line_item.description.clone().into_value(),
    );
    item.insert(
        "quantity".into(),
        format_quantity(line_item.quantity).into_value(),
    );
    item.insert("unit".into(), localized_unit(&line_item.unit).into_value());
    item.insert(
        "source_count".into(),
        count_label(line_item.source_count, "Buchung", "Buchungen").into_value(),
    );
    item
}

fn fallback_line_item() -> Dict {
    let mut item = Dict::new();
    item.insert(
        "description".into(),
        "Keine abrechenbaren Positionen erfasst"
            .to_string()
            .into_value(),
    );
    item.insert("quantity".into(), "-".to_string().into_value());
    item.insert("unit".into(), "-".to_string().into_value());
    item.insert("source_count".into(), "-".to_string().into_value());
    item
}

fn split_non_empty_lines(value: &str) -> Vec<String> {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn localized_unit(unit: &str) -> String {
    match unit {
        "hours" => "Std.".to_string(),
        "pieces" => "Stk.".to_string(),
        "days" => "Tage".to_string(),
        other => other.to_string(),
    }
}

fn count_label(count: i64, singular: &str, plural: &str) -> String {
    let suffix = if count == 1 { singular } else { plural };
    format!("{count} {suffix}")
}

fn format_hours(hours: f64) -> String {
    format!("{} Std.", format_quantity(hours))
}

fn format_quantity(quantity: f64) -> String {
    let rounded = if quantity.fract().abs() < f64::EPSILON {
        format!("{quantity:.0}")
    } else if (quantity * 10.0).fract().abs() < f64::EPSILON {
        format!("{quantity:.1}")
    } else {
        format!("{quantity:.2}")
    };

    rounded.replace('.', ",")
}

fn format_euro_cents(cents: i64) -> String {
    let euros = cents / 100;
    let remainder = (cents % 100).abs();
    format!("{euros},{remainder:02} EUR")
}

fn format_datetime(value: DateTime<Utc>) -> String {
    format!(
        "{:02}.{:02}.{:04}",
        value.day(),
        value.month(),
        value.year()
    )
}

fn format_date(value: NaiveDate) -> String {
    format!(
        "{:02}.{:02}.{:04}",
        value.day(),
        value.month(),
        value.year()
    )
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use super::*;
    use crate::common::types::{InvoiceId, SiteId, TenantId};
    use crate::modules::billing::domain::InvoiceStatus;

    #[test]
    fn generates_valid_pdf_bytes_from_snapshot() {
        let invoice = Invoice {
            id: InvoiceId(Uuid::new_v4()),
            tenant_id: TenantId(Uuid::new_v4()),
            site_id: SiteId(Uuid::new_v4()),
            invoice_number: 1,
            invoice_number_display: "RE-00001".to_string(),
            status: InvoiceStatus::Draft,
            sender_name: Some("Muster Schreinerei GmbH".to_string()),
            sender_address: Some("Werkstrasse 1\n10115 Berlin".to_string()),
            issued_at: None,
            due_on: None,
            voided_at: None,
            snapshot: Some(InvoiceSnapshot {
                project_name: "Kueche Meyer".to_string(),
                customer_name: "Meyer".to_string(),
                project_location: Some("Berlin".to_string()),
                billing_reference: Some("BR-1".to_string()),
                billing_notes: Some("Nach Aufwand".to_string()),
                quote_reference: None,
                budget_amount_cents: Some(125_000),
                labor_total_hours: 4.5,
                material_withdrawal_count: 1,
                line_items: vec![InvoiceSnapshotLineItem {
                    source: "labor_site".to_string(),
                    description: "Baustellenarbeit".to_string(),
                    quantity: 4.5,
                    unit: "hours".to_string(),
                    source_count: 1,
                    priced: false,
                }],
            }),
            pdf_artifact: None,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let generated = generate_invoice_pdf(&invoice).expect("pdf should generate");

        assert!(generated.bytes.starts_with(b"%PDF-"));
        assert!(generated.bytes.len() > 1_000);
        assert_eq!(generated.metadata.content_type, "application/pdf");
        assert_eq!(generated.metadata.size_bytes, generated.bytes.len() as i64);
        assert_eq!(generated.metadata.sha256_hash.len(), 64);
    }

    #[test]
    fn formats_sender_compact_line_without_empty_segments() {
        let invoice = Invoice {
            id: InvoiceId(Uuid::new_v4()),
            tenant_id: TenantId(Uuid::new_v4()),
            site_id: SiteId(Uuid::new_v4()),
            invoice_number: 1,
            invoice_number_display: "RE-00001".to_string(),
            status: InvoiceStatus::Draft,
            sender_name: Some("Musterbetrieb".to_string()),
            sender_address: Some("Hauptstrasse 1\n\n80331 Muenchen".to_string()),
            issued_at: None,
            due_on: None,
            voided_at: None,
            snapshot: None,
            pdf_artifact: None,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(
            sender_compact(&invoice),
            "Musterbetrieb • Hauptstrasse 1 • 80331 Muenchen"
        );
    }
}
