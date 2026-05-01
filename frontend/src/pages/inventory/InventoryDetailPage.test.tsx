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
  location: "Regal A",
  qr_code: null,
  is_low_stock: false,
  created_at: "2026-04-30T10:00:00.000Z",
}

beforeEach(() => {
  server.use(
    http.get("/api/v1/inventory/materials/mat-123", () =>
      HttpResponse.json(materialResponse)
    ),
    http.get("/api/v1/preferences", () =>
      HttpResponse.json({ active_site_id: null })
    ),
    http.get("/api/v1/sites", () => HttpResponse.json([]))
  )
})

describe("InventoryDetailPage history", () => {
  it("renders site_name for linked deduction entries", async () => {
    server.use(
      http.get("/api/v1/inventory/materials/mat-123/history", () =>
        HttpResponse.json([
          {
            id: "h-1",
            quantity_change: -4,
            quantity_after: 44,
            notes: "Entnahme Schalung",
            site_id: "site-1",
            site_name: "Baustelle Müller",
            created_at: "2026-04-30T11:30:00.000Z",
          },
        ])
      )
    )

    render(<InventoryDetailPage />)

    expect(await screen.findByText("Historie")).toBeInTheDocument()
    expect(await screen.findByText("Baustelle Müller")).toBeInTheDocument()
  })

  it("does not render site label when site_name is null", async () => {
    server.use(
      http.get("/api/v1/inventory/materials/mat-123/history", () =>
        HttpResponse.json([
          {
            id: "h-2",
            quantity_change: -2,
            quantity_after: 46,
            notes: "Lagerentnahme",
            site_id: null,
            site_name: null,
            created_at: "2026-04-30T12:00:00.000Z",
          },
        ])
      )
    )

    render(<InventoryDetailPage />)

    expect(await screen.findByText("Historie")).toBeInTheDocument()
    expect(screen.queryByText("Baustelle Müller")).not.toBeInTheDocument()
  })

  it("shows empty state copy when no history entries exist", async () => {
    server.use(
      http.get("/api/v1/inventory/materials/mat-123/history", () =>
        HttpResponse.json([])
      )
    )

    render(<InventoryDetailPage />)

    expect(await screen.findByText("Noch keine Entnahmen erfasst")).toBeInTheDocument()
  })
})

describe("InventoryDetailPage interactions", () => {
  it("renders stock-in, withdraw, and edit actions in the expected order", async () => {
    render(<InventoryDetailPage />)

    await screen.findByText("Betonschraube")

    const buttonLabels = screen
      .getAllByRole("button")
      .map((button) => button.textContent?.trim())
      .filter(Boolean)

    const stockInIndex = buttonLabels.indexOf("Material einlagern")
    const withdrawIndex = buttonLabels.indexOf("Material entnehmen")
    const editIndex = buttonLabels.indexOf("Material bearbeiten")

    expect(stockInIndex).toBeGreaterThanOrEqual(0)
    expect(withdrawIndex).toBeGreaterThan(stockInIndex)
    expect(editIndex).toBeGreaterThan(withdrawIndex)
  })

  it("submits stock-in with quantity and optional notes, then shows the success toast", async () => {
    const user = userEvent.setup()
    let stockInPayload: unknown = null

    server.use(
      http.get("/api/v1/inventory/materials/mat-123/history", () =>
        HttpResponse.json([])
      ),
      http.post("/api/v1/inventory/materials/mat-123/stock-in", async ({ request }) => {
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
      expect(stockInPayload).toEqual({ quantity: 3, notes: "Lieferung HolzLand" })
    })

    expect(await screen.findByText("3 Stück eingelagert")).toBeInTheDocument()
  })

  it("submits edit changes and translates target stock into an adjust delta", async () => {
    const user = userEvent.setup()
    let updatePayload: unknown = null
    let adjustPayload: unknown = null

    server.use(
      http.get("/api/v1/inventory/materials/mat-123/history", () =>
        HttpResponse.json([])
      ),
      http.patch("/api/v1/inventory/materials/mat-123", async ({ request }) => {
        updatePayload = await request.json()
        return HttpResponse.json({
          ...materialResponse,
          location: "Regal B2",
          min_quantity: 14,
        })
      }),
      http.post("/api/v1/inventory/materials/mat-123/adjust", async ({ request }) => {
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
