import { type ChangeEvent, useEffect, useMemo, useRef, useState } from "react"
import { FileText, X } from "lucide-react"
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
import { useCreateActivity, useUploadSiteAttachment } from "@/lib/api/hooks"
import { toast } from "sonner"

const ACCEPTED_MIME_TYPES = [
  "image/jpeg",
  "image/png",
  "image/webp",
  "application/pdf",
] as const

type ComposerFile = {
  id: string
  file: File
  previewUrl: string | null
}

interface CreateNoteModalProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  siteId: string
  onSuccess: () => void
  initialActivityType?: "note" | "photo"
}

function isAcceptedFile(file: File) {
  return ACCEPTED_MIME_TYPES.includes(file.type as (typeof ACCEPTED_MIME_TYPES)[number])
}

function buildComposerFile(file: File): ComposerFile {
  return {
    id: `${file.name}-${file.lastModified}-${file.size}`,
    file,
    previewUrl: file.type.startsWith("image/") ? URL.createObjectURL(file) : null,
  }
}

function revokePreview(file: ComposerFile) {
  if (file.previewUrl) {
    URL.revokeObjectURL(file.previewUrl)
  }
}

export function CreateNoteModal({
  open,
  onOpenChange,
  siteId,
  onSuccess,
}: CreateNoteModalProps) {
  const [content, setContent] = useState("")
  const [selectedFiles, setSelectedFiles] = useState<ComposerFile[]>([])
  const [selectionError, setSelectionError] = useState<string | null>(null)
  const inputRef = useRef<HTMLInputElement | null>(null)
  const createActivity = useCreateActivity()
  const uploadSiteAttachment = useUploadSiteAttachment()

  useEffect(() => {
    return () => {
      selectedFiles.forEach(revokePreview)
    }
  }, [selectedFiles])

  const isPending = createActivity.isPending || uploadSiteAttachment.isPending
  const hasContent = content.trim().length > 0
  const canSubmit = hasContent || selectedFiles.length > 0

  const helperText = useMemo(() => {
    if (!selectionError) {
      return "Unterstützt Bilder und PDFs."
    }

    return selectionError
  }, [selectionError])

  function resetForm() {
    selectedFiles.forEach(revokePreview)
    setContent("")
    setSelectedFiles([])
    setSelectionError(null)
    if (inputRef.current) {
      inputRef.current.value = ""
    }
  }

  function handleOpenChange(nextOpen: boolean) {
    if (!nextOpen) {
      resetForm()
    }
    onOpenChange(nextOpen)
  }

  function handleFilesSelected(event: ChangeEvent<HTMLInputElement>) {
    const files = Array.from(event.target.files ?? [])
    if (files.length === 0) {
      return
    }

    const validFiles = files.filter(isAcceptedFile)
    const invalidFiles = files.filter((file) => !isAcceptedFile(file))

    if (validFiles.length > 0) {
      setSelectedFiles((currentFiles) => [
        ...currentFiles,
        ...validFiles.map(buildComposerFile),
      ])
    }

    setSelectionError(
      invalidFiles.length > 0
        ? `Nicht unterstützte Dateien: ${invalidFiles
            .map((file) => `${file.name} (${file.type || "unbekannt"})`)
            .join(", ")}`
        : null
    )

    event.target.value = ""
  }

  function handleRemoveFile(fileId: string) {
    setSelectedFiles((currentFiles) => {
      const fileToRemove = currentFiles.find((file) => file.id === fileId)
      if (fileToRemove) {
        revokePreview(fileToRemove)
      }

      return currentFiles.filter((file) => file.id !== fileId)
    })
  }

  async function handleSubmit() {
    if (!canSubmit) {
      toast.error("Bitte fügen Sie eine Notiz oder mindestens eine Datei hinzu")
      return
    }

    try {
      const uploadedAttachments = await Promise.all(
        selectedFiles.map(({ file }) => uploadSiteAttachment.mutateAsync({ siteId, file }))
      )

      await createActivity.mutateAsync({
        siteId,
        activity_type: "note",
        content: hasContent ? content.trim() : undefined,
        attachment_ids: uploadedAttachments.map((attachment) => attachment.attachment_id),
      })

      toast.success("Eintrag gespeichert")
      resetForm()
      onSuccess()
      onOpenChange(false)
    } catch {
      toast.error("Upload fehlgeschlagen. Prüfen Sie Dateityp oder Verbindung und versuchen Sie es erneut.")
    }
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Dokumente hinzufügen</DialogTitle>
          <DialogDescription>
            Fügen Sie eine Notiz hinzu und laden Sie Bilder oder PDFs in einem Eintrag hoch.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          <Textarea
            placeholder="Notiz hinzufügen (optional)..."
            value={content}
            onChange={(event) => setContent(event.target.value)}
            rows={2}
            className="max-h-40 min-h-20 resize-y"
          />

          <div className="space-y-2">
            <input
              ref={inputRef}
              type="file"
              multiple
              accept="image/jpeg,image/png,image/webp,application/pdf"
              className="sr-only"
              onChange={handleFilesSelected}
            />
            <Button
              type="button"
              variant="outline"
              onClick={() => inputRef.current?.click()}
            >
              Dateien auswählen
            </Button>
          </div>

          <div className="space-y-2">
            {selectedFiles.map((selectedFile) => (
              <div
                key={selectedFile.id}
                className="flex items-center gap-3 rounded-lg border bg-background p-3"
              >
                <div className="flex h-16 w-16 items-center justify-center overflow-hidden rounded-md bg-muted">
                  {selectedFile.previewUrl ? (
                    <img
                      src={selectedFile.previewUrl}
                      alt={selectedFile.file.name}
                      className="h-full w-full object-cover"
                    />
                  ) : (
                    <div className="flex flex-col items-center gap-1 text-muted-foreground">
                      <FileText className="h-5 w-5" />
                      <span className="text-[10px] font-semibold uppercase">PDF</span>
                    </div>
                  )}
                </div>

                <div className="min-w-0 flex-1">
                  <p className="truncate text-sm font-semibold">{selectedFile.file.name}</p>
                  <p className="text-xs text-muted-foreground">
                    {selectedFile.file.type.startsWith("image/") ? "Bild" : "PDF"}
                  </p>
                </div>

                <Button
                  type="button"
                  variant="ghost"
                  size="icon"
                  className="h-10 w-10"
                  onClick={() => handleRemoveFile(selectedFile.id)}
                  aria-label={`Datei entfernen: ${selectedFile.file.name}`}
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>
            ))}
          </div>

          <p className="text-xs text-muted-foreground">{helperText}</p>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => handleOpenChange(false)}>
            Abbrechen
          </Button>
          <Button onClick={handleSubmit} disabled={isPending || !canSubmit}>
            {isPending ? "Speichern..." : "Eintrag speichern"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
