import { useNavigate } from "react-router-dom"
import { LogOut, User, QrCode } from "lucide-react"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Button } from "@/components/ui/button"
import { Separator } from "@/components/ui/separator"
import { SidebarContent } from "./SidebarContent"
import PendingActionsBadge from "@/components/offline/PendingActionsBadge"
import SyncButton from "@/components/offline/SyncButton"

export function DesktopSidebar() {
  const navigate = useNavigate()

  return (
    <aside className="hidden md:flex md:w-60 md:flex-col md:fixed md:inset-y-0 border-r bg-card">
      {/* Logo */}
      <div className="flex h-16 items-center border-b px-4">
        <h1 className="text-lg font-semibold text-primary">Schreinerei</h1>
      </div>

      {/* Navigation */}
      <div className="flex-1 overflow-auto py-4">
        <SidebarContent />
      </div>

      {/* Quick actions */}
      <div className="border-t p-2 flex justify-center gap-2">
        <PendingActionsBadge />
        <SyncButton />
        <Button
          variant="ghost"
          size="icon"
          onClick={() => navigate('/scan')}
          title="QR-Code scannen"
        >
          <QrCode className="h-4 w-4" />
        </Button>
      </div>

      {/* User section */}
      <div className="border-t p-4">
        <div className="flex items-center gap-3">
          <Avatar className="h-9 w-9">
            <AvatarFallback>
              <User className="h-4 w-4" />
            </AvatarFallback>
          </Avatar>
          <div className="flex-1 overflow-hidden">
            <p className="truncate text-sm font-medium">Max Mustermann</p>
            <p className="truncate text-xs text-muted-foreground">Admin</p>
          </div>
        </div>
        <Separator className="my-3" />
        <Button variant="ghost" size="sm" className="w-full justify-start gap-2">
          <LogOut className="h-4 w-4" />
          Abmelden
        </Button>
      </div>
    </aside>
  )
}
