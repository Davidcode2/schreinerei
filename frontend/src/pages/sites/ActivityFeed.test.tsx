import { beforeEach, describe, expect, it, vi } from "vitest"
import { render, screen, waitFor } from "@/test/utils"
import userEvent from "@testing-library/user-event"
import { ActivityFeed } from "./ActivityFeed"
import { toast } from "sonner"

vi.mock("@/lib/api/hooks", () => ({
  useSiteMaterialHistory: vi.fn(),
}))

const mutateAsyncMock = vi.fn()
const deleteMutationState = {
  isPending: false,
}

vi.mock("@/lib/api/hooks/useSites", () => ({
  useDeleteActivity: vi.fn(() => ({
    mutateAsync: mutateAsyncMock,
    get isPending() {
      return deleteMutationState.isPending
    },
  })),
}))

vi.mock("@/lib/api/client", () => ({
  apiClient: {
    getBlob: vi.fn(),
  },
}))

vi.mock("sonner", async (importOriginal) => {
  const actual = await importOriginal<typeof import("sonner")>()

  return {
    ...actual,
    toast: {
      ...actual.toast,
      success: vi.fn(),
      error: vi.fn(),
    },
  }
})

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
  creator_name: "Anna Tischler",
  activity_type: "note" as const,
  can_delete: false,
  photo_url: null,
  created_at: "2026-05-01T10:00:00.000Z",
  attachments: [],
}

beforeEach(() => {
  getBlobMock.mockReset()
  mutateAsyncMock.mockReset()
  deleteMutationState.isPending = false
  vi.mocked(toast.success).mockReset()
  vi.mocked(toast.error).mockReset()
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

  it("renders image attachments as clickable viewer links", async () => {
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
    expect(screen.getByRole("link", { name: "Medium öffnen: planung.jpg" })).toHaveAttribute(
      "href",
      "/sites/site-1/media/activity-1/att-1/planung-jpg"
    )
  })

  it("renders PDF cards as clickable viewer links", () => {
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
    expect(screen.getByRole("link", { name: "Medium öffnen: angebot.pdf" })).toHaveAttribute(
      "href",
      "/sites/site-1/media/activity-2/att-2/angebot-pdf"
    )
  })

  it("keeps fallback tiles clickable when protected previews fail", async () => {
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
    expect(screen.getByRole("link", { name: "Medium öffnen: planung.jpg" })).toHaveAttribute(
      "href",
      "/sites/site-1/media/activity-3/att-3/planung-jpg"
    )
  })

  it("routes legacy photo activities into the viewer", async () => {
    getBlobMock.mockResolvedValue(new Blob(["image-bytes"], { type: "image/jpeg" }))
    vi.spyOn(URL, "createObjectURL").mockReturnValue("blob:legacy-photo")
    vi.spyOn(URL, "revokeObjectURL").mockImplementation(() => undefined)

    render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            ...baseActivity,
            id: "activity-4",
            activity_type: "photo",
            content: null,
            photo_url: "/api/v1/attachments/legacy-photo-id",
          },
        ]}
      />
    )

    expect(await screen.findByAltText("Aktivitätsfoto")).toBeInTheDocument()
    expect(screen.getByRole("link", { name: "Medium öffnen: Aktivitätsfoto" })).toHaveAttribute(
      "href",
      "/sites/site-1/media/activity-4/legacy-photo-id/aktivitatsfoto"
    )
  })

  it("keeps status-change entries with attachments non-interactive", () => {
    render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            ...baseActivity,
            id: "activity-status-attachment",
            activity_type: "status_change",
            content: '{"old_status":"active","new_status":"completed"}',
            attachments: [
              {
                attachment_id: "att-status",
                filename: "sollte-nicht-klickbar.pdf",
                mime_type: "application/pdf",
                url: "/api/v1/attachments/att-status",
                thumbnail_url: null,
              },
            ],
          },
        ]}
      />
    )

    expect(screen.queryByRole("link", { name: "Medium öffnen: sollte-nicht-klickbar.pdf" })).not.toBeInTheDocument()
    expect(screen.queryByText("PDF")).not.toBeInTheDocument()
  })

  it("shows delete actions only for deletable activities", () => {
    render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            ...baseActivity,
            id: "activity-5",
            can_delete: true,
            content: "Kann gelöscht werden",
          },
          {
            ...baseActivity,
            id: "activity-6",
            can_delete: false,
            content: "Nicht löschbar",
          },
          {
            ...baseActivity,
            id: "activity-7",
            activity_type: "status_change",
            can_delete: false,
            content: '{"old_status":"planned","new_status":"active"}',
          },
        ]}
      />
    )

    expect(screen.getByLabelText("Eintrag löschen: activity-5")).toBeInTheDocument()
    expect(screen.queryByLabelText("Eintrag löschen: activity-6")).not.toBeInTheDocument()
    expect(screen.queryByLabelText("Eintrag löschen: activity-7")).not.toBeInTheDocument()
  })

  it("renders status changes without raw JSON fallback text", () => {
    render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            ...baseActivity,
            id: "activity-status",
            activity_type: "status_change",
            content: '{"old_status":"active","new_status":"completed"}',
          },
        ]}
      />
    )

    expect(screen.getByText("Aktiv → Abgeschlossen")).toBeInTheDocument()
    expect(
      screen.queryByText('{"old_status":"active","new_status":"completed"}')
    ).not.toBeInTheDocument()
  })

  it("confirms before deleting and closes after success", async () => {
    mutateAsyncMock.mockResolvedValue(undefined)

    render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            ...baseActivity,
            id: "activity-8",
            can_delete: true,
            content: "Montage abgeschlossen",
          },
        ]}
      />
    )

    await userEvent.click(screen.getByLabelText("Eintrag löschen: activity-8"))
    expect(screen.getByText("Wirklich löschen?")).toBeInTheDocument()
    expect(mutateAsyncMock).not.toHaveBeenCalled()

    await userEvent.click(screen.getByRole("button", { name: "Löschen" }))

    expect(mutateAsyncMock).toHaveBeenCalledWith({
      siteId: "site-1",
      activityId: "activity-8",
    })
    await waitFor(() => {
      expect(screen.queryByText("Wirklich löschen?")).not.toBeInTheDocument()
    })
    expect(toast.success).toHaveBeenCalledWith("Eintrag gelöscht")
  })

  it("shows an error toast when deletion fails", async () => {
    mutateAsyncMock.mockRejectedValue(new Error("Löschen fehlgeschlagen"))

    render(
      <ActivityFeed
        siteId="site-1"
        activities={[
          {
            ...baseActivity,
            id: "activity-9",
            can_delete: true,
            content: "Montage abgeschlossen",
          },
        ]}
      />
    )

    await userEvent.click(screen.getByLabelText("Eintrag löschen: activity-9"))
    await userEvent.click(screen.getByRole("button", { name: "Löschen" }))

    await waitFor(() => {
      expect(toast.error).toHaveBeenCalledWith("Löschen fehlgeschlagen")
    })
  })
})
