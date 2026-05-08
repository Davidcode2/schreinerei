ALTER TABLE sites
    ADD COLUMN budget_amount_cents BIGINT,
    ADD COLUMN billing_reference TEXT,
    ADD COLUMN billing_notes TEXT,
    ADD COLUMN quote_reference TEXT;
