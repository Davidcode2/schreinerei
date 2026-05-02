import { beforeEach, describe, expect, it, vi } from "vitest"
import userEvent from "@testing-library/user-event"

import { render, screen, waitFor } from "@/test/utils"
import { apiClient } from "@/lib/api/client"
import type { Activity } from "@/types/sites"
import { MediaViewer } from "./MediaViewer"

import {
  buildMediaViewerPath,
  buildSiteDetailPath,
  extractAttachmentIdFromPhotoUrl,
  resolveMediaViewerTarget,
} from "./mediaViewerRoute"

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
      success: vi.fn(),
      error: vi.fn(),
    },
  }
})

const getBlobMock = vi.mocked(apiClient.getBlob)

const activities: Activity[] = [
  {
    id: "activity-1",
    site_id: "site-1",
    user_id: "user-1",
    creator_name: "Anna Tischler",
    can_delete: true,
    activity_type: "note" as const,
    content: "Montage abgeschlossen",
    photo_url: null,
    attachments: [
      {
        attachment_id: "attachment-1",
        filename: "Montage Plan.pdf",
        mime_type: "application/pdf",
        url: "/api/v1/attachments/attachment-1",
        thumbnail_url: null,
      },
    ],
    created_at: "2026-05-01T10:00:00.000Z",
  },
  {
    id: "activity-2",
    site_id: "site-1",
    user_id: "user-2",
    creator_name: "Max Muster",
    can_delete: true,
    activity_type: "photo" as const,
    content: null,
    photo_url: "/api/v1/attachments/legacy-photo-id",
    attachments: [],
    created_at: "2026-05-01T09:00:00.000Z",
  },
]

const firstActivity = activities[0]!
const firstAttachment = firstActivity.attachments[0]!

describe("mediaViewerRoute", () => {
  it("builds canonical viewer paths with slugified filenames", () => {
    expect(
      buildMediaViewerPath("site-1", "activity-1", "attachment-1", "Montage Plan.pdf")
    ).toBe("/sites/site-1/media/activity-1/attachment-1/montage-plan-pdf")
  })

  it("extracts legacy attachment ids from photo urls", () => {
    expect(extractAttachmentIdFromPhotoUrl("/api/v1/attachments/legacy-photo-id")).toBe(
      "legacy-photo-id"
    )
    expect(extractAttachmentIdFromPhotoUrl("/api/v1/attachments/legacy-photo-id/thumbnail")).toBe(
      "legacy-photo-id"
    )
    expect(extractAttachmentIdFromPhotoUrl("https://example.com/public.jpg")).toBeNull()
  })

  it("resolves direct-link targets from attachments and legacy photo urls", () => {
    expect(resolveMediaViewerTarget(activities, "activity-1", "attachment-1")?.attachment.filename).toBe(
      "Montage Plan.pdf"
    )
    expect(resolveMediaViewerTarget(activities, "activity-2", "legacy-photo-id")?.title).toBe(
      "Aktivitätsfoto"
    )
  })

  it("builds the close path back to the site detail page", () => {
    expect(buildSiteDetailPath("site-1")).toBe("/sites/site-1")
  })
})

describe("MediaViewer", () => {
  const clipboardWriteText = vi.fn()

  beforeEach(() => {
    getBlobMock.mockReset()
    clipboardWriteText.mockReset()
    Object.assign(navigator, {
      clipboard: {
        writeText: clipboardWriteText,
      },
    })
    vi.spyOn(URL, "createObjectURL").mockReturnValue("blob:viewer-preview")
    vi.spyOn(URL, "revokeObjectURL").mockImplementation(() => undefined)
  })

  it("renders fullscreen viewer metadata and fallback note copy", async () => {
    getBlobMock.mockResolvedValue(new Blob(["pdf"], { type: "application/pdf" }))

    render(
        <MediaViewer
          open={true}
          target={{
            activity: {
              ...firstActivity,
              content: null,
            },
            attachment: firstAttachment,
            title: firstAttachment.filename,
          }}
        sharePath="/sites/site-1/media/activity-1/attachment-1/montage-plan-pdf"
        onClose={vi.fn()}
      />
    )

    expect(await screen.findByRole("heading", { name: "Montage Plan.pdf" })).toBeInTheDocument()
    expect(screen.getByText("Anna Tischler")).toBeInTheDocument()
    expect(screen.getByText("Keine Notiz vorhanden.")).toBeInTheDocument()
    expect(screen.getByText(/01\.05\.2026, .* Uhr/)).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "Viewer schließen" })).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "Link kopieren" })).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "Herunterladen" })).toBeInTheDocument()
  })

  it("copies the canonical viewer url to the clipboard", async () => {
    getBlobMock.mockResolvedValue(new Blob(["pdf"], { type: "application/pdf" }))

    render(
        <MediaViewer
          open={true}
          target={{
            activity: firstActivity,
            attachment: firstAttachment,
            title: firstAttachment.filename,
          }}
        sharePath="/sites/site-1/media/activity-1/attachment-1/montage-plan-pdf"
        onClose={vi.fn()}
      />
    )

    await userEvent.click(await screen.findByRole("button", { name: "Link kopieren" }))

    expect(clipboardWriteText).toHaveBeenCalledWith(
      "http://localhost:3000/sites/site-1/media/activity-1/attachment-1/montage-plan-pdf"
    )
  })

  it("downloads the protected blob with the attachment filename", async () => {
    const anchorClick = vi.fn()
    const originalCreateElement = document.createElement.bind(document)
    const createElementSpy = vi.spyOn(document, "createElement")
    createElementSpy.mockImplementation((tagName: string) => {
      if (tagName === "a") {
        return {
          click: anchorClick,
          href: "",
          download: "",
        } as unknown as HTMLAnchorElement
      }

      return originalCreateElement(tagName)
    })
    getBlobMock.mockResolvedValue(new Blob(["pdf"], { type: "application/pdf" }))

    render(
        <MediaViewer
          open={true}
          target={{
            activity: firstActivity,
            attachment: firstAttachment,
            title: firstAttachment.filename,
          }}
        sharePath="/sites/site-1/media/activity-1/attachment-1/montage-plan-pdf"
        onClose={vi.fn()}
      />
    )

    await userEvent.click(await screen.findByRole("button", { name: "Herunterladen" }))

    await waitFor(() => {
      expect(getBlobMock).toHaveBeenCalledWith("/api/v1/attachments/attachment-1")
      expect(anchorClick).toHaveBeenCalled()
    })
  })
})
