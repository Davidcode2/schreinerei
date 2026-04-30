-- Fix hours column type from DECIMAL to FLOAT8 for compatibility with Rust f64
ALTER TABLE time_entries ALTER COLUMN hours TYPE FLOAT8 USING hours::FLOAT8;
