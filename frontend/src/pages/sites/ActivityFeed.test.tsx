import { beforeEach, describe, expect, it, vi } from "vitest"
import { render, screen } from "@/test/utils"
import userEvent from "@testing-library/user-event"
import { ActivityFeed } from "./ActivityFeed"
import { waitFor } from "@testing-library/react"

vi.mock("@/lib/api/hooks", () => ({
  useSiteMaterialHistory: vi.fn(),
}))

vi.mock("@/lib/api/client", () => ({
  apiClient: {
    getBlob: vi.fn(),
  },
}))

import { useSiteMaterialHistory } from "@/lib/api/hooks"
import { apiClient } from "@/lib/api/client"

const getBlobMock = vi.mocked(apiClient.getBlob)

const defaultMaterialHistoryMock = {
  data: [],
  isLoading: false,
} as never

beforeEach(() => {
  getBlobMock.mockReset()
})

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

describe("ActivityFeed photo preview", () => {
  it("loads protected attachment photo via authenticated blob fetch", async () => {
    vi.mocked(useSiteMaterialHistory).mockReturnValue(defaultMaterialHistoryMock)
    getBlobMock.mockResolvedValue(new Blob(["image-bytes"], { type: "image/jpeg" }))

    const createObjectURLSpy = vi
      .spyOn(URL, "createObjectURL")
      .mockReturnValue("blob:secure-preview")
    const revokeObjectURLSpy = vi
      .spyOn(URL, "revokeObjectURL")
      .mockImplementation(() => undefined)

    const { unmount } = render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            id: "activity-1",
            site_id: "site-1",
            user_id: "user-1",
            activity_type: "photo",
            content: null,
            photo_url: "/api/v1/attachments/abc",
            created_at: "2026-05-01T10:00:00.000Z",
          },
        ]}
      />
    )

    await waitFor(() => {
      expect(getBlobMock).toHaveBeenCalledWith("/api/v1/attachments/abc")
    })

    const image = await screen.findByAltText("Aktivitätsfoto")
    expect(image).toHaveAttribute("src", "blob:secure-preview")

    unmount()
    expect(revokeObjectURLSpy).toHaveBeenCalledWith("blob:secure-preview")

    createObjectURLSpy.mockRestore()
    revokeObjectURLSpy.mockRestore()
  })

  it("renders preview image when photo_url exists", async () => {
    vi.mocked(useSiteMaterialHistory).mockReturnValue(defaultMaterialHistoryMock)
    getBlobMock.mockReset()

    render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            id: "activity-1",
            site_id: "site-1",
            user_id: "user-1",
            activity_type: "photo",
            content: null,
            photo_url: "https://example.com/photo.jpg",
            created_at: "2026-05-01T10:00:00.000Z",
          },
        ]}
      />
    )

    const image = await screen.findByAltText("Aktivitätsfoto")
    expect(image).toHaveAttribute("src", "https://example.com/photo.jpg")
    expect(getBlobMock).not.toHaveBeenCalled()
  })

  it("does not render image when photo_url is missing", () => {
    vi.mocked(useSiteMaterialHistory).mockReturnValue(defaultMaterialHistoryMock)

    render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            id: "activity-2",
            site_id: "site-1",
            user_id: "user-1",
            activity_type: "photo",
            content: null,
            photo_url: null,
            created_at: "2026-05-01T10:00:00.000Z",
          },
        ]}
      />
    )

    expect(screen.queryByAltText("Aktivitätsfoto")).not.toBeInTheDocument()
  })
})
