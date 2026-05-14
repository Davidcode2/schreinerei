import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import userEvent from "@testing-library/user-event"
import { fireEvent, screen, waitFor } from "@testing-library/react"
import { render } from "@/test/utils"
import { SignupPage } from "./SignupPage"

const mutateAsync = vi.fn()
const locationAssign = vi.fn()

vi.mock("@/lib/api/hooks", () => ({
  usePublicInvite: vi.fn(() => ({ data: undefined, isLoading: false, error: null })),
  useCreateOnboardingSession: vi.fn(() => ({
    mutateAsync,
    isPending: false,
    error: null,
  })),
}))

describe("SignupPage", () => {
  beforeEach(() => {
    mutateAsync.mockReset()
    locationAssign.mockReset()
    Object.defineProperty(window, "location", {
      configurable: true,
      value: {
        ...window.location,
        assign: locationAssign,
      },
    })
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("renders an enabled onboarding form", () => {
    render(<SignupPage />)

    expect(screen.getByRole("heading", { name: /organisation erstellen/i })).toBeInTheDocument()
    expect(screen.getByLabelText(/organisationsname/i)).toBeEnabled()
    expect(screen.getByLabelText(/name des admins/i)).toBeEnabled()
    expect(screen.getByLabelText(/e-mail des admins/i)).toBeEnabled()
    expect(screen.getByRole("button", { name: /onboarding starten/i })).toBeEnabled()
  })

  it("creates a session and redirects to mollie checkout", async () => {
    mutateAsync.mockResolvedValueOnce({
      session_id: "session-1",
      organization_slug: "schreinerei-beispiel",
      status: "pending_payment",
      payment_provider: "mollie",
      payment_id: "tr_123",
      checkout_url: "https://checkout.example.test/session-1",
    })

    const user = userEvent.setup()
    render(<SignupPage />)

    await user.type(screen.getByLabelText(/organisationsname/i), "Schreinerei Beispiel")
    await user.type(screen.getByLabelText(/name des admins/i), "Ada Admin")
    await user.type(screen.getByLabelText(/e-mail des admins/i), "ada@example.com")
    const form = screen.getByRole("button", { name: /onboarding starten/i }).closest("form")
    expect(form).not.toBeNull()
    fireEvent.submit(form!)

    await waitFor(() => {
      expect(mutateAsync).toHaveBeenCalledWith({
        organization_name: "Schreinerei Beispiel",
        admin_name: "Ada Admin",
        admin_email: "ada@example.com",
        selected_plan: "starter",
      })
    })
    expect(locationAssign).toHaveBeenCalledWith("https://checkout.example.test/session-1")
  })
})
