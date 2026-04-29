import { useQuery } from "@tanstack/react-query"
import { apiClient } from "../client"

export interface User {
  id: string
  email: string
  name: string | null
  role: string
  created_at: string
}

export function useUsers() {
  return useQuery({
    queryKey: ["users"],
    queryFn: () => apiClient.get<User[]>("/api/v1/users"),
  })
}
