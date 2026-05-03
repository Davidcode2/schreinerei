import { NavLink } from "react-router-dom"
import { cn } from "@/lib/utils"
import { mainNavItems } from "@/lib/navigation"

export function SidebarContent() {
  return (
    <nav className="flex flex-col gap-1 px-3">
      {mainNavItems.map((item) => (
        <NavLink
          key={item.href}
          to={item.href}
          className={({ isActive }) =>
            cn(
              "flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-all",
              isActive
                ? "bg-primary/10 text-primary border-l-2 border-primary"
                : "text-muted-foreground hover:bg-accent/60 hover:text-foreground border-l-2 border-transparent"
            )
          }
        >
          <item.icon className="h-5 w-5" />
          <span>{item.label}</span>
        </NavLink>
      ))}
    </nav>
  )
}
