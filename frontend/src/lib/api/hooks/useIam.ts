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

export interface PublicInviteResponse {
  email: string
  role: string
  status: string
  expires_at: string
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

export function useInviteUser() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (data: InviteUserRequest) =>
      apiClient.post<InviteUserResponse>("/api/v1/users/invite", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["users"] })
    },
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
