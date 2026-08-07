BEGIN;

INSERT INTO tenants (
    id, keycloak_realm, name, slug, keycloak_organization_alias,
    default_hourly_rate_cents, billing_tax_mode, billing_sender_name, billing_sender_address
)
VALUES (
    :'tenant_id'::uuid,
    'schreinerei',
    'Schreinerei Saur Affalterwang',
    'schreinerei-saur-affalterwang',
    :'org_alias',
    8500,
    'standard',
    'Schreinerei Saur Affalterwang',
    E'Hauptstrasse 18\n73453 Abtsgmuend'
)
ON CONFLICT (id) DO UPDATE
SET
    keycloak_realm = EXCLUDED.keycloak_realm,
    name = EXCLUDED.name,
    slug = EXCLUDED.slug,
    keycloak_organization_alias = EXCLUDED.keycloak_organization_alias,
    default_hourly_rate_cents = EXCLUDED.default_hourly_rate_cents,
    billing_tax_mode = EXCLUDED.billing_tax_mode,
    billing_sender_name = EXCLUDED.billing_sender_name,
    billing_sender_address = EXCLUDED.billing_sender_address;

DELETE FROM domain_events WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM organization_invites WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM invoices WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM invoice_number_sequences WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM goods_receipt_lines WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM goods_receipts WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM site_activity_attachments WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM site_activities WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM site_appointments WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM reservations WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM maintenance_due WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM maintenance_schedules WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM vehicle_display_colors WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM machine_details WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM tool_details WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM vehicle_details WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM assets WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM time_entries WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM site_assignments WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM stock_entries WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM order_requests WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM material_batches WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM materials WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM categories WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM tools WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM vehicles WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM sites WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM user_preferences WHERE tenant_id = :'tenant_id'::uuid;
DELETE FROM users WHERE tenant_id = :'tenant_id'::uuid;

