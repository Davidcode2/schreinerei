/**
 * Fleet module types matching backend DTOs
 */

// === Common Enums ===

export type VehicleType = 'car' | 'van' | 'truck' | 'trailer' | 'other'
export type ResourceStatus = 'available' | 'in_use' | 'maintenance' | 'reserved'
export type ResourceType = 'vehicle' | 'tool'
export type ReservationStatus = 'pending' | 'confirmed' | 'in_use' | 'completed' | 'cancelled'

// === Vehicle ===

export interface Vehicle {
  id: string
  name: string
  license_plate: string | null
  vehicle_type: VehicleType
  description: string | null
  status: ResourceStatus
  location: string | null
  qr_code: string | null
  created_at: string
  updated_at: string
}

export interface CreateVehicleRequest {
  name: string
  license_plate?: string
  vehicle_type: VehicleType
  description?: string
  location?: string
  qr_code?: string
}

export interface UpdateVehicleRequest {
  name?: string
  license_plate?: string
  vehicle_type?: VehicleType
  description?: string
  status?: ResourceStatus
  location?: string
  qr_code?: string
}

export interface ListVehiclesQuery {
  status?: ResourceStatus
}

// === Tool ===

export interface Tool {
  id: string
  name: string
  category: string | null
  description: string | null
  status: ResourceStatus
  location: string | null
  qr_code: string | null
  created_at: string
  updated_at: string
}

export interface CreateToolRequest {
  name: string
  category?: string
  description?: string
  location?: string
  qr_code?: string
}

export interface UpdateToolRequest {
  name?: string
  category?: string
  description?: string
  status?: ResourceStatus
  location?: string
  qr_code?: string
}

export interface ListToolsQuery {
  status?: ResourceStatus
  category?: string
}

// === Reservation ===

export interface Reservation {
  id: string
  resource_type: ResourceType
  resource_id: string
  resource_name: string
  user_id: string
  user_name: string
  site_id: string | null
  site_name: string | null
  start_time: string
  end_time: string
  status: ReservationStatus
  notes: string | null
  created_at: string
  updated_at: string
}

export interface CreateReservationRequest {
  resource_type: ResourceType
  resource_id: string
  site_id?: string | null
  start_time: string
  end_time: string
  notes?: string
}

export interface UpdateReservationRequest {
  start_time?: string
  end_time?: string
  site_id?: string | null
  notes?: string
  status?: ReservationStatus
}

export interface ListReservationsQuery {
  user_id?: string
  resource_type?: ResourceType
  resource_id?: string
}

// === Calendar ===

export interface ReservationSummary {
  id: string
  start_time: string
  end_time: string
  user_name: string
  site_name: string | null
  status: ReservationStatus
}

export interface CalendarEntry {
  resource_type: ResourceType
  resource_id: string
  resource_name: string
  reservations: ReservationSummary[]
}

export interface CalendarResponse {
  resources: CalendarEntry[]
}

export interface CalendarQuery {
  start_date: string
  end_date: string
  resource_type?: ResourceType
}

// === Availability ===

export interface ConflictDetail {
  id: string
  user_name: string | null
  start_time: string
  end_time: string
  status: string
}

export interface AvailabilityResponse {
  available: boolean
  conflicts?: ConflictDetail[]
}

export interface AvailabilityQuery {
  resource_type: ResourceType
  resource_id: string
  start_time: string
  end_time: string
}

// === QR Status ===

export interface QrStatusResponse {
  resource_type: ResourceType
  resource_id: string
  resource_name: string
  status: ResourceStatus
  current_reservation: ReservationSummary | null
  upcoming_reservations: ReservationSummary[]
}
