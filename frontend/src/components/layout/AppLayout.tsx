import type { ReactNode } from "react"
import { DesktopSidebar } from "./DesktopSidebar"
import { MobileNav } from "./MobileNav"

interface AppLayoutProps {
  children: ReactNode
}

export function AppLayout({ children }: AppLayoutProps) {
  return (
    <div className="min-h-screen bg-background">
      {/* Mobile navigation */}
      <MobileNav />

      {/* Desktop sidebar */}
      <DesktopSidebar />

      {/* Main content */}
      <main className="pt-16 md:pt-0 md:pl-60">
        <div className="container mx-auto p-4 md:p-6">
          {children}
        </div>
      </main>
    </div>
  )
}
