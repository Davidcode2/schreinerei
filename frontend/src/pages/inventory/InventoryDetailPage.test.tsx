import { beforeEach, describe, expect, it, vi } from "vitest"
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
