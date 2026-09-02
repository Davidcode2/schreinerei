import { useAuthStore } from "@/lib/auth/authStore"
import type { User } from "@/types/user"

export type TestRole = "admin" | "mitarbeiter"

export function setAuthRole(role: TestRole | null): void {
  const user: User | null = role
    ? {
        id: "user-1",
        email: `${role}@example.com`,
        name: role === "admin" ? "Admin" : "Mitarbeiter",
        role,
        tenant_id: "tenant-1",
        created_at: new Date().toISOString(),
      }
    : null

  useAuthStore.setState({
    user,
    tokens: null,
    isAuthenticated: role !== null,
    isLoading: false,
  })
}
