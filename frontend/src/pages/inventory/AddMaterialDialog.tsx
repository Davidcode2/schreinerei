import { useMemo, useState } from "react"
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { toast } from "sonner"
import { useCreateMaterial, useCreateCategory } from "@/lib/api/hooks"
import type { Category } from "@/types/inventory"
import { CategoryDialog } from "./CategoryDialog"

interface AddMaterialDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  categories: Category[]
}

const UNIT_OPTIONS = [
  { value: "Stück", label: "Stück" },
  { value: "Meter", label: "Meter" },
  { value: "Kilogramm", label: "Kilogramm" },
  { value: "Liter", label: "Liter" },
  { value: "Packung", label: "Packung" },
]

export function AddMaterialDialog({
  open,
  onOpenChange,
  categories,
}: AddMaterialDialogProps) {
  const [categoryId, setCategoryId] = useState("")
  const [name, setName] = useState("")
  const [quantity, setQuantity] = useState("")
  const [unit, setUnit] = useState("")
  const [minQuantity, setMinQuantity] = useState("")
  const [location, setLocation] = useState("")
  const [expiresOn, setExpiresOn] = useState("")
  const [isCategoryDialogOpen, setIsCategoryDialogOpen] = useState(false)

  const createMaterial = useCreateMaterial()
  const createCategory = useCreateCategory()

  const selectedCategory = useMemo(
    () => categories.find((category) => category.id === categoryId) ?? null,
    [categories, categoryId]
  )

  const quantityValue = Number(quantity) || 0
  const requiresExpiryDate = selectedCategory?.can_expire === true && quantityValue > 0

  const resetForm = () => {
    setCategoryId("")
    setName("")
    setQuantity("")
    setUnit("")
    setMinQuantity("")
    setLocation("")
    setExpiresOn("")
    setIsCategoryDialogOpen(false)
  }

  const handleOpenChange = (nextOpen: boolean) => {
    if (!nextOpen) {
      resetForm()
    }
    onOpenChange(nextOpen)
  }

  const isFormValid =
    categoryId && name && quantity && unit && minQuantity && (!requiresExpiryDate || expiresOn)

  const handleSubmit = () => {
    if (!isFormValid) {
      return
    }

    createMaterial.mutate(
      {
        category_id: categoryId,
        name,
        description: null,
        quantity: Number(quantity),
        unit,
        min_quantity: Number(minQuantity),
        location: location || null,
        expires_on: expiresOn || null,
      },
      {
        onSuccess: () => {
          toast.success("Material erstellt")
          handleOpenChange(false)
        },
        onError: (error) => {
          toast.error("Material konnte nicht erstellt werden")
          console.error("Create material error:", error)
        },
      }
    )
  }

  const handleCreateCategory = (values: {
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
        onSuccess: (newCategory) => {
          setCategoryId(newCategory.id)
          setIsCategoryDialogOpen(false)
          toast.success("Kategorie erstellt")
        },
      }
    )
  }

  return (
    <>
      <Dialog open={open} onOpenChange={handleOpenChange}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle>Material hinzufügen</DialogTitle>
            <DialogDescription>Neues Material zum Inventar hinzufügen</DialogDescription>
          </DialogHeader>

          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="category">Kategorie *</Label>
              <div className="flex gap-2">
                <Select value={categoryId} onValueChange={setCategoryId}>
                  <SelectTrigger id="category" className="flex-1">
                    <SelectValue placeholder="Kategorie wählen" />
                  </SelectTrigger>
                  <SelectContent>
                    {categories.map((category) => (
                      <SelectItem key={category.id} value={category.id}>
                        {category.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={() => setIsCategoryDialogOpen(true)}
                >
                  + Neu
                </Button>
              </div>
            </div>

            <div className="space-y-2">
              <Label htmlFor="name">Name *</Label>
              <Input
                id="name"
                placeholder="z.B. Schrauben M8"
                value={name}
                onChange={(event) => setName(event.target.value)}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="quantity">Menge *</Label>
              <Input
                id="quantity"
                type="number"
                min={0}
                step="1"
                placeholder="0"
                value={quantity}
                onChange={(event) => setQuantity(event.target.value)}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="unit">Einheit *</Label>
              <Select value={unit} onValueChange={setUnit}>
                <SelectTrigger id="unit">
                  <SelectValue placeholder="Einheit wählen" />
                </SelectTrigger>
                <SelectContent>
                  {UNIT_OPTIONS.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label htmlFor="minQuantity">Mindestbestand *</Label>
              <Input
                id="minQuantity"
                type="number"
                min={0}
                step="1"
                placeholder="0"
                value={minQuantity}
                onChange={(event) => setMinQuantity(event.target.value)}
              />
            </div>

            {selectedCategory?.can_expire && (
              <div className="space-y-2">
                <Label htmlFor="expiresOn" title="Mindesthaltbarkeitsdatum">
                  MHD *
                </Label>
                <Input
                  id="expiresOn"
                  type="date"
                  value={expiresOn}
                  onChange={(event) => setExpiresOn(event.target.value)}
                />
              </div>
            )}

            <div className="space-y-2">
              <Label htmlFor="location">Lagerort</Label>
              <Input
                id="location"
                placeholder="z.B. Regal A3"
                value={location}
                onChange={(event) => setLocation(event.target.value)}
              />
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => handleOpenChange(false)}>
              Abbrechen
            </Button>
            <Button onClick={handleSubmit} disabled={!isFormValid || createMaterial.isPending}>
              {createMaterial.isPending ? "Wird erstellt..." : "Erstellen"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <CategoryDialog
        open={isCategoryDialogOpen}
        onOpenChange={setIsCategoryDialogOpen}
        title="Kategorie anlegen"
        description="Legen Sie direkt beim Materialanlegen eine neue Kategorie an."
        submitLabel="Kategorie erstellen"
        isSubmitting={createCategory.isPending}
        onSubmit={handleCreateCategory}
      />
    </>
  )
}
