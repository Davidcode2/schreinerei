import { generateCodeVerifier, generateCodeChallenge } from './pkce'
import type { AuthTokens, User, KeycloakTokenPayload } from '../../types/user'

const KEYCLOAK_URL = import.meta.env.VITE_KEYCLOAK_URL || 'https://auth.jakob-lingel.dev'
const REALM = import.meta.env.VITE_KEYCLOAK_REALM || 'schreinerei'
const CLIENT_ID = import.meta.env.VITE_KEYCLOAK_CLIENT_ID || 'schreinerei_pwa'

const AUTH_URL = `${KEYCLOAK_URL}/realms/${REALM}/protocol/openid-connect/auth`
const TOKEN_URL = `${KEYCLOAK_URL}/realms/${REALM}/protocol/openid-connect/token`
const LOGOUT_URL = `${KEYCLOAK_URL}/realms/${REALM}/protocol/openid-connect/logout`

const REDIRECT_URI = `${window.location.origin}/auth/callback`

interface TokenResponse {
  access_token: string
  refresh_token: string
  expires_in: number
  token_type: string
}

export async function startLogin(): Promise<void> {
  const state = generateCodeVerifier()
  const verifier = generateCodeVerifier()
  const challenge = await generateCodeChallenge(verifier)

  sessionStorage.setItem('pkce_verifier', verifier)
  sessionStorage.setItem('auth_state', state)

  const params = new URLSearchParams({
    client_id: CLIENT_ID,
    redirect_uri: REDIRECT_URI,
    response_type: 'code',
    scope: 'openid profile email organization',
    code_challenge: challenge,
    code_challenge_method: 'S256',
    state: state,
  })

  window.location.href = `${AUTH_URL}?${params}`
}

export async function handleCallback(code: string, state: string): Promise<AuthTokens> {
  const storedState = sessionStorage.getItem('auth_state')
  const verifier = sessionStorage.getItem('pkce_verifier')

  if (state !== storedState || !verifier) {
    throw new Error('Invalid auth state')
  }

  sessionStorage.removeItem('pkce_verifier')
  sessionStorage.removeItem('auth_state')

  const response = await fetch(TOKEN_URL, {
    method: 'POST',
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body: new URLSearchParams({
      grant_type: 'authorization_code',
      client_id: CLIENT_ID,
      code: code,
      redirect_uri: REDIRECT_URI,
      code_verifier: verifier,
    }),
  })

  if (!response.ok) {
    throw new Error('Token exchange failed')
  }

  const tokens: TokenResponse = await response.json()

  return {
    access_token: tokens.access_token,
    refresh_token: tokens.refresh_token,
    expires_at: Date.now() + tokens.expires_in * 1000,
  }
}

export async function refreshAccessToken(refreshToken: string): Promise<AuthTokens> {
  const response = await fetch(TOKEN_URL, {
    method: 'POST',
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body: new URLSearchParams({
      grant_type: 'refresh_token',
      client_id: CLIENT_ID,
      refresh_token: refreshToken,
    }),
  })

  if (!response.ok) {
    throw new Error('Token refresh failed')
  }

  const tokens: TokenResponse = await response.json()

  return {
    access_token: tokens.access_token,
    refresh_token: tokens.refresh_token,
    expires_at: Date.now() + tokens.expires_in * 1000,
  }
}

export function getLogoutUrl(): string {
  const params = new URLSearchParams({
    client_id: CLIENT_ID,
    redirect_uri: window.location.origin,
  })
  return `${LOGOUT_URL}?${params}`
}

export function parseJwt(token: string): KeycloakTokenPayload {
  const parts = token.split('.')
  if (parts.length < 2 || !parts[1]) {
    throw new Error('Invalid JWT token')
  }
  const base64Url: string = parts[1]
  const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/')
  const jsonPayload = decodeURIComponent(
    atob(base64)
      .split('')
      .map(c => '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2))
      .join('')
  )
  return JSON.parse(jsonPayload)
}

export function extractUserFromToken(token: string): User {
  const payload = parseJwt(token)
  const roles = payload.realm_access?.roles || []
  const role = roles.includes('admin') ? 'admin' : 'mitarbeiter'

  const tenantId = payload.organization?.[0] || ''

  return {
    id: payload.sub,
    email: payload.email,
    name: payload.preferred_username,
    role: role as 'admin' | 'mitarbeiter',
    tenant_id: tenantId,
    created_at: new Date().toISOString(),
  }
}
