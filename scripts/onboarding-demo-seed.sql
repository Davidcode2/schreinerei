-- Minimal demo prefill for a newly onboarded tenant.
-- This dataset is intentionally separate from scripts/realistic-test-data.sql.
--
-- Required psql variables:
--   tenant_id      UUID of the newly created tenant
--   admin_user_id  UUID of the tenant-local admin user, for assignment/demo history

BEGIN;

INSERT INTO categories (id, tenant_id, name, description, can_expire, created_at, updated_at)
VALUES
    (uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-category-plates'), :'tenant_id'::uuid, 'Plattenwerkstoffe', 'Startbestand fuer Korpus- und Montagearbeiten.', FALSE, NOW(), NOW()),
    (uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-category-hardware'), :'tenant_id'::uuid, 'Beschlaege', 'Haeufig benoetigte Beschlaege fuer erste Beispielprojekte.', FALSE, NOW(), NOW()),
    (uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-category-consumables'), :'tenant_id'::uuid, 'Verbrauchsmaterial', 'Leim, Schrauben und Montagekleinteile.', TRUE, NOW(), NOW())
ON CONFLICT (tenant_id, name) DO NOTHING;

INSERT INTO materials (id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at)
VALUES
    (
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-material-multiplex'),
        :'tenant_id'::uuid,
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-category-plates'),
        'Birke Multiplex 18 mm',
        'Demo-Material fuer erste Entnahmen und Bestandswarnungen.',
        'Stueck',
        12,
        5,
        'Plattenlager A',
        'DEMO-MAT-001',
        NOW(),
        NOW()
    ),
    (
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-material-hinge'),
        :'tenant_id'::uuid,
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-category-hardware'),
        'Topfscharnier 110 Grad',
        'Demo-Beschlag fuer Montage- und Projektbeispiele.',
        'Stueck',
        80,
        20,
        'Beschlagschrank 1',
        'DEMO-MAT-002',
        NOW(),
        NOW()
    ),
    (
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-material-glue'),
        :'tenant_id'::uuid,
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-category-consumables'),
        'D4 Leim 500 g',
        'Demo-Verbrauchsmaterial mit Ablaufwarnung.',
        'Stueck',
        8,
        4,
        'Chemieschrank',
        'DEMO-MAT-003',
        NOW(),
        NOW()
    )
ON CONFLICT (tenant_id, name) DO NOTHING;

INSERT INTO sites (id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, budget_amount_cents, billing_reference, billing_notes, quote_reference, created_at, updated_at)
VALUES
    (
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-project-active'),
        :'tenant_id'::uuid,
        'external_site',
        'Demo Einbauschrank',
        'Familie Beispiel',
        'Musterstrasse 12',
        'Aktives Demo-Projekt fuer Dashboard, Zeitbuchung und Materialentnahme.',
        'active',
        CURRENT_DATE - 2,
        CURRENT_DATE + 3,
        4,
        450000,
        'DEMO-2026-001',
        'Demo-Daten fuer die erste Orientierung.',
        'ANG-DEMO-001',
        NOW(),
        NOW()
    ),
    (
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-project-planned'),
        :'tenant_id'::uuid,
        'workshop',
        'Demo Werkstattauftrag',
        'Intern',
        'Werkstatt',
        'Geplanter interner Auftrag fuer den Projektfilter.',
        'planned',
        CURRENT_DATE + 5,
        CURRENT_DATE + 7,
        2,
        NULL,
        NULL,
        NULL,
        NULL,
        NOW(),
        NOW()
    )
ON CONFLICT (tenant_id, name) DO NOTHING;

INSERT INTO site_assignments (id, tenant_id, site_id, user_id, role, created_at)
VALUES
    (
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-assignment-admin-active'),
        :'tenant_id'::uuid,
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-project-active'),
        :'admin_user_id'::uuid,
        'lead',
        NOW()
    )
ON CONFLICT (tenant_id, site_id, user_id) DO NOTHING;

INSERT INTO assets (id, tenant_id, asset_kind, name, description, status, location, qr_code, created_at, updated_at)
VALUES
    (
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-asset-vehicle'),
        :'tenant_id'::uuid,
        'vehicle',
        'Demo Montagebus',
        'Reservierbares Demo-Fahrzeug fuer die Fuhrparkansicht.',
        'available',
        'Hof',
        'DEMO-FLT-001',
        NOW(),
        NOW()
    ),
    (
        uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-asset-tool'),
        :'tenant_id'::uuid,
        'tool',
        'Demo Tauchsage',
        'Reservierbares Demo-Werkzeug fuer die Werkzeugansicht.',
        'available',
        'Werkzeugausgabe',
        'DEMO-TL-001',
        NOW(),
        NOW()
    )
ON CONFLICT (tenant_id, asset_kind, name) DO NOTHING;

INSERT INTO vehicle_details (asset_id, tenant_id, license_plate, vehicle_type, created_at, updated_at)
VALUES
    (uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-asset-vehicle'), :'tenant_id'::uuid, 'DE MO 1', 'van', NOW(), NOW())
ON CONFLICT (asset_id) DO NOTHING;

INSERT INTO tool_details (asset_id, tenant_id, category, created_at, updated_at)
VALUES
    (uuid_generate_v5(:'tenant_id'::uuid, 'onboarding-demo-asset-tool'), :'tenant_id'::uuid, 'Saegetechnik', NOW(), NOW())
ON CONFLICT (asset_id) DO NOTHING;

COMMIT;
