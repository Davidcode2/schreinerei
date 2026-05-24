ALTER TABLE materials
ADD COLUMN base_price_cents BIGINT,
ADD COLUMN price_markup_percentage INTEGER;

ALTER TABLE materials
ADD CONSTRAINT materials_base_price_non_negative_check
CHECK (
    base_price_cents IS NULL OR base_price_cents >= 0
);

ALTER TABLE materials
ADD CONSTRAINT materials_price_markup_non_negative_check
CHECK (
    price_markup_percentage IS NULL OR price_markup_percentage >= 0
);
