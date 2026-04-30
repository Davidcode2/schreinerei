import { useAuthStore } from '../auth/authStore'
import { refreshAccessToken } from '../auth/keycloak'
import { toast } from 'sonner'

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'
const MAX_REFRESH_RETRIES = 3
const REFRESH_BASE_DELAY = 1000

async function refreshWithRetry(
  refreshToken: string,
  attempt: number = 1
): Promise<import('../../types/user').AuthTokens> {
  try {
    return await refreshAccessToken(refreshToken)
  } catch (error) {
    if (attempt >= MAX_REFRESH_RETRIES) {
      throw error
    }
    const delay = REFRESH_BASE_DELAY * Math.pow(2, attempt - 1)
    console.warn(`Token refresh failed (attempt ${attempt}), retrying in ${delay}ms...`)
    await new Promise(resolve => setTimeout(resolve, delay))
    return refreshWithRetry(refreshToken, attempt + 1)
  }
}

class ApiClient {
  private baseUrl: string

  constructor(baseUrl: string = API_URL) {
    this.baseUrl = baseUrl
  }

  private async getAccessToken(): Promise<string | null> {
    const { tokens, setTokens, logout } = useAuthStore.getState()
    
    if (!tokens) return null

    if (Date.now() >= tokens.expires_at - 60000) {
      try {
        if (Date.now() >= tokens.expires_at - 120000) {
          toast.warning('Session wird bald ablaufen')
        }
        const newTokens = await refreshWithRetry(tokens.refresh_token)
        setTokens(newTokens)
        return newTokens.access_token
      } catch (error) {
        console.error('Token refresh failed after retries:', error)
        toast.error('Session abgelaufen. Bitte melden Sie sich erneut an.')
        logout()
        return null
      }
    }

    return tokens.access_token
  }

  async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const token = await this.getAccessToken()
    
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...(options.headers as Record<string, string>),
    }

    if (token) {
      headers['Authorization'] = `Bearer ${token}`
    }

    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      ...options,
      headers,
    })

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}))
      // Handle both "error" (from backend) and "message" (fallback) keys
      const errorMessage = errorData.error || errorData.message || `HTTP ${response.status}`
      throw new Error(errorMessage)
    }

    if (response.status === 204) {
      return {} as T
    }

    return response.json()
  }

  get<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: 'GET' })
  }

  post<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'POST',
      body: data ? JSON.stringify(data) : null,
    })
  }

  patch<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'PATCH',
      body: data ? JSON.stringify(data) : null,
    })
  }

  delete<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: 'DELETE' })
  }
}

export const apiClient = new ApiClient()
