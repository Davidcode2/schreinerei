/**
 * Thin inventory type facade backed by generated API DTOs.
 */

import type {
  AdjustStockRequest as GeneratedAdjustStockRequest,
  ApproveOrderRequestDto as GeneratedApproveOrderRequestDto,
  CategoryResponse,
  CreateCategoryRequest as GeneratedCreateCategoryRequest,
  CreateMaterialRequest as GeneratedCreateMaterialRequest,
  CreateOrderRequestDto as GeneratedCreateOrderRequestDto,
  EnrichedStockHistoryResponse,
  EntryType as GeneratedEntryType,
  FulfillOrderRequestDto as GeneratedFulfillOrderRequestDto,
  ListMaterialsQuery as GeneratedListMaterialsQuery,
  MaterialResponse,
  OrderRequestResponse as GeneratedOrderRequestResponse,
  OrderStatusQuery as GeneratedOrderStatusQuery,
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

export type OrderRequest = GeneratedOrderRequestResponse

export type OrderStatus = OrderRequest["status"]

export type CreateOrderRequestDto = GeneratedCreateOrderRequestDto

export type ApproveOrderRequestDto = GeneratedApproveOrderRequestDto

export type FulfillOrderRequestDto = GeneratedFulfillOrderRequestDto

export type OrderStatusQuery = GeneratedOrderStatusQuery
