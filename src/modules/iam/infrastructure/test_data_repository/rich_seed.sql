UPDATE categories SET description = 'Platten für Korpusbau, Fronten und passgenaue Einbauten.'
WHERE tenant_id = '{{tenant_id}}' AND id = uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-plates');
UPDATE categories SET description = 'Scharniere, Auszüge und Verbindungstechnik für Möbelmontagen.'
WHERE tenant_id = '{{tenant_id}}' AND id = uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-hardware');
UPDATE categories SET description = 'Leime, Öle und Verbrauchsmaterial mit Chargenüberwachung.'
WHERE tenant_id = '{{tenant_id}}' AND id = uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-consumables');

INSERT INTO categories (id, tenant_id, name, description, can_expire)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-timber'), '{{tenant_id}}', 'Massivholz', 'Leimholz und massive Werkstoffe für hochwertige Innenausbauten.', FALSE),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-edges'), '{{tenant_id}}', 'Kanten', 'ABS- und Echtholzkanten für sichtbare Werkstückseiten.', FALSE),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-finishes'), '{{tenant_id}}', 'Oberflächen', 'Öle und Lacke für widerstandsfähige Holzoberflächen.', TRUE)
ON CONFLICT (id) DO NOTHING;

UPDATE materials SET
  name = 'Birke Multiplex 18 mm',
  description = '2500 x 1250 mm, für Korpusse, Nischen und Sonderteile.',
  quantity = 12, legacy_quantity = 12, min_quantity = 5,
  location = 'Plattenlager A'
WHERE tenant_id = '{{tenant_id}}' AND id = uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-multiplex');
UPDATE materials SET
  name = 'Topfscharnier 110° mit Dämpfung',
  description = 'Clip-Scharnier für Küchenfronten und Einbauschränke.',
  quantity = 12, legacy_quantity = 12, min_quantity = 20,
  location = 'Beschlagschrank 1'
WHERE tenant_id = '{{tenant_id}}' AND id = uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-hinge');
UPDATE materials SET
  name = 'D4-Leim 500 g',
  description = 'Wasserfester Montageleim, die älteste Charge muss ausgesondert werden.',
  quantity = 8, legacy_quantity = 0, min_quantity = 4,
  location = 'Chemieschrank'
WHERE tenant_id = '{{tenant_id}}' AND id = uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-glue');
UPDATE material_batches SET expires_on = CURRENT_DATE - 3, initial_quantity = 8, remaining_quantity = 8,
  batch_code = 'D4-ALT-' || LEFT('{{tenant_id}}', 8)
WHERE tenant_id = '{{tenant_id}}' AND id = uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-glue-batch');

INSERT INTO materials
  (id, tenant_id, category_id, name, description, unit, quantity, legacy_quantity, min_quantity, location, qr_code, base_price_cents, price_markup_percentage)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-oak'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-timber'), 'Eiche Leimholz 26 mm', 'Durchgehende Lamelle für Arbeitsplatten und Sitzbänke.', 'Quadratmeter', 24, 24, 8, 'Holzlager rechts', 'TEST-EICHE-{{tenant_id}}', 12800, 30),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-mdf'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-plates'), 'MDF weiß 19 mm', 'Lackfähige Platte für Fronten und Sockel.', 'Stück', 18, 18, 6, 'Plattenlager B', 'TEST-MDF-{{tenant_id}}', 4250, 25),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-edge-oak'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-edges'), 'ABS-Kante Eiche 23 x 2 mm', 'Passend zu Eiche-Dekoren und furnierten Fronten.', 'Meter', 145, 145, 50, 'Kantenregal 1', 'TEST-KANTE-EICHE-{{tenant_id}}', 145, 35),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-screws'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-hardware'), 'Spanplattenschraube 4 x 50 TX20', 'Universalschraube für Korpus und Unterkonstruktion.', 'Stück', 1850, 1850, 800, 'Kleinteilewand A4', 'TEST-SCHRAUBE-{{tenant_id}}', 8, 40),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-drawers'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-hardware'), 'Vollauszug 500 mm links/rechts', 'Auszugssatz für breite Küchenschubladen.', 'Stück', 18, 18, 8, 'Beschlagschrank 3', 'TEST-AUSZUG-{{tenant_id}}', 6850, 25),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-oil'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-finishes'), 'Hartwachsöl farblos', 'Für strapazierfähige Eichenoberflächen im Innenausbau.', 'Liter', 7, 0, 3, 'Lacklager links', 'TEST-OEL-{{tenant_id}}', 2490, 30),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-silicone'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-category-consumables'), 'Montagesilikon transparent', 'Elastische Anschlussfugen an Arbeitsplatten und Nischen.', 'Stück', 14, 0, 6, 'Chemieschrank', 'TEST-SILIKON-{{tenant_id}}', 890, 30)
