ALTER TABLE tenants
ADD COLUMN default_hourly_rate_cents BIGINT;

ALTER TABLE sites
ADD COLUMN invoice_pricing_mode VARCHAR(50),
ADD COLUMN hourly_rate_cents BIGINT,
ADD COLUMN fixed_price_cents BIGINT;

ALTER TABLE sites
ADD CONSTRAINT sites_invoice_pricing_mode_check
CHECK (
    invoice_pricing_mode IS NULL
    OR invoice_pricing_mode IN ('hourly_rate', 'fixed_price')
);

ALTER TABLE tenants
ADD CONSTRAINT tenants_default_hourly_rate_non_negative_check
CHECK (
    default_hourly_rate_cents IS NULL OR default_hourly_rate_cents >= 0
);

ALTER TABLE sites
ADD CONSTRAINT sites_hourly_rate_non_negative_check
CHECK (
    hourly_rate_cents IS NULL OR hourly_rate_cents >= 0
);

ALTER TABLE sites
ADD CONSTRAINT sites_fixed_price_non_negative_check
CHECK (
    fixed_price_cents IS NULL OR fixed_price_cents >= 0
);
