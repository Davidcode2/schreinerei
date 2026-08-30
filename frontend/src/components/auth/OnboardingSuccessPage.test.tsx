import { beforeEach, describe, expect, it, vi } from "vitest"
import userEvent from "@testing-library/user-event"
import { render, screen } from "@/test/utils"
import { useAuthStore } from "@/lib/auth/authStore"
import { OnboardingSuccessPage } from "./OnboardingSuccessPage"

vi.mock("canvas-confetti", () => ({
  default: vi.fn(),
}))

const startLoginMock = vi.fn()
vi.mock("@/lib/auth/keycloak", () => ({
  startLogin: () => startLoginMock(),
}))

describe("OnboardingSuccessPage", () => {
  beforeEach(() => {
    startLoginMock.mockReset()
    useAuthStore.setState({
      user: null,
      tokens: null,
      isAuthenticated: false,
      isLoading: false,
    })
  })

  it("shows a welcome message and a login button", () => {
    render(<OnboardingSuccessPage />)

    expect(screen.getByRole("heading", { name: /willkommen bei schreinerei/i })).toBeInTheDocument()
    expect(
      screen.getByText(/ihr konto ist bereit/i)
    ).toBeInTheDocument()
    expect(
      screen.getByRole("button", { name: /zur anmeldung/i })
    ).toBeInTheDocument()
  })

  it("starts the Keycloak login when the login button is clicked", async () => {
    const user = userEvent.setup()

    render(<OnboardingSuccessPage />)

    await user.click(screen.getByRole("button", { name: /zur anmeldung/i }))

    expect(startLoginMock).toHaveBeenCalledTimes(1)
  })
})