ON CONFLICT (id) DO NOTHING;

INSERT INTO material_batches (id, tenant_id, material_id, expires_on, initial_quantity, remaining_quantity, batch_code)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-oil-batch'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-oil'), CURRENT_DATE + 6, 7, 7, 'OEL-KURZ-' || LEFT('{{tenant_id}}', 8)),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-silicone-batch'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-silicone'), CURRENT_DATE + 120, 14, 14, 'SIL-' || LEFT('{{tenant_id}}', 8))
ON CONFLICT (id) DO NOTHING;

UPDATE sites SET
  name = 'Küche Familie Winter', customer_name = 'Familie Winter',
  location = 'Ahornweg 8, Eichenried',
  description = 'Grifflose Küche in Eiche und Mattweiß mit Sitzfenster und deckenhohen Hochschränken.',
  status = 'active', start_date = CURRENT_DATE - 12, end_date = CURRENT_DATE + 4,
  estimated_days = 9, budget_amount_cents = 2840000,
  billing_reference = 'WINTER-KUECHE-26', quote_reference = 'ANG-26-041'
WHERE tenant_id = '{{tenant_id}}' AND id = uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active');
UPDATE sites SET
  name = 'Empfang Praxis Lindenhof', customer_name = 'Praxis Lindenhof',
  location = 'Lindenplatz 3, Sonnenfeld',
  description = 'Empfangstheke aus Eiche mit HPL-Arbeitsfläche, Stauraum und barrierefreiem Beratungsplatz.',
  status = 'planned', start_date = CURRENT_DATE + 8, end_date = CURRENT_DATE + 13,
  estimated_days = 5, budget_amount_cents = 1260000,
  billing_reference = 'LINDENHOF-EMPFANG', quote_reference = 'ANG-26-052'
WHERE tenant_id = '{{tenant_id}}' AND id = uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned');

