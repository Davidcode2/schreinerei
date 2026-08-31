import { useState } from "react"
import { Shield, Trash2, UserRound } from "lucide-react"
import { toast } from "sonner"

import { DeleteConfirmDialog } from "@/components/shared/DeleteConfirmDialog"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import {
  type User,
  type UserRole,
  useDeleteUser,
  useUpdateUserRole,
} from "@/lib/api/hooks"

interface EditUserDialogProps {
  user: User
  open: boolean
  onOpenChange: (open: boolean) => void
}

function errorMessage(error: Error, fallback: string) {
  return error.message.trim() || fallback
}

export function EditUserDialog({ user, open, onOpenChange }: EditUserDialogProps) {
  const [role, setRole] = useState<UserRole>(user.role === "admin" ? "admin" : "employee")
  const [deleteOpen, setDeleteOpen] = useState(false)
  const updateRole = useUpdateUserRole()
  const deleteUser = useDeleteUser()
  const displayName = user.name ?? user.email

  const handleSave = () => {
    updateRole.mutate(
      { id: user.id, role },
      {
        onSuccess: () => {
          toast.success("Benutzerrolle aktualisiert")
          onOpenChange(false)
        },
        onError: (error) => toast.error(errorMessage(error, "Benutzerrolle konnte nicht aktualisiert werden")),
      }
    )
  }

  const handleDelete = () => {
    deleteUser.mutate(user.id, {
      onSuccess: () => {
        toast.success("Benutzer gelöscht")
        setDeleteOpen(false)
        onOpenChange(false)
      },
      onError: (error) => toast.error(errorMessage(error, "Benutzer konnte nicht gelöscht werden")),
    })
  }

  return (
    <>
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle className="flex items-center gap-3">
              <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
                <UserRound className="h-4 w-4" />
              </span>
              Benutzer bearbeiten
            </DialogTitle>
            <DialogDescription>
              Rolle und Zugriff für {displayName} verwalten.
            </DialogDescription>
          </DialogHeader>

          <fieldset className="space-y-3 py-3" role="radiogroup">
            <legend className="mb-2 text-sm font-medium">Rolle</legend>
            <RoleOption
              checked={role === "admin"}
              label="Admin"
              description="Kann Benutzer und Einstellungen verwalten."
              icon={<Shield className="h-4 w-4" />}
              onChange={() => setRole("admin")}
            />
            <RoleOption
              checked={role === "employee"}
              label="Mitarbeiter"
              description="Nutzt die täglichen Arbeitsbereiche ohne Administration."
              icon={<UserRound className="h-4 w-4" />}
              onChange={() => setRole("employee")}
            />
          </fieldset>

          <div className="rounded-xl border border-destructive/20 bg-destructive/5 p-4">
            <p className="text-sm font-medium">Benutzer entfernen</p>
            <p className="mt-1 text-sm text-muted-foreground">
              Der Zugriff auf diese Organisation wird dauerhaft entfernt.
            </p>
            <Button
              variant="outline"
              className="mt-3 w-full gap-2 text-destructive hover:bg-destructive/10 hover:text-destructive sm:w-auto"
              onClick={() => setDeleteOpen(true)}
              disabled={deleteUser.isPending}
            >
              <Trash2 className="h-4 w-4" />
              Benutzer löschen
            </Button>
          </div>

          <DialogFooter className="gap-2 sm:gap-0">
            <Button variant="outline" onClick={() => onOpenChange(false)}>
              Abbrechen
            </Button>
            <Button onClick={handleSave} disabled={updateRole.isPending || role === user.role}>
              {updateRole.isPending ? "Wird gespeichert..." : "Änderungen speichern"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <DeleteConfirmDialog
        open={deleteOpen}
        onOpenChange={setDeleteOpen}
        onConfirm={handleDelete}
        itemName={displayName}
        title="Benutzer löschen"
        isPending={deleteUser.isPending}
        closeOnConfirm={false}
      />
    </>
  )
}

interface RoleOptionProps {
  checked: boolean
  label: string
  description: string
  icon: React.ReactNode
  onChange: () => void
}

function RoleOption({ checked, label, description, icon, onChange }: RoleOptionProps) {
  return (
    <button
      type="button"
      role="radio"
      aria-checked={checked}
      aria-label={label}
      onClick={onChange}
      className={`flex w-full items-start gap-3 rounded-xl border p-3 text-left shadow-sm transition-colors hover:bg-accent/40 ${
        checked ? "border-primary/40 bg-accent/60" : "border-border/70 bg-background"
      }`}
    >
      <span
        className={`mt-1 h-4 w-4 shrink-0 rounded-full border-4 ${
          checked ? "border-primary bg-primary" : "border-muted-foreground/50"
        }`}
      />
      <span className="mt-0.5 text-muted-foreground">{icon}</span>
      <span className="min-w-0">
        <span className="block text-sm font-medium">{label}</span>
        <span className="mt-0.5 block text-sm text-muted-foreground">{description}</span>
      </span>
    </button>
  )
}
