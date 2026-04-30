use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::types::{
    TenantId, ReservationId, ReservationStatus, ResourceType, SiteId, UserId,
};

/// Reservation aggregate representing a reservation for a vehicle or tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub id: ReservationId,
    pub tenant_id: TenantId,
    pub resource_type: ResourceType,
    pub resource_id: Uuid, // Polymorphic - references vehicle or tool
    pub user_id: UserId,
    pub site_id: Option<SiteId>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: ReservationStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Reservation {
    /// Check if this reservation overlaps with another reservation
    /// Two reservations overlap if their time ranges intersect
    pub fn overlaps(&self, other: &Reservation) -> bool {
        // Same resource check should be done by caller
        // This just checks time overlap
        self.start_time < other.end_time && other.start_time < self.end_time
    }

    /// Check if status transition is valid
    /// Transitions: Pending → Confirmed → InUse → Completed, any → Cancelled
    pub fn can_transition_to(&self, new_status: ReservationStatus) -> bool {
        match (&self.status, &new_status) {
            // Pending → Confirmed
            (ReservationStatus::Pending, ReservationStatus::Confirmed) => true,
            // Pending → Cancelled
            (ReservationStatus::Pending, ReservationStatus::Cancelled) => true,
            // Confirmed → InUse
            (ReservationStatus::Confirmed, ReservationStatus::InUse) => true,
            // Confirmed → Cancelled
            (ReservationStatus::Confirmed, ReservationStatus::Cancelled) => true,
            // InUse → Completed
            (ReservationStatus::InUse, ReservationStatus::Completed) => true,
            // Any → Cancelled (except already cancelled)
            (_, ReservationStatus::Cancelled) => self.status != ReservationStatus::Cancelled,
            // Same status (no change)
            _ if self.status == new_status => true,
            _ => false,
        }
    }

    /// Check if the reservation is currently active (time is within reservation window)
    pub fn is_active_now(&self) -> bool {
        let now = Utc::now();
        now >= self.start_time && now <= self.end_time
    }
}

/// Command to create a new reservation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReservation {
    pub resource_type: ResourceType,
    pub resource_id: Uuid,
    pub site_id: Option<SiteId>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub notes: Option<String>,
}

impl CreateReservation {
    /// Validate the create reservation command
    pub fn validate(&self) -> Result<(), String> {
        // end_time must be after start_time
        if self.end_time <= self.start_time {
            return Err("End time must be after start time".to_string());
        }

        // start_time must not be in the past (for new reservations)
        let now = Utc::now();
        if self.start_time < now {
            return Err("Start time cannot be in the past".to_string());
        }

        Ok(())
    }
}

/// Command to update a reservation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReservation {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub site_id: Option<SiteId>,
    pub notes: Option<String>,
    pub status: Option<ReservationStatus>,
}

impl UpdateReservation {
    /// Validate the update reservation command
    pub fn validate(&self, current: &Reservation) -> Result<(), String> {
        // Determine the effective start and end times after update
        let effective_start = self.start_time.as_ref().unwrap_or(&current.start_time);
        let effective_end = self.end_time.as_ref().unwrap_or(&current.end_time);

        // Validate end is after start
        if effective_end <= effective_start {
            return Err("End time must be after start time".to_string());
        }

        // If status is being changed, validate the transition
        if let Some(new_status) = &self.status {
            if !current.can_transition_to(*new_status) {
                return Err(format!(
                    "Invalid status transition from {} to {}",
                    current.status, new_status
                ));
            }
        }

        Ok(())
    }
}

/// Reservation with additional details for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationWithDetails {
    pub reservation: Reservation,
    pub resource_name: String,
    pub user_name: Option<String>,
    pub site_name: Option<String>,
}
