import { beforeEach, describe, expect, it } from "vitest"
import { screen, within } from "@testing-library/react"
import { render } from "@/test/utils"
import { mockData } from "@/test/mocks/handlers"
import { createCategory } from "@/test/factories/category"
import { createMaterial } from "@/test/factories/material"
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
      }),
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
})
