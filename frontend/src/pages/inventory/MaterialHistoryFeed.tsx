import { Link } from "react-router-dom"
import { Badge } from "@/components/ui/badge"
import { EmptyState } from "@/components/shared"
import { cn } from "@/lib/utils"
import type { EnrichedStockHistoryEntry, EntryType } from "@/types/inventory"
import { MapPin, Package } from "lucide-react"

const entryTypeConfig: Record<
  EntryType,
  {
    label: string
    badgeClassName: string
  }
> = {
  material_added: {
    label: "Eingelagert",
    badgeClassName: "bg-success/15 text-success border-success/20",
  },
  withdrawn: {
    label: "Entnommen",
    badgeClassName: "bg-destructive/10 text-destructive border-destructive/20",
  },
  disposed: {
    label: "Entsorgt",
    badgeClassName: "bg-destructive/10 text-destructive border-destructive/20",
  },
  adjusted: {
    label: "Bestand korrigiert",
    badgeClassName: "bg-primary/10 text-primary border-primary/20",
  },
  location_changed: {
    label: "Lagerort geändert",
    badgeClassName: "bg-primary/10 text-primary border-primary/20",
  },
  min_quantity_changed: {
    label: "Mindestbestand geändert",
    badgeClassName: "bg-primary/10 text-primary border-primary/20",
  },
}

interface MaterialHistoryFeedProps {
  entries: EnrichedStockHistoryEntry[]
}

function formatQuantityChange(quantityChange: number) {
  return quantityChange > 0 ? `+${quantityChange}` : `${quantityChange}`
}

function SiteReference({ entry }: { entry: EnrichedStockHistoryEntry }) {
  if (entry.entry_type !== "withdrawn" || !entry.site_id || !entry.site_name) {
    return null
  }

  return (
    <div className="mt-2 flex items-center gap-2 text-sm text-muted-foreground">
      <MapPin className="h-4 w-4" />
        <Link to={`/sites/${entry.site_id}`} className="text-primary hover:underline">
        {entry.site_name}
      </Link>
    </div>
  )
}

function MaterialHistoryItem({ entry }: { entry: EnrichedStockHistoryEntry }) {
  const config = entryTypeConfig[entry.entry_type]

  return (
    <div className="rounded-lg border border-border/60 p-4 hover:bg-accent/20 transition-colors">
      <div className="flex items-start justify-between gap-3">
        <div className="space-y-2">
          <div className="flex flex-wrap items-center gap-2">
            <Badge variant="outline" className={cn("border", config.badgeClassName)}>
              {config.label}
            </Badge>
            <span className="text-sm text-muted-foreground">
              {new Date(entry.created_at).toLocaleString("de-DE")}
            </span>
          </div>
          <p className="text-sm text-muted-foreground">von {entry.user_name}</p>
        </div>

        <span className="text-sm font-semibold">
          {formatQuantityChange(entry.quantity_change)}
        </span>
      </div>

      {entry.notes && <p className="mt-2 text-sm text-muted-foreground">{entry.notes}</p>}

      <SiteReference entry={entry} />
    </div>
  )
}

export function MaterialHistoryFeed({ entries }: MaterialHistoryFeedProps) {
  if (entries.length === 0) {
    return (
      <EmptyState
        icon={Package}
        title="Noch keine Materialbewegungen"
        description="Einlagerungen, Entnahmen und Korrekturen erscheinen hier, sobald die erste Änderung erfasst wurde."
      />
    )
  }

  return (
    <div className="space-y-3">
      {entries.map((entry) => (
        <MaterialHistoryItem key={entry.id} entry={entry} />
      ))}
    </div>
  )
}
