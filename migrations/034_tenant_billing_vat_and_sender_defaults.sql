ALTER TABLE tenants
ADD COLUMN billing_tax_mode VARCHAR(50) NOT NULL DEFAULT 'standard',
ADD COLUMN billing_sender_name VARCHAR(255),
ADD COLUMN billing_sender_address TEXT;

ALTER TABLE tenants
ADD CONSTRAINT tenants_billing_tax_mode_check
CHECK (billing_tax_mode IN ('standard', 'kleinunternehmer'));
