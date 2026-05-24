/**
 * Thin inventory type facade backed by generated API DTOs.
 */

import type {
  ApproveOrderRequestDto as GeneratedApproveOrderRequestDto,
  CategoryResponse,
  CreateCategoryRequest as GeneratedCreateCategoryRequest,
  CreateOrderRequestDto as GeneratedCreateOrderRequestDto,
  EnrichedStockHistoryResponse,
  EntryType as GeneratedEntryType,
  FulfillOrderRequestDto as GeneratedFulfillOrderRequestDto,
  ListMaterialsQuery as GeneratedListMaterialsQuery,
  OrderRequestResponse as GeneratedOrderRequestResponse,
  OrderStatusQuery as GeneratedOrderStatusQuery,
  QrCodeResponse as GeneratedQrCodeResponse,
  QrSvgResponse as GeneratedQrSvgResponse,
  SiteStockHistoryResponse,
  StockEntryResponse,
  StockInRequest as GeneratedStockInRequest,
  UpdateCategoryRequest as GeneratedUpdateCategoryRequest,
  WithdrawRequest as GeneratedWithdrawRequest,
} from "@/types/generated"

// === Category ===

export type Category = CategoryResponse

export type CreateCategoryRequest = GeneratedCreateCategoryRequest

export type UpdateCategoryRequest = GeneratedUpdateCategoryRequest

// === Material ===

export interface Material {
  id: string
  category_id: string
  name: string
  description: string | null
  unit: string
  quantity: number
  min_quantity: number
  can_expire: boolean
  legacy_quantity: number
  expired_quantity: number
  expiring_soon_quantity: number
  next_expiry_on: string | null
  expiry_batches: Array<{
    id: string
    batch_code: string | null
    expires_on: string
    quantity: number
    received_at: string
    is_expired: boolean
    is_expiring_soon: boolean
  }>
  location: string | null
  base_price_cents: number | null
  price_markup_percentage: number | null
  qr_code: string | null
  is_low_stock: boolean
  created_at: string
}

export type MaterialStockHistoryEntry = StockEntryResponse

export type EntryType = GeneratedEntryType

export type EnrichedStockHistoryEntry = EnrichedStockHistoryResponse

export type SiteMaterialHistoryEntry = SiteStockHistoryResponse

export interface CreateMaterialRequest {
  category_id: string
  name: string
  description: string | null
  unit: string
  quantity: number
  min_quantity: number
  location: string | null
  base_price_cents: number | null
  price_markup_percentage: number | null
  expires_on: string | null
  batch_code: string | null
}

export interface UpdateMaterialRequest {
  location?: string
  min_quantity?: number
  base_price_cents?: number | null
  price_markup_percentage?: number | null
  clear_location?: boolean
  clear_base_price_cents?: boolean
  clear_price_markup_percentage?: boolean
}

export type WithdrawRequest = GeneratedWithdrawRequest

export interface AdjustStockRequest {
  quantity: number
  reason: string
}

export type StockInRequest = GeneratedStockInRequest

export type ListMaterialsQuery = GeneratedListMaterialsQuery

// === QR Code ===

export type QrCodeResponse = GeneratedQrCodeResponse

export type QrSvgResponse = GeneratedQrSvgResponse

// === Order Request ===

export type OrderRequest = GeneratedOrderRequestResponse

export type OrderStatus = OrderRequest["status"]

export type CreateOrderRequestDto = GeneratedCreateOrderRequestDto

export type ApproveOrderRequestDto = GeneratedApproveOrderRequestDto

export type FulfillOrderRequestDto = GeneratedFulfillOrderRequestDto

export type OrderStatusQuery = GeneratedOrderStatusQuery