INSERT INTO sites
  (id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, budget_amount_cents, billing_reference, billing_notes, quote_reference, invoice_pricing_mode, hourly_rate_cents, fixed_price_cents)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), '{{tenant_id}}', 'external_site', 'Bibliothek Villa Falkenried', 'Familie Falk', 'Falkenstraße 17, Falkenau', 'Raumhohe Bibliothek mit Leiter, integriertem Schreibtisch und verdeckter Beleuchtung.', 'active', CURRENT_DATE - 7, CURRENT_DATE + 8, 10, 1980000, 'FALK-BIB-26', 'Abschlag nach Korpusmontage.', 'ANG-26-047', 'hourly_rate', 8900, NULL),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-kindergarten'), '{{tenant_id}}', 'external_site', 'Garderobe Kita Sonnenbogen', 'Kita Sonnenbogen', 'Wiesenring 5, Moosgrund', 'Kindgerechte Garderoben mit Sitzbank, Schuhrost und farbigen Eigentumsfächern.', 'completed', CURRENT_DATE - 35, CURRENT_DATE - 21, 8, 1420000, 'SONNENBOGEN-26', 'Abnahme ohne Restpunkte.', 'ANG-26-018', 'fixed_price', NULL, 1420000),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-farmshop'), '{{tenant_id}}', 'external_site', 'Verkaufstheke Hofladen Morgenrot', 'Hofladen Morgenrot', 'Feldrain 2, Morgenried', 'Massive Verkaufstheke aus Eiche mit Kassenauszug, Kühlvitrine und Präsentationsregalen.', 'archived', CURRENT_DATE - 90, CURRENT_DATE - 72, 11, 2210000, 'MORGENROT-25', 'Schlussrechnung bezahlt.', 'ANG-25-188', 'fixed_price', NULL, 2210000),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-showroom'), '{{tenant_id}}', 'internal_workshop', 'Ausstellungsküche Werkstatt', 'Interner Auftrag', 'Werkstatt', 'Neue Musterküche für Beratungstermine mit wechselbaren Front- und Griffmustern.', 'active', CURRENT_DATE - 4, CURRENT_DATE + 18, 12, 850000, 'INTERN-AUSSTELLUNG', NULL, NULL, NULL, NULL, NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO users (id, tenant_id, keycloak_user_id, email, name, role)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), '{{tenant_id}}', 'test-data-lena-{{tenant_id}}', 'lena.hartmann@example.invalid', 'Lena Hartmann', 'employee'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-moritz'), '{{tenant_id}}', 'test-data-moritz-{{tenant_id}}', 'moritz-keller@example.invalid', 'Moritz Keller', 'employee'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), '{{tenant_id}}', 'test-data-aylin-{{tenant_id}}', 'aylin-demir@example.invalid', 'Aylin Demir', 'employee'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix'), '{{tenant_id}}', 'test-data-felix-{{tenant_id}}', 'felix-baumann@example.invalid', 'Felix Baumann', 'employee'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), '{{tenant_id}}', 'test-data-mara-{{tenant_id}}', 'mara-vogt@example.invalid', 'Mara Vogt', 'employee')
ON CONFLICT (id) DO NOTHING;

INSERT INTO site_assignments (id, tenant_id, site_id, user_id, role)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-assignment-winter-lena'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), 'lead'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-assignment-winter-moritz'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-moritz'), 'worker'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-assignment-library-aylin'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), 'lead'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-assignment-library-felix'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix'), 'worker'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-assignment-practice-mara'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), 'lead'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-assignment-kita-moritz'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-kindergarten'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-moritz'), 'lead'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-assignment-farmshop-felix'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-farmshop'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix'), 'lead'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-assignment-showroom-aylin'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-showroom'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), 'lead'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-assignment-showroom-mara'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-showroom'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), 'worker')
ON CONFLICT (id) DO NOTHING;

UPDATE assets SET name = 'Montagebus Nord', description = 'Großer Montagebus mit Regalsystem und Langgutträger.', location = 'Hof 1', status = 'available'
WHERE tenant_id = '{{tenant_id}}' AND id = uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle');
UPDATE assets SET name = 'Festool TS 55 Tauchsäge', description = 'Tauchsäge für passgenaue Zuschnitte auf der Baustelle.', location = 'Werkzeugausgabe', status = 'in_use'
WHERE tenant_id = '{{tenant_id}}' AND id = uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool');

