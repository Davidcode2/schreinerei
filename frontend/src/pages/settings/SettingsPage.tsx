import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Info, LogOut } from "lucide-react"
import { useAuth } from "@/hooks/useAuth"
import { useAuthStore } from "@/lib/auth/authStore"
import { PageHeader } from "@/components/shared"
import { ProfileSection } from "./ProfileSection"
import { UserManagementSection } from "./UserManagementSection"

const APP_VERSION = "1.0.0"

export default function SettingsPage() {
  const { user } = useAuth()
  const logout = useAuthStore((state) => state.logout)

  const handleLogout = () => {
    logout()
  }

  const isAdmin = user?.role === "admin"

  return (
    <div className="space-y-6 max-w-2xl mx-auto">
      <PageHeader
        title="Einstellungen"
        description="App-Einstellungen und Profil"
      />

      <ProfileSection />

      <UserManagementSection isAdmin={isAdmin} />

      {/* About Section */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Info className="h-5 w-5" />
            Über
          </CardTitle>
          <CardDescription>Informationen zur Anwendung</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <p className="text-muted-foreground">Version</p>
              <p className="font-medium">{APP_VERSION}</p>
            </div>
            <div>
              <p className="text-muted-foreground">Umgebung</p>
              <p className="font-medium capitalize">
                {import.meta.env.MODE || "development"}
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Logout */}
      <Card className="border-destructive/50">
        <CardContent className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="font-medium">Abmelden</p>
              <p className="text-sm text-muted-foreground">
                Von Ihrem Konto abmelden
              </p>
            </div>
            <Button
              variant="outline"
              onClick={handleLogout}
              className="gap-2 text-destructive hover:text-destructive"
            >
              <LogOut className="h-4 w-4" />
              Abmelden
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
