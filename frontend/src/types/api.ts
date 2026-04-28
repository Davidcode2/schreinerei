/**
 * Common API types shared across all modules
 */

export interface ApiError {
  message: string
  code?: string
}

export interface PaginatedResponse<T> {
  items: T[]
  total: number
  page: number
  pageSize: number
}

export interface SuccessResponse {
  success: boolean
}
