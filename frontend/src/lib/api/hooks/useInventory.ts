import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query"
import { apiClient } from "../client"
import type {
  Category,
  Material,
  CreateMaterialRequest,
  WithdrawRequest,
  OrderRequest,
  CreateOrderRequestDto,
  OrderStatusQuery,
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
    mutationFn: (data: { name: string; description?: string }) =>
      apiClient.post<Category>("/api/v1/inventory/categories", data),
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
