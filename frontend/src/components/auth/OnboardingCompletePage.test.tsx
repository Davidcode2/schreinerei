import { describe, expect, it, vi } from "vitest"
import { render, screen } from "@/test/utils"
import { OnboardingCompletePage } from "./OnboardingCompletePage"

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
  it("shows a missing session message when the query param is absent", () => {
    window.history.pushState({}, "", "/onboarding/complete")

    render(<OnboardingCompletePage />)

    expect(screen.getByText(/session-id fehlt/i)).toBeInTheDocument()
  })

  it("explains that the user should finish signup via the email after provisioning completed", () => {
    window.history.pushState({}, "", "/onboarding/complete?session=session-1")

    render(<OnboardingCompletePage />)

    expect(screen.getByText(/ihre organisation ist bereit/i)).toBeInTheDocument()
    expect(
      screen.getByText(/sie erhalten in kuerze eine einladung per e-mail/i)
    ).toBeInTheDocument()
    expect(screen.queryByRole("button", { name: /weiter zur anmeldung/i })).not.toBeInTheDocument()
  })
})
