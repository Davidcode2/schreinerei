import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query"
import { apiClient } from "../client"
import type {
  AdjustStockRequest,
  Category,
  CreateCategoryRequest,
  Material,
  CreateMaterialRequest,
  CreateOrderRequestDto,
  EnrichedStockHistoryEntry,
  MaterialStockHistoryEntry,
  OrderRequest,
  OrderStatusQuery,
  SiteMaterialHistoryEntry,
  StockInRequest,
  UpdateCategoryRequest,
  UpdateMaterialRequest,
  WithdrawRequest,
} from "@/types/inventory"

// === Categories ===

export function useCategories() {
  return useQuery({
    queryKey: ["categories"],
    queryFn: () => apiClient.get<Category[]>("/api/v1/inventory/categories"),
    staleTime: 30000,
  })
}

export function useCreateCategory() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: CreateCategoryRequest) =>
      apiClient.post<Category>("/api/v1/inventory/categories", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["categories"] })
    },
  })
}

export function useUpdateCategory() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateCategoryRequest }) =>
      apiClient.patch<Category>(`/api/v1/inventory/categories/${id}`, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["categories"] })
    },
  })
}

export function useDeleteCategory() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) => apiClient.delete(`/api/v1/inventory/categories/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["categories"] })
    },
  })
}

// === Materials ===

export function useMaterials(categoryId?: string) {
  return useQuery({
    queryKey: ["materials", categoryId],
    queryFn: () => {
      const params = categoryId ? `?category_id=${categoryId}` : ""
      return apiClient.get<Material[]>(`/api/v1/inventory/materials${params}`)
    },
    staleTime: 30000,
  })
}

export function useMaterial(id: string) {
  return useQuery({
    queryKey: ["material", id],
    queryFn: () => apiClient.get<Material>(`/api/v1/inventory/materials/${id}`),
    enabled: !!id,
    staleTime: 30000,
  })
}

export function useMaterialHistory(id: string) {
  return useQuery({
    queryKey: ["material-history", id],
    queryFn: () =>
      apiClient.get<MaterialStockHistoryEntry[]>(
        `/api/v1/inventory/materials/${id}/history`
      ),
    enabled: !!id,
    staleTime: 30000,
  })
}

export function useEnrichedMaterialHistory(id: string) {
  return useQuery({
    queryKey: ["material-history-enriched", id],
    queryFn: () =>
      apiClient.get<EnrichedStockHistoryEntry[]>(
        `/api/v1/inventory/materials/${id}/history/enriched`
      ),
    enabled: !!id,
    staleTime: 30000,
  })
}

export function useSiteMaterialHistory(siteId: string, limit = 50) {
  return useQuery({
    queryKey: ["site-material-history", siteId, limit],
    queryFn: () =>
      apiClient.get<SiteMaterialHistoryEntry[]>(
        `/api/v1/inventory/sites/${siteId}/history`
      ),
    enabled: !!siteId,
    staleTime: 30000,
  })
}

export function useCreateMaterial() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: CreateMaterialRequest) =>
      apiClient.post<Material>("/api/v1/inventory/materials", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["materials"] })
    },
  })
}

export function useUpdateMaterial() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateMaterialRequest }) =>
      apiClient.patch<Material>(`/api/v1/inventory/materials/${id}`, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["materials"] })
      queryClient.invalidateQueries({ queryKey: ["material"] })
      queryClient.invalidateQueries({ queryKey: ["low-stock"] })
    },
  })
}

export function useAdjustMaterialStock() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, ...data }: AdjustStockRequest & { id: string }) =>
      apiClient.post<Material>(`/api/v1/inventory/materials/${id}/adjust`, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["materials"] })
      queryClient.invalidateQueries({ queryKey: ["material"] })
      queryClient.invalidateQueries({ queryKey: ["low-stock"] })
    },
  })
}

export function useStockInMaterial() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, ...data }: StockInRequest & { id: string }) =>
      apiClient.post<Material>(`/api/v1/inventory/materials/${id}/stock-in`, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["materials"] })
      queryClient.invalidateQueries({ queryKey: ["material"] })
      queryClient.invalidateQueries({ queryKey: ["low-stock"] })
    },
  })
}

export function useWithdrawMaterial() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({
      id,
      ...data
    }: WithdrawRequest & { id: string }) =>
      apiClient.post<Material>(`/api/v1/inventory/materials/${id}/withdraw`, {
        quantity: data.quantity,
        notes: data.notes,
        site_id: data.site_id ?? null,
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["materials"] })
      queryClient.invalidateQueries({ queryKey: ["material"] })
      queryClient.invalidateQueries({ queryKey: ["low-stock"] })
    },
  })
}

// === Low Stock ===

export function useLowStockMaterials() {
  return useQuery({
    queryKey: ["low-stock"],
    queryFn: () => apiClient.get<Material[]>("/api/v1/inventory/low-stock"),
    staleTime: 30000,
  })
}

// === QR Lookup ===

export function useQrLookup(code: string | null) {
  return useQuery({
    queryKey: ["qr-lookup", code],
    queryFn: () => apiClient.get<Material>(`/api/v1/inventory/qr/${code}`),
    enabled: !!code,
    staleTime: 30000,
  })
}

// === Order Requests ===

export function useOrderRequests(query?: OrderStatusQuery) {
  return useQuery({
    queryKey: ["order-requests", query],
    queryFn: () => {
      const params = query?.status ? `?status=${query.status}` : ""
      return apiClient.get<OrderRequest[]>(`/api/v1/inventory/orders${params}`)
    },
    staleTime: 30000,
  })
}

export function useCreateOrderRequest() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: CreateOrderRequestDto) =>
      apiClient.post<OrderRequest>("/api/v1/inventory/orders", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["order-requests"] })
    },
  })
}

export function useDeleteMaterial() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) =>
      apiClient.delete(`/api/v1/inventory/materials/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["materials"] })
      queryClient.invalidateQueries({ queryKey: ["low-stock"] })
    },
  })
}
