import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query"
import { apiClient } from "../client"
import type {
  Site,
  CreateSiteRequest,
  UpdateSiteRequest,
  ListSitesQuery,
  SiteAssignment,
  SiteAppointment,
  SiteAppointmentsQuery,
  AssignUserRequest,
  CreateSiteAppointmentRequest,
  TimeEntry,
  CreateTimeEntryRequest,
  UpdateTimeEntryRequest,
  UpdateSiteAppointmentRequest,
  Activity,
  CreateActivityRequest,
  ActivityQuery,
  DashboardSite,
  SiteHistoryReportQuery,
  SiteHistoryReportRow,
  SiteInvoice,
  SiteInvoiceSummary,
  SiteProjectSummary,
} from "@/types/sites"
import type {
  ProjectInvoiceDraftResponse,
  UploadPhotoAttachmentResponse,
  UploadSiteAttachmentResponse,
} from "@/types/generated"

// === Sites ===

export function useSites(query?: ListSitesQuery) {
  return useQuery({
    queryKey: ["sites", query],
    queryFn: () => {
      const params = query?.status ? `?status=${query.status}` : ""
      return apiClient.get<Site[]>(`/api/v1/sites${params}`)
    },
    staleTime: 30000,
  })
}

export function useSiteHistoryReport(query?: SiteHistoryReportQuery, enabled: boolean = true) {
  return useQuery({
    queryKey: ["site-history-report", query],
    queryFn: () => {
      const params = new URLSearchParams()
      if (query?.customer) params.set('customer', query.customer)
      if (query?.project_type) params.set('project_type', query.project_type)
      if (query?.worker_id) params.set('worker_id', query.worker_id)
      if (query?.date_from) params.set('date_from', query.date_from)
      if (query?.date_to) params.set('date_to', query.date_to)
      if (query?.duration_min_hours != null) params.set('duration_min_hours', String(query.duration_min_hours))
      if (query?.duration_max_hours != null) params.set('duration_max_hours', String(query.duration_max_hours))
      if (query?.cost_basis) params.set('cost_basis', query.cost_basis)
      const queryString = params.toString()
      return apiClient.get<SiteHistoryReportRow[]>(`/api/v1/sites/history-report${queryString ? `?${queryString}` : ''}`)
    },
    enabled,
    staleTime: 30000,
  })
}

export function useSite(id: string) {
  return useQuery({
    queryKey: ["site", id],
    queryFn: () => apiClient.get<Site>(`/api/v1/sites/${id}`),
    enabled: !!id,
    staleTime: 30000,
  })
}

export function useSiteSummary(id: string) {
  return useQuery({
    queryKey: ["site-summary", id],
    queryFn: () => apiClient.get<SiteProjectSummary>(`/api/v1/sites/${id}/summary`),
    enabled: !!id,
    staleTime: 30000,
  })
}

export function useSiteInvoiceSummary(id: string, enabled: boolean = true) {
  return useQuery({
    queryKey: ["site-invoice-summary", id],
    queryFn: () => apiClient.get<SiteInvoiceSummary>(`/api/v1/sites/${id}/invoice-summary`),
    enabled: !!id && enabled,
    staleTime: 30000,
  })
}

export function useSiteInvoices(siteId: string, enabled: boolean = true) {
  return useQuery({
    queryKey: ["site-invoices", siteId],
    queryFn: () =>
      apiClient.get<SiteInvoice[]>(`/api/v1/sites/${siteId}/invoices`),
    enabled: !!siteId && enabled,
    staleTime: 30000,
  })
}

export function useCreateSiteInvoice() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (siteId: string) =>
      apiClient
        .post<ProjectInvoiceDraftResponse>(`/api/v1/billing/projects/${siteId}/invoices`, {})
        .then((draft) => draft.invoice),
    onSuccess: (_, siteId) => {
      queryClient.invalidateQueries({ queryKey: ["site-invoices", siteId] })
      queryClient.invalidateQueries({ queryKey: ["site-summary", siteId] })
      queryClient.invalidateQueries({ queryKey: ["site-invoice-summary", siteId] })
    },
  })
}

