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
import { Mail } from "lucide-react"
import { toast } from "sonner"
import { useInviteUser, type InviteUserResponse } from "@/lib/api/hooks"

interface InviteUserDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function InviteUserDialog({
  open,
  onOpenChange,
}: InviteUserDialogProps) {
  const [email, setEmail] = useState("")
  const [generatedInvite, setGeneratedInvite] = useState<InviteUserResponse | null>(null)
  const inviteUser = useInviteUser()

  const isValidEmail = (email: string) => {
    return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)
  }

  const handleSendInvite = async () => {
    if (!isValidEmail(email)) {
      toast.error("Bitte geben Sie eine gültige E-Mail-Adresse ein")
      return
    }

    try {
      const invite = await inviteUser.mutateAsync({
        email,
        role: "employee",
      })
      setGeneratedInvite(invite)
      toast.success(`Einladung an ${invite.email} versendet`)
      setEmail("")
    } catch (error) {
      toast.error(error instanceof Error ? error.message : "Einladung konnte nicht gesendet werden")
      console.error("Send invite error:", error)
    }
  }

  const handleOpenChange = (nextOpen: boolean) => {
    onOpenChange(nextOpen)
    if (!nextOpen) {
      setEmail("")
      setGeneratedInvite(null)
      inviteUser.reset()
    }
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-3">
            <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
              <Mail className="h-4 w-4" />
            </span>
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
              className="h-10"
            />
          </div>

          {generatedInvite && (
            <div className="rounded-xl bg-accent/50 p-4">
              <p className="text-sm font-medium">Einladung erstellt</p>
              <p className="mt-2 text-sm text-muted-foreground">
                Keycloak versendet die Einladung direkt an <span className="font-medium text-foreground">{generatedInvite.email}</span>.
              </p>
              <p className="mt-2 text-xs text-muted-foreground">
                Gueltig bis {new Date(generatedInvite.expires_at).toLocaleString("de-DE")}
              </p>
            </div>
          )}
        </div>

        <DialogFooter className="gap-2 sm:gap-0">
          <Button
            variant="outline"
            onClick={() => handleOpenChange(false)}
            className="active:scale-[0.97] transition-transform"
          >
            Abbrechen
          </Button>
          <Button
            onClick={handleSendInvite}
            disabled={!email || !isValidEmail(email) || inviteUser.isPending}
            className="shadow-sm active:scale-[0.97] transition-transform"
          >
            {inviteUser.isPending ? "Wird erstellt..." : "Einladung erstellen"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
