import { useNavigate } from "react-router-dom"
import { Menu, QrCode, LogOut, User } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Sheet, SheetContent, SheetTrigger, SheetTitle } from "@/components/ui/sheet"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { SidebarContent } from "./SidebarContent"
import { ActiveSiteIndicator } from "./ActiveSiteIndicator"
import PendingActionsBadge from "@/components/offline/PendingActionsBadge"
import SyncButton from "@/components/offline/SyncButton"
import { useAuthStore } from "@/lib/auth/authStore"
import { getLogoutUrl } from "@/lib/auth/keycloak"
import { getDisplayName, getRoleLabel } from "./userDisplay"

export function MobileNav() {
  const navigate = useNavigate()
  const user = useAuthStore((state) => state.user)

  return (
    <header className="fixed top-0 left-0 right-0 z-50 flex h-14 items-center gap-3 border-b bg-card/95 backdrop-blur-md px-4 md:hidden safe-area-top">
      <Sheet>
        <SheetTrigger asChild>
          <Button variant="ghost" size="icon" className="flex-shrink-0 h-10 w-10">
            <Menu className="h-5 w-5" />
            <span className="sr-only">Menü öffnen</span>
          </Button>
        </SheetTrigger>
        <SheetContent side="left" className="w-72 p-0 flex flex-col">
          <SheetTitle className="sr-only">Navigation</SheetTitle>
          <div className="flex h-16 items-center border-b px-5 gap-3">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary">
              <span className="text-primary-foreground font-display text-sm font-bold">S</span>
            </div>
            <h1 className="text-lg font-display text-foreground">Schreinerei</h1>
          </div>
          <div className="py-4 flex-1 overflow-auto">
            <ActiveSiteIndicator />
            <SidebarContent />
          </div>
          <div className="border-t p-4">
            <button
              type="button"
              className="mb-3 flex w-full items-center gap-3 rounded-lg p-2 -m-2 text-left transition-colors hover:bg-accent/40"
              onClick={() => navigate('/settings')}
            >
              <Avatar className="h-9 w-9 border border-border">
                <AvatarFallback className="bg-accent text-accent-foreground">
                  <User className="h-4 w-4" />
                </AvatarFallback>
              </Avatar>
              <div className="min-w-0 flex-1 overflow-hidden">
                <p className="truncate text-sm font-medium">{getDisplayName(user?.name ?? null, user?.email)}</p>
                <p className="truncate text-xs text-muted-foreground">{getRoleLabel(user?.role)}</p>
              </div>
            </button>
            <Button
              variant="ghost"
              size="sm"
              className="w-full justify-start gap-3 h-11 text-muted-foreground hover:text-destructive"
              onClick={() => {
                useAuthStore.getState().logout()
                window.location.href = getLogoutUrl()
              }}
            >
              <LogOut className="h-4 w-4" />
              Abmelden
            </Button>
          </div>
        </SheetContent>
      </Sheet>

      <ActiveSiteIndicator compact className="flex-1 min-w-0" />

      <div className="flex flex-shrink-0 items-center gap-1">
        <PendingActionsBadge />
        <SyncButton />
        <Button
          variant="ghost"
          size="icon"
          className="h-10 w-10"
          onClick={() => navigate('/scan')}
          title="QR-Code scannen"
        >
          <QrCode className="h-5 w-5" />
        </Button>
      </div>
    </header>
  )
}
