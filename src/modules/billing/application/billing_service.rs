use crate::common::error::AppError;
use crate::common::types::{InvoiceId, SiteId, TenantId};
use crate::modules::billing::domain::{AttachInvoicePdf, CreateInvoiceDraft, Invoice};
use crate::modules::billing::infrastructure::InvoiceRepository;

pub struct BillingService {
    invoice_repo: InvoiceRepository,
}

impl BillingService {
    pub fn new(invoice_repo: InvoiceRepository) -> Self {
        Self { invoice_repo }
    }

    pub async fn create_draft_invoice(
        &self,
        tenant_id: TenantId,
        create: CreateInvoiceDraft,
    ) -> Result<Invoice, AppError> {
        self.invoice_repo.create_draft(tenant_id, create).await
    }

    pub async fn attach_pdf(
        &self,
        tenant_id: TenantId,
        attach: AttachInvoicePdf,
    ) -> Result<Invoice, AppError> {
        self.invoice_repo.attach_pdf(tenant_id, attach).await
    }

    pub async fn find_invoice(
        &self,
        tenant_id: TenantId,
        invoice_id: InvoiceId,
    ) -> Result<Option<Invoice>, AppError> {
        self.invoice_repo.find_by_id(tenant_id, invoice_id).await
    }

    pub async fn list_project_invoices(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
    ) -> Result<Vec<Invoice>, AppError> {
        self.invoice_repo.list_for_site(tenant_id, site_id).await
    }
}
