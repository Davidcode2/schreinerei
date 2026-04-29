export interface User {
  id: string
  email: string
  name: string | null
  role: 'admin' | 'mitarbeiter'
  tenant_id: string
  created_at: string
}

export interface AuthTokens {
  access_token: string
  refresh_token: string
  expires_at: number
}

export interface KeycloakTokenPayload {
  sub: string
  email: string
  preferred_username: string
  tenant_id: string
  realm_access: { roles: string[] }
}
