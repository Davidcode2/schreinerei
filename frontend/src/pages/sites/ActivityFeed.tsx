import { useEffect, useState } from "react"
import { Link } from "react-router-dom"
import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Camera, FileText, ArrowRight } from "lucide-react"
import type { Activity } from "@/types/sites"
import { useSiteMaterialHistory } from "@/lib/api/hooks"
import { apiClient } from "@/lib/api/client"

const statusLabels: Record<string, string> = {
  planned: "Geplant",
  active: "Aktiv",
  completed: "Abgeschlossen",
  archived: "Archiviert",
}

interface ActivityFeedProps {
  activities: Activity[]
  siteId: string
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

function isProtectedAttachmentPath(url: string): boolean {
  return url.startsWith("/api/v1/attachments/")
}

function ActivityImage({ photoUrl }: { photoUrl: string }) {
  const [resolvedSrc, setResolvedSrc] = useState<string>(photoUrl)

  useEffect(() => {
    if (!isProtectedAttachmentPath(photoUrl)) {
      setResolvedSrc(photoUrl)
      return
    }

    let objectUrl: string | null = null
    let mounted = true

    apiClient
      .getBlob(photoUrl)
      .then((blob) => {
        if (!mounted) {
          return
        }
        objectUrl = URL.createObjectURL(blob)
        setResolvedSrc(objectUrl)
      })
      .catch(() => {
        if (mounted) {
          setResolvedSrc("")
        }
      })

    return () => {
      mounted = false
      if (objectUrl) {
        URL.revokeObjectURL(objectUrl)
      }
    }
  }, [photoUrl])

  if (!resolvedSrc) {
    return null
  }

  return (
    <img
      src={resolvedSrc}
      alt="Aktivitätsfoto"
      className="rounded-lg max-h-32 object-cover"
    />
  )
}

function ActivityCard({ activity }: { activity: Activity }) {
  return (
    <Card className="overflow-hidden">
      <div className="flex gap-3 p-3">
        <div className="flex-shrink-0">
          {activity.activity_type === "photo" ? (
            <div className="rounded-lg bg-blue-100 dark:bg-blue-900 p-2">
              <Camera className="h-4 w-4 text-blue-600 dark:text-blue-300" />
            </div>
          ) : activity.activity_type === "status_change" ? (
            <div className="rounded-lg bg-green-100 dark:bg-green-900 p-2">
              <ArrowRight className="h-4 w-4 text-green-600 dark:text-green-300" />
            </div>
          ) : (
            <div className="rounded-lg bg-yellow-100 dark:bg-yellow-900 p-2">
              <FileText className="h-4 w-4 text-yellow-600 dark:text-yellow-300" />
            </div>
          )}
        </div>

        <div className="flex-1 min-w-0">
          <div className="flex items-start justify-between gap-2">
            <p className="text-sm">
              {activity.activity_type === "photo" ? (
                <span className="font-medium">Foto hinzugefügt</span>
              ) : activity.activity_type === "status_change" ? (
                <span className="font-medium">Status geändert</span>
              ) : (
                <span className="font-medium">Notiz</span>
              )}
            </p>
            <span className="text-xs text-muted-foreground whitespace-nowrap">
              {formatRelativeTime(activity.created_at)}
            </span>
          </div>

          {activity.activity_type === "status_change" && activity.content ? (
            <div className="mt-1">
              <p className="text-sm text-muted-foreground">
                {(() => {
                  try {
                    const data = JSON.parse(activity.content)
                    const oldStatus = statusLabels[data.old_status] || data.old_status
                    const newStatus = statusLabels[data.new_status] || data.new_status
                    return `${oldStatus} → ${newStatus}`
                  } catch {
                    return activity.content
                  }
                })()}
              </p>
            </div>
          ) : activity.content ? (
            <p className="text-sm text-muted-foreground mt-1 line-clamp-2">
              {activity.content}
            </p>
          ) : null}

          {activity.photo_url && (
            <div className="mt-2">
              <ActivityImage photoUrl={activity.photo_url} />
            </div>
          )}
        </div>
      </div>
    </Card>
  )
}

export function ActivityFeed({ activities, siteId, maxItems }: ActivityFeedProps) {
  const [activeTab, setActiveTab] = useState("notes")
  const { data: materialHistory, isLoading: isMaterialHistoryLoading } =
    useSiteMaterialHistory(siteId)

  const noteActivities = activities.filter(
    (a) => a.activity_type === "photo" || a.activity_type === "note" || a.activity_type === "status_change"
  )

  const displayedActivities = maxItems
    ? noteActivities.slice(0, maxItems)
    : noteActivities

  return (
    <Tabs value={activeTab} onValueChange={setActiveTab}>
      <TabsList>
        <TabsTrigger value="notes">Notizen/Dokumente</TabsTrigger>
        <TabsTrigger value="materials">Material</TabsTrigger>
      </TabsList>

      <TabsContent value="notes" className="space-y-4 mt-4">
        {displayedActivities.length === 0 ? (
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
        ) : (
          <>
            {displayedActivities.map((activity) => (
              <ActivityCard key={activity.id} activity={activity} />
            ))}
            {noteActivities.length > (maxItems || 0) && maxItems && (
              <p className="text-xs text-muted-foreground text-center">
                +{noteActivities.length - maxItems} weitere Aktivitäten
              </p>
            )}
          </>
        )}
      </TabsContent>

      <TabsContent value="materials" className="mt-0">
        {isMaterialHistoryLoading ? (
          <p className="text-sm text-muted-foreground text-center py-8">
            Material-Historie wird geladen...
          </p>
        ) : materialHistory && materialHistory.length > 0 ? (
          <div className="space-y-3 mt-4">
            {materialHistory.map((entry) => (
              <Card key={entry.id} className="p-3">
                <div className="flex items-start justify-between gap-3">
                  <div>
                    <p className="text-sm font-medium">{entry.material_name}</p>
                    <p className="text-xs text-muted-foreground">
                      Kategorie: {entry.category_name}
                    </p>
                    <p className="text-xs text-muted-foreground">
                      Entnommen von: {entry.extracted_by}
                    </p>
                    {entry.site_id && entry.site_name && (
                      <Link
                        to={`/sites/${entry.site_id}`}
                        className="text-xs text-blue-600 hover:underline"
                      >
                        {entry.site_name}
                      </Link>
                    )}
                  </div>
                  <div className="text-right">
                    <p className="text-sm font-medium">
                      {entry.quantity_change < 0
                        ? `Entnahme ${Math.abs(entry.quantity_change)}`
                        : `+${entry.quantity_change}`}
                    </p>
                    <p className="text-xs text-muted-foreground">
                      {formatRelativeTime(entry.created_at)}
                    </p>
                  </div>
                </div>
              </Card>
            ))}
          </div>
        ) : (
          <p className="text-sm text-muted-foreground text-center py-8">
            Noch keine Materialentnahmen für diese Baustelle
          </p>
        )}
      </TabsContent>
    </Tabs>
  )
}
