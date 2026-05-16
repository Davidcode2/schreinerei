import { useEffect } from 'react'
import { useNavigate, useSearchParams } from 'react-router-dom'
import { extractUserFromToken, handleCallback, refreshAccessToken } from '../../lib/auth/keycloak'
import { useAuthStore } from '../../lib/auth/authStore'

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'

interface CurrentUserResponse {
  role: string
}

async function syncAdminRole(tokens: { access_token: string; refresh_token: string }) {
  const response = await fetch(`${API_URL}/api/v1/auth/me`, {
    headers: {
      Authorization: `Bearer ${tokens.access_token}`,
    },
  })

  if (!response.ok) {
    return null
  }

  const user = (await response.json()) as CurrentUserResponse
  if (user.role !== 'admin') {
    return null
  }

  const tokenUser = extractUserFromToken(tokens.access_token)
  if (tokenUser.role === 'admin') {
    return null
  }

  return refreshAccessToken(tokens.refresh_token)
}

export function AuthCallback() {
  const navigate = useNavigate()
  const [searchParams] = useSearchParams()
  const { setUser, setTokens } = useAuthStore()

  useEffect(() => {
    const code = searchParams.get('code')
    const state = searchParams.get('state')
    const error = searchParams.get('error')

    if (error) {
      console.error('Auth error:', error)
      navigate('/login')
      return
    }

    if (!code || !state) {
      navigate('/login')
      return
    }

    const exchangeKey = `auth_exchanging_${code}`
    if (sessionStorage.getItem(exchangeKey)) {
      console.log('Token exchange already in progress, skipping')
      return
    }
    sessionStorage.setItem(exchangeKey, 'pending')

    handleCallback(code, state)
      .then(async (tokens) => {
        sessionStorage.removeItem(exchangeKey)
        const syncedTokens = await syncAdminRole(tokens).catch((error) => {
          console.warn('Silent admin role refresh failed:', error)
          return null
        }) ?? tokens
        setTokens(syncedTokens)
        const user = extractUserFromToken(syncedTokens.access_token)
        setUser(user)
        navigate('/')
      })
      .catch((err) => {
        sessionStorage.removeItem(exchangeKey)
        console.error('Auth callback failed:', err)
        navigate('/login')
      })
  }, [searchParams, navigate, setUser, setTokens])

  return (
    <div className="flex items-center justify-center min-h-screen">
      <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
    </div>
  )
}
