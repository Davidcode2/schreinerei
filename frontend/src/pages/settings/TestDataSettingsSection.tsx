import { useState } from "react"
import { DatabaseZap, Download, Trash2 } from "lucide-react"
import { toast } from "sonner"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { DeleteConfirmDialog } from "@/components/shared/DeleteConfirmDialog"
import {
  useInstallTestData,
  useRemoveTestData,
  useTestDataStatus,
} from "@/lib/api/hooks/useIam"

export function TestDataSettingsSection() {
  const [confirmRemoval, setConfirmRemoval] = useState(false)
  const status = useTestDataStatus()
  const install = useInstallTestData()
  const remove = useRemoveTestData()
  const isPending = install.isPending || remove.isPending
  const isInstalled = status.data?.installed ?? false

  function installTestData() {
    install.mutate(undefined, {
      onSuccess: () => toast.success("Testdaten wurden importiert"),
      onError: () => toast.error("Testdaten konnten nicht importiert werden"),
    })
  }

  function removeTestData() {
    remove.mutate(undefined, {
      onSuccess: () => {
        setConfirmRemoval(false)
        toast.success("Testdaten wurden entfernt")
      },
      onError: () => toast.error("Testdaten konnten nicht entfernt werden"),
    })
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-3 font-display text-lg">
          <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
            <DatabaseZap className="h-4 w-4" />
          </span>
          Testdaten
        </CardTitle>
        <CardDescription>
          Beispielprojekte, Material und Betriebsmittel für diese Organisation verwalten.
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <p className="text-sm text-muted-foreground">
          Beim Entfernen bleiben alle unabhängig angelegten Daten erhalten.
        </p>
        <div className="flex flex-col gap-3 sm:flex-row sm:justify-end">
          <Button
            variant="outline"
            className="gap-2"
            onClick={installTestData}
            disabled={status.isLoading || isInstalled || isPending}
          >
            <Download className="h-4 w-4" />
            {install.isPending ? "Wird importiert..." : "Testdaten importieren"}
          </Button>
          <Button
            variant="destructive"
            className="gap-2"
            onClick={() => setConfirmRemoval(true)}
            disabled={status.isLoading || !isInstalled || isPending}
          >
            <Trash2 className="h-4 w-4" />
            Testdaten entfernen
          </Button>
        </div>
      </CardContent>
      <DeleteConfirmDialog
        open={confirmRemoval}
        onOpenChange={setConfirmRemoval}
        onConfirm={removeTestData}
        itemName="die importierten Testdaten"
        title="Testdaten entfernen?"
        actionLabel="Entfernen"
        isPending={remove.isPending}
      />
    </Card>
  )
}
