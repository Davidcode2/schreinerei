import { beforeEach, describe, expect, it } from "vitest"
import { screen } from "@testing-library/react"
import { render } from "@/test/utils"
import { mockData } from "@/test/mocks/handlers"
import { createSite } from "@/test/factories/site"
import { setAuthRole } from "@/test/auth"
import SitesListPage from "./SitesListPage"

describe("SitesListPage role visibility", () => {
  beforeEach(() => {
    mockData.sites = [
      createSite({ name: "Dach Neudeckung" }) as unknown as Record<string, unknown>,
    ]
    mockData.preferences = { active_site_id: null }
  })

  it("shows create and reporting actions to admins", async () => {
    setAuthRole("admin")

    render(<SitesListPage />)

    expect(
      await screen.findByRole("button", { name: /projekt anlegen/i })
    ).toBeInTheDocument()
    expect(
      screen.getByRole("button", { name: /historische auswertung/i })
    ).toBeInTheDocument()
  })

  it("hides create and reporting actions from mitarbeiter users", async () => {
    setAuthRole("mitarbeiter")

    render(<SitesListPage />)

    expect(await screen.findByText("Dach Neudeckung")).toBeInTheDocument()
    expect(
      screen.queryByRole("button", { name: /projekt anlegen/i })
    ).not.toBeInTheDocument()
    expect(
      screen.queryByRole("button", { name: /historische auswertung/i })
    ).not.toBeInTheDocument()
  })

  it("hides the project delete button from mitarbeiter users", async () => {
    setAuthRole("mitarbeiter")

    const { container } = render(<SitesListPage />)

    await screen.findByText("Dach Neudeckung")
    expect(container.querySelector(".lucide-trash2")).not.toBeInTheDocument()
  })

  it("shows the project delete button to admins", async () => {
    setAuthRole("admin")

    const { container } = render(<SitesListPage />)

    await screen.findByText("Dach Neudeckung")
    expect(container.querySelector(".lucide-trash2")).toBeInTheDocument()
  })
})
