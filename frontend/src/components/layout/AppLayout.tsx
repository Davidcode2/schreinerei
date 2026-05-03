import type { ReactNode } from "react"
import { DesktopSidebar } from "./DesktopSidebar"
import { MobileNav } from "./MobileNav"

interface AppLayoutProps {
  children: ReactNode
}

export function AppLayout({ children }: AppLayoutProps) {
  return (
    <div className="min-h-screen bg-background">
      <MobileNav />
      <DesktopSidebar />
      <main className="pt-14 md:pt-0 md:pl-64">
        <div className="mx-auto max-w-6xl px-4 py-5 md:px-8 md:py-6">
          {children}
        </div>
      </main>
    </div>
  )
}
