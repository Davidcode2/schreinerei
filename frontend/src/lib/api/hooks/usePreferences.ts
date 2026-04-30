import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { apiClient } from "../client"
import type {
  PreferencesResponse,
  UpdatePreferencesRequest,
} from "@/types/generated"

export function usePreferences() {
  return useQuery({
    queryKey: ["preferences"],
    queryFn: () => apiClient.get<PreferencesResponse>("/api/v1/preferences"),
    staleTime: 30000,
  })
}

export function useUpdatePreferences() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (data: UpdatePreferencesRequest) =>
      apiClient.patch<PreferencesResponse>("/api/v1/preferences", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["preferences"] })
      queryClient.invalidateQueries({ queryKey: ["sites"] })
      queryClient.invalidateQueries({ queryKey: ["dashboard-sites"] })
    },
  })
}
