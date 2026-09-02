import { beforeEach, describe, expect, it } from "vitest"
import { screen, within } from "@testing-library/react"
import { render } from "@/test/utils"
import { mockData } from "@/test/mocks/handlers"
import { createCategory } from "@/test/factories/category"
import { createMaterial } from "@/test/factories/material"
import { setAuthRole } from "@/test/auth"
import InventoryListPage from "./InventoryListPage"

describe("InventoryListPage", () => {
  beforeEach(() => {
    const category = createCategory({
      id: "cat-1",
      name: "Holzwerkstoffe",
      description: "Platten und Leisten",
    })

    mockData.categories = [category]
    mockData.materials = [
      createMaterial({
        id: "mat-1",
        category_id: category.id,
        name: "Multiplexplatte",
        description: "Birke 18 mm",
        location: "Regal A1",
      }) as unknown as Record<string, unknown>,
    ]
  })

  it("shows the settings gear entrypoint in the inventory header", async () => {
    render(<InventoryListPage />)

    const settingsButton = await screen.findByRole("button", {
      name: /inventar-einstellungen öffnen/i,
    })

    expect(settingsButton).toBeInTheDocument()
  })

  it("renders the category name directly under the material name", async () => {
    render(<InventoryListPage />)

    const materialLink = await screen.findByRole("link", { name: /multiplexplatte/i })

    expect(within(materialLink).getByText("Holzwerkstoffe")).toBeInTheDocument()
    expect(within(materialLink).getByText("Birke 18 mm")).toBeInTheDocument()
  })

  it("surfaces expiry alerts above the material list", async () => {
    mockData.materials = [
      createMaterial({
        id: "mat-2",
        name: "Lack weiss",
        can_expire: true,
        legacy_quantity: 0,
        expired_quantity: 3,
        expiring_soon_quantity: 0,
      }) as unknown as Record<string, unknown>,
    ]

    render(<InventoryListPage />)

    expect(await screen.findByText("Ablaufwarnungen")).toBeInTheDocument()
    expect(screen.getByText(/3 Stück abgelaufen/i)).toBeInTheDocument()
  })

  it("renders Quadratmeter as m² in the inventory amount chip", async () => {
    mockData.materials = [
      createMaterial({
        id: "mat-3",
        name: "Sperrholzplatte",
        unit: "Quadratmeter",
        quantity: 12,
        min_quantity: 4,
      }) as unknown as Record<string, unknown>,
    ]

    render(<InventoryListPage />)

    expect(await screen.findByText("12 m²")).toBeInTheDocument()
    expect(screen.getByText("Min: 4 m²")).toBeInTheDocument()
  })

  it("hides the material create action from mitarbeiter users but keeps the gear", async () => {
    setAuthRole("mitarbeiter")

    render(<InventoryListPage />)

    expect(await screen.findByText("Multiplexplatte")).toBeInTheDocument()
    expect(
      screen.queryByRole("button", { name: /material hinzufügen/i })
    ).not.toBeInTheDocument()
    expect(
      screen.getByRole("button", { name: /inventar-einstellungen öffnen/i })
    ).toBeInTheDocument()
  })

  it("shows the material create action to admins", async () => {
    setAuthRole("admin")

    render(<InventoryListPage />)

    expect(
      await screen.findByRole("button", { name: /material hinzufügen/i })
    ).toBeInTheDocument()
  })
})
