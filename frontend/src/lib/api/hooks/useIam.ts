import { useQuery } from "@tanstack/react-query"
import { apiClient } from "../client"
import { useAuthStore } from "../../auth/authStore"

export interface User {
  id: string
  email: string
  name: string | null
  role: string
  created_at: string
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