INSERT INTO assets (id, tenant_id, asset_kind, name, description, status, location, qr_code)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-south'), '{{tenant_id}}', 'vehicle', 'Montagebus Süd', 'Montagefahrzeug für Zweierkolonne mit Dachträger.', 'reserved', 'Hof 1', 'TEST-FZG-SUED-{{tenant_id}}'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-pickup'), '{{tenant_id}}', 'vehicle', 'Zugfahrzeug Waldgrün', 'Allradfahrzeug für Anhänger und Materialfahrten.', 'in_use', 'Baustelle Villa Falkenried', 'TEST-FZG-ZUG-{{tenant_id}}'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-trailer'), '{{tenant_id}}', 'vehicle', 'Kofferanhänger Langgut', 'Geschlossener Anhänger für Bauteile und Werkzeug.', 'maintenance', 'Werkstatt hinten', 'TEST-FZG-ANH-{{tenant_id}}'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-lamello'), '{{tenant_id}}', 'tool', 'Lamello Zeta P2', 'Verbinderfräse für Korpusmontage und Verkleidungen.', 'in_use', 'Küche Familie Winter', 'TEST-WZG-LAMELLO-{{tenant_id}}'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-saw'), '{{tenant_id}}', 'tool', 'Mafell Erika 85', 'Mobile Unterflurzugkreissäge für den Innenausbau.', 'maintenance', 'Serviceecke Werkstatt', 'TEST-WZG-ERIKA-{{tenant_id}}'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-drill'), '{{tenant_id}}', 'tool', 'Makita DHR243 Bohrhammer', 'Akkubohrhammer für Beton und Mauerwerk.', 'available', 'Montagewagen 2', 'TEST-WZG-BOHR-{{tenant_id}}'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-vacuum'), '{{tenant_id}}', 'tool', 'Festool CTL MIDI Absaugmobil', 'Mobiles Absauggerät für Montage und Zuschnitt.', 'available', 'Werkzeugausgabe', 'TEST-WZG-ABSAUG-{{tenant_id}}'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-multitool'), '{{tenant_id}}', 'tool', 'Fein MultiMaster', 'Multifunktionswerkzeug für Ausschnitte und Nacharbeiten.', 'available', 'Montagewagen 1', 'TEST-WZG-MULTI-{{tenant_id}}'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-router'), '{{tenant_id}}', 'tool', 'Festool OF 1400 Oberfräse', 'Oberfräse für Kanten, Nuten und Beschlagarbeiten.', 'reserved', 'Werkzeugausgabe', 'TEST-WZG-FRAESE-{{tenant_id}}'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-laser'), '{{tenant_id}}', 'tool', 'Leica Kreuzlinienlaser', 'Selbstnivellierender Linienlaser für Montageachsen.', 'available', 'Messmittelschrank', 'TEST-WZG-LASER-{{tenant_id}}')
ON CONFLICT (id) DO NOTHING;

INSERT INTO vehicle_details (asset_id, tenant_id, license_plate, vehicle_type)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-south'), '{{tenant_id}}', 'SF-WK 218', 'van'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-pickup'), '{{tenant_id}}', 'SF-WK 407', 'truck'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-trailer'), '{{tenant_id}}', 'SF-WK 912', 'trailer')
ON CONFLICT (asset_id) DO NOTHING;
INSERT INTO vehicle_display_colors (asset_id, tenant_id, display_color)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-south'), '{{tenant_id}}', '#16a34a'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-pickup'), '{{tenant_id}}', '#ea580c'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-trailer'), '{{tenant_id}}', '#7c3aed')
ON CONFLICT (asset_id) DO NOTHING;
INSERT INTO tool_details (asset_id, tenant_id, category)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-lamello'), '{{tenant_id}}', 'Verbindungstechnik'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-saw'), '{{tenant_id}}', 'Sägetechnik'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-drill'), '{{tenant_id}}', 'Bohrtechnik'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-vacuum'), '{{tenant_id}}', 'Absaugung'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-multitool'), '{{tenant_id}}', 'Nacharbeit'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-router'), '{{tenant_id}}', 'Frästechnik'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-laser'), '{{tenant_id}}', 'Messtechnik')
ON CONFLICT (asset_id) DO NOTHING;

