import { useNavigate } from "react-router-dom"
import { LogOut, User, QrCode } from "lucide-react"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Button } from "@/components/ui/button"
import { Separator } from "@/components/ui/separator"
import { SidebarContent } from "./SidebarContent"
import { ActiveSiteIndicator } from "./ActiveSiteIndicator"
import PendingActionsBadge from "@/components/offline/PendingActionsBadge"
import SyncButton from "@/components/offline/SyncButton"
import { useAuthStore } from "@/lib/auth/authStore"
import { getLogoutUrl } from "@/lib/auth/keycloak"

export function DesktopSidebar() {
  const navigate = useNavigate()
  const logout = useAuthStore((state) => state.logout)

  const handleLogout = () => {
    logout()
    window.location.href = getLogoutUrl()
  }

  return (
    <aside className="hidden md:flex md:w-64 md:flex-col md:fixed md:inset-y-0 border-r bg-card">
      <div className="flex h-16 items-center border-b px-5 gap-3">
        <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary">
          <span className="text-primary-foreground font-display text-sm font-bold">S</span>
        </div>
        <h1 className="text-lg font-display text-foreground">Schreinerei</h1>
      </div>

      <div className="flex-1 overflow-auto py-4">
        <ActiveSiteIndicator />
        <SidebarContent />
      </div>

      <div className="border-t p-3 flex justify-center gap-2">
        <PendingActionsBadge />
        <SyncButton />
        <Button
          variant="ghost"
          size="icon"
          className="h-10 w-10"
          onClick={() => navigate('/scan')}
          title="QR-Code scannen"
        >
          <QrCode className="h-4 w-4" />
        </Button>
      </div>

      <div className="border-t p-4">
        <div className="flex items-center gap-3">
          <Avatar className="h-9 w-9 border border-border">
            <AvatarFallback className="bg-accent text-accent-foreground">
              <User className="h-4 w-4" />
            </AvatarFallback>
          </Avatar>
          <div className="flex-1 overflow-hidden">
            <p className="truncate text-sm font-medium">Max Mustermann</p>
            <p className="truncate text-xs text-muted-foreground">Admin</p>
          </div>
        </div>
        <Separator className="my-3" />
        <Button
          variant="ghost"
          size="sm"
          className="w-full justify-start gap-3 h-10 text-muted-foreground hover:text-destructive"
          onClick={handleLogout}
        >
          <LogOut className="h-4 w-4" />
          Abmelden
        </Button>
      </div>
    </aside>
  )
}
