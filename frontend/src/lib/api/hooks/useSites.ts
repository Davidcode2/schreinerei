import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query"
import { apiClient } from "../client"
import type {
  Site,
  CreateSiteRequest,
  UpdateSiteRequest,
  ListSitesQuery,
  SiteAssignment,
  AssignUserRequest,
  TimeEntry,
  CreateTimeEntryRequest,
  Activity,
  CreateActivityRequest,
  ActivityQuery,
  DashboardSite,
} from "@/types/sites"

// === Sites ===

export function useSites(query?: ListSitesQuery) {
  return useQuery({
    queryKey: ["sites", query],
    queryFn: () => {
      const params = query?.status ? `?status=${query.status}` : ""
      return apiClient.get<Site[]>(`/api/v1/sites${params}`)
    },
  })
}

export function useSite(id: string) {
  return useQuery({
    queryKey: ["site", id],
    queryFn: () => apiClient.get<Site>(`/api/v1/sites/${id}`),
    enabled: !!id,
  })
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

// === Assignments ===

export function useSiteAssignments(siteId: string) {
  return useQuery({
    queryKey: ["site-assignments", siteId],
    queryFn: () =>
      apiClient.get<SiteAssignment[]>(`/api/v1/sites/${siteId}/assignments`),
    enabled: !!siteId,
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
        `/api/v1/sites/${siteId}/assignments`,
        data
      ),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: ["site-assignments", variables.siteId],
      })
    },
  })
}

// === Time Entries ===

export function useTimeEntries(siteId?: string) {
  return useQuery({
    queryKey: ["time-entries", siteId],
    queryFn: () => {
      const params = siteId ? `?site_id=${siteId}` : ""
      return apiClient.get<TimeEntry[]>(`/api/v1/time-entries${params}`)
    },
  })
}

export function useMyTimeEntries() {
  return useQuery({
    queryKey: ["my-time-entries"],
    queryFn: () => apiClient.get<TimeEntry[]>("/api/v1/time-entries/my"),
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

// === Dashboard ===

export function useDashboardSites() {
  return useQuery({
    queryKey: ["dashboard-sites"],
    queryFn: () =>
      apiClient.get<DashboardSite[]>("/api/v1/dashboard/sites"),
  })
}
