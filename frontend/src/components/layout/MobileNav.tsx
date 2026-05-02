import { useNavigate } from "react-router-dom"
import { Menu, QrCode, LogOut } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Sheet, SheetContent, SheetTrigger, SheetTitle } from "@/components/ui/sheet"
import { SidebarContent } from "./SidebarContent"
import { ActiveSiteIndicator } from "./ActiveSiteIndicator"
import PendingActionsBadge from "@/components/offline/PendingActionsBadge"
import SyncButton from "@/components/offline/SyncButton"
import { useAuthStore } from "@/lib/auth/authStore"
import { getLogoutUrl } from "@/lib/auth/keycloak"

export function MobileNav() {
  const navigate = useNavigate()

  return (
    <header className="fixed top-0 left-0 right-0 z-50 flex h-16 items-center gap-2 border-b bg-card px-3 md:hidden">
      <Sheet>
        <SheetTrigger asChild>
          <Button variant="ghost" size="icon" className="flex-shrink-0">
            <Menu className="h-5 w-5" />
            <span className="sr-only">Menü öffnen</span>
          </Button>
        </SheetTrigger>
        <SheetContent side="left" className="w-60 p-0 flex flex-col">
          <SheetTitle className="sr-only">Navigation</SheetTitle>
          <div className="flex h-16 items-center border-b px-4">
            <h1 className="text-lg font-semibold text-primary">Schreinerei</h1>
          </div>
          <div className="py-4 flex-1">
            <ActiveSiteIndicator />
            <SidebarContent />
          </div>
          <div className="border-t p-4">
            <Button 
              variant="ghost" 
              size="sm" 
              className="w-full justify-start gap-2"
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

      <ActiveSiteIndicator compact className="flex-1" />

      <div className="flex flex-shrink-0 items-center gap-1">
        <PendingActionsBadge />
        <SyncButton />
        <Button
          variant="ghost"
          size="icon"
          onClick={() => navigate('/scan')}
          title="QR-Code scannen"
        >
          <QrCode className="h-5 w-5" />
        </Button>
      </div>
    </header>
  )
}
