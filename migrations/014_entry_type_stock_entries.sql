-- Migration 014: Add entry_type column to stock_entries with backfill
-- Step 1: Add nullable column
ALTER TABLE stock_entries ADD COLUMN entry_type VARCHAR(20);

-- Step 2: Backfill existing rows based on quantity_change, site_id, and notes
-- Withdrawals with a site link are "withdrawn" (material taken to a Baustelle)
UPDATE stock_entries SET entry_type = 'withdrawn' WHERE quantity_change < 0 AND site_id IS NOT NULL;
-- Negative changes without site link are "adjusted" (admin adjustment)
UPDATE stock_entries SET entry_type = 'adjusted' WHERE quantity_change < 0 AND site_id IS NULL;
-- Positive changes with notes are "adjusted" (admin stock adjustments with reason)
UPDATE stock_entries SET entry_type = 'adjusted' WHERE quantity_change > 0 AND notes IS NOT NULL;
-- Remaining positive changes without notes are "adjusted" (order fulfillments etc.)
UPDATE stock_entries SET entry_type = 'adjusted' WHERE entry_type IS NULL;

-- Step 3: Make column NOT NULL
ALTER TABLE stock_entries ALTER COLUMN entry_type SET NOT NULL;

-- Step 4: Index for filtering by entry type
CREATE INDEX idx_stock_entries_entry_type ON stock_entries(entry_type);