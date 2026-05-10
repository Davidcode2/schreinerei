use chrono::Datelike;
use sha2::{Digest, Sha256};

use crate::modules::billing::domain::{AttachInvoicePdf, Invoice, InvoiceSnapshot};

const CONTENT_TYPE: &str = "application/pdf";

pub struct GeneratedInvoicePdf {
    pub bytes: Vec<u8>,
    pub metadata: AttachInvoicePdf,
}

pub fn generate_invoice_pdf(invoice: &Invoice) -> Result<GeneratedInvoicePdf, String> {
    let snapshot = invoice
        .snapshot
        .as_ref()
        .ok_or_else(|| "Invoice snapshot is missing".to_string())?;
    let lines = invoice_pdf_lines(invoice, snapshot);
    let bytes = build_text_pdf(&lines);
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

fn invoice_pdf_lines(invoice: &Invoice, snapshot: &InvoiceSnapshot) -> Vec<String> {
    let mut lines = vec![
        format!("Rechnung {}", invoice.invoice_number_display),
        format!("Projekt: {}", snapshot.project_name),
        format!("Kunde: {}", snapshot.customer_name),
    ];

    if let Some(location) = &snapshot.project_location {
        lines.push(format!("Ort: {location}"));
    }
    if let Some(issued_at) = invoice.issued_at.or(Some(invoice.created_at)) {
        lines.push(format!(
            "Datum: {:04}-{:02}-{:02}",
            issued_at.year(),
            issued_at.month(),
            issued_at.day()
        ));
    }
    if let Some(reference) = &snapshot.billing_reference {
        lines.push(format!("Abrechnungsreferenz: {reference}"));
    }
    if let Some(reference) = &snapshot.quote_reference {
        lines.push(format!("Angebot: {reference}"));
    }
    if let Some(budget) = snapshot.budget_amount_cents {
        lines.push(format!("Budget: {}", format_euro_cents(budget)));
    }
    if let Some(sender_name) = &invoice.sender_name {
        lines.push(format!("Absender: {sender_name}"));
    }
    if let Some(sender_address) = &invoice.sender_address {
        lines.push(format!("Adresse: {sender_address}"));
    }

    lines.push(String::new());
    lines.push("Positionen".to_string());
    for line in &snapshot.line_items {
        lines.push(format!(
            "- {}: {} {} ({} Buchungen)",
            line.description, line.quantity, line.unit, line.source_count
        ));
    }
    if snapshot.line_items.is_empty() {
        lines.push("- Keine abrechenbaren Ist-Daten erfasst".to_string());
    }

    lines.push(String::new());
    lines.push(format!(
        "Arbeitszeit gesamt: {} Stunden",
        snapshot.labor_total_hours
    ));
    lines.push(format!(
        "Materialbuchungen: {}",
        snapshot.material_withdrawal_count
    ));
    if let Some(notes) = &snapshot.billing_notes {
        lines.push(String::new());
        lines.push("Abrechnungsnotizen".to_string());
        lines.extend(wrap_text(notes, 86));
    }

    lines
}

fn format_euro_cents(cents: i64) -> String {
    let euros = cents / 100;
    let remainder = (cents % 100).abs();
    format!("{euros}.{remainder:02} EUR")
}

fn wrap_text(value: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in value.split_whitespace() {
        let separator = if current.is_empty() { 0 } else { 1 };
        if current.len() + separator + word.len() > max_chars && !current.is_empty() {
            lines.push(current);
            current = String::new();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn build_text_pdf(lines: &[String]) -> Vec<u8> {
    let content = pdf_text_content(lines);
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>".to_string(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
        format!(
            "<< /Length {} >>\nstream\n{}\nendstream",
            content.len(),
            content
        ),
    ];

    let mut pdf = Vec::new();
    pdf.extend_from_slice(b"%PDF-1.4\n");
    let mut offsets = vec![0usize];

    for (index, object) in objects.iter().enumerate() {
        offsets.push(pdf.len());
        pdf.extend_from_slice(format!("{} 0 obj\n{}\nendobj\n", index + 1, object).as_bytes());
    }

    let xref_offset = pdf.len();
    pdf.extend_from_slice(format!("xref\n0 {}\n", offsets.len()).as_bytes());
    pdf.extend_from_slice(b"0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        pdf.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    pdf.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            offsets.len(),
            xref_offset
        )
        .as_bytes(),
    );

    pdf
}

fn pdf_text_content(lines: &[String]) -> String {
    let mut content = String::from("BT\n/F1 16 Tf\n50 790 Td\n");
    for (index, line) in lines.iter().take(36).enumerate() {
        if index > 0 {
            content.push_str("0 -20 Td\n");
        }
        content.push_str(&format!("({}) Tj\n", escape_pdf_text(line)));
    }
    content.push_str("ET");
    content
}

fn escape_pdf_text(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '(' => "\\(".to_string(),
            ')' => "\\)".to_string(),
            '\\' => "\\\\".to_string(),
            '\n' | '\r' | '\t' => " ".to_string(),
            character if character.is_ascii() && !character.is_control() => character.to_string(),
            _ => "?".to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use super::*;
    use crate::common::types::{InvoiceId, SiteId, TenantId};
    use crate::modules::billing::domain::{InvoiceSnapshotLineItem, InvoiceStatus};

    #[test]
    fn generates_valid_pdf_bytes_from_snapshot() {
        let invoice = Invoice {
            id: InvoiceId(Uuid::new_v4()),
            tenant_id: TenantId(Uuid::new_v4()),
            site_id: SiteId(Uuid::new_v4()),
            invoice_number: 1,
            invoice_number_display: "RE-00001".to_string(),
            status: InvoiceStatus::Draft,
            sender_name: Some("Schreinerei".to_string()),
            sender_address: Some("Werkstrasse 1".to_string()),
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

        assert!(generated.bytes.starts_with(b"%PDF-1.4"));
        assert!(generated.bytes.ends_with(b"%%EOF\n"));
        assert_eq!(generated.metadata.content_type, "application/pdf");
        assert_eq!(generated.metadata.size_bytes, generated.bytes.len() as i64);
        assert_eq!(generated.metadata.sha256_hash.len(), 64);
    }
}
