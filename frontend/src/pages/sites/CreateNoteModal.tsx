import { useState } from "react"
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
import { useCreateActivity } from "@/lib/api/hooks"
import { toast } from "sonner"

interface CreateNoteModalProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  siteId: string
  onSuccess: () => void
}

export function CreateNoteModal({
  open,
  onOpenChange,
  siteId,
  onSuccess,
}: CreateNoteModalProps) {
  const [content, setContent] = useState("")
  const createActivity = useCreateActivity()

  const handleSubmit = async () => {
    if (!content.trim()) {
      toast.error("Bitte geben Sie eine Notiz ein")
      return
    }

    try {
      await createActivity.mutateAsync({
        siteId,
        activity_type: "note",
        content: content.trim(),
      })
      toast.success("Notiz hinzugefügt")
      setContent("")
      onSuccess()
      onOpenChange(false)
    } catch (error) {
      toast.error("Notiz konnte nicht erstellt werden")
    }
  }

  const handleOpenChange = (newOpen: boolean) => {
    if (!newOpen) {
      setContent("")
    }
    onOpenChange(newOpen)
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Notiz hinzufügen</DialogTitle>
          <DialogDescription>
            Fügen Sie eine Notiz zur Baustelle hinzu
          </DialogDescription>
        </DialogHeader>

        <div className="py-4">
          <Textarea
            placeholder="Notiz eingeben..."
            value={content}
            onChange={(e) => setContent(e.target.value)}
            rows={4}
            className="resize-none"
          />
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
            disabled={!content.trim() || createActivity.isPending}
          >
            {createActivity.isPending ? "Speichern..." : "Speichern"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