INSERT INTO time_entries (id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-01'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), 'site', 8.0, CURRENT_DATE - 6, 'Hochschränke gestellt und ausgerichtet.', NOW() - INTERVAL '6 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-02'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-moritz'), 'site', 7.5, CURRENT_DATE - 6, 'Unterschränke montiert und Sockelhöhe geprüft.', NOW() - INTERVAL '6 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-03'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-showroom'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), 'workshop', 6.5, CURRENT_DATE - 5, 'Korpusse zugeschnitten und Kanten angefahren.', NOW() - INTERVAL '5 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-04'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), 'site', 8.25, CURRENT_DATE - 4, 'Sockel und erste Regalachse eingepasst.', NOW() - INTERVAL '4 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-05'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix'), 'travel', 1.25, CURRENT_DATE - 4, 'Materialfahrt nach Falkenau.', NOW() - INTERVAL '4 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-06'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned'), '{{admin_id}}', 'other', 2.0, CURRENT_DATE - 3, 'Baubesprechung und Freigabe der Elektroplanung.', NOW() - INTERVAL '3 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-07'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), 'site', 3.0, CURRENT_DATE - 3, 'Aufmaß und Leitungsführung vor Ort geprüft.', NOW() - INTERVAL '3 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-08'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-showroom'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), 'workshop', 7.0, CURRENT_DATE - 2, 'Beschläge kommissioniert und Auszüge vormontiert.', NOW() - INTERVAL '2 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-09'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), 'site', 8.5, CURRENT_DATE - 1, 'Arbeitsplatte angepasst und Nischenmaß kontrolliert.', NOW() - INTERVAL '1 day'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-10'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-moritz'), 'travel', 1.0, CURRENT_DATE - 1, 'Restmaterial zurück in die Werkstatt gebracht.', NOW() - INTERVAL '1 day'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-11'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-kindergarten'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-moritz'), 'site', 7.75, CURRENT_DATE - 24, 'Garderoben montiert und Hakenleisten gesetzt.', NOW() - INTERVAL '24 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-12'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-kindergarten'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), 'workshop', 5.5, CURRENT_DATE - 25, 'Fronten lackfertig geschliffen.', NOW() - INTERVAL '25 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-13'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-farmshop'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix'), 'site', 8.0, CURRENT_DATE - 78, 'Thekenanlage ausgerichtet und verschraubt.', NOW() - INTERVAL '78 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-14'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-farmshop'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), 'site', 7.0, CURRENT_DATE - 77, 'Schubladen und Kassenauszug eingestellt.', NOW() - INTERVAL '77 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-15'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), 'workshop', 7.5, CURRENT_DATE - 7, 'Regalseiten gebohrt und LED-Nuten gefräst.', NOW() - INTERVAL '7 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-16'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix'), 'workshop', 6.75, CURRENT_DATE - 6, 'Leiterführung und Sockelblenden vorbereitet.', NOW() - INTERVAL '6 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-17'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), 'workshop', 4.5, CURRENT_DATE - 8, 'Fronten geprüft und nachsortiert.', NOW() - INTERVAL '8 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-18'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-showroom'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), 'workshop', 5.0, CURRENT_DATE - 3, 'Musterfronten beschriftet und eingehängt.', NOW() - INTERVAL '3 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-19'), '{{tenant_id}}', NULL, '{{admin_id}}', 'other', 1.5, CURRENT_DATE - 2, 'Wochenplanung und Kapazitätsabgleich.', NOW() - INTERVAL '2 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-time-20'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), 'site', 4.0, CURRENT_DATE, 'Passleisten montiert und Restpunkte aufgenommen.', NOW() - INTERVAL '2 hours')
ON CONFLICT (id) DO NOTHING;

