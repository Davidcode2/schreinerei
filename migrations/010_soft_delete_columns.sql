-- Migration 010: Add soft delete columns
-- Adds deleted_at columns to materials, sites, vehicles, and tools tables
-- Enables soft delete semantics for offline sync compatibility

-- Add deleted_at column to materials
ALTER TABLE materials ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ DEFAULT NULL;

-- Add deleted_at column to sites
ALTER TABLE sites ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ DEFAULT NULL;

-- Add deleted_at column to vehicles
ALTER TABLE vehicles ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ DEFAULT NULL;

-- Add deleted_at column to tools
ALTER TABLE tools ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ DEFAULT NULL;

-- Create partial indexes for efficient queries on non-deleted items
-- These indexes improve performance of list queries that exclude soft-deleted items
CREATE INDEX IF NOT EXISTS idx_materials_not_deleted ON materials(tenant_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_sites_not_deleted ON sites(tenant_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_vehicles_not_deleted ON vehicles(tenant_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_tools_not_deleted ON tools(tenant_id) WHERE deleted_at IS NULL;

-- Create composite indexes for filtered queries
CREATE INDEX IF NOT EXISTS idx_materials_category_not_deleted ON materials(tenant_id, category_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_sites_status_not_deleted ON sites(tenant_id, status) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_vehicles_status_not_deleted ON vehicles(tenant_id, status) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_tools_status_not_deleted ON tools(tenant_id, status) WHERE deleted_at IS NULL;
