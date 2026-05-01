import type { Activity, ActivityAttachment } from "@/types/sites"

export interface MediaViewerTarget {
  activity: Activity
  attachment: ActivityAttachment
  title: string
}

export function buildSiteDetailPath(siteId: string): string {
  return `/sites/${siteId}`
}

export function slugifyFilename(filename: string): string {
  return filename
    .normalize("NFKD")
    .replace(/[^\w\s.-]/g, "")
    .trim()
    .toLowerCase()
    .replace(/[\s_.]+/g, "-")
    .replace(/-+/g, "-")
}

export function buildMediaViewerPath(
  siteId: string,
  activityId: string,
  attachmentId: string,
  filenameOrFallback: string
): string {
  const slug = slugifyFilename(filenameOrFallback)
  return `/sites/${siteId}/media/${activityId}/${attachmentId}/${slug}`
}

export function extractAttachmentIdFromPhotoUrl(photoUrl: string | null | undefined): string | null {
  if (!photoUrl) {
    return null
  }

  const match = photoUrl.match(/\/api\/v1\/attachments\/([^/]+)(?:\/thumbnail)?$/)
  return match?.[1] ?? null
}

export function resolveMediaViewerTarget(
  activities: Activity[],
  activityId: string | undefined,
  attachmentId: string | undefined
): MediaViewerTarget | null {
  if (!activityId || !attachmentId) {
    return null
  }

  const activity = activities.find((item) => item.id === activityId)
  if (!activity) {
    return null
  }

  const attachment = activity.attachments.find((item) => item.attachment_id === attachmentId)
  if (attachment) {
    return {
      activity,
      attachment,
      title: attachment.filename,
    }
  }

  const legacyAttachmentId = extractAttachmentIdFromPhotoUrl(activity.photo_url)
  if (activity.activity_type === "photo" && legacyAttachmentId === attachmentId && activity.photo_url) {
    return {
      activity,
      attachment: {
        attachment_id: attachmentId,
        filename: "Aktivitätsfoto",
        mime_type: "image/jpeg",
        url: activity.photo_url,
        thumbnail_url: activity.photo_url,
      },
      title: "Aktivitätsfoto",
    }
  }

  return null
}
