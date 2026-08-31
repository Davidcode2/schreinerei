import { useAuthStore } from "@/lib/auth/authStore"
import { getLogoutUrl } from "@/lib/auth/keycloak"

export function useLogout() {
  const logout = useAuthStore((state) => state.logout)

  return () => {
    logout()
    window.location.href = getLogoutUrl()
  }
}
