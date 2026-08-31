import { beforeEach, describe, expect, it, vi } from "vitest"
import { renderHook } from "@testing-library/react"
import { useAuthStore } from "@/lib/auth/authStore"
import { useLogout } from "./useLogout"

const getLogoutUrlMock = vi.hoisted(() => vi.fn(() => "#logged-out"))

vi.mock("@/lib/auth/keycloak", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@/lib/auth/keycloak")>()
  return {
    ...actual,
    getLogoutUrl: getLogoutUrlMock,
  }
})

describe("useLogout", () => {
  beforeEach(() => {
    getLogoutUrlMock.mockReturnValue("#logged-out")
    window.location.hash = ""
    useAuthStore.setState({
      user: {
        id: "user-1",
        tenant_id: "tenant-1",
        email: "admin@example.com",
        name: "Admin",
        role: "admin",
        created_at: new Date().toISOString(),
      },
      tokens: { access_token: "a", refresh_token: "r", expires_at: Date.now() + 60_000 },
      isAuthenticated: true,
      isLoading: false,
    })
  })

  it("clears the auth state and navigates to the Keycloak logout URL", () => {
    const { result } = renderHook(() => useLogout())

    result.current()

    expect(useAuthStore.getState().isAuthenticated).toBe(false)
    expect(useAuthStore.getState().user).toBeNull()
    expect(useAuthStore.getState().tokens).toBeNull()
    expect(getLogoutUrlMock).toHaveBeenCalled()
    expect(window.location.hash).toBe("#logged-out")
  })
})
