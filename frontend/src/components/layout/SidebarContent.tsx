import { NavLink } from "react-router-dom"
import { cn } from "@/lib/utils"
import { mainNavItems } from "@/lib/navigation"

export function SidebarContent() {
  return (
    <nav className="flex flex-col gap-1 p-2">
      {mainNavItems.map((item) => (
        <NavLink
          key={item.href}
          to={item.href}
          className={({ isActive }) =>
            cn(
              "flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors",
              isActive
                ? "bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
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
