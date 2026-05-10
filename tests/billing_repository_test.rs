use schreinerei::common::error::AppError;
use schreinerei::common::types::{SiteId, TenantId};
use schreinerei::modules::billing::domain::{AttachInvoicePdf, CreateInvoiceDraft, InvoiceStatus};
use schreinerei::modules::billing::infrastructure::InvoiceRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn allocates_invoice_numbers_per_tenant(pool: PgPool) {
    let tenant_a = create_tenant(&pool, "Tenant A").await;
    let tenant_b = create_tenant(&pool, "Tenant B").await;
    let site_a = create_site(&pool, tenant_a, "Project A").await;
    let site_b = create_site(&pool, tenant_b, "Project B").await;
    let repo = InvoiceRepository::new(pool);

    let first = repo
        .create_draft(tenant_a, draft_for(site_a))
        .await
        .expect("first invoice should be created");
    let second = repo
        .create_draft(tenant_a, draft_for(site_a))
        .await
        .expect("second invoice should be created");
    let other_tenant = repo
        .create_draft(tenant_b, draft_for(site_b))
        .await
        .expect("other tenant invoice should be created");

    assert_eq!(first.invoice_number, 1);
    assert_eq!(first.invoice_number_display, "RE-00001");
    assert_eq!(second.invoice_number, 2);
    assert_eq!(second.invoice_number_display, "RE-00002");
    assert_eq!(other_tenant.invoice_number, 1);
    assert_eq!(other_tenant.invoice_number_display, "RE-00001");
}

#[sqlx::test]
async fn find_and_list_are_tenant_scoped(pool: PgPool) {
    let tenant_a = create_tenant(&pool, "Tenant A").await;
    let tenant_b = create_tenant(&pool, "Tenant B").await;
    let site_a = create_site(&pool, tenant_a, "Project A").await;
    let site_b = create_site(&pool, tenant_b, "Project B").await;
    let repo = InvoiceRepository::new(pool);

    let invoice_a = repo
        .create_draft(tenant_a, draft_for(site_a))
        .await
        .expect("tenant a invoice should be created");
    let _invoice_b = repo
        .create_draft(tenant_b, draft_for(site_b))
        .await
        .expect("tenant b invoice should be created");

    assert!(repo
        .find_by_id(tenant_b, invoice_a.id)
        .await
        .expect("tenant scoped lookup should work")
        .is_none());

    let visible = repo
        .list_for_site(tenant_a, site_a)
        .await
        .expect("tenant scoped list should work");
    assert_eq!(visible.len(), 1);
}

#[sqlx::test]
async fn list_for_site_requires_tenant_owned_site(pool: PgPool) {
    let tenant_a = create_tenant(&pool, "Tenant A").await;
    let tenant_b = create_tenant(&pool, "Tenant B").await;
    let site_a = create_site(&pool, tenant_a, "Project A").await;
    let repo = InvoiceRepository::new(pool);

    let cross_tenant = repo
        .list_for_site(tenant_b, site_a)
        .await
        .expect_err("cross tenant site should be rejected");
    let missing_site = repo
        .list_for_site(tenant_a, SiteId(Uuid::new_v4()))
        .await
        .expect_err("missing site should be rejected");

    assert_not_found(cross_tenant, "Project not found");
    assert_not_found(missing_site, "Project not found");
}

#[sqlx::test]
async fn list_for_site_returns_newest_first(pool: PgPool) {
    let tenant = create_tenant(&pool, "Tenant A").await;
    let site = create_site(&pool, tenant, "Project A").await;
    let repo = InvoiceRepository::new(pool);

    let first = repo
        .create_draft(tenant, draft_for(site))
        .await
        .expect("first invoice should be created");
    let second = repo
        .create_draft(tenant, draft_for(site))
        .await
        .expect("second invoice should be created");

    let visible = repo
        .list_for_site(tenant, site)
        .await
        .expect("tenant scoped list should work");

    assert_eq!(visible.len(), 2);
    assert_eq!(visible[0].id, second.id);
    assert_eq!(visible[1].id, first.id);
}

#[sqlx::test]
async fn attach_pdf_marks_invoice_generated(pool: PgPool) {
    let tenant = create_tenant(&pool, "Tenant A").await;
    let site = create_site(&pool, tenant, "Project A").await;
    let repo = InvoiceRepository::new(pool);
    let invoice = repo
        .create_draft(tenant, draft_for(site))
        .await
        .expect("invoice should be created");

    let updated = repo
        .attach_pdf(
            tenant,
            AttachInvoicePdf {
                invoice_id: invoice.id,
                storage_path: "invoices/tenant/invoice.pdf".to_string(),
                sha256_hash: "abc123".to_string(),
                content_type: "application/pdf".to_string(),
                size_bytes: 42,
            },
        )
        .await
        .expect("pdf metadata should attach");

    let artifact = updated.pdf_artifact.expect("pdf artifact should exist");
    assert_eq!(updated.status, InvoiceStatus::Generated);
    assert_eq!(artifact.storage_path, "invoices/tenant/invoice.pdf");
    assert_eq!(artifact.size_bytes, 42);
    assert!(updated.issued_at.is_some());
}

async fn create_tenant(pool: &PgPool, name: &str) -> TenantId {
    let id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO tenants (id, keycloak_realm, name, slug, keycloak_organization_alias)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(id)
    .bind(format!("realm-{}", id))
    .bind(name)
    .bind(format!("slug-{}", id))
    .bind(format!("alias-{}", id))
    .execute(pool)
    .await
    .expect("tenant should be inserted");

    TenantId(id)
}

async fn create_site(pool: &PgPool, tenant_id: TenantId, name: &str) -> SiteId {
    let id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO sites (id, tenant_id, name, customer_name, status)
        VALUES ($1, $2, $3, $4, 'planned')
        "#,
    )
    .bind(id)
    .bind(tenant_id.0)
    .bind(name)
    .bind("Customer")
    .execute(pool)
    .await
    .expect("site should be inserted");

    SiteId(id)
}

fn draft_for(site_id: SiteId) -> CreateInvoiceDraft {
    CreateInvoiceDraft {
        site_id,
        sender_name: Some("Schreinerei".to_string()),
        sender_address: Some("Werkstrasse 1".to_string()),
        created_by: None,
    }
}

fn assert_not_found(error: AppError, expected: &str) {
    match error {
        AppError::NotFound(message) => assert_eq!(message, expected),
        other => panic!("expected not found error, got {other:?}"),
    }
}
