import { Menu } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Sheet, SheetContent, SheetTrigger, SheetTitle } from "@/components/ui/sheet"
import { SidebarContent } from "./SidebarContent"

export function MobileNav() {
  return (
    <header className="flex md:hidden h-16 items-center border-b bg-card px-4 fixed top-0 left-0 right-0 z-50">
      <Sheet>
        <SheetTrigger asChild>
          <Button variant="ghost" size="icon" className="mr-2">
            <Menu className="h-5 w-5" />
            <span className="sr-only">Menü öffnen</span>
          </Button>
        </SheetTrigger>
        <SheetContent side="left" className="w-60 p-0">
          <SheetTitle className="sr-only">Navigation</SheetTitle>
          <div className="flex h-16 items-center border-b px-4">
            <h1 className="text-lg font-semibold text-primary">Schreinerei</h1>
          </div>
          <div className="py-4">
            <SidebarContent />
          </div>
        </SheetContent>
      </Sheet>
      <h1 className="text-lg font-semibold text-primary">Schreinerei</h1>
    </header>
  )
}
