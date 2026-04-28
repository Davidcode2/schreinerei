import { BrowserRouter, Routes, Route } from "react-router-dom"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { Toaster } from "@/components/ui/sonner"
import { AppLayout } from "@/components/layout/AppLayout"

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000,
      retry: 1,
    },
  },
})

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <AppLayout>
          <Routes>
            <Route path="/" element={<DashboardPage />} />
            <Route path="/inventory/*" element={<InventoryPage />} />
            <Route path="/sites/*" element={<SitesPage />} />
            <Route path="/fleet/*" element={<FleetPage />} />
            <Route path="/settings" element={<SettingsPage />} />
            <Route path="*" element={<NotFoundPage />} />
          </Routes>
        </AppLayout>
        <Toaster />
      </BrowserRouter>
    </QueryClientProvider>
  )
}

// Placeholder pages - will be implemented in future plans
function DashboardPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Dashboard</h1>
      <p className="text-muted-foreground">Übersicht über alle Aktivitäten</p>
    </div>
  )
}

function InventoryPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Inventar</h1>
      <p className="text-muted-foreground">Materialverwaltung</p>
    </div>
  )
}

function SitesPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Baustellen</h1>
      <p className="text-muted-foreground">Baustellenverwaltung</p>
    </div>
  )
}

function FleetPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Fuhrpark & Werkzeuge</h1>
      <p className="text-muted-foreground">Fahrzeuge und Werkzeuge verwalten</p>
    </div>
  )
}

function SettingsPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Einstellungen</h1>
      <p className="text-muted-foreground">App-Einstellungen</p>
    </div>
  )
}

function NotFoundPage() {
  return (
    <div className="flex flex-col items-center justify-center min-h-[50vh]">
      <h1 className="text-4xl font-bold mb-2">404</h1>
      <p className="text-muted-foreground">Seite nicht gefunden</p>
    </div>
  )
}

export default App
