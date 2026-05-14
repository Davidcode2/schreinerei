import { beforeEach, describe, expect, it, vi } from "vitest"
import userEvent from "@testing-library/user-event"
import { render, screen } from "@/test/utils"
import { OnboardingCompletePage } from "./OnboardingCompletePage"

const { startLogin } = vi.hoisted(() => ({
  startLogin: vi.fn(),
}))

vi.mock("@/lib/auth/keycloak", () => ({
  startLogin,
}))

vi.mock("@/lib/api/hooks", () => ({
  useOnboardingSession: vi.fn((sessionId: string | null) => ({
    data: sessionId
      ? {
          session_id: sessionId,
          organization_slug: "schreinerei-beispiel",
          status: "completed",
          error_message: null,
        }
      : undefined,
    isLoading: false,
    error: null,
  })),
}))

describe("OnboardingCompletePage", () => {
  beforeEach(() => {
    startLogin.mockReset()
  })

  it("shows a missing session message when the query param is absent", () => {
    window.history.pushState({}, "", "/onboarding/complete")

    render(<OnboardingCompletePage />)

    expect(screen.getByText(/session-id fehlt/i)).toBeInTheDocument()
  })

  it("lets the user continue to login after provisioning completed", async () => {
    window.history.pushState({}, "", "/onboarding/complete?session=session-1")

    const user = userEvent.setup()
    render(<OnboardingCompletePage />)

    expect(screen.getByText(/ihre organisation ist bereit/i)).toBeInTheDocument()
    await user.click(screen.getByRole("button", { name: /weiter zur anmeldung/i }))

    expect(startLogin).toHaveBeenCalledOnce()
  })
})
