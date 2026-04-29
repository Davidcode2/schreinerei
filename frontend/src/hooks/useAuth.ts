import { useEffect } from 'react'
import { useAuthStore } from '../lib/auth/authStore'
import { extractUserFromToken } from '../lib/auth/keycloak'

export function useAuth() {
  const { user, tokens, isAuthenticated, isLoading, setUser, setLoading, logout } = useAuthStore()

  useEffect(() => {
    if (tokens?.access_token && !user) {
      try {
        const extractedUser = extractUserFromToken(tokens.access_token)
        setUser(extractedUser)
      } catch {
        logout()
      }
    }
    setLoading(false)
  }, [tokens, user, setUser, setLoading, logout])

  return {
    user,
    isAuthenticated,
    isLoading,
    logout,
  }
}