INSERT INTO site_activities (id, tenant_id, site_id, user_id, activity_type, content, photo_url, created_at)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-winter-start'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), '{{admin_id}}', 'status_change', '{"old_status":"planned","new_status":"active"}', NULL, NOW() - INTERVAL '11 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-winter-measure'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), 'note', 'Aufmaß bestätigt: Wandbreite 3,84 m, Raumhöhe 2,47 m. Wasseranschluss 62 cm ab linker Wand, Brüstungshöhe 91 cm.', NULL, NOW() - INTERVAL '10 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-winter-progress'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-moritz'), 'note', 'Hochschrankzeile steht. Frontfugen umlaufend auf 3 mm eingestellt, Arbeitsplatte kommt morgen.', NULL, NOW() - INTERVAL '2 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-library-start'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), '{{admin_id}}', 'status_change', '{"old_status":"planned","new_status":"active"}', NULL, NOW() - INTERVAL '7 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-library-note'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), 'note', 'Erste Regalachse lotrecht montiert. LED-Zuleitung liegt hinter der linken Blende, Leiterführung läuft frei.', NULL, NOW() - INTERVAL '1 day'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-practice-measure'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), 'note', 'Empfangsbereich ausgemessen. Barrierefreier Abschnitt 74 cm hoch, Kabeldurchlass mit Elektriker abgestimmt.', NULL, NOW() - INTERVAL '3 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-kita-complete'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-kindergarten'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-moritz'), 'status_change', '{"old_status":"active","new_status":"completed"}', NULL, NOW() - INTERVAL '21 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-kita-note'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-kindergarten'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), 'note', 'Abnahme ohne Restpunkte. Zwei Ersatzhaken und Pflegeanleitung im Hausmeisterraum hinterlegt.', NULL, NOW() - INTERVAL '21 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-showroom-progress'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-showroom'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), 'note', 'Korpusse in der Vormontage. Auszüge laufen sauber, Musterfronten werden morgen eingehängt.', NULL, NOW() - INTERVAL '8 hours'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-farmshop-complete'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-farmshop'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix'), 'status_change', '{"old_status":"active","new_status":"completed"}', NULL, NOW() - INTERVAL '72 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-farmshop-note'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-farmshop'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix'), 'note', 'Kassenauszug und Kühlvitrinenanschluss geprüft. Oberfläche nachgeölt und Pflegehinweise übergeben.', NULL, NOW() - INTERVAL '72 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-practice-visual'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), 'note', 'Visualisierung nach dem Aufmaß: Montagehöhe 1,12 m, abgesenkter Beratungsplatz und Kabeldurchlass berücksichtigt.', NULL, NOW() - INTERVAL '2 days')
ON CONFLICT (id) DO NOTHING;

INSERT INTO site_activity_attachments
  (id, tenant_id, activity_id, site_id, storage_key, thumbnail_key, mime_type, size_bytes, original_bytes, thumbnail_bytes, original_filename)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-attachment-kitchen'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-winter-measure'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), 'kitchen-measurement.png', 'kitchen-measurement-thumb.png', 'image/png', octet_length(decode('{{kitchen_image}}', 'hex')), decode('{{kitchen_image}}', 'hex'), decode('{{kitchen_thumb}}', 'hex'), 'aufmass-kueche-winter.png'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-attachment-workshop'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-showroom-progress'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-showroom'), 'workshop-progress.png', 'workshop-progress-thumb.png', 'image/png', octet_length(decode('{{workshop_image}}', 'hex')), decode('{{workshop_image}}', 'hex'), decode('{{workshop_thumb}}', 'hex'), 'vormontage-ausstellungskueche.png'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-attachment-reception'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-activity-practice-visual'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned'), 'reception-installation.png', 'reception-installation-thumb.png', 'image/png', octet_length(decode('{{reception_image}}', 'hex')), decode('{{reception_image}}', 'hex'), decode('{{reception_thumb}}', 'hex'), 'planung-empfang-lindenhof.png')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stock_entries (id, tenant_id, material_id, user_id, quantity_change, quantity_after, notes, site_id, entry_type, created_at)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-multiplex-in'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-multiplex'), '{{admin_id}}', 6, 18, 'Lieferung für laufende Innenausbauaufträge', NULL, 'material_added', NOW() - INTERVAL '10 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-multiplex-out'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-multiplex'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), -6, 12, 'Korpusse und Passleisten Küche Winter', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), 'withdrawn', NOW() - INTERVAL '6 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-hinge-in'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-hinge'), '{{admin_id}}', 40, 52, 'Scharnierlieferung eingelagert', NULL, 'material_added', NOW() - INTERVAL '14 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-hinge-out'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-hinge'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-moritz'), -40, 12, 'Fronten Küche Winter und Ausstellungsküche', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), 'withdrawn', NOW() - INTERVAL '2 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-oak-out'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-oak'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), -6, 24, 'Sockel und Leiterführung Villa Falkenried', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), 'withdrawn', NOW() - INTERVAL '5 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-mdf-out'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-mdf'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), -4, 18, 'Musterfronten Ausstellungsküche', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-showroom'), 'withdrawn', NOW() - INTERVAL '4 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-edge-out'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-edge-oak'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), -35, 145, 'Sichtkanten Küche Winter', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), 'withdrawn', NOW() - INTERVAL '3 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-screws-out'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-screws'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix'), -350, 1850, 'Regalmontage Villa Falkenried', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), 'withdrawn', NOW() - INTERVAL '4 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-drawers-out'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-drawers'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-moritz'), -8, 18, 'Auszüge Küche Winter', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), 'withdrawn', NOW() - INTERVAL '2 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-oil-out'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-oil'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix'), -2, 7, 'Nachpflege Verkaufstheke Morgenrot', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-farmshop'), 'withdrawn', NOW() - INTERVAL '72 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-silicone-out'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-silicone'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), -3, 14, 'Anschlussfugen Küche Winter', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), 'withdrawn', NOW() - INTERVAL '1 day'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-stock-mdf-location'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-mdf'), '{{admin_id}}', 0, 18, 'Von Plattenlager C nach Plattenlager B umgeräumt', NULL, 'location_changed', NOW() - INTERVAL '1 day')
