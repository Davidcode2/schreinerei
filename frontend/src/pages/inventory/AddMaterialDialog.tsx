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
  const [showNewCategory, setShowNewCategory] = useState(false)
  const [newCategoryName, setNewCategoryName] = useState("")

  const createMaterial = useCreateMaterial()
  const createCategory = useCreateCategory()

  const resetForm = () => {
    setCategoryId("")
    setName("")
    setQuantity("")
    setUnit("")
    setMinQuantity("")
    setLocation("")
    setShowNewCategory(false)
    setNewCategoryName("")
  }

  const handleOpenChange = (open: boolean) => {
    if (!open) {
      resetForm()
    }
    onOpenChange(open)
  }

  const isFormValid =
    categoryId && name && quantity && unit && minQuantity

  const handleSubmit = () => {
    if (!isFormValid) return

    const payload: {
      category_id: string
      name: string
      quantity: number
      unit: string
      min_quantity: number
      location?: string
    } = {
      category_id: categoryId,
      name,
      quantity: parseFloat(quantity),
      unit,
      min_quantity: parseFloat(minQuantity),
    }

    if (location) {
      payload.location = location
    }

    createMaterial.mutate(payload, {
      onSuccess: () => {
        toast.success("Material erstellt")
        handleOpenChange(false)
      },
      onError: (error) => {
        toast.error("Material konnte nicht erstellt werden")
        console.error("Create material error:", error)
      },
    })
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Material hinzufügen</DialogTitle>
          <DialogDescription>
            Neues Material zum Inventar hinzufügen
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          {/* Category */}
          <div className="space-y-2">
            <Label htmlFor="category">Kategorie *</Label>
            {showNewCategory ? (
              <div className="flex gap-2">
                <Input
                  placeholder="Kategoriename"
                  value={newCategoryName}
                  onChange={(e) => setNewCategoryName(e.target.value)}
                  className="flex-1"
                />
                <Button
                  type="button"
                  size="sm"
                  onClick={() => {
                    if (newCategoryName.trim()) {
                      createCategory.mutate(
                        { name: newCategoryName.trim() },
                        {
                          onSuccess: (newCat) => {
                            setCategoryId(newCat.id)
                            setShowNewCategory(false)
                            setNewCategoryName("")
                          },
                        }
                      )
                    }
                  }}
                  disabled={!newCategoryName.trim() || createCategory.isPending}
                >
                  Erstellen
                </Button>
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  onClick={() => {
                    setShowNewCategory(false)
                    setNewCategoryName("")
                  }}
                >
                  Abbrechen
                </Button>
              </div>
            ) : (
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
                  onClick={() => setShowNewCategory(true)}
                >
                  + Neu
                </Button>
              </div>
            )}
          </div>

          {/* Name */}
          <div className="space-y-2">
            <Label htmlFor="name">Name *</Label>
            <Input
              id="name"
              placeholder="z.B. Schrauben M8"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>

          {/* Quantity */}
          <div className="space-y-2">
            <Label htmlFor="quantity">Menge *</Label>
            <Input
              id="quantity"
              type="number"
              min={0}
              step="0.01"
              placeholder="0"
              value={quantity}
              onChange={(e) => setQuantity(e.target.value)}
            />
          </div>

          {/* Unit */}
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

          {/* Min Quantity */}
          <div className="space-y-2">
            <Label htmlFor="minQuantity">Mindestbestand *</Label>
            <Input
              id="minQuantity"
              type="number"
              min={0}
              step="0.01"
              placeholder="0"
              value={minQuantity}
              onChange={(e) => setMinQuantity(e.target.value)}
            />
          </div>

          {/* Location */}
          <div className="space-y-2">
            <Label htmlFor="location">Lagerort</Label>
            <Input
              id="location"
              placeholder="z.B. Regal A3"
              value={location}
              onChange={(e) => setLocation(e.target.value)}
            />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => handleOpenChange(false)}>
            Abbrechen
          </Button>
          <Button
            onClick={handleSubmit}
            disabled={!isFormValid || createMaterial.isPending}
          >
            {createMaterial.isPending ? "Wird erstellt..." : "Erstellen"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
