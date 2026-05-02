import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"
import type { Category } from "@/types/inventory"

interface CategoryFilterProps {
  categories: Category[]
  selectedId: string | undefined
  onSelect: (id: string | undefined) => void
  className?: string
}

export function CategoryFilter({
  categories,
  selectedId,
  onSelect,
  className,
}: CategoryFilterProps) {
  return (
    <div
      className={cn(
        "w-full overflow-x-auto pb-2",
        className
      )}
    >
      <div className="flex gap-2 whitespace-nowrap">
        <Button
          variant={selectedId === undefined ? "default" : "outline"}
          size="sm"
          onClick={() => onSelect(undefined)}
          className="rounded-full"
        >
          Alle
        </Button>
        {categories.map((category) => (
          <Button
            key={category.id}
            variant={selectedId === category.id ? "default" : "outline"}
            size="sm"
            onClick={() => onSelect(category.id)}
            className="rounded-full"
          >
            {category.name}
          </Button>
        ))}
      </div>
    </div>
  )
}
