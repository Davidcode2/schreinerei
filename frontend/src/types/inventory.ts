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

export interface CreateMaterialRequest {
  category_id: string
  name: string
  description?: string
  unit: string
  quantity: number
  min_quantity: number
  location?: string
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
