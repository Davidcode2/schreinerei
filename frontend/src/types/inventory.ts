/**
 * Thin inventory type facade backed by generated API DTOs.
 */

import type {
  AdjustStockRequest as GeneratedAdjustStockRequest,
  CategoryResponse,
  CreateCategoryRequest as GeneratedCreateCategoryRequest,
  CreateMaterialRequest as GeneratedCreateMaterialRequest,
  EnrichedStockHistoryResponse,
  EntryType as GeneratedEntryType,
  ListMaterialsQuery as GeneratedListMaterialsQuery,
  MaterialResponse,
  QrCodeResponse as GeneratedQrCodeResponse,
  QrSvgResponse as GeneratedQrSvgResponse,
  SiteStockHistoryResponse,
  StockEntryResponse,
  StockInRequest as GeneratedStockInRequest,
  UpdateCategoryRequest as GeneratedUpdateCategoryRequest,
  UpdateMaterialRequest as GeneratedUpdateMaterialRequest,
  WithdrawRequest as GeneratedWithdrawRequest,
} from "@/types/generated"

// === Category ===

export type Category = CategoryResponse

export type CreateCategoryRequest = GeneratedCreateCategoryRequest

export type UpdateCategoryRequest = GeneratedUpdateCategoryRequest

// === Material ===

export type Material = MaterialResponse

export type MaterialStockHistoryEntry = StockEntryResponse

export type EntryType = GeneratedEntryType

export type EnrichedStockHistoryEntry = EnrichedStockHistoryResponse

export type SiteMaterialHistoryEntry = SiteStockHistoryResponse

export type CreateMaterialRequest = GeneratedCreateMaterialRequest

export type UpdateMaterialRequest = GeneratedUpdateMaterialRequest

export type WithdrawRequest = GeneratedWithdrawRequest

export type AdjustStockRequest = GeneratedAdjustStockRequest

export type StockInRequest = GeneratedStockInRequest

export type ListMaterialsQuery = GeneratedListMaterialsQuery

// === QR Code ===

export type QrCodeResponse = GeneratedQrCodeResponse

export type QrSvgResponse = GeneratedQrSvgResponse

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
