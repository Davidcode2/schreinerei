import { beforeEach, describe, expect, it, vi } from "vitest"
import { render, screen, waitFor } from "@/test/utils"
import userEvent from "@testing-library/user-event"
import { ActivityFeed } from "./ActivityFeed"

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

const baseActivity = {
  site_id: "site-1",
  user_id: "user-1",
  activity_type: "note" as const,
  photo_url: null,
  created_at: "2026-05-01T10:00:00.000Z",
  attachments: [],
}

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
})

describe("ActivityFeed document entries", () => {
  beforeEach(() => {
    vi.mocked(useSiteMaterialHistory).mockReturnValue(defaultMaterialHistoryMock)
  })

  it("renders document heading, note text, and attachment tiles", async () => {
    getBlobMock.mockResolvedValue(new Blob(["image-bytes"], { type: "image/jpeg" }))
    vi.spyOn(URL, "createObjectURL").mockReturnValue("blob:doc-preview")
    vi.spyOn(URL, "revokeObjectURL").mockImplementation(() => undefined)

    render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            ...baseActivity,
            id: "activity-1",
            content: "Montage abgeschlossen",
            attachments: [
              {
                attachment_id: "att-1",
                filename: "planung.jpg",
                mime_type: "image/jpeg",
                url: "/api/v1/attachments/att-1",
                thumbnail_url: "/api/v1/attachments/att-1/thumbnail",
              },
            ],
          },
        ]}
      />
    )

    expect(screen.getByText("Dokument hinzugefügt")).toBeInTheDocument()
    expect(screen.getByText("Montage abgeschlossen")).toBeInTheDocument()

    await waitFor(() => {
      expect(getBlobMock).toHaveBeenCalledWith("/api/v1/attachments/att-1")
    })

    expect(await screen.findByAltText("planung.jpg")).toBeInTheDocument()
  })

  it("renders PDF cards with visible PDF labeling", () => {
    render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            ...baseActivity,
            id: "activity-2",
            content: null,
            attachments: [
              {
                attachment_id: "att-2",
                filename: "angebot.pdf",
                mime_type: "application/pdf",
                url: "/api/v1/attachments/att-2",
                thumbnail_url: null,
              },
            ],
          },
        ]}
      />
    )

    expect(screen.getByText("PDF")).toBeInTheDocument()
    expect(screen.getByText("angebot.pdf")).toBeInTheDocument()
  })

  it("renders a fallback shell when protected previews fail", async () => {
    getBlobMock.mockRejectedValue(new Error("preview failed"))

    render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            ...baseActivity,
            id: "activity-3",
            content: null,
            attachments: [
              {
                attachment_id: "att-3",
                filename: "planung.jpg",
                mime_type: "image/jpeg",
                url: "/api/v1/attachments/att-3",
                thumbnail_url: "/api/v1/attachments/att-3/thumbnail",
              },
            ],
          },
        ]}
      />
    )

    expect(await screen.findByText("Vorschau nicht verfügbar")).toBeInTheDocument()
    expect(screen.getByText("planung.jpg")).toBeInTheDocument()
  })
})
