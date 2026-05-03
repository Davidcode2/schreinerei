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
        "w-full overflow-x-auto pb-1 -mb-1 scrollbar-none",
        className
      )}
    >
      <div className="flex gap-2 whitespace-nowrap">
        <button
          onClick={() => onSelect(undefined)}
          className={cn(
            "flex-shrink-0 rounded-full px-4 py-1.5 text-sm font-medium transition-all",
            selectedId === undefined
              ? "bg-primary text-primary-foreground shadow-sm"
              : "bg-card border border-border text-muted-foreground hover:bg-accent hover:text-foreground"
          )}
        >
          Alle
        </button>
        {categories.map((category) => (
          <button
            key={category.id}
            onClick={() => onSelect(category.id)}
            className={cn(
              "flex-shrink-0 rounded-full px-4 py-1.5 text-sm font-medium transition-all",
              selectedId === category.id
                ? "bg-primary text-primary-foreground shadow-sm"
                : "bg-card border border-border text-muted-foreground hover:bg-accent hover:text-foreground"
            )}
          >
            {category.name}
          </button>
        ))}
      </div>
    </div>
  )
}