ON CONFLICT (id) DO NOTHING;

INSERT INTO order_requests (id, tenant_id, material_id, quantity, requested_by, status, reason, notes)
VALUES (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-order-hinges'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-material-hinge'), 80, '{{admin_id}}', 'pending', 'Mindestbestand nach Entnahme unterschritten', 'Für Praxis Lindenhof und nächste Küchenmontage nachbestellen.')
ON CONFLICT (id) DO NOTHING;

INSERT INTO reservations
  (id, tenant_id, resource_type, resource_id, asset_id, user_id, site_id, project_id, start_time, end_time, status, purpose, notes)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-reservation-lamello'), '{{tenant_id}}', 'tool', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-lamello'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-lamello'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), NOW() - INTERVAL '2 hours', NOW() + INTERVAL '5 hours', 'in_use', 'Passleisten und Sockel montieren', 'Verbinderfräse für die Abschlussarbeiten.'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-reservation-bus-south'), '{{tenant_id}}', 'vehicle', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-south'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-south'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned'), date_trunc('day', NOW()) + INTERVAL '8 days 06:30', date_trunc('day', NOW()) + INTERVAL '9 days 17:00', 'confirmed', 'Praxiseinbau und Materialtransport', 'HPL-Platten und Empfangskorpusse transportieren.'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-reservation-drill'), '{{tenant_id}}', 'tool', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-drill'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-drill'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned'), date_trunc('day', NOW()) + INTERVAL '8 days 07:00', date_trunc('day', NOW()) + INTERVAL '8 days 16:00', 'confirmed', 'Empfangsunterkonstruktion befestigen', NULL),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-reservation-pickup'), '{{tenant_id}}', 'vehicle', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-pickup'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-pickup'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), NOW() - INTERVAL '3 hours', NOW() + INTERVAL '4 hours', 'in_use', 'Materialfahrt Villa Falkenried', 'Leiterführung und Regalböden liefern.'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-reservation-router'), '{{tenant_id}}', 'tool', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-router'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-router'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), date_trunc('day', NOW()) + INTERVAL '1 day 07:00', date_trunc('day', NOW()) + INTERVAL '1 day 16:00', 'pending', 'Nuten für LED-Profile nacharbeiten', NULL),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-reservation-vacuum'), '{{tenant_id}}', 'tool', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-vacuum'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-vacuum'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-moritz'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-kindergarten'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-kindergarten'), NOW() - INTERVAL '25 days', NOW() - INTERVAL '24 days', 'completed', 'Endmontage Garderoben', 'Absaugung bei Anpassarbeiten.'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-reservation-laser'), '{{tenant_id}}', 'tool', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-laser'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-laser'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-farmshop'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-farmshop'), NOW() - INTERVAL '80 days', NOW() - INTERVAL '79 days', 'completed', 'Montageachsen Verkaufstheke', NULL),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-reservation-multitool'), '{{tenant_id}}', 'tool', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-multitool'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-multitool'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena'), NULL, NULL, NOW() - INTERVAL '12 days', NOW() - INTERVAL '11 days', 'cancelled', 'Montagetermin verschoben', NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO maintenance_schedules (id, tenant_id, asset_id, task_description, interval_days, is_active)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-saw'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-saw'), 'Sägeblatt, Anschlag und Schutzhaube prüfen', 90, TRUE),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-bus'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle'), 'Fahrzeugcheck und Ladungssicherung', 30, TRUE),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-vacuum'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-vacuum'), 'Filter und Absaugleistung kontrollieren', 60, TRUE),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-trailer'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-trailer'), 'Beleuchtung, Reifen und Auflaufbremse prüfen', 180, TRUE),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-router'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-router'), 'Spannzange reinigen und Rundlauf prüfen', 120, TRUE)
