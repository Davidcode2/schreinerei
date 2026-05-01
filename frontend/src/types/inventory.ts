/**
 * Inventory module types matching backend DTOs
 */

// === Category ===

export interface Category {
  id: string
  name: string
  description: string | null
  created_at: string
}

export interface CreateCategoryRequest {
  name: string
  description?: string
}

export interface UpdateCategoryRequest {
  name?: string
  description?: string
}

// === Material ===

export interface Material {
  id: string
  category_id: string
  name: string
  description: string | null
  unit: string
  quantity: number
  min_quantity: number
  location: string | null
  qr_code: string | null
  is_low_stock: boolean
  created_at: string
}

export interface MaterialStockHistoryEntry {
  id: string
  quantity_change: number
  quantity_after: number
  notes: string | null
  site_id: string | null
  site_name: string | null
  created_at: string
}

export type EntryType = 'withdrawn' | 'adjusted' | 'material_added' | 'location_changed' | 'min_quantity_changed'

export interface EnrichedStockHistoryEntry {
  id: string
  material_id: string
  user_id: string
  user_name: string
  entry_type: EntryType
  quantity_change: number
  quantity_after: number
  notes: string | null
  site_id: string | null
  site_name: string | null
  category_name: string
  created_at: string
}

export interface SiteMaterialHistoryEntry {
  id: string
  material_id: string
  material_name: string
  category_name: string
  quantity_change: number
  quantity_after: number
  notes: string | null
  site_id: string | null
  site_name: string | null
  extracted_by: string
  created_at: string
}

export interface CreateMaterialRequest {
  category_id: string
  name: string
  description?: string
  unit: string
  quantity: number
  min_quantity: number
  location?: string
}

export interface UpdateMaterialRequest {
  location?: string
  min_quantity?: number
  clear_location?: boolean
}

export interface WithdrawRequest {
  quantity: number
  notes?: string
  site_id?: string | null
}

export interface AdjustStockRequest {
  quantity: number
  reason: string
}

export interface StockInRequest {
  quantity: number
  notes?: string
}

export interface ListMaterialsQuery {
  category_id?: string
}

// === QR Code ===

export interface QrCodeResponse {
  qr_code: string
  material_id: string
  material_name: string
}

export interface QrSvgResponse {
  svg: string
  qr_code: string
}

// === Order Request ===

export type OrderStatus = 'pending' | 'approved' | 'fulfilled' | 'cancelled'

export interface OrderRequest {
  id: string
  material_id: string
  material_name: string
  quantity: number
  requested_by: string
  status: OrderStatus
  reason: string | null
  approved_by: string | null
  approved_at: string | null
  fulfilled_at: string | null
  notes: string | null
  created_at: string
}

export interface CreateOrderRequestDto {
  material_id: string
  quantity: number
  reason?: string
}

export interface ApproveOrderRequestDto {
  notes?: string
}

export interface FulfillOrderRequestDto {
  actual_quantity: number
  notes?: string
}

export interface OrderStatusQuery {
  status?: OrderStatus
}
