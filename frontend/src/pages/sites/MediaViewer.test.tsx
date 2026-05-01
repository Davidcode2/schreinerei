import { describe, expect, it } from "vitest"

import {
  buildMediaViewerPath,
  buildSiteDetailPath,
  extractAttachmentIdFromPhotoUrl,
  resolveMediaViewerTarget,
} from "./mediaViewerRoute"

const activities = [
  {
    id: "activity-1",
    site_id: "site-1",
    user_id: "user-1",
    creator_name: "Anna Tischler",
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
    activity_type: "photo" as const,
    content: null,
    photo_url: "/api/v1/attachments/legacy-photo-id",
    attachments: [],
    created_at: "2026-05-01T09:00:00.000Z",
  },
]

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