export function downloadSiteInvoicePdf(invoiceId: string): Promise<Blob> {
  return apiClient.getBlob(`/api/v1/billing/invoices/${invoiceId}/pdf`)
}

export function useCreateSite() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: CreateSiteRequest) =>
      apiClient.post<Site>("/api/v1/sites", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["sites"] })
      queryClient.invalidateQueries({ queryKey: ["dashboard-sites"] })
    },
  })
}

export function useUpdateSite() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, ...data }: UpdateSiteRequest & { id: string }) =>
      apiClient.patch<Site>(`/api/v1/sites/${id}`, data),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ["sites"] })
      queryClient.invalidateQueries({ queryKey: ["site", variables.id] })
      queryClient.invalidateQueries({ queryKey: ["dashboard-sites"] })
    },
  })
}

export function useDeleteSite() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) =>
      apiClient.delete(`/api/v1/sites/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["sites"] })
      queryClient.invalidateQueries({ queryKey: ["dashboard-sites"] })
    },
  })
}

// === Assignments ===

export function useSiteAssignments(siteId: string) {
  return useQuery({
    queryKey: ["site-assignments", siteId],
    queryFn: () =>
      apiClient.get<SiteAssignment[]>(`/api/v1/sites/${siteId}/assignments`),
    enabled: !!siteId,
    staleTime: 30000,
  })
}

export function useAssignUser() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({
      siteId,
      ...data
    }: AssignUserRequest & { siteId: string }) =>
      apiClient.post<SiteAssignment>(
        `/api/v1/sites/${siteId}/assign`,
        data
      ),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: ["site-assignments", variables.siteId],
      })
    },
  })
}

export function useRemoveAssignment() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({
      siteId,
      userId,
    }: {
      siteId: string
      userId: string
    }) => apiClient.delete(`/api/v1/sites/${siteId}/assign/${userId}`),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: ["site-assignments", variables.siteId],
      })
      queryClient.invalidateQueries({ queryKey: ["site", variables.siteId] })
      queryClient.invalidateQueries({ queryKey: ["dashboard-sites"] })
    },
  })
}

// === Site Appointments ===

export function useSiteAppointments(siteId: string, query?: SiteAppointmentsQuery) {
  return useQuery({
    queryKey: ["site-appointments", siteId, query],
    queryFn: () => {
      const params = new URLSearchParams()
      if (query?.start_date) params.set("start_date", query.start_date)
      if (query?.end_date) params.set("end_date", query.end_date)
      const queryString = params.toString()
      return apiClient.get<SiteAppointment[]>(
        `/api/v1/sites/${siteId}/appointments${queryString ? `?${queryString}` : ""}`
      )
    },
    enabled: !!siteId,
    staleTime: 30000,
  })
}

export function useCreateSiteAppointment() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({
      siteId,
      ...data
    }: CreateSiteAppointmentRequest & { siteId: string }) =>
      apiClient.post<SiteAppointment>(`/api/v1/sites/${siteId}/appointments`, data),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ["site-appointments", variables.siteId] })
    },
  })
}

export function useUpdateSiteAppointment() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({
      siteId,
      appointmentId,
      ...data
    }: UpdateSiteAppointmentRequest & { siteId: string; appointmentId: string }) =>
      apiClient.patch<SiteAppointment>(
        `/api/v1/sites/${siteId}/appointments/${appointmentId}`,
        data
      ),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ["site-appointments", variables.siteId] })
    },
  })
}

export function useDeleteSiteAppointment() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({
      siteId,
      appointmentId,
    }: {
      siteId: string
      appointmentId: string
    }) => apiClient.delete(`/api/v1/sites/${siteId}/appointments/${appointmentId}`),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ["site-appointments", variables.siteId] })
    },
  })
}

// === Time Entries ===

export function useTimeEntries(siteId?: string) {
  return useQuery({
    queryKey: ["time-entries", siteId],
    queryFn: () => {
      if (siteId) {
        return apiClient.get<TimeEntry[]>(`/api/v1/sites/${siteId}/time-entries`)
      }
      return apiClient.get<TimeEntry[]>("/api/v1/time-entries/my")
    },
    staleTime: 30000,
  })
}

export function useMyTimeEntries() {
  return useQuery({
    queryKey: ["my-time-entries"],
    queryFn: () => apiClient.get<TimeEntry[]>("/api/v1/time-entries/my"),
    staleTime: 30000,
  })
}

export function useCreateTimeEntry() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: CreateTimeEntryRequest) =>
      apiClient.post<TimeEntry>("/api/v1/time-entries", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["time-entries"] })
      queryClient.invalidateQueries({ queryKey: ["my-time-entries"] })
      queryClient.invalidateQueries({ queryKey: ["dashboard-sites"] })
      queryClient.invalidateQueries({ queryKey: ["site-summary"] })
    },
  })
}

export function useUpdateTimeEntry() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, ...data }: UpdateTimeEntryRequest & { id: string }) =>
      apiClient.patch<TimeEntry>(`/api/v1/time-entries/${id}`, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["time-entries"] })
      queryClient.invalidateQueries({ queryKey: ["my-time-entries"] })
      queryClient.invalidateQueries({ queryKey: ["dashboard-sites"] })
      queryClient.invalidateQueries({ queryKey: ["site-summary"] })
    },
  })
}

export function useDeleteTimeEntry() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) =>
      apiClient.delete(`/api/v1/time-entries/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["time-entries"] })
      queryClient.invalidateQueries({ queryKey: ["my-time-entries"] })
      queryClient.invalidateQueries({ queryKey: ["dashboard-sites"] })
      queryClient.invalidateQueries({ queryKey: ["site-summary"] })
    },
  })
}

