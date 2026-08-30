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
    mockData.billingSettings = {
      default_hourly_rate_cents: 8500,
      billing_tax_mode: 'standard',
      sender_name: 'Schreinerei Mustermann',
      sender_address: 'Werkstrasse 1\n12345 Musterstadt',
    }
    mockData.testDataInstalled = false
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

  it("shows test data controls to administrators", async () => {
    render(<SettingsPage />)

    expect(await screen.findByText("Testdaten")).toBeInTheDocument()
    expect(screen.getByRole("button", { name: /testdaten importieren/i })).toBeInTheDocument()
  })

  it("hides test data controls from regular users", () => {
    useAuthStore.setState((state) => ({
      ...state,
      user: state.user ? { ...state.user, role: "mitarbeiter" } : null,
    }))

    render(<SettingsPage />)

    expect(screen.queryByText("Testdaten")).not.toBeInTheDocument()
  })

  it("imports test data", async () => {
    const user = userEvent.setup()
    render(<SettingsPage />)

    await user.click(await screen.findByRole("button", { name: /testdaten importieren/i }))

    await waitFor(() => expect(mockData.testDataInstalled).toBe(true))
    expect(await screen.findByText("Testdaten wurden importiert")).toBeInTheDocument()
  })

  it("confirms before removing test data", async () => {
    mockData.testDataInstalled = true
    const user = userEvent.setup()
    render(<SettingsPage />)
    const removeButton = await screen.findByRole("button", { name: /testdaten entfernen/i })
    await waitFor(() => expect(removeButton).toBeEnabled())

    await user.click(removeButton)
    expect(screen.getByRole("alertdialog")).toBeInTheDocument()
    await user.click(screen.getByRole("button", { name: "Entfernen" }))

    await waitFor(() => expect(mockData.testDataInstalled).toBe(false))
    expect(await screen.findByText("Testdaten wurden entfernt")).toBeInTheDocument()
  })

  it("updates the billing settings", async () => {
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
    await user.click(screen.getByRole("combobox", { name: /umsatzsteuer/i }))
    await user.click(screen.getByRole("option", { name: /kleinunternehmer/i }))
    await user.clear(screen.getByLabelText(/absendername/i))
    await user.type(screen.getByLabelText(/absendername/i), "Schreinerei Neu")
    await user.clear(screen.getByLabelText(/absenderadresse/i))
    await user.type(screen.getByLabelText(/absenderadresse/i), "Holzweg 5\n12345 Berlin")
    await user.click(screen.getByRole("button", { name: /speichern/i }))

    await waitFor(() => {
      expect(mockData.billingSettings.default_hourly_rate_cents).toBe(9250)
      expect(mockData.billingSettings.billing_tax_mode).toBe("kleinunternehmer")
      expect(mockData.billingSettings.sender_name).toBe("Schreinerei Neu")
      expect(mockData.billingSettings.sender_address).toBe("Holzweg 5\n12345 Berlin")
    })
    expect(await screen.findByText("Abrechnungseinstellungen gespeichert")).toBeInTheDocument()
  })
})
