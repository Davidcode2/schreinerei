import { useEffect, useState } from "react"
import { Calendar, ClipboardList, Home, PencilRuler } from "lucide-react"
import { toast } from "sonner"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle } from "@/components/ui/sheet"
import { Textarea } from "@/components/ui/textarea"
import { useUpdateSite } from "@/lib/api/hooks"
import type { ProjectType, Site } from "@/types/sites"

interface ProjectPlanningSheetProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  site: Site
}

const projectTypeOptions: Array<{ value: ProjectType; label: string; description: string }> = [
  {
    value: "external_site",
    label: "Baustelle extern",
    description: "Einsatz beim Kunden vor Ort",
  },
  {
    value: "internal_workshop",
    label: "Werkstatt intern",
    description: "Interne Vorbereitung oder Werkstattauftrag",
  },
]

export function ProjectPlanningSheet({ open, onOpenChange, site }: ProjectPlanningSheetProps) {
  const updateSite = useUpdateSite()
  const [projectType, setProjectType] = useState<ProjectType>(site.project_type)
  const [name, setName] = useState(site.name)
  const [customerName, setCustomerName] = useState(site.customer_name)
  const [location, setLocation] = useState(site.location ?? "")
  const [description, setDescription] = useState(site.description ?? "")
  const [startDate, setStartDate] = useState(site.start_date ?? "")
  const [endDate, setEndDate] = useState(site.end_date ?? "")
  const [estimatedDays, setEstimatedDays] = useState(site.estimated_days?.toString() ?? "")

  useEffect(() => {
    if (!open) return
    setProjectType(site.project_type)
    setName(site.name)
    setCustomerName(site.customer_name)
    setLocation(site.location ?? "")
    setDescription(site.description ?? "")
    setStartDate(site.start_date ?? "")
    setEndDate(site.end_date ?? "")
    setEstimatedDays(site.estimated_days?.toString() ?? "")
  }, [open, site])

  const customerRequired = projectType === "external_site"
  const isValid = name.trim().length > 0 && (!customerRequired || customerName.trim().length > 0)

  function handleSave() {
    if (!isValid) return

    const payload: { id: string } & Record<string, unknown> = {
      id: site.id,
      project_type: projectType,
      name,
      customer_name: customerName,
    }

    if (location) payload.location = location
    if (description) payload.description = description
    if (startDate) payload.start_date = startDate
    if (endDate) payload.end_date = endDate
    if (estimatedDays) payload.estimated_days = Number(estimatedDays)

    updateSite.mutate(
      payload,
      {
        onSuccess: () => {
          toast.success("Projektplanung gespeichert")
          onOpenChange(false)
        },
        onError: () => {
          toast.error("Projektplanung konnte nicht gespeichert werden")
        },
      }
    )
  }

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent side="bottom" className="max-h-[92vh] overflow-y-auto rounded-t-3xl px-4 pb-6 pt-4 sm:px-6">
        <SheetHeader className="mb-5 text-left">
          <SheetTitle className="font-display text-lg">Projekt planen</SheetTitle>
          <SheetDescription className="sr-only">
            Projektart, Planung und Team direkt im Hauptprojekt pflegen.
          </SheetDescription>
        </SheetHeader>

        <div className="space-y-5">
          <div className="space-y-2">
            <Label htmlFor="project-type">Projektart</Label>
            <Select value={projectType} onValueChange={(value) => setProjectType(value as ProjectType)}>
              <SelectTrigger id="project-type" className="h-11">
                <SelectValue placeholder="Projektart wählen" />
              </SelectTrigger>
              <SelectContent>
                {projectTypeOptions.map((option) => (
                  <SelectItem key={option.value} value={option.value}>
                    {option.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <p className="text-xs text-muted-foreground">
              {projectTypeOptions.find((option) => option.value === projectType)?.description}
            </p>
          </div>

          <div className="space-y-2">
            <Label htmlFor="project-name">Projektname *</Label>
            <div className="relative">
              <PencilRuler className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input id="project-name" className="h-11 pl-10" value={name} onChange={(e) => setName(e.target.value)} />
            </div>
          </div>

          <div className="space-y-2">
            <Label htmlFor="project-customer">{customerRequired ? "Kunde *" : "Kunde / Bezug"}</Label>
            <div className="relative">
              <ClipboardList className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                id="project-customer"
                className="h-11 pl-10"
                value={customerName}
                onChange={(e) => setCustomerName(e.target.value)}
                placeholder={customerRequired ? "z.B. Familie Müller" : "Optional für interne Projekte"}
              />
            </div>
          </div>

          <div className="space-y-2">
            <Label htmlFor="project-location">Standort</Label>
            <div className="relative">
              <Home className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                id="project-location"
                className="h-11 pl-10"
                value={location}
                onChange={(e) => setLocation(e.target.value)}
                placeholder={projectType === "external_site" ? "Baustellenadresse" : "z.B. Werkstatt, CNC-Bereich"}
              />
            </div>
          </div>

          <div className="grid gap-4 sm:grid-cols-2">
            <div className="space-y-2">
              <Label htmlFor="project-start">Start</Label>
              <div className="relative">
                <Calendar className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                <Input id="project-start" type="date" className="h-11 pl-10" value={startDate} onChange={(e) => setStartDate(e.target.value)} />
              </div>
            </div>
            <div className="space-y-2">
              <Label htmlFor="project-end">Ende</Label>
              <Input id="project-end" type="date" className="h-11" value={endDate} onChange={(e) => setEndDate(e.target.value)} />
            </div>
          </div>

          <div className="space-y-2">
            <Label htmlFor="project-days">Geplante Tage</Label>
            <Input id="project-days" type="number" min="0" className="h-11" value={estimatedDays} onChange={(e) => setEstimatedDays(e.target.value)} />
          </div>

          <div className="space-y-2">
            <Label htmlFor="project-description">Planungsnotiz</Label>
            <Textarea id="project-description" rows={4} value={description} onChange={(e) => setDescription(e.target.value)} placeholder="Wichtige Hinweise, Besonderheiten, Vorbereitung" />
          </div>

          <div className="flex gap-2 pt-2">
            <Button variant="outline" className="h-11 flex-1" onClick={() => onOpenChange(false)}>
              Abbrechen
            </Button>
            <Button className="h-11 flex-1" disabled={!isValid || updateSite.isPending} onClick={handleSave}>
              {updateSite.isPending ? "Speichert..." : "Speichern"}
            </Button>
          </div>
        </div>
      </SheetContent>
    </Sheet>
  )
}
