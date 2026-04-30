/**
 * Sites module types matching backend DTOs
 */

// === Site ===

export type SiteStatus = 'planned' | 'active' | 'completed' | 'archived'

export interface Site {
  id: string
  name: string
  customer_name: string
  location: string | null
  description: string | null
  status: SiteStatus
  start_date: string | null
  end_date: string | null
  estimated_days: number | null
  created_at: string
}

export interface CreateSiteRequest {
  name: string
  customer_name: string
  location?: string
  description?: string
  start_date?: string
  end_date?: string
  estimated_days?: number
}

export interface UpdateSiteRequest {
  name?: string
  customer_name?: string
  location?: string
  description?: string
  status?: SiteStatus
  start_date?: string
  end_date?: string
  estimated_days?: number
}

export interface ListSitesQuery {
  status?: SiteStatus
}

// === Assignment ===

export type AssignmentRole = 'manager' | 'worker'

export interface SiteAssignment {
  id: string
  site_id: string
  user_id: string
  role: AssignmentRole
  created_at: string
}

export interface AssignUserRequest {
  user_id: string
  role?: AssignmentRole
}

// === Time Entry ===

export type WorkType = 'workshop' | 'site' | 'travel' | 'other'

export interface TimeEntry {
  id: string
  site_id: string | null
  user_id: string
  work_type: WorkType
  hours: number
  work_date: string
  notes: string | null
  created_at: string
}

export interface CreateTimeEntryRequest {
  site_id?: string | null
  work_type: WorkType
  hours: number
  work_date: string
  notes?: string
}

export interface UpdateTimeEntryRequest {
  site_id?: string | null
  work_type?: WorkType
  hours?: number
  work_date?: string
  notes?: string
}

// === Activity ===

export type ActivityType = 'photo' | 'note'

export interface Activity {
  id: string
  site_id: string
  user_id: string
  activity_type: ActivityType
  content: string | null
  photo_url: string | null
  created_at: string
}

export interface CreateActivityRequest {
  activity_type: ActivityType
  content?: string
  photo_url?: string
}

export interface ActivityQuery {
  limit?: number
}

// === Dashboard ===

export interface DashboardSite {
  id: string
  name: string
  customer_name: string
  location: string | null
  status: string
  start_date: string | null
  end_date: string | null
  estimated_days: number | null
  assigned_users: number
  total_hours: number
}
