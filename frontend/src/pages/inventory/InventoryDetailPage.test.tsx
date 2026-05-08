import { beforeEach, describe, expect, it, vi } from "vitest"
import { waitFor } from "@testing-library/react"
import userEvent from "@testing-library/user-event"
import { http, HttpResponse } from "msw"
import { render, screen } from "@/test/utils"
import { server } from "@/test/mocks/server"
import InventoryDetailPage from "./InventoryDetailPage"

vi.mock("react-router-dom", async () => {
  const actual = await vi.importActual<typeof import("react-router-dom")>(
    "react-router-dom"
  )
  return {
    ...actual,
    useParams: () => ({ id: "mat-123" }),
  }
})

const materialResponse = {
  id: "mat-123",
  category_id: "cat-1",
  name: "Betonschraube",
  description: "6x50",
  unit: "Stück",
  quantity: 48,
  min_quantity: 10,
  can_expire: false,
  legacy_quantity: 48,
  expired_quantity: 0,
  expiring_soon_quantity: 0,
  next_expiry_on: null,
  expiry_batches: [],
  location: "Regal A",
  qr_code: null,
  is_low_stock: false,
  created_at: "2026-04-30T10:00:00.000Z",
}

const apiPath = (path: string) => `*/api/v1${path}`

function createHistoryEntry(overrides: Record<string, unknown> = {}) {
  return {
    id: "h-1",
    material_id: "mat-123",
    user_id: "user-1",
    user_name: "Max Mustermann",
    entry_type: "withdrawn",
    quantity_change: -4,
    quantity_after: 44,
    notes: "Entnahme Schalung",
    site_id: "site-1",
    site_name: "Baustelle Müller",
    category_name: "Schrauben",
    created_at: "2026-04-30T11:30:00.000Z",
    ...overrides,
  }
}

beforeEach(() => {
  server.use(
    http.get(apiPath("/inventory/materials/mat-123"), () =>
      HttpResponse.json(materialResponse)
    ),
    http.get(apiPath("/inventory/materials/mat-123/history/enriched"), () =>
      HttpResponse.json([])
    ),
    http.get(apiPath("/preferences"), () =>
      HttpResponse.json({ active_site_id: null })
    ),
    http.get(apiPath("/sites"), () => HttpResponse.json([]))
  )
})

describe("InventoryDetailPage history", () => {
  it("renders material_added entries with badge, positive quantity, and attribution", async () => {
    server.use(
      http.get(apiPath("/inventory/materials/mat-123/history/enriched"), () =>
        HttpResponse.json([
          createHistoryEntry({
            entry_type: "material_added",
            quantity_change: 3,
            quantity_after: 51,
            notes: "Lieferung HolzLand",
            site_id: null,
            site_name: null,
          }),
        ])
      )
    )

    render(<InventoryDetailPage />)

    expect(await screen.findByText("Historie")).toBeInTheDocument()
    expect(await screen.findByText("Eingelagert")).toBeInTheDocument()
    expect(screen.getByText("+3")).toBeInTheDocument()
    expect(screen.getByText("von Max Mustermann")).toBeInTheDocument()
  })

  it("renders withdrawn entries with the red badge treatment, attribution, and Baustelle link", async () => {
    server.use(
      http.get(apiPath("/inventory/materials/mat-123/history/enriched"), () =>
        HttpResponse.json([createHistoryEntry()])
      )
    )

    render(<InventoryDetailPage />)

    expect(await screen.findByText("Historie")).toBeInTheDocument()
    const withdrawnBadge = await screen.findByText("Entnommen")
    expect(withdrawnBadge).toHaveClass("bg-destructive/10", "text-destructive", "border-destructive/20")
    expect(screen.getByText("von Max Mustermann")).toBeInTheDocument()
    const siteLink = screen.getByRole("link", { name: "Baustelle Müller" })
    expect(siteLink).toHaveAttribute("href", "/sites/site-1")
  })

  it("renders adjusted entries with the correction label", async () => {
    server.use(
      http.get(apiPath("/inventory/materials/mat-123/history/enriched"), () =>
        HttpResponse.json([
          createHistoryEntry({
            entry_type: "adjusted",
            quantity_change: 2,
            quantity_after: 50,
            site_id: null,
            site_name: null,
          }),
        ])
      )
    )

    render(<InventoryDetailPage />)

    expect(await screen.findByText("Bestand korrigiert")).toBeInTheDocument()
  })

  it("shows the enriched history empty state copy when no entries exist", async () => {
    render(<InventoryDetailPage />)

    expect(
      await screen.findByText("Noch keine Materialbewegungen")
    ).toBeInTheDocument()
    expect(
      screen.getByText(
        "Einlagerungen, Entnahmen und Korrekturen erscheinen hier, sobald die erste Änderung erfasst wurde."
      )
    ).toBeInTheDocument()
  })
})

