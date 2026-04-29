import { useEffect } from 'react'
import { useNavigate, useSearchParams } from 'react-router-dom'
import { handleCallback, extractUserFromToken } from '../../lib/auth/keycloak'
import { useAuthStore } from '../../lib/auth/authStore'

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

    handleCallback(code, state)
      .then((tokens) => {
        setTokens(tokens)
        const user = extractUserFromToken(tokens.access_token)
        setUser(user)
        navigate('/')
      })
      .catch((err) => {
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
