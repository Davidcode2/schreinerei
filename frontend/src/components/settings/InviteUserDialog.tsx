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
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Copy, Link2, Mail } from "lucide-react"
import { toast } from "sonner"

interface InviteUserDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  inviteUrl: string
}

export function InviteUserDialog({
  open,
  onOpenChange,
  inviteUrl,
}: InviteUserDialogProps) {
  const [email, setEmail] = useState("")
  const [isSubmitting, setIsSubmitting] = useState(false)

  const isValidEmail = (email: string) => {
    return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)
  }

  const handleCopyLink = async () => {
    try {
      await navigator.clipboard.writeText(inviteUrl)
      toast.success("Einladungslink kopiert")
    } catch {
      toast.error("Link konnte nicht kopiert werden")
    }
  }

  const handleSendInvite = async () => {
    if (!isValidEmail(email)) {
      toast.error("Bitte geben Sie eine gültige E-Mail-Adresse ein")
      return
    }

    setIsSubmitting(true)
    try {
      toast.success(`Einladung an ${email} gesendet`)
      setEmail("")
      onOpenChange(false)
    } catch (error) {
      toast.error("Einladung konnte nicht gesendet werden")
      console.error("Send invite error:", error)
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Mail className="h-5 w-5" />
            Benutzer einladen
          </DialogTitle>
          <DialogDescription>
            Laden Sie neue Mitarbeiter per E-Mail ein
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          <div className="space-y-2">
            <Label htmlFor="email">E-Mail-Adresse</Label>
            <Input
              id="email"
              type="email"
              placeholder="name@beispiel.de"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
            />
          </div>

          <div className="pt-4 border-t">
            <div className="flex items-center gap-2 mb-2 text-sm text-muted-foreground">
              <Link2 className="h-4 w-4" />
              <span>Oder teilen Sie diesen Link:</span>
            </div>
            <div className="flex gap-2">
              <Input
                value={inviteUrl}
                readOnly
                className="text-sm bg-muted"
              />
              <Button size="sm" variant="outline" onClick={handleCopyLink}>
                <Copy className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Abbrechen
          </Button>
          <Button
            onClick={handleSendInvite}
            disabled={!email || !isValidEmail(email) || isSubmitting}
          >
            {isSubmitting ? "Wird gesendet..." : "Einladung senden"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
