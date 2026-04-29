import { Card } from "@/components/ui/card"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Camera, FileText, User } from "lucide-react"
import type { Activity } from "@/types/sites"

interface ActivityFeedProps {
  activities: Activity[]
  maxItems?: number
}

function formatRelativeTime(dateString: string): string {
  const date = new Date(dateString)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.round(diffMs / 60000)
  const diffHours = Math.round(diffMs / 3600000)
  const diffDays = Math.round(diffMs / 86400000)

  if (diffMins < 60) {
    return `vor ${diffMins} Min.`
  } else if (diffHours < 24) {
    return `vor ${diffHours} Std.`
  } else if (diffDays < 7) {
    return `vor ${diffDays} Tag${diffDays !== 1 ? "en" : ""}`
  } else {
    return date.toLocaleDateString("de-DE", {
      day: "2-digit",
      month: "2-digit",
    })
  }
}

function getInitials(name: string): string {
  return name
    .split(" ")
    .map((n) => n[0])
    .join("")
    .toUpperCase()
    .slice(0, 2)
}

export function ActivityFeed({ activities, maxItems }: ActivityFeedProps) {
  const displayedActivities = maxItems
    ? activities.slice(0, maxItems)
    : activities

  if (activities.length === 0) {
    return (
      <div className="text-center py-8">
        <div className="rounded-full bg-muted p-4 mx-auto w-fit mb-3">
          <FileText className="h-6 w-6 text-muted-foreground" />
        </div>
        <p className="text-sm text-muted-foreground">
          Noch keine Aktivitäten
        </p>
        <p className="text-xs text-muted-foreground mt-1">
          Fügen Sie Fotos oder Notizen hinzu
        </p>
      </div>
    )
  }

  return (
    <div className="space-y-4">
      {displayedActivities.map((activity) => (
        <Card key={activity.id} className="overflow-hidden">
          <div className="flex gap-3 p-3">
            {/* Icon */}
            <div className="flex-shrink-0">
              {activity.activity_type === "photo" ? (
                <div className="rounded-lg bg-blue-100 dark:bg-blue-900 p-2">
                  <Camera className="h-4 w-4 text-blue-600 dark:text-blue-300" />
                </div>
              ) : (
                <div className="rounded-lg bg-yellow-100 dark:bg-yellow-900 p-2">
                  <FileText className="h-4 w-4 text-yellow-600 dark:text-yellow-300" />
                </div>
              )}
            </div>

            {/* Content */}
            <div className="flex-1 min-w-0">
              <div className="flex items-start justify-between gap-2">
                <p className="text-sm">
                  {activity.activity_type === "photo" ? (
                    <span className="font-medium">Foto hinzugefügt</span>
                  ) : (
                    <span className="font-medium">Notiz</span>
                  )}
                </p>
                <span className="text-xs text-muted-foreground whitespace-nowrap">
                  {formatRelativeTime(activity.created_at)}
                </span>
              </div>

              {activity.content && (
                <p className="text-sm text-muted-foreground mt-1 line-clamp-2">
                  {activity.content}
                </p>
              )}

              {/* Photo preview */}
              {activity.photo_url && (
                <div className="mt-2">
                  <img
                    src={activity.photo_url}
                    alt="Activity"
                    className="rounded-lg max-h-32 object-cover"
                  />
                </div>
              )}
            </div>
          </div>
        </Card>
      ))}

      {activities.length > (maxItems || 0) && maxItems && (
        <p className="text-xs text-muted-foreground text-center">
          +{activities.length - maxItems} weitere Aktivitäten
        </p>
      )}
    </div>
  )
}
