import { useEffect, useRef, useState } from "react"
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

interface CameraUploadFlowProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  siteId: string
  onSuccess: () => void
}

export function CameraUploadFlow({
  open,
  onOpenChange,
  siteId,
  onSuccess,
}: CameraUploadFlowProps) {
  const [selectedFile, setSelectedFile] = useState<File | null>(null)
  const [note, setNote] = useState("")
  const [previewUrl, setPreviewUrl] = useState<string | null>(null)
  const [pickerTriggered, setPickerTriggered] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)
  const createActivity = useCreateActivity()
  const uploadPhoto = useUploadSitePhoto()

  useEffect(() => {
    if (open && !pickerTriggered) {
      setTimeout(() => fileInputRef.current?.click(), 50)
      setPickerTriggered(true)
    }
    if (!open) {
      setPickerTriggered(false)
    }
  }, [open, pickerTriggered])

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (!file) {
      onOpenChange(false)
      return
    }

    setSelectedFile(file)
    const url = URL.createObjectURL(file)
    setPreviewUrl(url)
  }

  const handleSubmit = async () => {
    if (!selectedFile) {
      toast.error("Bitte wählen Sie ein Foto aus")
      return
    }

    try {
      const trimmedNote = note.trim() || undefined

      if (!isOnline()) {
        await queuePhotoUploadAction({
          siteId,
          file: selectedFile,
          content: trimmedNote,
        })

        toast.success("Foto offline gespeichert", {
          description: "Wird automatisch hochgeladen, sobald Sie wieder online sind.",
        })
        resetForm()
        onSuccess()
        onOpenChange(false)
        return
      }

      const uploadResponse = await uploadPhoto.mutateAsync({
        siteId,
        file: selectedFile,
      })

      await createActivity.mutateAsync({
        siteId,
        activity_type: "photo",
        content: trimmedNote,
        photo_url: uploadResponse.photo_url,
      })

      toast.success("Foto hinzugefügt")
      resetForm()
      onSuccess()
      onOpenChange(false)
    } catch {
      toast.error("Foto konnte nicht hochgeladen werden")
    }
  }

  const resetForm = () => {
    if (previewUrl) {
      URL.revokeObjectURL(previewUrl)
    }
    setSelectedFile(null)
    setNote("")
    setPreviewUrl(null)
    if (fileInputRef.current) {
      fileInputRef.current.value = ""
    }
  }

  const handleOpenChange = (newOpen: boolean) => {
    if (!newOpen) {
      resetForm()
    }
    onOpenChange(newOpen)
  }

  const isPending = createActivity.isPending || uploadPhoto.isPending
  const showDialog = selectedFile !== null

  return (
    <>
      <input
        ref={fileInputRef}
        type="file"
        accept="image/*"
        capture="environment"
        className="hidden"
        onChange={handleFileChange}
        data-testid="camera-file-input"
      />

      {showDialog && (
        <Dialog open={open} onOpenChange={handleOpenChange}>
          <DialogContent className="sm:max-w-md">
            <DialogHeader>
              <DialogTitle>Foto hinzufügen</DialogTitle>
              <DialogDescription>
                Fügen Sie optional eine Notiz zu Ihrem Foto hinzu
              </DialogDescription>
            </DialogHeader>

            <div className="py-4 space-y-4">
              {previewUrl && (
                <div className="relative">
                  <img
                    src={previewUrl}
                    alt="Vorschau"
                    className="w-full max-h-64 object-contain rounded-md"
                  />
                </div>
              )}

              <Textarea
                placeholder="Notiz hinzufügen (optional)..."
                value={note}
                onChange={(e) => setNote(e.target.value)}
                rows={2}
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
                disabled={isPending || !selectedFile}
              >
                {isPending ? "Hochladen..." : "Hochladen"}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      )}
    </>
  )
}