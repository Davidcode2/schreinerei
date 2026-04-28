/**
 * User and authentication types
 */

export interface User {
  id: string
  email: string
  name: string | null
  role: 'admin' | 'mitarbeiter'
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

export interface TokenResponse {
  access_token: string
  refresh_token: string
  expires_in: number
  token_type: string
}

// Request types
export interface InviteUserRequest {
  email: string
  name?: string
  role: string
}

export interface UpdateRoleRequest {
  role: string
}

export interface UpdateProfileRequest {
  name?: string
}
