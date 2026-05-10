import type {
  CreateSiteAppointmentRequest as GeneratedCreateSiteAppointmentRequest,
  InvoiceResponse as GeneratedInvoiceResponse,
  SiteHistoryReportQuery as GeneratedSiteHistoryReportQuery,
  SiteHistoryReportRowResponse as GeneratedSiteHistoryReportRowResponse,
  SiteAppointmentResponse as GeneratedSiteAppointmentResponse,
  SiteAppointmentsQuery as GeneratedSiteAppointmentsQuery,
  SiteInvoiceSummaryResponse as GeneratedSiteInvoiceSummaryResponse,
  SiteProjectSummaryResponse as GeneratedSiteProjectSummaryResponse,
  UpdateSiteAppointmentRequest as GeneratedUpdateSiteAppointmentRequest,
} from '@/types/generated'

/**
 * Sites module types matching backend DTOs
 */

// === Site ===

export type SiteStatus = 'planned' | 'active' | 'completed' | 'archived'
export type ProjectType = 'external_site' | 'internal_workshop'

export interface Site {
  id: string
  project_type: ProjectType
  name: string
  customer_name: string
  location: string | null
  description: string | null
  status: SiteStatus
  start_date: string | null
  end_date: string | null
  estimated_days: number | null
  budget_amount_cents: number | null
  billing_reference: string | null
  billing_notes: string | null
  quote_reference: string | null
  created_at: string
}

export interface CreateSiteRequest {
  project_type: ProjectType
  name: string
  customer_name: string
  location?: string
  description?: string
  start_date?: string
  end_date?: string
  estimated_days?: number
  budget_amount_cents?: number | null
  billing_reference?: string | null
  billing_notes?: string | null
  quote_reference?: string | null
}

export interface UpdateSiteRequest {
  project_type?: ProjectType
  name?: string
  customer_name?: string
  location?: string
  description?: string
  status?: SiteStatus
  start_date?: string
  end_date?: string
  estimated_days?: number
  budget_amount_cents?: number | null
  billing_reference?: string | null
  billing_notes?: string | null
  quote_reference?: string | null
  clear_budget_amount?: boolean
  clear_billing_reference?: boolean
  clear_billing_notes?: boolean
  clear_quote_reference?: boolean
}

export interface ListSitesQuery {
  status?: SiteStatus
}

// === Assignment ===

export type AssignmentRole = 'lead' | 'worker'

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

// === Site Appointments ===

export type SiteAppointment = GeneratedSiteAppointmentResponse

export type SiteAppointmentKind =
  SiteAppointment["appointment_kind"]

export type SiteAppointmentsQuery = Partial<GeneratedSiteAppointmentsQuery>

export type CreateSiteAppointmentRequest = GeneratedCreateSiteAppointmentRequest

export type UpdateSiteAppointmentRequest = Partial<GeneratedUpdateSiteAppointmentRequest>

// === Time Entry ===

export type WorkType = 'workshop' | 'site' | 'travel' | 'other'

export interface TimeEntry {
  id: string
  site_id: string | null
  user_id: string
  creator_name: string
  can_edit: boolean
  can_delete: boolean
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

export type ActivityType = 'photo' | 'note' | 'status_change'

export interface ActivityAttachment {
  attachment_id: string
  filename: string
  mime_type: string
  url: string
  thumbnail_url: string | null
}

export interface Activity {
  id: string
  site_id: string
  user_id: string
  creator_name: string
  can_delete: boolean
  activity_type: ActivityType
  content: string | null
  photo_url: string | null
  attachments: ActivityAttachment[]
  created_at: string
}

export interface CreateActivityRequest {
  activity_type: ActivityType
  content?: string
  photo_url?: string
  attachment_ids?: string[]
}

export interface UploadPhotoAttachmentResponse {
  attachment_id: string
  photo_url: string
  thumbnail_url: string
}

export interface UploadSiteAttachmentResponse extends ActivityAttachment {}

export interface ActivityQuery {
  limit?: number
}

// === Dashboard ===

export interface DashboardSite {
  id: string
  project_type: ProjectType
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

export type SiteProjectSummary = GeneratedSiteProjectSummaryResponse

export type SiteInvoiceSummary = GeneratedSiteInvoiceSummaryResponse

export type SiteInvoice = GeneratedInvoiceResponse

export type SiteHistoryReportRow = GeneratedSiteHistoryReportRowResponse

export type SiteHistoryReportQuery = Partial<GeneratedSiteHistoryReportQuery>
