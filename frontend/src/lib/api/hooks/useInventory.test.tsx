import { describe, it, expect, vi, afterEach } from "vitest"
import { renderHook } from "@testing-library/react"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { useMaterialHistory } from "./useInventory"
import { apiClient } from "../client"

vi.mock("../client", () => ({
  apiClient: {
    get: vi.fn(),
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