INSERT INTO users (id, tenant_id, keycloak_user_id, email, name, role, created_at, updated_at)
VALUES
    ('00000000-0000-0000-0000-000000000101', :'tenant_id'::uuid, :'auth_keycloak_user_id', :'auth_email', :'auth_name', 'admin', NOW() - INTERVAL '180 days', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000000102', :'tenant_id'::uuid, 'seed-martin-brenner', 'martin.brenner@schreinerei-saur.de', 'Martin Brenner', 'employee', NOW() - INTERVAL '170 days', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000103', :'tenant_id'::uuid, 'seed-lukas-eisele', 'lukas.eisele@schreinerei-saur.de', 'Lukas Eisele', 'employee', NOW() - INTERVAL '160 days', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000104', :'tenant_id'::uuid, 'seed-jonas-maier', 'jonas.maier@schreinerei-saur.de', 'Jonas Maier', 'employee', NOW() - INTERVAL '140 days', NOW() - INTERVAL '3 days'),
    ('00000000-0000-0000-0000-000000000105', :'tenant_id'::uuid, 'seed-tobias-gruber', 'tobias.gruber@schreinerei-saur.de', 'Tobias Gruber', 'employee', NOW() - INTERVAL '120 days', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000000106', :'tenant_id'::uuid, 'seed-anna-rieger', 'anna.rieger@schreinerei-saur.de', 'Anna Rieger', 'employee', NOW() - INTERVAL '110 days', NOW() - INTERVAL '4 days');

INSERT INTO user_preferences (id, tenant_id, user_id, preferences, created_at, updated_at)
VALUES
    ('00000000-0000-0000-0000-000000001101', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000101', '{"active_site_id":"00000000-0000-0000-0000-000000000401"}', NOW() - INTERVAL '90 days', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000001102', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000102', '{"active_site_id":"00000000-0000-0000-0000-000000000401"}', NOW() - INTERVAL '90 days', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000001103', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000103', '{"active_site_id":"00000000-0000-0000-0000-000000000404"}', NOW() - INTERVAL '90 days', NOW() - INTERVAL '3 days');

INSERT INTO categories (id, tenant_id, name, description, can_expire, created_at, updated_at)
VALUES
    ('00000000-0000-0000-0000-000000000201', :'tenant_id'::uuid, 'Plattenwerkstoffe', 'Dekorplatten, MDF und Multiplex für Korpus- und Möbelbau.', FALSE, NOW() - INTERVAL '120 days', NOW() - INTERVAL '10 days'),
    ('00000000-0000-0000-0000-000000000202', :'tenant_id'::uuid, 'Massivholz', 'Leimholz, Friesen und Stäbe für Treppen, Fensterbänke und Sonderteile.', FALSE, NOW() - INTERVAL '120 days', NOW() - INTERVAL '12 days'),
    ('00000000-0000-0000-0000-000000000203', :'tenant_id'::uuid, 'Kanten', 'ABS- und Echtholzkanten für Sichtseiten und Fronten.', FALSE, NOW() - INTERVAL '120 days', NOW() - INTERVAL '8 days'),
    ('00000000-0000-0000-0000-000000000204', :'tenant_id'::uuid, 'Verbindungsmittel', 'Schrauben, Confirmats und Montagekleinteile.', FALSE, NOW() - INTERVAL '120 days', NOW() - INTERVAL '6 days'),
    ('00000000-0000-0000-0000-000000000205', :'tenant_id'::uuid, 'Beschläge', 'Scharniere, Auszüge und Klappenbeschläge für Möbelmontage.', FALSE, NOW() - INTERVAL '120 days', NOW() - INTERVAL '5 days'),
    ('00000000-0000-0000-0000-000000000206', :'tenant_id'::uuid, 'Leim und Chemie', 'Leime, Reiniger und Verbrauchsmaterial für die Montage.', TRUE, NOW() - INTERVAL '120 days', NOW() - INTERVAL '4 days'),
    ('00000000-0000-0000-0000-000000000207', :'tenant_id'::uuid, 'Oberflächen', 'Öle, Lacke und Pflegemittel für Holzoberflächen.', TRUE, NOW() - INTERVAL '120 days', NOW() - INTERVAL '3 days');

INSERT INTO materials (
    id, tenant_id, category_id, name, description, unit, quantity, legacy_quantity,
    min_quantity, location, qr_code, base_price_cents, price_markup_percentage, created_at, updated_at
)
VALUES
    ('00000000-0000-0000-0000-000000000301', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000201', 'Birke Multiplex 18 mm 2500 x 1250', 'Standardplatte für Korpusse, Werkbankeinlagen und Sonderteile.', 'Stück', 14, 14, 6, 'Plattenlager A', 'MAT-PLATTE-001', 8950, 25, NOW() - INTERVAL '110 days', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000000302', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000202', 'Eiche Leimholz 26 mm', 'Für Treppenwangen, Fensterbänke und massive Abdeckplatten.', 'Quadratmeter', 28, 28, 10, 'Holzlager rechts', 'MAT-HOLZ-002', 12800, 30, NOW() - INTERVAL '110 days', NOW() - INTERVAL '6 days'),
    ('00000000-0000-0000-0000-000000000303', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000201', 'MDF weiß 19 mm', 'Lackfähige Platte für Sockel, Einbauten und Hilfsteile.', 'Stück', 22, 22, 8, 'Plattenlager B', 'MAT-PLATTE-003', 4250, 25, NOW() - INTERVAL '110 days', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000304', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000201', 'HPL Schichtstoff Egger W1100 ST9', 'Weißer Schichtstoff für stark beanspruchte Sichtflächen.', 'Quadratmeter', 18, 18, 6, 'Schichtstoffwagen', 'MAT-PLATTE-004', 3150, 30, NOW() - INTERVAL '110 days', NOW() - INTERVAL '7 days'),
    ('00000000-0000-0000-0000-000000000305', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000203', 'ABS-Kante Eiche 23 x 2 mm', 'Passend zu Eiche-Dekoren und furnierten Fronten.', 'Meter', 180, 180, 60, 'Kantenregal 1', 'MAT-KANTE-005', 145, 35, NOW() - INTERVAL '110 days', NOW() - INTERVAL '4 days'),
    ('00000000-0000-0000-0000-000000000306', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000203', 'ABS-Kante Weiß 23 x 2 mm', 'Für weiße Korpusse und Einbauschränke.', 'Meter', 220, 220, 80, 'Kantenregal 2', 'MAT-KANTE-006', 110, 35, NOW() - INTERVAL '110 days', NOW() - INTERVAL '5 days'),
    ('00000000-0000-0000-0000-000000000307', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000204', 'Spanplattenschraube 4 x 50 TX20', 'Universalschraube für Korpusse und Unterkonstruktionen.', 'Stück', 4200, 4200, 1500, 'Kleinteilewand A4', 'MAT-SCHR-007', 8, 40, NOW() - INTERVAL '110 days', NOW() - INTERVAL '3 days'),
    ('00000000-0000-0000-0000-000000000308', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000204', 'Confirmat Schraube 7 x 50', 'Korpusverbinder für zerlegbare Möbelteile.', 'Stück', 950, 950, 300, 'Kleinteilewand A6', 'MAT-SCHR-008', 12, 40, NOW() - INTERVAL '110 days', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000309', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000205', 'Topfscharnier Blum Clip Top 110 Grad', 'Gedämpftes Standardband für Küchen und Schränke.', 'Stück', 180, 180, 60, 'Beschlagschrank 1', 'MAT-BESCHLAG-009', 520, 30, NOW() - INTERVAL '110 days', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000310', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000205', 'Tandembox Vollauszug 500 mm links/rechts', 'Auszugssatz für breite Küchenauszüge.', 'Stück', 36, 36, 12, 'Beschlagschrank 3', 'MAT-BESCHLAG-010', 6850, 25, NOW() - INTERVAL '110 days', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000000311', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000206', 'PUR-Leim D4 500 g', 'Wasserfester Klebstoff für Montage und Außenbereiche.', 'Stück', 24, 0, 30, 'Chemieschrank unten', 'MAT-CHEMIE-011', NULL, NULL, NOW() - INTERVAL '110 days', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000000312', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000207', 'Hartwachsöl farblos', 'Für geölte Eichenoberflächen im Innenausbau.', 'Liter', 11, 0, 4, 'Lacklager links', 'MAT-OBERFL-012', 2490, 30, NOW() - INTERVAL '110 days', NOW() - INTERVAL '9 days'),
    ('00000000-0000-0000-0000-000000000313', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000207', 'Weißlack seidenmatt', 'Decklack für MDF-Fronten und Einbauschränke.', 'Liter', 18, 0, 6, 'Lacklager rechts', 'MAT-OBERFL-013', 2990, 30, NOW() - INTERVAL '110 days', NOW() - INTERVAL '8 days');

INSERT INTO sites (
    id, tenant_id, name, customer_name, location, description, status, start_date, end_date,
    estimated_days, project_type, budget_amount_cents, billing_reference, billing_notes,
    quote_reference, invoice_pricing_mode, hourly_rate_cents, fixed_price_cents, created_at, updated_at
)
VALUES
    ('00000000-0000-0000-0000-000000000401', :'tenant_id'::uuid, 'Einbauschrank Dachgeschoss', 'Familie Maier', 'Birkenweg 12, 73453 Abtsgmuend-Pommertsweiler', 'Passgenauer Einbauschrank mit Sitznische und Garderobe im Dachgeschoss.', 'active', CURRENT_DATE - 12, CURRENT_DATE + 3, 7, 'external_site', 1450000, 'MAIER-DG-2026', 'Abschlag bereits berechnet.', 'ANG-2026-041', 'hourly_rate', 8900, NULL, NOW() - INTERVAL '14 days', NOW() - INTERVAL '4 hours'),
    ('00000000-0000-0000-0000-000000000402', :'tenant_id'::uuid, 'Empfang Praxis Dr. Seidel', 'Praxis Dr. Seidel', 'Heidenheimer Strasse 8, 73447 Oberkochen', 'Neuer Empfangstresen mit HPL-Arbeitsplatte und Stauraum fuer Unterlagen.', 'planned', CURRENT_DATE + 5, CURRENT_DATE + 10, 4, 'external_site', 980000, 'SEIDEL-EMPFANG', NULL, 'ANG-2026-052', NULL, NULL, NULL, NOW() - INTERVAL '9 days', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000000403', :'tenant_id'::uuid, 'Garderoben Kindergarten St. Martin', 'Kath. Kirchengemeinde Ellwangen', 'Schulstrasse 3, 73479 Ellwangen-Rindelbach', 'Kindgerechte Garderobenmoebel mit Sitzbank und Schuhrost.', 'completed', CURRENT_DATE - 30, CURRENT_DATE - 18, 6, 'external_site', 1120000, 'KIGA-2026', 'Schlussrechnung nach Abnahme.', 'ANG-2026-017', 'fixed_price', NULL, 1120000, NOW() - INTERVAL '35 days', NOW() - INTERVAL '18 days'),
    ('00000000-0000-0000-0000-000000000404', :'tenant_id'::uuid, 'Verkaufstheke Hofladen Braun', 'Hofladen Braun', 'Aalener Strasse 22, 73466 Lauchheim', 'Massive Verkaufstheke aus Eiche mit integriertem Kassenauszug.', 'active', CURRENT_DATE - 4, CURRENT_DATE + 2, 3, 'external_site', 760000, 'BRAUN-THEKE', NULL, 'ANG-2026-049', 'hourly_rate', 8500, NULL, NOW() - INTERVAL '7 days', NOW() - INTERVAL '5 hours'),
    ('00000000-0000-0000-0000-000000000405', :'tenant_id'::uuid, 'Treppenaufgang Familie Riek', 'Familie Riek', 'Dorfstrasse 5, 73460 Huettlingen-Sulzdorf', 'Setzstufen, Handlauf und Verkleidung fuer einen sanierten Treppenaufgang.', 'archived', CURRENT_DATE - 90, CURRENT_DATE - 70, 10, 'external_site', 1850000, 'RIEK-TREPPE', NULL, 'ANG-2025-188', 'fixed_price', NULL, 1850000, NOW() - INTERVAL '95 days', NOW() - INTERVAL '60 days');

INSERT INTO site_assignments (id, tenant_id, site_id, user_id, role, created_at)
VALUES
    ('00000000-0000-0000-0000-000000000451', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000102', 'lead', NOW() - INTERVAL '12 days'),
    ('00000000-0000-0000-0000-000000000452', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000103', 'worker', NOW() - INTERVAL '12 days'),
    ('00000000-0000-0000-0000-000000000453', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000104', 'worker', NOW() - INTERVAL '11 days'),
    ('00000000-0000-0000-0000-000000000454', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000402', '00000000-0000-0000-0000-000000000105', 'lead', NOW() - INTERVAL '3 days'),
    ('00000000-0000-0000-0000-000000000455', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000402', '00000000-0000-0000-0000-000000000106', 'worker', NOW() - INTERVAL '3 days'),
    ('00000000-0000-0000-0000-000000000456', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000403', '00000000-0000-0000-0000-000000000103', 'lead', NOW() - INTERVAL '28 days'),
    ('00000000-0000-0000-0000-000000000457', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000404', '00000000-0000-0000-0000-000000000104', 'lead', NOW() - INTERVAL '4 days'),
    ('00000000-0000-0000-0000-000000000458', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000404', '00000000-0000-0000-0000-000000000105', 'worker', NOW() - INTERVAL '4 days');

INSERT INTO time_entries (id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at)
VALUES
    ('00000000-0000-0000-0000-000000000801', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000102', 'site', 8.00, CURRENT_DATE - 6, 'Korpusse gesetzt und Sitznische eingepasst.', NOW() - INTERVAL '6 days'),
    ('00000000-0000-0000-0000-000000000802', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000103', 'site', 7.50, CURRENT_DATE - 6, 'Schiebetueren montiert und Frontfugen eingestellt.', NOW() - INTERVAL '6 days'),
    ('00000000-0000-0000-0000-000000000803', :'tenant_id'::uuid, NULL, '00000000-0000-0000-0000-000000000104', 'workshop', 6.50, CURRENT_DATE - 5, 'Zuschnitt Multiplex und MDF fuer Hofladen Braun.', NOW() - INTERVAL '5 days'),
    ('00000000-0000-0000-0000-000000000804', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000404', '00000000-0000-0000-0000-000000000104', 'site', 8.00, CURRENT_DATE - 4, 'Verkaufstheke eingepasst und Kassenauszug montiert.', NOW() - INTERVAL '4 days'),
    ('00000000-0000-0000-0000-000000000805', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000404', '00000000-0000-0000-0000-000000000105', 'travel', 1.25, CURRENT_DATE - 4, 'Fahrt mit Material und Absaugmobil nach Lauchheim.', NOW() - INTERVAL '4 days'),
    ('00000000-0000-0000-0000-000000000806', :'tenant_id'::uuid, NULL, '00000000-0000-0000-0000-000000000101', 'other', 2.00, CURRENT_DATE - 3, 'Baubesprechung und Auftragsfreigabe fuer Praxis Dr. Seidel.', NOW() - INTERVAL '3 days'),
    ('00000000-0000-0000-0000-000000000807', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000402', '00000000-0000-0000-0000-000000000105', 'site', 3.00, CURRENT_DATE - 3, 'Aufmass vor Ort und Leitungsfuehrung geprueft.', NOW() - INTERVAL '3 days'),
    ('00000000-0000-0000-0000-000000000808', :'tenant_id'::uuid, NULL, '00000000-0000-0000-0000-000000000106', 'workshop', 7.00, CURRENT_DATE - 2, 'Beschlaege kommissioniert und Auszuege vormontiert.', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000809', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000102', 'site', 8.50, CURRENT_DATE - 1, 'Scharniere eingestellt, Sockel angepasst und Baustelle uebergeben.', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000000810', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000104', 'travel', 1.00, CURRENT_DATE - 1, 'Rueckfahrt und Restmaterial eingelagert.', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000000811', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000403', '00000000-0000-0000-0000-000000000103', 'site', 7.75, CURRENT_DATE - 20, 'Kindergarten-Garderoben montiert und Hakenleisten gesetzt.', NOW() - INTERVAL '20 days'),
    ('00000000-0000-0000-0000-000000000812', :'tenant_id'::uuid, NULL, '00000000-0000-0000-0000-000000000103', 'workshop', 5.50, CURRENT_DATE - 19, 'Fronten lackfertig geschliffen fuer Kindergartenauftrag.', NOW() - INTERVAL '19 days');

INSERT INTO site_activities (
    id, tenant_id, site_id, user_id, activity_type, content, photo_url, created_at
)
VALUES
    ('00000000-0000-0000-0000-000000001401', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000101', 'status_change', '{"old_status":"planned","new_status":"active"}', NULL, NOW() - INTERVAL '11 days'),
    ('00000000-0000-0000-0000-000000001402', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000102', 'note', 'Korpusse stehen. Passleisten und Sitznische werden nach der Abnahme final eingestellt.', NULL, NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000001403', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000103', 'note', 'Beschlaege vollstaendig montiert, Frontfugen kontrolliert und Restpunkte dokumentiert.', NULL, NOW() - INTERVAL '6 hours');

INSERT INTO assets (id, tenant_id, asset_kind, name, description, status, location, qr_code, created_at, updated_at, deleted_at)
VALUES
    ('00000000-0000-0000-0000-000000000501', :'tenant_id'::uuid, 'vehicle', 'VW Crafter Montage 1', 'Grosses Montagefahrzeug mit Regalsystem fuer Baustellenmaterial.', 'available', 'Hof 1', 'FLT-VEH-501', NOW() - INTERVAL '300 days', NOW() - INTERVAL '2 days', NULL),
    ('00000000-0000-0000-0000-000000000502', :'tenant_id'::uuid, 'vehicle', 'Mercedes Sprinter Montage 2', 'Montagebus fuer Zweierkolonne mit Dachtraeger.', 'available', 'Hof 1', 'FLT-VEH-502', NOW() - INTERVAL '260 days', NOW() - INTERVAL '3 hours', NULL),
    ('00000000-0000-0000-0000-000000000503', :'tenant_id'::uuid, 'vehicle', 'Ford Ranger Zugfahrzeug', 'Zugfahrzeug fuer Anhaenger und Materialfahrten.', 'in_use', 'Baustelle Hofladen Braun', 'FLT-VEH-503', NOW() - INTERVAL '220 days', NOW() - INTERVAL '5 hours', NULL),
    ('00000000-0000-0000-0000-000000000504', :'tenant_id'::uuid, 'vehicle', 'Humbaur Kofferanhaenger', 'Geschlossener Anhaenger fuer lange Bauteile und Werkzeug.', 'maintenance', 'Werkstatt hinten', 'FLT-VEH-504', NOW() - INTERVAL '200 days', NOW() - INTERVAL '1 day', NULL),
    ('00000000-0000-0000-0000-000000000601', :'tenant_id'::uuid, 'tool', 'Festool TS 55 Tauchsäge', 'Tauchsaege fuer passgenaue Zuschnitte auf der Baustelle.', 'available', 'Werkzeugausgabe', 'FLT-TL-601', NOW() - INTERVAL '240 days', NOW() - INTERVAL '2 days', NULL),
    ('00000000-0000-0000-0000-000000000602', :'tenant_id'::uuid, 'tool', 'Lamello Zeta P2', 'Profilnut- und Verbinderfraese fuer Korpusmontage und Verkleidungen.', 'in_use', 'Baustelle Einbauschrank Dachgeschoss', 'FLT-TL-602', NOW() - INTERVAL '230 days', NOW() - INTERVAL '4 hours', NULL),
    ('00000000-0000-0000-0000-000000000603', :'tenant_id'::uuid, 'tool', 'Mafell Erika 85', 'Mobile Unterflurzugsaege fuer praezise Zuschnitte im Ausbau.', 'maintenance', 'Serviceecke Werkstatt', 'FLT-TL-603', NOW() - INTERVAL '220 days', NOW() - INTERVAL '1 day', NULL),
    ('00000000-0000-0000-0000-000000000604', :'tenant_id'::uuid, 'tool', 'Makita DHR243 Bohrhammer', 'Akkubohrhammer fuer Montage in Beton und Mauerwerk.', 'available', 'Montagewagen 2', 'FLT-TL-604', NOW() - INTERVAL '210 days', NOW() - INTERVAL '3 hours', NULL),
    ('00000000-0000-0000-0000-000000000605', :'tenant_id'::uuid, 'tool', 'Festool CTL MIDI Absaugmobil', 'Mobiles Absauggeraet fuer Baustellenmontage und Zuschnitt.', 'available', 'Werkzeugausgabe', 'FLT-TL-605', NOW() - INTERVAL '200 days', NOW() - INTERVAL '5 days', NULL),
    ('00000000-0000-0000-0000-000000000606', :'tenant_id'::uuid, 'tool', 'Fein MultiMaster', 'Oszillierendes Werkzeug fuer Nacharbeiten und Ausschnitte.', 'available', 'Montagewagen 1', 'FLT-TL-606', NOW() - INTERVAL '190 days', NOW() - INTERVAL '2 hours', NULL);

INSERT INTO vehicle_details (asset_id, tenant_id, license_plate, vehicle_type, created_at, updated_at)
VALUES
    ('00000000-0000-0000-0000-000000000501', :'tenant_id'::uuid, 'AA-SA 241', 'van', NOW() - INTERVAL '300 days', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000502', :'tenant_id'::uuid, 'AA-SA 318', 'van', NOW() - INTERVAL '260 days', NOW() - INTERVAL '3 hours'),
    ('00000000-0000-0000-0000-000000000503', :'tenant_id'::uuid, 'AA-SA 519', 'truck', NOW() - INTERVAL '220 days', NOW() - INTERVAL '5 hours'),
    ('00000000-0000-0000-0000-000000000504', :'tenant_id'::uuid, 'AA-SA 904', 'trailer', NOW() - INTERVAL '200 days', NOW() - INTERVAL '1 day');

INSERT INTO tool_details (asset_id, tenant_id, category, created_at, updated_at)
VALUES
    ('00000000-0000-0000-0000-000000000601', :'tenant_id'::uuid, 'Saegetechnik', NOW() - INTERVAL '240 days', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000602', :'tenant_id'::uuid, 'Verbindungstechnik', NOW() - INTERVAL '230 days', NOW() - INTERVAL '4 hours'),
    ('00000000-0000-0000-0000-000000000603', :'tenant_id'::uuid, 'Maschinen', NOW() - INTERVAL '220 days', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000000604', :'tenant_id'::uuid, 'Bohr- und Montagetechnik', NOW() - INTERVAL '210 days', NOW() - INTERVAL '3 hours'),
    ('00000000-0000-0000-0000-000000000605', :'tenant_id'::uuid, 'Absaugung', NOW() - INTERVAL '200 days', NOW() - INTERVAL '5 days'),
    ('00000000-0000-0000-0000-000000000606', :'tenant_id'::uuid, 'Multifunktionswerkzeug', NOW() - INTERVAL '190 days', NOW() - INTERVAL '2 hours');

INSERT INTO vehicle_display_colors (asset_id, tenant_id, display_color, created_at, updated_at)
VALUES
    ('00000000-0000-0000-0000-000000000501', :'tenant_id'::uuid, '#2563eb', NOW() - INTERVAL '300 days', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000502', :'tenant_id'::uuid, '#16a34a', NOW() - INTERVAL '260 days', NOW() - INTERVAL '3 hours'),
    ('00000000-0000-0000-0000-000000000503', :'tenant_id'::uuid, '#ea580c', NOW() - INTERVAL '220 days', NOW() - INTERVAL '5 hours'),
    ('00000000-0000-0000-0000-000000000504', :'tenant_id'::uuid, '#7c3aed', NOW() - INTERVAL '200 days', NOW() - INTERVAL '1 day');

INSERT INTO reservations (
    id, tenant_id, resource_type, resource_id, asset_id, user_id, site_id, project_id,
    start_time, end_time, status, purpose, notes, created_at, updated_at
)
VALUES
    ('00000000-0000-0000-0000-000000000701', :'tenant_id'::uuid, 'tool', '00000000-0000-0000-0000-000000000602', '00000000-0000-0000-0000-000000000602', '00000000-0000-0000-0000-000000000102', '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000401', NOW() - INTERVAL '2 hours', NOW() + INTERVAL '5 hours', 'in_use', 'Passleisten und Sockelblenden montieren', 'Lamello fuer finale Passleisten und Sockelblenden.', NOW() - INTERVAL '2 days', NOW() - INTERVAL '3 hours'),
    ('00000000-0000-0000-0000-000000000702', :'tenant_id'::uuid, 'vehicle', '00000000-0000-0000-0000-000000000502', '00000000-0000-0000-0000-000000000502', '00000000-0000-0000-0000-000000000105', '00000000-0000-0000-0000-000000000402', '00000000-0000-0000-0000-000000000402', date_trunc('day', NOW()) + INTERVAL '5 days 06:30', date_trunc('day', NOW()) + INTERVAL '6 days 17:30', 'confirmed', 'Praxiseinbau und Materialtransport', 'Montagebus fuer Praxiseinbau und HPL-Platten.', NOW() - INTERVAL '1 day', NOW() - INTERVAL '3 hours'),
    ('00000000-0000-0000-0000-000000000703', :'tenant_id'::uuid, 'tool', '00000000-0000-0000-0000-000000000604', '00000000-0000-0000-0000-000000000604', '00000000-0000-0000-0000-000000000105', '00000000-0000-0000-0000-000000000402', '00000000-0000-0000-0000-000000000402', date_trunc('day', NOW()) + INTERVAL '5 days 06:30', date_trunc('day', NOW()) + INTERVAL '5 days 16:00', 'confirmed', 'Empfangsunterkonstruktion befestigen', 'Bohrhammer fuer Befestigung der Empfangsunterkonstruktion.', NOW() - INTERVAL '1 day', NOW() - INTERVAL '3 hours'),
    ('00000000-0000-0000-0000-000000000704', :'tenant_id'::uuid, 'vehicle', '00000000-0000-0000-0000-000000000503', '00000000-0000-0000-0000-000000000503', '00000000-0000-0000-0000-000000000104', '00000000-0000-0000-0000-000000000404', '00000000-0000-0000-0000-000000000404', NOW() - INTERVAL '3 hours', NOW() + INTERVAL '4 hours', 'in_use', 'Materialfahrt zur Verkaufstheke', 'Material- und Werkzeugfahrt fuer Hofladen Braun.', NOW() - INTERVAL '2 days', NOW() - INTERVAL '5 hours'),
    ('00000000-0000-0000-0000-000000000705', :'tenant_id'::uuid, 'tool', '00000000-0000-0000-0000-000000000606', '00000000-0000-0000-0000-000000000606', '00000000-0000-0000-0000-000000000103', '00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000401', date_trunc('day', NOW()) + INTERVAL '1 day 07:00', date_trunc('day', NOW()) + INTERVAL '1 day 16:00', 'pending', 'Sockelausschnitte nacharbeiten', 'Nacharbeit an Sockelausschnitten und Blendrahmen.', NOW() - INTERVAL '3 hours', NOW() - INTERVAL '2 hours'),
    ('00000000-0000-0000-0000-000000000706', :'tenant_id'::uuid, 'tool', '00000000-0000-0000-0000-000000000605', '00000000-0000-0000-0000-000000000605', '00000000-0000-0000-0000-000000000103', '00000000-0000-0000-0000-000000000403', '00000000-0000-0000-0000-000000000403', date_trunc('day', NOW()) - INTERVAL '20 days' + INTERVAL '07:00', date_trunc('day', NOW()) - INTERVAL '20 days' + INTERVAL '15:30', 'completed', 'Endmontage absaugen', 'Absaugung fuer Endmontage im Kindergarten.', NOW() - INTERVAL '22 days', NOW() - INTERVAL '20 days'),
    ('00000000-0000-0000-0000-000000000707', :'tenant_id'::uuid, 'tool', '00000000-0000-0000-0000-000000000601', '00000000-0000-0000-0000-000000000601', '00000000-0000-0000-0000-000000000102', NULL, NULL, NOW() - INTERVAL '12 days', NOW() - INTERVAL '11 days', 'cancelled', 'Keine Zuordnung', 'Auftrag kurzfristig verschoben.', NOW() - INTERVAL '14 days', NOW() - INTERVAL '12 days');

INSERT INTO site_appointments (
    id, tenant_id, site_id, title, appointment_kind, starts_at, ends_at,
    notes, assigned_user_ids, created_at, updated_at
)
VALUES
    ('00000000-0000-0000-0000-000000001201', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', 'Abnahme mit Familie Maier', 'customer_appointment', date_trunc('week', NOW()) + INTERVAL '2 days 10 hours', date_trunc('week', NOW()) + INTERVAL '2 days 11 hours 30 minutes', 'Restpunkte und Pflegehinweise besprechen.', ARRAY['00000000-0000-0000-0000-000000000102'::uuid], NOW() - INTERVAL '4 days', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000001202', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', 'Montage Restarbeiten', 'worker_deployment', date_trunc('week', NOW()) + INTERVAL '3 days 7 hours', date_trunc('week', NOW()) + INTERVAL '3 days 15 hours', 'Sockel, Passleisten und Endreinigung.', ARRAY['00000000-0000-0000-0000-000000000102'::uuid, '00000000-0000-0000-0000-000000000103'::uuid], NOW() - INTERVAL '3 days', NOW() - INTERVAL '2 hours'),
    ('00000000-0000-0000-0000-000000001203', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000401', 'Projektabschluss', 'milestone', date_trunc('week', NOW()) + INTERVAL '4 days 16 hours', date_trunc('week', NOW()) + INTERVAL '4 days 17 hours', NULL, '{}', NOW() - INTERVAL '2 days', NOW() - INTERVAL '2 hours');

INSERT INTO maintenance_schedules (
    id, tenant_id, asset_id, task_description, interval_days, is_active, created_at, updated_at
)
VALUES
    ('00000000-0000-0000-0000-000000001301', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000603', 'Saegeblatt, Anschlag und Schutzhaube pruefen', 90, TRUE, NOW() - INTERVAL '180 days', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000001302', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000501', 'Fahrzeugcheck und Ladungssicherung', 30, TRUE, NOW() - INTERVAL '90 days', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000001303', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000605', 'Filter und Absaugleistung kontrollieren', 60, TRUE, NOW() - INTERVAL '60 days', NOW() - INTERVAL '3 days');

INSERT INTO maintenance_due (
    id, tenant_id, schedule_id, asset_id, due_date, status, resolved_at,
    resolved_by, resolution_notes, created_at, updated_at
)
VALUES
    ('00000000-0000-0000-0000-000000001311', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000001301', '00000000-0000-0000-0000-000000000603', CURRENT_DATE - 5, 'open', NULL, NULL, NULL, NOW() - INTERVAL '95 days', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000001312', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000001302', '00000000-0000-0000-0000-000000000501', CURRENT_DATE, 'open', NULL, NULL, NULL, NOW() - INTERVAL '30 days', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000001313', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000001303', '00000000-0000-0000-0000-000000000605', CURRENT_DATE + 12, 'open', NULL, NULL, NULL, NOW() - INTERVAL '5 days', NOW() - INTERVAL '3 days');

INSERT INTO stock_entries (id, tenant_id, material_id, user_id, quantity_change, quantity_after, notes, site_id, entry_type, created_at)
VALUES
    ('00000000-0000-0000-0000-000000000901', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000301', '00000000-0000-0000-0000-000000000101', 6, 18, 'Lieferung fuer laufende Innenausbauauftraege', NULL, 'material_added', NOW() - INTERVAL '10 days'),
    ('00000000-0000-0000-0000-000000000902', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000301', '00000000-0000-0000-0000-000000000104', -2, 16, 'Wangen und Fachboeden fuer Hofladen Braun', '00000000-0000-0000-0000-000000000404', 'withdrawn', NOW() - INTERVAL '4 days'),
    ('00000000-0000-0000-0000-000000000903', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000301', '00000000-0000-0000-0000-000000000102', -2, 14, 'Passleisten fuer Einbauschrank Maier', '00000000-0000-0000-0000-000000000401', 'withdrawn', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000000904', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000305', '00000000-0000-0000-0000-000000000101', 60, 210, 'Nachbestellung Eichenkante eingetroffen', NULL, 'material_added', NOW() - INTERVAL '9 days'),
    ('00000000-0000-0000-0000-000000000905', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000305', '00000000-0000-0000-0000-000000000102', -30, 180, 'Fronten und Sichtseiten fuer Einbauschrank Maier', '00000000-0000-0000-0000-000000000401', 'withdrawn', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000906', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000307', '00000000-0000-0000-0000-000000000101', 1000, 4500, 'Lagerauffuellung Verbindungsmittel', NULL, 'material_added', NOW() - INTERVAL '8 days'),
    ('00000000-0000-0000-0000-000000000907', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000307', '00000000-0000-0000-0000-000000000104', -300, 4200, 'Montage Verkaufstheke und Unterkonstruktion', '00000000-0000-0000-0000-000000000404', 'withdrawn', NOW() - INTERVAL '4 days'),
    ('00000000-0000-0000-0000-000000000908', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000308', '00000000-0000-0000-0000-000000000101', 250, 1050, 'Confirmat-Schrauben aus Grosspackung umgebucht', NULL, 'material_added', NOW() - INTERVAL '7 days'),
    ('00000000-0000-0000-0000-000000000909', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000308', '00000000-0000-0000-0000-000000000102', -100, 950, 'Korpusverbindungen fuer Dachgeschossschrank', '00000000-0000-0000-0000-000000000401', 'withdrawn', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000910', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000309', '00000000-0000-0000-0000-000000000101', 40, 200, 'Blum-Scharniere geliefert', NULL, 'material_added', NOW() - INTERVAL '6 days'),
    ('00000000-0000-0000-0000-000000000911', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000309', '00000000-0000-0000-0000-000000000102', -20, 180, 'Tueren fuer Einbauschrank Maier', '00000000-0000-0000-0000-000000000401', 'withdrawn', NOW() - INTERVAL '1 day'),
    ('00000000-0000-0000-0000-000000000912', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000310', '00000000-0000-0000-0000-000000000101', 8, 44, 'Auszugssaetze fuer Hofladenprojekt erhalten', NULL, 'material_added', NOW() - INTERVAL '6 days'),
    ('00000000-0000-0000-0000-000000000913', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000310', '00000000-0000-0000-0000-000000000104', -8, 36, 'Kassenauszug und Schubkaesten Hofladen Braun', '00000000-0000-0000-0000-000000000404', 'withdrawn', NOW() - INTERVAL '4 days'),
    ('00000000-0000-0000-0000-000000000914', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000311', '00000000-0000-0000-0000-000000000101', 12, 30, 'Leim fuer Montagewagen aufgefuellt', NULL, 'material_added', NOW() - INTERVAL '5 days'),
    ('00000000-0000-0000-0000-000000000915', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000311', '00000000-0000-0000-0000-000000000102', -6, 24, 'Verleimung Sitznische und Sockelleisten', '00000000-0000-0000-0000-000000000401', 'withdrawn', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000916', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000312', '00000000-0000-0000-0000-000000000101', 4, 15, 'Nachbestellung Hartwachsoel eingelagert', NULL, 'material_added', NOW() - INTERVAL '14 days'),
    ('00000000-0000-0000-0000-000000000917', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000312', '00000000-0000-0000-0000-000000000103', -4, 11, 'Treppenstufen Familie Riek geoehlt', '00000000-0000-0000-0000-000000000405', 'withdrawn', NOW() - INTERVAL '12 days'),
    ('00000000-0000-0000-0000-000000000918', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000313', '00000000-0000-0000-0000-000000000101', 6, 20, 'Weisslack fuer Kindergartenfronten geliefert', NULL, 'material_added', NOW() - INTERVAL '22 days'),
    ('00000000-0000-0000-0000-000000000919', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000313', '00000000-0000-0000-0000-000000000103', -2, 18, 'Lackierung Garderobenfronten abgeschlossen', '00000000-0000-0000-0000-000000000403', 'withdrawn', NOW() - INTERVAL '20 days'),
    ('00000000-0000-0000-0000-000000000920', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000303', '00000000-0000-0000-0000-000000000101', 0, 22, 'Von Plattenlager C nach Plattenlager B umgeraeumt', NULL, 'location_changed', NOW() - INTERVAL '2 days'),
    ('00000000-0000-0000-0000-000000000921', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000304', '00000000-0000-0000-0000-000000000101', 0, 18, 'Mindestbestand wegen Praxisausbau auf sechs Quadratmeter gesetzt', NULL, 'min_quantity_changed', NOW() - INTERVAL '3 days');

INSERT INTO material_batches (id, tenant_id, material_id, expires_on, initial_quantity, remaining_quantity, batch_code, created_at)
VALUES
    ('00000000-0000-0000-0000-000000000951', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000311', CURRENT_DATE - 2, 10, 10, 'PUR-ALT-01', NOW() - INTERVAL '21 days'),
    ('00000000-0000-0000-0000-000000000952', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000311', CURRENT_DATE + 5, 8, 8, 'PUR-2026-08-A', NOW() - INTERVAL '12 days'),
    ('00000000-0000-0000-0000-000000000953', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000311', CURRENT_DATE + 45, 6, 6, 'PUR-2026-09-B', NOW() - INTERVAL '5 days'),
    ('00000000-0000-0000-0000-000000000954', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000312', CURRENT_DATE + 8, 11, 11, 'OEL-2026-08-01', NOW() - INTERVAL '14 days'),
    ('00000000-0000-0000-0000-000000000955', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000313', CURRENT_DATE - 1, 6, 6, 'LACK-ALT-B', NOW() - INTERVAL '24 days'),
    ('00000000-0000-0000-0000-000000000956', :'tenant_id'::uuid, '00000000-0000-0000-0000-000000000313', CURRENT_DATE + 18, 12, 12, 'LACK-2026-08-C', NOW() - INTERVAL '10 days');

COMMIT;
