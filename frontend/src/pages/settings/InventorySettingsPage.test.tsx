import { beforeEach, describe, expect, it, vi } from "vitest"
import { screen, waitFor } from "@testing-library/react"
import userEvent from "@testing-library/user-event"
import { http, HttpResponse } from "msw"
import { toast } from "sonner"
import { render } from "@/test/utils"
import { server } from "@/test/mocks/server"
import { mockData } from "@/test/mocks/handlers"
import { createCategory } from "@/test/factories/category"
import InventorySettingsPage from "./InventorySettingsPage"

vi.mock("sonner", async () => {
  const actual = await vi.importActual<typeof import("sonner")>("sonner")
  return {
    ...actual,
    toast: {
      ...actual.toast,
      error: vi.fn(),
    },
  }
})

describe("InventorySettingsPage", () => {
  const categories = [
    createCategory({
      id: "cat-1",
      name: "Holz",
      description: "Massivholz und Leisten",
    }),
    createCategory({
      id: "cat-2",
      name: "Beschläge",
      description: "Scharniere und Verbinder",
    }),
  ]

  beforeEach(() => {
    vi.clearAllMocks()
    mockData.categories = structuredClone(categories)
  })

  it("renders the settings title, category rows, and the empty state copy", async () => {
    mockData.categories = []

    render(<InventorySettingsPage />)

    expect(await screen.findByText("Inventar-Einstellungen")).toBeInTheDocument()
    expect(await screen.findByText("Noch keine Kategorien")).toBeInTheDocument()
    expect(
      screen.getByText(
        "Legen Sie die erste Kategorie an, damit Materialien sauber zugeordnet werden können."
      )
    ).toBeInTheDocument()
  })

  it("edits a category and shows the updated values after success", async () => {
    const user = userEvent.setup()

    server.use(
      http.patch("*/api/v1/inventory/categories/:id", async ({ params, request }) => {
        const payload = (await request.json()) as {
          name?: string
          description?: string
        }

        mockData.categories = mockData.categories.map((category) =>
          category.id === params.id
            ? { ...category, name: payload.name, description: payload.description ?? null }
            : category
        )

        return HttpResponse.json(
          mockData.categories.find((category) => category.id === params.id),
          { status: 200 }
        )
      })
    )

    render(<InventorySettingsPage />)

    expect(await screen.findByText("Holz")).toBeInTheDocument()

    await user.click(screen.getByRole("button", { name: /holz bearbeiten/i }))
    expect(
      screen.getByText(
        "Nur fur verderbliche Kategorien aktivieren. Neue Zugange brauchen dann ein MHD, vorhandener Bestand ohne MHD bleibt sichtbar."
      )
    ).toBeInTheDocument()
    await user.clear(screen.getByLabelText(/^name$/i))
    await user.type(screen.getByLabelText(/^name$/i), "Plattenwerkstoffe")
    await user.clear(screen.getByLabelText(/^beschreibung$/i))
    await user.type(screen.getByLabelText(/^beschreibung$/i), "MDF und Multiplex")
    await user.click(screen.getByRole("button", { name: /änderungen speichern/i }))

    await waitFor(() => {
      expect(screen.getByText("Plattenwerkstoffe")).toBeInTheDocument()
    })

    expect(screen.getByText("MDF und Multiplex")).toBeInTheDocument()
  })

  it("keeps the edit dialog open and shows the expiry-toggle validation copy on conflict", async () => {
    const user = userEvent.setup()

    server.use(
      http.patch("*/api/v1/inventory/categories/:id", () =>
        HttpResponse.json(
          {
            message:
              "Expiry tracking can only be disabled after all live stock in this category is depleted",
          },
          { status: 409 }
        )
      )
    )

    mockData.categories = [
      createCategory({
        id: "cat-3",
        name: "Lacke",
        description: "Mit Mindesthaltbarkeit",
        can_expire: true,
      }),
    ]

    render(<InventorySettingsPage />)

    expect(await screen.findByText("Lacke")).toBeInTheDocument()

    await user.click(screen.getByRole("button", { name: /lacke bearbeiten/i }))
    await user.click(screen.getByLabelText(/mhd/i))
    await user.click(screen.getByRole("button", { name: /änderungen speichern/i }))

    await waitFor(() => {
      expect(toast.error).toHaveBeenCalledWith(
        "MHD kann erst deaktiviert werden, wenn in dieser Kategorie kein laufender Bestand mehr liegt."
      )
    })

    expect(screen.getByRole("button", { name: /änderungen speichern/i })).toBeInTheDocument()
  })

  it("keeps the category row visible and shows the blocked delete copy on conflict", async () => {
    const user = userEvent.setup()

    server.use(
      http.delete("*/api/v1/inventory/categories/:id", () =>
        HttpResponse.json(
          {
            message: "Cannot delete category: material history must be preserved",
          },
          { status: 409 }
        )
      )
    )

    render(<InventorySettingsPage />)

    expect(await screen.findByText("Holz")).toBeInTheDocument()

    await user.click(screen.getByRole("button", { name: /holz löschen/i }))
    expect(
      await screen.findByText(
        "Möchten Sie diese Kategorie wirklich löschen? Diese Aktion kann nicht rückgängig gemacht werden."
      )
    ).toBeInTheDocument()

    await user.click(screen.getByRole("button", { name: /^löschen$/i }))

    expect(
      await screen.findByText(
        "Kategorie konnte nicht gelöscht werden. Entfernen oder verschieben Sie zuerst alle Materialien dieser Kategorie und versuchen Sie es dann erneut."
      )
    ).toBeInTheDocument()
    expect(screen.getByText("Holz")).toBeInTheDocument()
  })
})
