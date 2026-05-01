import { describe, expect, it, vi } from "vitest"
import { render, screen } from "@/test/utils"
import userEvent from "@testing-library/user-event"
import { ActivityFeed } from "./ActivityFeed"

vi.mock("@/lib/api/hooks", () => ({
  useSiteMaterialHistory: vi.fn(),
}))

import { useSiteMaterialHistory } from "@/lib/api/hooks"

describe("ActivityFeed material tab", () => {
  it("renders enriched material history row and site link", async () => {
    vi.mocked(useSiteMaterialHistory).mockReturnValue({
      data: [
        {
          id: "entry-1",
          material_id: "mat-1",
          material_name: "Betonschraube",
          category_name: "Befestigung",
          quantity_change: -3,
          quantity_after: 17,
          notes: null,
          site_id: "site-1",
          site_name: "Baustelle Nord",
          extracted_by: "Max Mustermann",
          created_at: "2026-05-01T10:00:00.000Z",
        },
      ],
      isLoading: false,
    } as never)

    render(<ActivityFeed activities={[]} siteId="site-1" />)
    await userEvent.click(screen.getByRole("tab", { name: "Material" }))

    expect(await screen.findByText("Betonschraube")).toBeInTheDocument()
    expect(screen.getByText("Kategorie: Befestigung")).toBeInTheDocument()
    expect(screen.getByText("Entnommen von: Max Mustermann")).toBeInTheDocument()
    expect(screen.getByRole("link", { name: "Baustelle Nord" })).toHaveAttribute(
      "href",
      "/sites/site-1"
    )
  })

  it("renders empty state when no material entries exist", async () => {
    vi.mocked(useSiteMaterialHistory).mockReturnValue({
      data: [],
      isLoading: false,
    } as never)

    render(<ActivityFeed activities={[]} siteId="site-1" />)
    await userEvent.click(screen.getByRole("tab", { name: "Material" }))

    expect(
      await screen.findByText("Noch keine Materialentnahmen für diese Baustelle")
    ).toBeInTheDocument()
  })
})
