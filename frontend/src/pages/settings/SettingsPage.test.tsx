import { beforeEach, describe, expect, it } from "vitest"
import userEvent from "@testing-library/user-event"
import { render, screen, waitFor } from "@/test/utils"
import { useAuthStore } from "@/lib/auth/authStore"
import { mockData } from "@/test/mocks/handlers"
import SettingsPage from "./SettingsPage"

describe("SettingsPage", () => {
  beforeEach(() => {
    useAuthStore.setState({
      user: {
        id: "user-1",
        tenant_id: "tenant-1",
        email: "admin@example.com",
        name: "Admin",
        role: "admin",
        created_at: new Date().toISOString(),
      },
      tokens: null,
      isAuthenticated: true,
      isLoading: false,
    })
    mockData.billingSettings = { default_hourly_rate_cents: 8500 }
  })

  it("renders the default hourly rate section for admins", async () => {
    render(<SettingsPage />)

    expect(await screen.findByText("Abrechnung")).toBeInTheDocument()
    const input = await screen.findByRole("spinbutton", {
      name: /standard-stundensatz/i,
    })
    await waitFor(() => {
      expect(input).toHaveValue(85)
    })
  })

  it("updates the default hourly rate", async () => {
    const user = userEvent.setup()
    render(<SettingsPage />)

    const input = await screen.findByRole("spinbutton", {
      name: /standard-stundensatz/i,
    })
    await waitFor(() => {
      expect(input).toHaveValue(85)
    })
    await user.clear(input)
    await user.type(input, "92.50")
    await user.click(screen.getByRole("button", { name: /speichern/i }))

    await waitFor(() => {
      expect(mockData.billingSettings.default_hourly_rate_cents).toBe(9250)
    })
    expect(await screen.findByText("Standard-Stundensatz gespeichert")).toBeInTheDocument()
  })
})
