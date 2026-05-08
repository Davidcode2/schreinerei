import { useEffect } from "react"
import { BrowserRouter, Routes, Route } from "react-router-dom"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { Toaster } from "@/components/ui/sonner"
import { AppLayout } from "@/components/layout/AppLayout"
import { AuthGuard } from "@/components/auth/AuthGuard"
import { LoginPage } from "@/components/auth/LoginPage"
import { AuthCallback } from "@/components/auth/AuthCallback"
import { useAuth } from "@/hooks/useAuth"
import { initSync } from "@/lib/offline/sync"
import OfflineIndicator from "@/components/offline/OfflineIndicator"
import InstallPrompt from "@/components/pwa/InstallPrompt"
import DashboardPage from "@/pages/DashboardPage"
import { InventoryListPage, InventoryDetailPage } from "@/pages/inventory"
import { SitesListPage, SiteDetailPage, SiteHistoryReportPage } from "@/pages/sites"
import { FleetPage, ToolDetailPage, ToolsPage, VehicleDetailPage } from "@/pages/fleet"
import { InventorySettingsPage, SettingsPage } from "@/pages/settings"
import ScanPage from "@/pages/qr/ScanPage"
import NotFoundPage from "@/pages/NotFoundPage"

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000,
      retry: 1,
    },
  },
})

function AppRoutes() {
  const { isLoading, isAuthenticated } = useAuth()

  // Initialize offline sync when authenticated
  useEffect(() => {
    if (isAuthenticated) {
      const cleanup = initSync()
      return cleanup
    }
  }, [isAuthenticated])

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      </div>
    )
  }

  return (
    <Routes>
      {/* Public routes */}
      <Route path="/login" element={<LoginPage />} />
      <Route path="/auth/callback" element={<AuthCallback />} />

      {/* Protected routes */}
      <Route
        path="/*"
        element={
          <AuthGuard>
            <AppLayout>
              <Routes>
                <Route path="/" element={<DashboardPage />} />

                {/* Inventory */}
                <Route path="/inventory" element={<InventoryListPage />} />
                <Route path="/inventory/:id" element={<InventoryDetailPage />} />

                {/* Sites */}
                <Route path="/sites" element={<SitesListPage />} />
                <Route path="/sites/history" element={<SiteHistoryReportPage />} />
                <Route path="/sites/:id" element={<SiteDetailPage />} />
                <Route path="/sites/:id/media/:activityId/:attachmentId/:slug?" element={<SiteDetailPage />} />

                {/* Fleet */}
                <Route path="/fleet" element={<FleetPage />} />
                <Route path="/fleet/:id" element={<VehicleDetailPage />} />
                <Route path="/tools" element={<ToolsPage />} />
                <Route path="/tools/:id" element={<ToolDetailPage />} />

                {/* QR Scanner */}
                <Route path="/scan" element={<ScanPage />} />

                {/* Settings */}
                <Route path="/settings" element={<SettingsPage />} />
                <Route
                  path="/settings/inventory"
                  element={<InventorySettingsPage />}
                />

                {/* 404 */}
                <Route path="*" element={<NotFoundPage />} />
              </Routes>
            </AppLayout>
          </AuthGuard>
        }
      />
    </Routes>
  )
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <AppRoutes />
        <Toaster />
        <OfflineIndicator />
        <InstallPrompt />
      </BrowserRouter>
    </QueryClientProvider>
  )
}

export default App