ON CONFLICT (id) DO NOTHING;
INSERT INTO maintenance_due
  (id, tenant_id, schedule_id, asset_id, due_date, status, resolved_at, resolved_by, resolution_notes, created_at)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-due-saw'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-saw'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-saw'), CURRENT_DATE - 5, 'open', NULL, NULL, NULL, NOW() - INTERVAL '95 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-due-bus-resolved'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-bus'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle'), CURRENT_DATE - 30, 'resolved', NOW() - INTERVAL '28 days', '{{admin_id}}', 'Reifendruck, Ölstand und Ladungssicherung geprüft.', NOW() - INTERVAL '60 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-due-bus-open'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-bus'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle'), CURRENT_DATE + 2, 'open', NULL, NULL, NULL, NOW() - INTERVAL '28 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-due-vacuum'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-vacuum'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-vacuum'), CURRENT_DATE + 12, 'open', NULL, NULL, NULL, NOW() - INTERVAL '48 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-due-trailer'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-trailer'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-vehicle-trailer'), CURRENT_DATE - 2, 'open', NULL, NULL, NULL, NOW() - INTERVAL '182 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-due-router-resolved'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-router'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-router'), CURRENT_DATE - 12, 'resolved', NOW() - INTERVAL '10 days', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), 'Spannzange gereinigt, Rundlauf ohne Befund.', NOW() - INTERVAL '132 days'),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-due-router-open'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-maintenance-router'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-asset-tool-router'), CURRENT_DATE + 110, 'open', NULL, NULL, NULL, NOW() - INTERVAL '10 days')
ON CONFLICT (id) DO NOTHING;

INSERT INTO site_appointments (id, tenant_id, site_id, title, appointment_kind, starts_at, ends_at, notes, assigned_user_ids)
VALUES
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-appointment-winter'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-active'), 'Abnahme Küche Winter', 'customer_appointment', date_trunc('week', NOW()) + INTERVAL '4 days 14 hours', date_trunc('week', NOW()) + INTERVAL '4 days 15 hours 30 minutes', 'Restpunkte und Pflegehinweise besprechen.', ARRAY[uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-lena')]),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-appointment-library'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-library'), 'Montage Bibliotheksleiter', 'worker_deployment', date_trunc('week', NOW()) + INTERVAL '3 days 7 hours', date_trunc('week', NOW()) + INTERVAL '3 days 15 hours', 'Leiterführung, Blenden und Beleuchtung.', ARRAY[uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin'), uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-felix')]),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-appointment-practice'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-planned'), 'Freigabe Oberflächenmuster', 'customer_appointment', date_trunc('week', NOW()) + INTERVAL '2 days 10 hours', date_trunc('week', NOW()) + INTERVAL '2 days 11 hours', 'Eicheton und HPL-Muster final festlegen.', ARRAY[uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-mara')]),
  (uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-appointment-showroom'), '{{tenant_id}}', uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-project-showroom'), 'Musterküche präsentationsbereit', 'milestone', date_trunc('week', NOW()) + INTERVAL '12 days 16 hours', date_trunc('week', NOW()) + INTERVAL '12 days 17 hours', NULL, ARRAY[uuid_generate_v5('{{tenant_id}}', 'onboarding-demo-user-aylin')])
ON CONFLICT (id) DO NOTHING;
