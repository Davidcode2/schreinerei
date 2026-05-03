import { useMemo, useState } from "react"
import { Pencil, Plus, Trash2 } from "lucide-react"
import { toast } from "sonner"
import { PageHeader } from "@/components/shared"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
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
import {
  useCategories,
  useCreateCategory,
  useDeleteCategory,
  useUpdateCategory,
} from "@/lib/api/hooks"
import type { Category } from "@/types/inventory"
import { CategoryDialog } from "@/pages/inventory/CategoryDialog"

const EMPTY_STATE_TITLE = "Noch keine Kategorien"
const EMPTY_STATE_BODY =
  "Legen Sie die erste Kategorie an, damit Materialien sauber zugeordnet werden können."
const DELETE_CONFIRMATION =
  "Möchten Sie diese Kategorie wirklich löschen? Diese Aktion kann nicht rückgängig gemacht werden."
const BLOCKED_DELETE_MESSAGE =
  "Kategorie konnte nicht gelöscht werden. Entfernen oder verschieben Sie zuerst alle Materialien dieser Kategorie und versuchen Sie es dann erneut."
const BLOCKED_DELETE_PREFIX = "Cannot delete category:"

function getDeleteConflictMessage(error: Error) {
  const message = error.message.trim()

  if (
    message === "Cannot delete category: material history must be preserved" ||
    message === "Cannot delete category: materials still reference it" ||
    message.startsWith(BLOCKED_DELETE_PREFIX)
  ) {
    return BLOCKED_DELETE_MESSAGE
  }

  return message
}

export default function InventorySettingsPage() {
  const [editingCategory, setEditingCategory] = useState<Category | null>(null)
  const [isCreateOpen, setIsCreateOpen] = useState(false)
  const [deleteCategoryId, setDeleteCategoryId] = useState<string | null>(null)
  const [conflictMessages, setConflictMessages] = useState<Record<string, string>>({})

  const { data: categories, isLoading } = useCategories()
  const createCategory = useCreateCategory()
  const updateCategory = useUpdateCategory()
  const deleteCategory = useDeleteCategory()

  const categoriesById = useMemo(
    () => new Map((categories ?? []).map((category) => [category.id, category])),
    [categories]
  )

  const selectedDeleteCategory = deleteCategoryId
    ? categoriesById.get(deleteCategoryId) ?? null
    : null
  const editingInitialValues = editingCategory
    ? {
        name: editingCategory.name,
        description: editingCategory.description ?? "",
        canExpire: editingCategory.can_expire,
      }
    : null

  const handleEditOpen = (category: Category) => {
    setConflictMessages((current) => {
      const next = { ...current }
      delete next[category.id]
      return next
    })
    setEditingCategory(category)
  }

  const handleCreate = (values: {
    name: string
    description: string
    canExpire: boolean
  }) => {
    createCategory.mutate(
      {
        name: values.name,
        description: values.description || null,
        can_expire: values.canExpire,
      },
      {
        onSuccess: () => {
          toast.success("Kategorie erstellt")
          setIsCreateOpen(false)
        },
      }
    )
  }

  const handleSave = (values: {
    name: string
    description: string
    canExpire: boolean
  }) => {
    if (!editingCategory) {
      return
    }

    updateCategory.mutate(
      {
        id: editingCategory.id,
        data: {
          name: values.name,
          description: values.description,
          can_expire: values.canExpire,
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
          [deleteCategoryId]: getDeleteConflictMessage(error),
        }))
        setDeleteCategoryId(null)
      },
    })
  }

  return (
    <div className="space-y-6 max-w-2xl mx-auto">
      <PageHeader
        title="Inventar-Einstellungen"
        description="Kategorien und Materialpflege zentral verwalten"
        action={
          <Button className="gap-2" onClick={() => setIsCreateOpen(true)}>
            <Plus className="h-4 w-4" />
            Kategorie anlegen
          </Button>
        }
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
                    {category.can_expire && (
                      <p
                        className="text-xs font-medium text-amber-700"
                        title="Mindesthaltbarkeitsdatum"
                      >
                        MHD aktiviert
                      </p>
                    )}
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

      <CategoryDialog
        open={isCreateOpen}
        onOpenChange={setIsCreateOpen}
        title="Kategorie anlegen"
        description="Legen Sie eine neue Materialkategorie an."
        submitLabel="Kategorie erstellen"
        isSubmitting={createCategory.isPending}
        onSubmit={handleCreate}
      />

      <CategoryDialog
        open={editingCategory !== null}
        onOpenChange={(open) => {
          if (!open) {
            setEditingCategory(null)
          }
        }}
        title="Kategorie bearbeiten"
        description="Aktualisieren Sie Name, Beschreibung und MHD-Verhalten der Kategorie."
        submitLabel="Änderungen speichern"
        isSubmitting={updateCategory.isPending}
        {...(editingInitialValues ? { initialValues: editingInitialValues } : {})}
        onSubmit={handleSave}
      />

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
