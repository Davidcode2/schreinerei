import { describe, it, expect, vi, afterEach } from "vitest"
import { renderHook } from "@testing-library/react"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import {
  useAdjustMaterialStock,
  useCreateOrderRequest,
  useDeleteCategory,
  useEnrichedMaterialHistory,
  useMaterialHistory,
  useOrderRequests,
  useSiteMaterialHistory,
  useStockInMaterial,
  useUpdateCategory,
  useUpdateMaterial,
  useWithdrawMaterial,
} from "./useInventory"
import { apiClient } from "../client"
import type {
  CreateOrderRequestDto,
  OrderRequest,
  OrderStatusQuery,
} from "@/types/inventory"
import type {
  CreateOrderRequestDto as GeneratedCreateOrderRequestDto,
  OrderRequestResponse,
  OrderStatusQuery as GeneratedOrderStatusQuery,
} from "@/types/generated"

const generatedInventoryTypeCompatibility: [
  OrderRequestResponse extends OrderRequest ? true : never,
  CreateOrderRequestDto extends GeneratedCreateOrderRequestDto ? true : never,
  GeneratedOrderStatusQuery extends OrderStatusQuery ? true : never,
] = [true, true, true]

vi.mock("../client", () => ({
  apiClient: {
    get: vi.fn(),
    patch: vi.fn(),
    post: vi.fn(),
    delete: vi.fn(),
  },
}))

