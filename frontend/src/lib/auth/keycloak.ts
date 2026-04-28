/**
 * Keycloak OAuth2 client with PKCE flow
 * 
 * Implements the Authorization Code flow with PKCE for secure
 * authentication against Keycloak.
 */

import { generateCodeVerifier, generateCodeChallenge } from './pkce'
import type { TokenResponse } from '@/types/user'

// Environment configuration
const KEYCLOAK_URL = import.meta.env.VITE_KEYCLOAK_URL
const REALM = import.meta.env.VITE_KEYCLOAK_REALM
const CLIENT_ID = import.meta.env.VITE_KEYCLOAK_CLIENT_ID

// OAuth endpoints
const AUTH_URL = `${KEYCLOAK_URL}/realms/${REALM}/protocol/openid-connect/auth`
const TOKEN_URL = `${KEYCLOAK_URL}/realms/${REALM}/protocol/openid-connect/token`
const LOGOUT_URL = `${KEYCLOAK_URL}/realms/${REALM}/protocol/openid-connect/logout`

// Redirect URI for callback
const REDIRECT_URI = `${window.location.origin}/auth/callback`

/**
 * Start the OAuth2 login flow
 * Redirects user to Keycloak login page
 */
export async function startLogin(): Promise<void> {
  const state = generateCodeVerifier()
  const verifier = generateCodeVerifier()
  const challenge = await generateCodeChallenge(verifier)

  // Store PKCE state for callback verification
  sessionStorage.setItem('pkce_verifier', verifier)
  sessionStorage.setItem('auth_state', state)

  // Build authorization URL
  const params = new URLSearchParams({
    client_id: CLIENT_ID,
    redirect_uri: REDIRECT_URI,
    response_type: 'code',
    scope: 'openid profile email',
    code_challenge: challenge,
    code_challenge_method: 'S256',
    state: state,
  })

  window.location.href = `${AUTH_URL}?${params}`
}

/**
 * Handle callback from Keycloak
 * Exchange authorization code for tokens
 */
export async function handleCallback(code: string, state: string): Promise<TokenResponse> {
  const storedState = sessionStorage.getItem('auth_state')
  const verifier = sessionStorage.getItem('pkce_verifier')

  // Validate state to prevent CSRF
  if (state !== storedState || !verifier) {
    throw new Error('Invalid auth state')
  }

  // Exchange code for tokens
  const response = await fetch(TOKEN_URL, {
    method: 'POST',
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body: new URLSearchParams({
      client_id: CLIENT_ID,
      grant_type: 'authorization_code',
      code,
      redirect_uri: REDIRECT_URI,
      code_verifier: verifier,
    }),
  })

  if (!response.ok) {
    const errorText = await response.text()
    console.error('Token exchange failed:', errorText)
    throw new Error('Token exchange failed')
  }

  // Clear PKCE state
  sessionStorage.removeItem('pkce_verifier')
  sessionStorage.removeItem('auth_state')

  return response.json()
}

/**
 * Refresh access token using refresh token
 */
export async function refreshToken(refreshToken: string): Promise<TokenResponse> {
  const response = await fetch(TOKEN_URL, {
    method: 'POST',
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body: new URLSearchParams({
      client_id: CLIENT_ID,
      grant_type: 'refresh_token',
      refresh_token: refreshToken,
    }),
  })

  if (!response.ok) {
    throw new Error('Token refresh failed')
  }

  return response.json()
}

/**
 * Get logout URL for redirecting to Keycloak logout
 */
export function getLogoutUrl(): string {
  return `${LOGOUT_URL}?client_id=${CLIENT_ID}&post_logout_redirect_uri=${window.location.origin}`
}
