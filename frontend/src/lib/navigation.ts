import {
  Building2,
  LayoutDashboard,
  Package,
  Settings,
  Truck,
  Wrench,
} from "lucide-react"
import type { LucideIcon } from "lucide-react"

export interface NavItem {
  label: string
  href: string
  icon: LucideIcon
}

export const mainNavItems: NavItem[] = [
  { label: "Dashboard", href: "/", icon: LayoutDashboard },
  { label: "Inventar", href: "/inventory", icon: Package },
  { label: "Baustellen", href: "/sites", icon: Building2 },
  { label: "Fuhrpark", href: "/fleet", icon: Truck },
  { label: "Werkzeuge", href: "/tools", icon: Wrench },
  { label: "Einstellungen", href: "/settings", icon: Settings },
]
