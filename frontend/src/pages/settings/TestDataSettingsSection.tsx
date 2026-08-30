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
  const testDataState = status.data?.state ?? "absent"

  function installTestData() {
    install.mutate(undefined, {
      onSuccess: () => toast.success("Testdaten wurden importiert"),
      onError: () => toast.error("Testdaten konnten nicht importiert werden"),
    })
  }

  function removeTestData() {
    remove.mutate(undefined, {
      onSuccess: (result) => {
        setConfirmRemoval(false)
        if (result.state === "partial") {
          toast.warning(
            `${result.removed_records} Testdatensätze entfernt, ${result.retained_records} wegen eigener Verknüpfungen beibehalten`,
          )
          return
        }
        toast.success(`${result.removed_records} Testdatensätze wurden entfernt`)
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
        {testDataState === "partial" && (
          <p className="rounded-md border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm text-amber-800 dark:text-amber-200">
            {status.data?.retained_records} Testdatensätze bleiben wegen Verknüpfungen mit eigenen
            Daten erhalten. Ein erneuter Import ergänzt fehlende Testdaten.
          </p>
        )}
        <div className="flex flex-col gap-3 sm:flex-row sm:justify-end">
          <Button
            variant="outline"
            className="gap-2"
            onClick={installTestData}
            disabled={status.isLoading || testDataState === "complete" || isPending}
          >
            <Download className="h-4 w-4" />
            {install.isPending ? "Wird importiert..." : "Testdaten importieren"}
          </Button>
          <Button
            variant="destructive"
            className="gap-2"
            onClick={() => setConfirmRemoval(true)}
            disabled={status.isLoading || testDataState === "absent" || isPending}
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