describe("InventoryDetailPage interactions", () => {
  it("renders stock-in, withdraw, and edit actions in the expected order", async () => {
    render(<InventoryDetailPage />)

    await screen.findByText("Betonschraube")

    const detailsTitle = screen.getByText("Details")
    const actionsTitle = screen.getByText("Aktionen")

    expect(
      screen.getByRole("button", { name: /material bearbeiten/i })
    ).toBeInTheDocument()
    expect(
      screen.getByRole("button", { name: /material einlagern/i })
    ).toBeInTheDocument()
    expect(
      screen.getByRole("button", { name: /material entnehmen/i })
    ).toBeInTheDocument()

    expect(detailsTitle.parentElement).toContainElement(
      screen.getByRole("button", { name: /material bearbeiten/i })
    )
    expect(actionsTitle.parentElement?.parentElement).toContainElement(
      screen.getByRole("button", { name: /material einlagern/i })
    )
    expect(actionsTitle.parentElement?.parentElement).toContainElement(
      screen.getByRole("button", { name: /material entnehmen/i })
    )

    const withdrawButton = screen.getByRole("button", {
      name: /material entnehmen/i,
    })
    const stockInButton = screen.getByRole("button", {
      name: /material einlagern/i,
    })

    expect(
      withdrawButton.compareDocumentPosition(stockInButton) &
        Node.DOCUMENT_POSITION_FOLLOWING
    ).toBeTruthy()
  })

  it("submits stock-in with quantity and optional notes, then shows the success toast", async () => {
    const user = userEvent.setup()
    let stockInPayload: unknown = null

    server.use(
      http.get(apiPath("/inventory/materials/mat-123/history/enriched"), () =>
        HttpResponse.json([])
      ),
      http.post(apiPath("/inventory/materials/mat-123/stock-in"), async ({ request }) => {
        stockInPayload = await request.json()
        return HttpResponse.json({ ...materialResponse, quantity: 51 })
      })
    )

    render(<InventoryDetailPage />)

    await user.click(await screen.findByRole("button", { name: /material einlagern/i }))

    expect(await screen.findByText("Aktueller Bestand")).toBeInTheDocument()

    await user.clear(screen.getByLabelText(/^menge$/i))
    await user.type(screen.getByLabelText(/^menge$/i), "3")
    await user.type(screen.getByLabelText(/notizen/i), "Lieferung HolzLand")
    await user.click(screen.getByRole("button", { name: /einlagern$/i }))

    await waitFor(() => {
      expect(stockInPayload).toEqual({ quantity: 3, notes: "Lieferung HolzLand", expires_on: null })
    })

    expect(await screen.findByText("3 Stück eingelagert")).toBeInTheDocument()
  })

  it("defaults withdrawals to the active project and sends the linked project id", async () => {
    const user = userEvent.setup()
    let withdrawPayload: unknown = null

    server.use(
      http.get(apiPath("/preferences"), () =>
        HttpResponse.json({ active_site_id: "site-9" })
      ),
      http.get(apiPath("/sites"), () =>
        HttpResponse.json([
          {
            id: "site-9",
            name: "Werkstattauftrag",
            project_type: "internal_workshop",
          },
        ])
      ),
      http.post(apiPath("/inventory/materials/mat-123/withdraw"), async ({ request }) => {
        withdrawPayload = await request.json()
        return HttpResponse.json({ ...materialResponse, quantity: 47 })
      })
    )

    render(<InventoryDetailPage />)

    await user.click(await screen.findByRole("button", { name: /material entnehmen/i }))

    expect(
      await screen.findByText("Reale Materialentnahmen werden immer einem Projekt zugeordnet.")
    ).toBeInTheDocument()
    expect(screen.getByRole("combobox")).toHaveValue("site-9")

    await user.click(screen.getByRole("button", { name: /1 Stück entnehmen/i }))

    await waitFor(() => {
      expect(withdrawPayload).toEqual({
        quantity: 1,
        notes: null,
        site_id: "site-9",
        disposal: false,
      })
    })
  })

  it("submits edit changes and translates target stock into an adjust delta", async () => {
    const user = userEvent.setup()
    let updatePayload: unknown = null
    let adjustPayload: unknown = null

    server.use(
      http.get(apiPath("/inventory/materials/mat-123/history/enriched"), () =>
        HttpResponse.json([])
      ),
      http.patch(apiPath("/inventory/materials/mat-123"), async ({ request }) => {
        updatePayload = await request.json()
        return HttpResponse.json({
          ...materialResponse,
          location: "Regal B2",
          min_quantity: 14,
        })
      }),
      http.post(apiPath("/inventory/materials/mat-123/adjust"), async ({ request }) => {
        adjustPayload = await request.json()
        return HttpResponse.json({ ...materialResponse, quantity: 60 })
      })
    )

    render(<InventoryDetailPage />)

    await user.click(await screen.findByRole("button", { name: /material bearbeiten/i }))

    expect(await screen.findByText("Setzt den verfügbaren Bestand direkt auf diesen Wert.")).toBeInTheDocument()

    await user.clear(screen.getByLabelText(/lagerort/i))
    await user.type(screen.getByLabelText(/lagerort/i), "Regal B2")
    await user.clear(screen.getByLabelText(/mindestbestand/i))
    await user.type(screen.getByLabelText(/mindestbestand/i), "14")
    await user.clear(screen.getByLabelText(/bestand korrigieren/i))
    await user.type(screen.getByLabelText(/bestand korrigieren/i), "60")
    await user.click(screen.getByRole("button", { name: /änderungen speichern/i }))

    await waitFor(() => {
      expect(updatePayload).toEqual({ location: "Regal B2", min_quantity: 14 })
      expect(adjustPayload).toEqual({
        quantity: 12,
        reason: "Bestandskorrektur über Materialdialog",
      })
    })

    expect(await screen.findByText("Material aktualisiert")).toBeInTheDocument()
  })
})
