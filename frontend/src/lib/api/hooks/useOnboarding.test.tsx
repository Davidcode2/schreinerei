import { describe, it, expect, vi, afterEach } from "vitest"
import { renderHook, waitFor } from "@testing-library/react"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import type { ReactNode } from "react"
import { useCreateOnboardingSession, useOnboardingSession } from "./useOnboarding"
import { apiClient } from "../client"

vi.mock("../client", () => ({
  apiClient: {
    get: vi.fn(),
    post: vi.fn(),
  },
}))

function createWrapper(queryClient: QueryClient) {
  return ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  )
}

function createQueryClient() {
  return new QueryClient({
    defaultOptions: {
      queries: { retry: false },
      mutations: { retry: false },
    },
  })
}

describe("onboarding hooks", () => {
  afterEach(() => {
    vi.clearAllMocks()
  })

  it("creates onboarding sessions through the public API", async () => {
    vi.mocked(apiClient.post).mockResolvedValueOnce({
      session_id: "session-1",
      organization_slug: "schreinerei-beispiel",
      status: "pending_payment",
      payment_provider: "mollie",
      payment_id: "tr_123",
      checkout_url: "https://checkout.example.test/session-1",
    })

    const queryClient = createQueryClient()
    const { result } = renderHook(() => useCreateOnboardingSession(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync({
      organization_name: "Schreinerei Beispiel",
      admin_name: "Ada Admin",
      admin_email: "ada@example.com",
      selected_plan: "starter",
    })

    expect(apiClient.post).toHaveBeenCalledWith("/api/v1/onboarding/sessions", {
      organization_name: "Schreinerei Beispiel",
      admin_name: "Ada Admin",
      admin_email: "ada@example.com",
      selected_plan: "starter",
    })
  })

  it("fetches onboarding session status by id", async () => {
    vi.mocked(apiClient.get).mockResolvedValueOnce({
      session_id: "session-1",
      organization_slug: "schreinerei-beispiel",
      status: "completed",
      error_message: null,
    })

    const queryClient = createQueryClient()
    const { result } = renderHook(() => useOnboardingSession("session-1"), {
      wrapper: createWrapper(queryClient),
    })

    await waitFor(() => expect(result.current.isSuccess).toBe(true))

    expect(apiClient.get).toHaveBeenCalledWith("/api/v1/onboarding/sessions/session-1")
    expect(result.current.data?.status).toBe("completed")
  })
})