function createWrapper(queryClient: QueryClient) {
  return ({ children }: { children: React.ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  )
}

function createQueryClient() {
  return new QueryClient({
    defaultOptions: {
      queries: { retry: false },
    },
  })
}

describe("useMaterialHistory", () => {
  afterEach(() => {
    vi.clearAllMocks()
  })

  it("issues GET to material history endpoint with expected query key", async () => {
    vi.mocked(apiClient.get).mockResolvedValueOnce([])
    const queryClient = createQueryClient()

    const { result } = renderHook(() => useMaterialHistory("mat-123"), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.refetch()

    expect(apiClient.get).toHaveBeenCalledWith(
      "/api/v1/inventory/materials/mat-123/history"
    )
    expect(
      queryClient.getQueryCache().find({ queryKey: ["material-history", "mat-123"] })
    ).toBeDefined()
  })

  it("is disabled when id is empty", () => {
    const queryClient = createQueryClient()

    const { result } = renderHook(() => useMaterialHistory(""), {
      wrapper: createWrapper(queryClient),
    })

    expect(result.current.isEnabled).toBe(false)
    expect(apiClient.get).not.toHaveBeenCalled()
  })
})

describe("useEnrichedMaterialHistory", () => {
  afterEach(() => {
    vi.clearAllMocks()
  })

  it("issues GET to enriched material history endpoint with dedicated query key", async () => {
    vi.mocked(apiClient.get).mockResolvedValueOnce([])
    const queryClient = createQueryClient()

    const { result } = renderHook(() => useEnrichedMaterialHistory("mat-123"), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.refetch()

    expect(apiClient.get).toHaveBeenCalledWith(
      "/api/v1/inventory/materials/mat-123/history/enriched"
    )
    expect(
      queryClient.getQueryCache().find({
        queryKey: ["material-history-enriched", "mat-123"],
      })
    ).toBeDefined()
  })

  it("is disabled when id is empty", () => {
    const queryClient = createQueryClient()

    const { result } = renderHook(() => useEnrichedMaterialHistory(""), {
      wrapper: createWrapper(queryClient),
    })

    expect(result.current.isEnabled).toBe(false)
    expect(apiClient.get).not.toHaveBeenCalled()
  })
})

describe("useSiteMaterialHistory", () => {
  afterEach(() => {
    vi.clearAllMocks()
  })

  it("issues GET to site history endpoint", async () => {
    vi.mocked(apiClient.get).mockResolvedValueOnce([])
    const queryClient = createQueryClient()

    const { result } = renderHook(() => useSiteMaterialHistory("site-123"), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.refetch()

    expect(apiClient.get).toHaveBeenCalledWith(
      "/api/v1/inventory/sites/site-123/history"
    )
  })
})

describe("inventory mutations", () => {
  afterEach(() => {
    vi.clearAllMocks()
  })

  it("updates categories through the PATCH endpoint and invalidates categories", async () => {
    vi.mocked(apiClient.patch).mockResolvedValueOnce({ id: "cat-123" })
    const queryClient = createQueryClient()
    const invalidateQueries = vi.spyOn(queryClient, "invalidateQueries")

    const { result } = renderHook(() => useUpdateCategory(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync({
      id: "cat-123",
      data: { name: "Massivholz", description: "Lager A", can_expire: true },
    })

    expect(apiClient.patch).toHaveBeenCalledWith(
      "/api/v1/inventory/categories/cat-123",
      { name: "Massivholz", description: "Lager A", can_expire: true }
    )
    expect(invalidateQueries).toHaveBeenCalledWith({ queryKey: ["categories"] })
  })

  it("deletes categories through the DELETE endpoint and invalidates categories", async () => {
    vi.mocked(apiClient.delete).mockResolvedValueOnce(undefined)
    const queryClient = createQueryClient()
    const invalidateQueries = vi.spyOn(queryClient, "invalidateQueries")

    const { result } = renderHook(() => useDeleteCategory(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync("cat-123")

    expect(apiClient.delete).toHaveBeenCalledWith(
      "/api/v1/inventory/categories/cat-123"
    )
    expect(invalidateQueries).toHaveBeenCalledWith({ queryKey: ["categories"] })
  })

  it("updates materials through the PATCH endpoint and invalidates material queries", async () => {
    vi.mocked(apiClient.patch).mockResolvedValueOnce({ id: "mat-123" })
    const queryClient = createQueryClient()
    const invalidateQueries = vi.spyOn(queryClient, "invalidateQueries")

    const { result } = renderHook(() => useUpdateMaterial(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync({
      id: "mat-123",
      data: {
        location: "Regal 4",
        min_quantity: 12,
        clear_location: true,
      },
    })

    expect(apiClient.patch).toHaveBeenCalledWith(
      "/api/v1/inventory/materials/mat-123",
      {
        location: "Regal 4",
        min_quantity: 12,
        clear_location: true,
      }
    )
    expect(invalidateQueries).toHaveBeenNthCalledWith(1, { queryKey: ["materials"] })
    expect(invalidateQueries).toHaveBeenNthCalledWith(2, { queryKey: ["material"] })
    expect(invalidateQueries).toHaveBeenNthCalledWith(3, { queryKey: ["low-stock"] })
  })

  it("adjusts stock through the adjust endpoint", async () => {
    vi.mocked(apiClient.post).mockResolvedValueOnce({ id: "mat-123" })
    const queryClient = createQueryClient()

    const { result } = renderHook(() => useAdjustMaterialStock(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync({
      id: "mat-123",
      quantity: 4,
      reason: "Inventur",
    })

    expect(apiClient.post).toHaveBeenCalledWith(
      "/api/v1/inventory/materials/mat-123/adjust",
      { quantity: 4, reason: "Inventur" }
    )
  })

  it("stocks in material through the stock-in endpoint and invalidates material queries", async () => {
    vi.mocked(apiClient.post).mockResolvedValueOnce({ id: "mat-123" })
    const queryClient = createQueryClient()
    const invalidateQueries = vi.spyOn(queryClient, "invalidateQueries")

    const { result } = renderHook(() => useStockInMaterial(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync({
      id: "mat-123",
      quantity: 10,
      notes: "Lieferung 1234",
      expires_on: "2026-05-20",
      batch_code: "LOT-2026-05",
      supplier_name: "HolzLand",
      receipt_reference: "LS-1234",
      receipt_date: "2026-05-18",
    })

    expect(apiClient.post).toHaveBeenCalledWith(
      "/api/v1/inventory/materials/mat-123/stock-in",
      {
        quantity: 10,
        notes: "Lieferung 1234",
        expires_on: "2026-05-20",
        batch_code: "LOT-2026-05",
        supplier_name: "HolzLand",
        receipt_reference: "LS-1234",
        receipt_date: "2026-05-18",
      }
    )
    expect(invalidateQueries).toHaveBeenNthCalledWith(1, { queryKey: ["materials"] })
    expect(invalidateQueries).toHaveBeenNthCalledWith(2, { queryKey: ["material"] })
    expect(invalidateQueries).toHaveBeenNthCalledWith(3, { queryKey: ["low-stock"] })
  })

  it("forwards disposal through the withdraw endpoint", async () => {
    vi.mocked(apiClient.post).mockResolvedValueOnce({ id: "mat-123" })
    const queryClient = createQueryClient()

    const { result } = renderHook(() => useWithdrawMaterial(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync({
      id: "mat-123",
      quantity: 2,
      notes: "Abgelaufen",
      site_id: null,
      disposal: true,
    })

    expect(apiClient.post).toHaveBeenCalledWith(
      "/api/v1/inventory/materials/mat-123/withdraw",
      {
        quantity: 2,
        notes: "Abgelaufen",
        site_id: null,
        disposal: true,
        last_package_taken: false,
      }
    )
  })

  it("keeps generated inventory order types compatible", () => {
    expect(generatedInventoryTypeCompatibility).toHaveLength(3)
  })

  it("fetches order requests with generated-backed query typing", async () => {
    vi.mocked(apiClient.get).mockResolvedValueOnce([])
    const queryClient = createQueryClient()

    const { result } = renderHook(
      () => useOrderRequests({ status: "pending" }),
      { wrapper: createWrapper(queryClient) }
    )

    await result.current.refetch()

    expect(apiClient.get).toHaveBeenCalledWith("/api/v1/inventory/orders?status=pending")
  })

  it("creates order requests through the orders endpoint", async () => {
    vi.mocked(apiClient.post).mockResolvedValueOnce({ id: "order-123" })
    const queryClient = createQueryClient()
    const invalidateQueries = vi.spyOn(queryClient, "invalidateQueries")

    const { result } = renderHook(() => useCreateOrderRequest(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync({
      material_id: "mat-123",
      quantity: 5,
      reason: "Niedriger Bestand",
    })

    expect(apiClient.post).toHaveBeenCalledWith("/api/v1/inventory/orders", {
      material_id: "mat-123",
      quantity: 5,
      reason: "Niedriger Bestand",
    })
    expect(invalidateQueries).toHaveBeenCalledWith({ queryKey: ["order-requests"] })
  })
})
