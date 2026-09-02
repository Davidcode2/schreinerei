import { useAuthStore } from "@/lib/auth/authStore"

export function useIsAdmin(): boolean {
  const user = useAuthStore((state) => state.user)
  return user?.role === "admin"
}
