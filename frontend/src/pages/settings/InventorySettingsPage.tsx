import { useEffect, useMemo, useState } from "react"
import { Pencil, Trash2 } from "lucide-react"
import { toast } from "sonner"
import { PageHeader } from "@/components/shared"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog"
import { useCategories, useDeleteCategory, useUpdateCategory } from "@/lib/api/hooks"
import type { Category } from "@/types/inventory"

const EMPTY_STATE_TITLE = "Noch keine Kategorien"
const EMPTY_STATE_BODY =
  "Legen Sie die erste Kategorie an, damit Materialien sauber zugeordnet werden können."
const DELETE_CONFIRMATION =
  "Möchten Sie diese Kategorie wirklich löschen? Diese Aktion kann nicht rückgängig gemacht werden."
const BLOCKED_DELETE_MESSAGE =
  "Kategorie konnte nicht gelöscht werden. Entfernen oder verschieben Sie zuerst alle Materialien dieser Kategorie und versuchen Sie es dann erneut."

export default function InventorySettingsPage() {
  const [editingCategory, setEditingCategory] = useState<Category | null>(null)
  const [deleteCategoryId, setDeleteCategoryId] = useState<string | null>(null)
  const [name, setName] = useState("")
  const [description, setDescription] = useState("")
  const [conflictMessages, setConflictMessages] = useState<Record<string, string>>({})

  const { data: categories, isLoading } = useCategories()
  const updateCategory = useUpdateCategory()
  const deleteCategory = useDeleteCategory()

  const categoriesById = useMemo(
    () => new Map((categories ?? []).map((category) => [category.id, category])),
    [categories]
  )

  useEffect(() => {
    if (!editingCategory) {
      setName("")
      setDescription("")
      return
    }

    setName(editingCategory.name)
    setDescription(editingCategory.description ?? "")
  }, [editingCategory])

  const selectedDeleteCategory = deleteCategoryId
    ? categoriesById.get(deleteCategoryId) ?? null
    : null

  const handleEditOpen = (category: Category) => {
    setConflictMessages((current) => {
      const next = { ...current }
      delete next[category.id]
      return next
    })
    setEditingCategory(category)
  }

  const handleSave = () => {
    if (!editingCategory) {
      return
    }

    updateCategory.mutate(
      {
        id: editingCategory.id,
        data: {
          name: name.trim(),
          description: description.trim(),
        },
      },
      {
        onSuccess: () => {
          toast.success("Kategorie aktualisiert")
          setEditingCategory(null)
        },
      }
    )
  }

  const handleDelete = () => {
    if (!deleteCategoryId) {
      return
    }

    deleteCategory.mutate(deleteCategoryId, {
      onSuccess: () => {
        toast.success("Kategorie gelöscht")
        setDeleteCategoryId(null)
      },
      onError: (error) => {
        setConflictMessages((current) => ({
          ...current,
          [deleteCategoryId]: error.message || BLOCKED_DELETE_MESSAGE,
        }))
        setDeleteCategoryId(null)
      },
    })
  }

  const isSaveDisabled = name.trim().length === 0 || updateCategory.isPending

  return (
    <div className="space-y-6 max-w-2xl mx-auto">
      <PageHeader
        title="Inventar-Einstellungen"
        description="Kategorien und Materialpflege zentral verwalten"
      />

      <Card>
        <CardHeader>
          <CardTitle>Kategorien</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          {isLoading ? (
            <p className="text-sm text-muted-foreground">Kategorien werden geladen...</p>
          ) : categories && categories.length > 0 ? (
            categories.map((category) => (
              <div
                key={category.id}
                className="rounded-lg border border-border/60 p-4"
              >
                <div className="flex items-start justify-between gap-4">
                  <div className="space-y-1">
                    <p className="font-medium">{category.name}</p>
                    <p className="text-sm text-muted-foreground">
                      {category.description || "Keine Beschreibung hinterlegt."}
                    </p>
                  </div>

                  <div className="flex items-center gap-2">
                    <Button
                      variant="outline"
                      size="sm"
                      className="gap-2"
                      onClick={() => handleEditOpen(category)}
                      aria-label={`${category.name} bearbeiten`}
                    >
                      <Pencil className="h-4 w-4" />
                      Bearbeiten
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="gap-2 text-destructive hover:text-destructive"
                      onClick={() => setDeleteCategoryId(category.id)}
                      aria-label={`${category.name} löschen`}
                    >
                      <Trash2 className="h-4 w-4" />
                      Löschen
                    </Button>
                  </div>
                </div>

                {conflictMessages[category.id] && (
                  <p className="mt-3 text-sm text-destructive">
                    {conflictMessages[category.id]}
                  </p>
                )}
              </div>
            ))
          ) : (
            <div className="rounded-lg border border-dashed p-6 text-center">
              <p className="font-medium">{EMPTY_STATE_TITLE}</p>
              <p className="mt-2 text-sm text-muted-foreground">{EMPTY_STATE_BODY}</p>
            </div>
          )}
        </CardContent>
      </Card>

      <Dialog
        open={editingCategory !== null}
        onOpenChange={(open) => {
          if (!open) {
            setEditingCategory(null)
          }
        }}
      >
        <DialogContent className="sm:max-w-lg">
          <DialogHeader>
            <DialogTitle>Kategorie bearbeiten</DialogTitle>
            <DialogDescription>
              Aktualisieren Sie Name und Beschreibung der Kategorie.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="category-name">Name</Label>
              <Input
                id="category-name"
                value={name}
                onChange={(event) => setName(event.target.value)}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="category-description">Beschreibung</Label>
              <Textarea
                id="category-description"
                value={description}
                onChange={(event) => setDescription(event.target.value)}
                rows={4}
              />
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setEditingCategory(null)}>
              Abbrechen
            </Button>
            <Button onClick={handleSave} disabled={isSaveDisabled}>
              {updateCategory.isPending ? "Speichert..." : "Änderungen speichern"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <AlertDialog
        open={deleteCategoryId !== null}
        onOpenChange={(open) => {
          if (!open) {
            setDeleteCategoryId(null)
          }
        }}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Kategorie löschen</AlertDialogTitle>
            <AlertDialogDescription>{DELETE_CONFIRMATION}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel disabled={deleteCategory.isPending}>
              Abbrechen
            </AlertDialogCancel>
            <AlertDialogAction
              onClick={handleDelete}
              disabled={deleteCategory.isPending || !selectedDeleteCategory}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              {deleteCategory.isPending ? "Löscht..." : "Löschen"}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  )
}