// === Activities ===

export function useActivities(siteId: string, query?: ActivityQuery) {
  return useQuery({
    queryKey: ["activities", siteId, query],
    queryFn: () => {
      const params = query?.limit ? `?limit=${query.limit}` : ""
      return apiClient.get<Activity[]>(
        `/api/v1/sites/${siteId}/activities${params}`
      )
    },
    enabled: !!siteId,
    staleTime: 30000,
  })
}

export function useCreateActivity() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({
      siteId,
      ...data
    }: CreateActivityRequest & { siteId: string }) =>
      apiClient.post<Activity>(`/api/v1/sites/${siteId}/activities`, data),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: ["activities", variables.siteId],
      })
    },
  })
}

export function useDeleteActivity() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({
      siteId,
      activityId,
    }: {
      siteId: string
      activityId: string
    }) => apiClient.delete(`/api/v1/sites/${siteId}/activities/${activityId}`),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: ["activities", variables.siteId],
      })
    },
  })
}

export function useUploadSitePhoto() {
  return useMutation({
    mutationFn: async ({
      siteId,
      file,
    }: {
      siteId: string
      file: File
    }) => {
      const formData = new FormData()
      formData.append("photo", file)

      return apiClient.post<UploadPhotoAttachmentResponse>(
        `/api/v1/sites/${siteId}/attachments/photo`,
        formData
      )
    },
  })
}

export function useUploadSiteAttachment() {
  return useMutation({
    mutationFn: async ({
      siteId,
      file,
    }: {
      siteId: string
      file: File
    }) => {
      const formData = new FormData()
      formData.append("attachment", file)

      return apiClient.post<UploadSiteAttachmentResponse>(
        `/api/v1/sites/${siteId}/attachments`,
        formData
      )
    },
  })
}

// === Dashboard ===

export function useDashboardSites() {
  return useQuery({
    queryKey: ["dashboard-sites"],
    queryFn: () =>
      apiClient.get<DashboardSite[]>("/api/v1/dashboard/sites"),
    staleTime: 30000,
  })
}
