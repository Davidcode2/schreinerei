import { useEffect, useState } from "react"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { useCreateActivity, useUploadSitePhoto } from "@/lib/api/hooks"
import { isOnline } from "@/lib/offline/sync"
import { queuePhotoUploadAction } from "@/lib/offline/queue"
import { toast } from "sonner"

interface CreateNoteModalProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  siteId: string
  onSuccess: () => void
  initialActivityType?: "note" | "photo"
}

export function CreateNoteModal({
  open,
  onOpenChange,
  siteId,
  onSuccess,
  initialActivityType = "note",
}: CreateNoteModalProps) {
  const [content, setContent] = useState("")
  const [selectedFile, setSelectedFile] = useState<File | null>(null)
  const [activityType, setActivityType] = useState<"note" | "photo">("note")
  const createActivity = useCreateActivity()
  const uploadPhoto = useUploadSitePhoto()

  useEffect(() => {
    if (open) {
      setActivityType(initialActivityType)
    }
  }, [open, initialActivityType])

  const handleSubmit = async () => {
    if (activityType === "note" && !content.trim()) {
      toast.error("Bitte geben Sie eine Notiz ein")
      return
    }

    if (activityType === "photo" && !selectedFile) {
      toast.error("Bitte wählen Sie ein Foto aus")
      return
    }

    try {
      let photoUrl: string | undefined

      if (activityType === "photo" && selectedFile) {
        if (!isOnline()) {
          await queuePhotoUploadAction({
            siteId,
            file: selectedFile,
            content: content.trim() || undefined,
          })

          toast.success("Foto offline gespeichert", {
            description: "Wird automatisch hochgeladen, sobald Sie wieder online sind.",
          })
          setContent("")
          setSelectedFile(null)
          setActivityType("note")
          onSuccess()
          onOpenChange(false)
          return
        }

        const uploadResponse = await uploadPhoto.mutateAsync({
          siteId,
          file: selectedFile,
        })
        photoUrl = uploadResponse.photo_url
      }

      await createActivity.mutateAsync({
        siteId,
        activity_type: activityType,
        content: activityType === "note" ? content.trim() : undefined,
        photo_url: photoUrl,
      })

      toast.success(activityType === "note" ? "Notiz hinzugefügt" : "Foto hinzugefügt")
      setContent("")
      setSelectedFile(null)
      setActivityType("note")
      onSuccess()
      onOpenChange(false)
    } catch (error) {
      toast.error(
        activityType === "note"
          ? "Notiz konnte nicht erstellt werden"
          : "Foto konnte nicht hochgeladen werden"
      )
    }
  }

  const handleOpenChange = (newOpen: boolean) => {
    if (!newOpen) {
      setContent("")
      setSelectedFile(null)
      setActivityType("note")
    }
    onOpenChange(newOpen)
  }

  const isPending = createActivity.isPending || uploadPhoto.isPending

  const handleDialogOpenChange = (newOpen: boolean) => {
    if (newOpen) {
      setActivityType(initialActivityType)
      onOpenChange(true)
      return
    }

    handleOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={handleDialogOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Notiz hinzufügen</DialogTitle>
          <DialogDescription>
            Fügen Sie eine Notiz hinzu oder laden Sie ein Foto hoch
          </DialogDescription>
        </DialogHeader>

        <div className="py-4 space-y-4">
          <div className="flex gap-2">
            <Button
              type="button"
              variant={activityType === "note" ? "default" : "outline"}
              onClick={() => setActivityType("note")}
            >
              Notiz
            </Button>
            <Button
              type="button"
              variant={activityType === "photo" ? "default" : "outline"}
              onClick={() => setActivityType("photo")}
            >
              Foto
            </Button>
          </div>

          {activityType === "note" ? (
            <Textarea
              placeholder="Notiz eingeben..."
              value={content}
              onChange={(e) => setContent(e.target.value)}
              rows={4}
              className="resize-none"
            />
          ) : (
            <div className="space-y-2">
              <input
                type="file"
                accept="image/*"
                capture="environment"
                onChange={(event) =>
                  setSelectedFile(event.target.files?.[0] ?? null)
                }
              />
              {selectedFile && (
                <p className="text-xs text-muted-foreground">{selectedFile.name}</p>
              )}
            </div>
          )}
        </div>

        <DialogFooter>
          <Button
            variant="outline"
            onClick={() => handleOpenChange(false)}
          >
            Abbrechen
          </Button>
          <Button
            onClick={handleSubmit}
            disabled={
              isPending ||
              (activityType === "note" && !content.trim()) ||
              (activityType === "photo" && !selectedFile)
            }
          >
            {isPending ? "Speichern..." : "Speichern"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
