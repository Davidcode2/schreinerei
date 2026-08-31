import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Info, LogOut } from "lucide-react"
import { useAuth } from "@/hooks/useAuth"
import { useLogout } from "@/hooks/useLogout"
import { PageHeader } from "@/components/shared"
import { BillingSettingsSection } from "./BillingSettingsSection"
import { ProfileSection } from "./ProfileSection"
import { UserManagementSection } from "./UserManagementSection"
import { TestDataSettingsSection } from "./TestDataSettingsSection"

const APP_VERSION = "1.0.0"

export default function SettingsPage() {
  const { user } = useAuth()
  const handleLogout = useLogout()

  const isAdmin = user?.role === "admin"

  return (
    <div className="space-y-6 max-w-2xl mx-auto">
      <PageHeader
        title="Einstellungen"
        description="App-Einstellungen und Profil"
      />

      <ProfileSection />

      <BillingSettingsSection isAdmin={isAdmin} />

      <UserManagementSection isAdmin={isAdmin} />

      {isAdmin ? <TestDataSettingsSection /> : null}

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-3 font-display text-lg">
            <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
              <Info className="h-4 w-4" />
            </span>
            Über
          </CardTitle>
          <CardDescription>Informationen zur Anwendung</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 gap-4 text-sm sm:grid-cols-2">
            <div className="rounded-lg bg-accent/50 p-3">
              <p className="text-muted-foreground">Version</p>
              <p className="font-medium mt-0.5">{APP_VERSION}</p>
            </div>
            <div className="rounded-lg bg-accent/50 p-3">
              <p className="text-muted-foreground">Umgebung</p>
              <p className="font-medium mt-0.5 capitalize">
                {import.meta.env.MODE || "development"}
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card className="border-destructive/30 overflow-hidden">
        <CardContent className="p-4">
          <div className="flex flex-col items-start gap-4 sm:flex-row sm:items-center sm:justify-between">
            <div className="flex items-center gap-3">
              <span className="flex h-9 w-9 flex-shrink-0 items-center justify-center rounded-lg bg-destructive/10">
                <LogOut className="h-4 w-4 text-destructive" />
              </span>
              <div>
                <p className="font-medium">Abmelden</p>
                <p className="text-sm text-muted-foreground">
                  Von Ihrem Konto abmelden
                </p>
              </div>
            </div>
            <Button
              variant="outline"
              onClick={handleLogout}
              className="w-full gap-2 text-destructive transition-transform hover:bg-destructive/10 hover:text-destructive active:scale-[0.97] sm:w-auto"
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
