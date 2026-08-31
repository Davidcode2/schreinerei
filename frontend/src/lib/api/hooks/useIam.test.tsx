import type { ReactNode } from "react"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { renderHook } from "@testing-library/react"
import { afterEach, describe, expect, it, vi } from "vitest"

import { apiClient } from "../client"
import { useDeleteUser, useUpdateUserRole } from "./useIam"

vi.mock("../client", () => ({
  apiClient: {
    delete: vi.fn(),
    patch: vi.fn(),
  },
}))

function createQueryClient() {
  return new QueryClient({ defaultOptions: { mutations: { retry: false } } })
}

function createWrapper(queryClient: QueryClient) {
  return ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  )
}

describe("IAM user mutations", () => {
  afterEach(() => vi.clearAllMocks())

  it("updates a user role and invalidates the users list", async () => {
    vi.mocked(apiClient.patch).mockResolvedValueOnce({ id: "user-1", role: "admin" })
    const queryClient = createQueryClient()
    const invalidateQueries = vi.spyOn(queryClient, "invalidateQueries")
    const { result } = renderHook(() => useUpdateUserRole(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync({ id: "user-1", role: "admin" })

    expect(apiClient.patch).toHaveBeenCalledWith("/api/v1/users/user-1/role", {
      role: "admin",
    })
    expect(invalidateQueries).toHaveBeenCalledWith({ queryKey: ["users"] })
  })

  it("deletes a user and invalidates the users list", async () => {
    vi.mocked(apiClient.delete).mockResolvedValueOnce(undefined)
    const queryClient = createQueryClient()
    const invalidateQueries = vi.spyOn(queryClient, "invalidateQueries")
    const { result } = renderHook(() => useDeleteUser(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync("user-1")

    expect(apiClient.delete).toHaveBeenCalledWith("/api/v1/users/user-1")
    expect(invalidateQueries).toHaveBeenCalledWith({ queryKey: ["users"] })
  })
})
