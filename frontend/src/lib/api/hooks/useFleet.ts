import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query"
import { apiClient } from "../client"
import type {
  Vehicle,
  CreateVehicleRequest,
  UpdateVehicleRequest,
  ListVehiclesQuery,
  Tool,
  CreateToolRequest,
  UpdateToolRequest,
  ListToolsQuery,
  Machine,
  CreateMachineRequest,
  UpdateMachineRequest,
  ListMachinesQuery,
  Reservation,
  CreateReservationRequest,
  UpdateReservationRequest,
  ListReservationsQuery,
  CalendarResponse,
  CalendarQuery,
  AvailabilityResponse,
  AvailabilityQuery,
} from "@/types/fleet"

// === Vehicles ===

export function useVehicles(query?: ListVehiclesQuery) {
  return useQuery({
    queryKey: ["vehicles", query],
    queryFn: () => {
      const params = query?.status ? `?status=${query.status}` : ""
      return apiClient.get<Vehicle[]>(`/api/v1/fleet/vehicles${params}`)
    },
    staleTime: 30000,
  })
}

export function useVehicle(id: string) {
  return useQuery({
    queryKey: ["vehicle", id],
    queryFn: () => apiClient.get<Vehicle>(`/api/v1/fleet/vehicles/${id}`),
    enabled: !!id,
    staleTime: 30000,
  })
}

export function useCreateVehicle() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: CreateVehicleRequest) =>
      apiClient.post<Vehicle>("/api/v1/fleet/vehicles", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["vehicles"] })
    },
  })
}

export function useUpdateVehicle() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, ...data }: UpdateVehicleRequest & { id: string }) =>
      apiClient.patch<Vehicle>(`/api/v1/fleet/vehicles/${id}`, data),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ["vehicles"] })
      queryClient.invalidateQueries({ queryKey: ["vehicle", variables.id] })
    },
  })
}

export function useDeleteVehicle() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) =>
      apiClient.delete(`/api/v1/fleet/vehicles/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["vehicles"] })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}

// === Tools ===

export function useTools(query?: ListToolsQuery) {
  return useQuery({
    queryKey: ["tools", query],
    queryFn: () => {
      const params = new URLSearchParams()
      if (query?.status) params.set("status", query.status)
      if (query?.category) params.set("category", query.category)
      const queryString = params.toString()
      return apiClient.get<Tool[]>(
        `/api/v1/fleet/tools${queryString ? `?${queryString}` : ""}`
      )
    },
    staleTime: 30000,
  })
}

export function useTool(id: string) {
  return useQuery({
    queryKey: ["tool", id],
    queryFn: () => apiClient.get<Tool>(`/api/v1/fleet/tools/${id}`),
    enabled: !!id,
    staleTime: 30000,
  })
}

export function useCreateTool() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: CreateToolRequest) =>
      apiClient.post<Tool>("/api/v1/fleet/tools", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["tools"] })
    },
  })
}

export function useUpdateTool() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, ...data }: UpdateToolRequest & { id: string }) =>
      apiClient.patch<Tool>(`/api/v1/fleet/tools/${id}`, data),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ["tools"] })
      queryClient.invalidateQueries({ queryKey: ["tool", variables.id] })
    },
  })
}

export function useDeleteTool() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) =>
      apiClient.delete(`/api/v1/fleet/tools/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["tools"] })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}

// === Machines ===

export function useMachines(query?: ListMachinesQuery) {
  return useQuery({
    queryKey: ["machines", query],
    queryFn: () => {
      const params = query?.status ? `?status=${query.status}` : ""
      return apiClient.get<Machine[]>(`/api/v1/fleet/machines${params}`)
    },
    staleTime: 30000,
  })
}

export function useMachine(id: string) {
  return useQuery({
    queryKey: ["machine", id],
    queryFn: () => apiClient.get<Machine>(`/api/v1/fleet/machines/${id}`),
    enabled: !!id,
    staleTime: 30000,
  })
}

export function useCreateMachine() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: CreateMachineRequest) =>
      apiClient.post<Machine>("/api/v1/fleet/machines", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["machines"] })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}

export function useUpdateMachine() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, ...data }: UpdateMachineRequest & { id: string }) =>
      apiClient.patch<Machine>(`/api/v1/fleet/machines/${id}`, data),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ["machines"] })
      queryClient.invalidateQueries({ queryKey: ["machine", variables.id] })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}

export function useDeleteMachine() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) =>
      apiClient.delete(`/api/v1/fleet/machines/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["machines"] })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}

// === Reservations ===

export function useReservations(query?: ListReservationsQuery) {
  return useQuery({
    queryKey: ["reservations", query],
    queryFn: () => {
      const params = new URLSearchParams()
      if (query?.user_id) params.set("user_id", query.user_id)
      if (query?.resource_type) params.set("resource_type", query.resource_type)
      if (query?.resource_id) params.set("resource_id", query.resource_id)
      if (query?.site_id) params.set("site_id", query.site_id)
      const queryString = params.toString()
      return apiClient.get<Reservation[]>(
        `/api/v1/fleet/reservations${queryString ? `?${queryString}` : ""}`
      )
    },
    staleTime: 30000,
  })
}

export function useMyReservations() {
  return useQuery({
    queryKey: ["reservations", "my"],
    queryFn: () => apiClient.get<Reservation[]>("/api/v1/fleet/reservations/my"),
    staleTime: 30000,
  })
}

export function useReservation(id: string | null) {
  return useQuery({
    queryKey: ["reservation", id],
    queryFn: () => apiClient.get<Reservation>(`/api/v1/fleet/reservations/${id}`),
    enabled: !!id,
    staleTime: 30000,
  })
}

export function useCreateReservation() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: CreateReservationRequest) =>
      apiClient.post<Reservation>("/api/v1/fleet/reservations", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["reservations"] })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}

export function useUpdateReservation() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({
      id,
      ...data
    }: UpdateReservationRequest & { id: string }) =>
      apiClient.patch<Reservation>(`/api/v1/fleet/reservations/${id}`, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["reservations"] })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}

export function useCancelReservation() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) =>
      apiClient.delete<void>(`/api/v1/fleet/reservations/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["reservations"] })
      queryClient.invalidateQueries({ queryKey: ["calendar"] })
    },
  })
}

// === Calendar ===

export function useCalendar(query: CalendarQuery) {
  return useQuery({
    queryKey: ["calendar", query],
    queryFn: () => {
      const params = new URLSearchParams()
      params.set("start_date", query.start_date)
      params.set("end_date", query.end_date)
      if (query.resource_type) params.set("resource_type", query.resource_type)
      if (query.site_id) params.set("site_id", query.site_id)
      return apiClient.get<CalendarResponse>(
        `/api/v1/fleet/calendar?${params.toString()}`
      )
    },
    enabled: !!query.start_date && !!query.end_date,
    staleTime: 30000,
  })
}

// === Availability ===

export function useAvailability(query: AvailabilityQuery) {
  return useQuery({
    queryKey: ["availability", query],
    queryFn: () => {
      const params = new URLSearchParams()
      params.set("resource_type", query.resource_type)
      params.set("resource_id", query.resource_id)
      params.set("start_time", query.start_time)
      params.set("end_time", query.end_time)
      return apiClient.get<AvailabilityResponse>(
        `/api/v1/fleet/availability?${params.toString()}`
      )
    },
    enabled:
      !!query.resource_type &&
      !!query.resource_id &&
      !!query.start_time &&
      !!query.end_time,
    staleTime: 30000,
  })
}
