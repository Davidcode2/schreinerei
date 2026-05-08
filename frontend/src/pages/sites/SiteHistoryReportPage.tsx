import { useMemo, useState } from 'react'
import { Link } from 'react-router-dom'
import { Filter, Search } from 'lucide-react'
import { PageHeader, LoadingSpinner, EmptyState, ErrorState } from '@/components/shared'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { useSiteHistoryReport } from '@/lib/api/hooks'
import { useUsers } from '@/lib/api/hooks/useIam'

const costBasisOptions = [
  ['budget_only', 'Nur Budget'],
  ['actuals_only', 'Nur Ist-Daten'],
  ['budget_vs_actual', 'Budget vs. Ist'],
  ['invoice_ready', 'Rechnungsreif'],
] as const

export default function SiteHistoryReportPage() {
  const [customer, setCustomer] = useState('')
  const [projectType, setProjectType] = useState('')
  const [workerId, setWorkerId] = useState('')
  const [dateFrom, setDateFrom] = useState('')
  const [dateTo, setDateTo] = useState('')
  const [durationMinHours, setDurationMinHours] = useState('')
  const [durationMaxHours, setDurationMaxHours] = useState('')
  const [costBasis, setCostBasis] = useState('')

  const query = useMemo(
    () => ({
      ...(customer ? { customer } : {}),
      ...(projectType ? { project_type: projectType } : {}),
      ...(workerId ? { worker_id: workerId } : {}),
      ...(dateFrom ? { date_from: dateFrom } : {}),
      ...(dateTo ? { date_to: dateTo } : {}),
      ...(durationMinHours ? { duration_min_hours: Number(durationMinHours) } : {}),
      ...(durationMaxHours ? { duration_max_hours: Number(durationMaxHours) } : {}),
      ...(costBasis ? { cost_basis: costBasis } : {}),
    }),
    [costBasis, customer, dateFrom, dateTo, durationMaxHours, durationMinHours, projectType, workerId]
  )

  const { data: rows, isLoading, error, refetch } = useSiteHistoryReport(query)
  const { data: users } = useUsers()

  return (
    <div className="space-y-6">
      <PageHeader
        title="Historische Auswertung"
        description="Abgeschlossene und archivierte Projekte filtern und auswerten"
      />

      <div className="grid gap-4 md:grid-cols-4">
        <div className="relative md:col-span-2">
          <Search className="absolute left-3.5 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input className="h-10 pl-10" placeholder="Kunde suchen..." value={customer} onChange={(e) => setCustomer(e.target.value)} />
        </div>

        <Select value={projectType} onValueChange={setProjectType}>
          <SelectTrigger className="h-10">
            <SelectValue placeholder="Projektart" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="external_site">Baustelle</SelectItem>
            <SelectItem value="internal_workshop">Werkstatt</SelectItem>
          </SelectContent>
        </Select>

        <Select value={costBasis} onValueChange={setCostBasis}>
          <SelectTrigger className="h-10">
            <SelectValue placeholder="Kostenbasis" />
          </SelectTrigger>
          <SelectContent>
            {costBasisOptions.map(([value, label]) => (
              <SelectItem key={value} value={value}>{label}</SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Select value={workerId} onValueChange={setWorkerId}>
          <SelectTrigger className="h-10">
            <SelectValue placeholder="Mitarbeiter" />
          </SelectTrigger>
          <SelectContent>
            {(users ?? []).map((user) => (
              <SelectItem key={user.id} value={user.id}>{user.name || user.email}</SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Input className="h-10" type="date" value={dateFrom} onChange={(e) => setDateFrom(e.target.value)} />
        <Input className="h-10" type="date" value={dateTo} onChange={(e) => setDateTo(e.target.value)} />
        <Input className="h-10" type="number" min="0" placeholder="Min Stunden" value={durationMinHours} onChange={(e) => setDurationMinHours(e.target.value)} />
        <Input className="h-10" type="number" min="0" placeholder="Max Stunden" value={durationMaxHours} onChange={(e) => setDurationMaxHours(e.target.value)} />
      </div>

      {isLoading ? (
        <LoadingSpinner className="py-8" />
      ) : error ? (
        <ErrorState message="Historische Auswertung konnte nicht geladen werden" onRetry={() => refetch()} />
      ) : !rows || rows.length === 0 ? (
        <EmptyState icon={Filter} title="Keine historischen Projekte" description="Passen Sie die Filter an, um abgeschlossene Projekte anzuzeigen." />
      ) : (
        <div className="space-y-3">
          {rows.map((row) => (
            <Link key={row.site_id} to={`/sites/${row.site_id}`} className="block rounded-xl border bg-card p-4 shadow-sm transition hover:border-primary/30 hover:shadow-md">
              <div className="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
                <div>
                  <p className="font-medium">{row.name}</p>
                  <p className="text-sm text-muted-foreground">{row.customer_name || 'Ohne Kundenbezug'}</p>
                </div>
                <div className="text-right text-sm text-muted-foreground">
                  <p>{row.project_type === 'internal_workshop' ? 'Werkstatt' : 'Baustelle'}</p>
                  <p>{row.status}</p>
                </div>
              </div>
              <div className="mt-4 grid gap-3 sm:grid-cols-4 text-sm">
                <div>
                  <p className="text-muted-foreground">Stunden</p>
                  <p className="font-medium">{row.total_hours.toFixed(1)}h</p>
                </div>
                <div>
                  <p className="text-muted-foreground">Mitarbeiter</p>
                  <p className="font-medium">{row.worker_count}</p>
                </div>
                <div>
                  <p className="text-muted-foreground">Materialien</p>
                  <p className="font-medium">{row.distinct_material_count}</p>
                </div>
                <div>
                  <p className="text-muted-foreground">Kostenbasis</p>
                  <p className="font-medium">{row.cost_basis}</p>
                </div>
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  )
}
