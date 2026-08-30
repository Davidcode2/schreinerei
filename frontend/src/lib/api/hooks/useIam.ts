import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { apiClient } from "../client"
import { useAuthStore } from "../../auth/authStore"

export interface User {
  id: string
  email: string
  name: string | null
  role: string
  created_at: string
}

export interface InviteUserRequest {
  email: string
  name?: string | null
  role: string
}

export interface InviteUserResponse {
  id: string
  email: string
  role: string
  status: string
  invite_url: string
  organization_alias: string
  expires_at: string
}

export interface PendingInviteResponse {
  id: string
  email: string
  role: string
  status: string
  expires_at: string
  created_at: string
}

export interface PublicInviteResponse {
  email: string
  role: string
  status: string
  expires_at: string
}

export interface BillingSettings {
  default_hourly_rate_cents: number | null
  billing_tax_mode: "standard" | "kleinunternehmer"
  sender_name: string
  sender_address: string | null
}

export interface UpdateBillingSettingsRequest {
  default_hourly_rate_cents: number | null
  billing_tax_mode?: "standard" | "kleinunternehmer" | null
  sender_name?: string | null
  sender_address?: string | null
}

export interface TestDataStatus {
  installed: boolean
}

export function useUsers() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated)
  return useQuery({
    queryKey: ["users"],
    queryFn: () => apiClient.get<User[]>("/api/v1/users"),
    staleTime: 30000,
    enabled: isAuthenticated,
  })
}

export function useBillingSettings() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated)

  return useQuery({
    queryKey: ["billing-settings"],
    queryFn: () => apiClient.get<BillingSettings>("/api/v1/settings/billing"),
    staleTime: 30000,
    enabled: isAuthenticated,
  })
}

export function useUpdateBillingSettings() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: UpdateBillingSettingsRequest) =>
      apiClient.patch<BillingSettings>("/api/v1/settings/billing", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["billing-settings"] })
    },
  })
}

export function useTestDataStatus() {
  return useQuery({
    queryKey: ["test-data-status"],
    queryFn: () => apiClient.get<TestDataStatus>("/api/v1/settings/test-data"),
  })
}

export function useInstallTestData() {
  return useTestDataMutation(() =>
    apiClient.post<TestDataStatus>("/api/v1/settings/test-data")
  )
}

export function useRemoveTestData() {
  return useTestDataMutation(() =>
    apiClient.delete<TestDataStatus>("/api/v1/settings/test-data")
  )
}

function useTestDataMutation(mutationFn: () => Promise<TestDataStatus>) {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn,
    onSuccess: (status) => {
      queryClient.setQueryData(["test-data-status"], status)
      queryClient.invalidateQueries({
        predicate: (query) => query.queryKey[0] !== "test-data-status",
      })
    },
  })
}

export function useInviteUser() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (data: InviteUserRequest) =>
      apiClient.post<InviteUserResponse>("/api/v1/users/invite", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["users"] })
      queryClient.invalidateQueries({ queryKey: ["pending-invites"] })
    },
  })
}

export function usePendingInvites() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated)
  return useQuery({
    queryKey: ["pending-invites"],
    queryFn: () => apiClient.get<PendingInviteResponse[]>("/api/v1/users/invites"),
    staleTime: 30000,
    enabled: isAuthenticated,
  })
}

export function usePublicInvite(token: string | null) {
  return useQuery({
    queryKey: ["public-invite", token],
    queryFn: () =>
      apiClient.get<PublicInviteResponse>(
        `/api/v1/onboarding/invites/${encodeURIComponent(token ?? "")}`
      ),
    enabled: Boolean(token),
    retry: false,
  })
}
