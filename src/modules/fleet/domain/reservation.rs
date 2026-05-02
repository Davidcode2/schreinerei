use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::types::{
    ReservationId, ReservationStatus, ResourceType, SiteId, TenantId, UserId,
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    fn test_time(hour: u8) -> DateTime<Utc> {
        Utc::now()
            .with_hour(hour as u32)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
    }

    fn test_reservation(start_hour: u8, end_hour: u8, status: ReservationStatus) -> Reservation {
        Reservation {
            id: ReservationId::new(),
            tenant_id: TenantId::new(),
            resource_type: ResourceType::Vehicle,
            resource_id: Uuid::new_v4(),
            user_id: UserId::new(),
            site_id: None,
            start_time: test_time(start_hour),
            end_time: test_time(end_hour),
            status,
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // Overlap detection tests
    #[test]
    fn reservation_overlaps_when_ranges_intersect() {
        let r1 = test_reservation(10, 12, ReservationStatus::Pending);
        let r2 = test_reservation(11, 13, ReservationStatus::Pending);
        assert!(r1.overlaps(&r2));
    }

    #[test]
    fn reservation_does_not_overlap_when_adjacent() {
        let r1 = test_reservation(10, 12, ReservationStatus::Pending);
        let r2 = test_reservation(12, 14, ReservationStatus::Pending);
        assert!(!r1.overlaps(&r2));
    }

    #[test]
    fn reservation_overlaps_when_one_contains_other() {
        let r1 = test_reservation(10, 14, ReservationStatus::Pending);
        let r2 = test_reservation(11, 12, ReservationStatus::Pending);
        assert!(r1.overlaps(&r2));
    }

    #[test]
    fn reservation_does_not_overlap_when_disjoint() {
        let r1 = test_reservation(10, 12, ReservationStatus::Pending);
        let r2 = test_reservation(14, 16, ReservationStatus::Pending);
        assert!(!r1.overlaps(&r2));
    }

    // State machine tests
    #[test]
    fn reservation_can_transition_from_pending_to_confirmed() {
        let r = test_reservation(10, 12, ReservationStatus::Pending);
        assert!(r.can_transition_to(ReservationStatus::Confirmed));
    }

    #[test]
    fn reservation_can_transition_from_pending_to_cancelled() {
        let r = test_reservation(10, 12, ReservationStatus::Pending);
        assert!(r.can_transition_to(ReservationStatus::Cancelled));
    }

    #[test]
    fn reservation_cannot_transition_from_pending_to_in_use() {
        let r = test_reservation(10, 12, ReservationStatus::Pending);
        assert!(!r.can_transition_to(ReservationStatus::InUse));
    }

    #[test]
    fn reservation_can_transition_from_confirmed_to_in_use() {
        let r = test_reservation(10, 12, ReservationStatus::Confirmed);
        assert!(r.can_transition_to(ReservationStatus::InUse));
    }

    #[test]
    fn reservation_can_transition_from_confirmed_to_cancelled() {
        let r = test_reservation(10, 12, ReservationStatus::Confirmed);
        assert!(r.can_transition_to(ReservationStatus::Cancelled));
    }

    #[test]
    fn reservation_cannot_transition_from_confirmed_to_pending() {
        let r = test_reservation(10, 12, ReservationStatus::Confirmed);
        assert!(!r.can_transition_to(ReservationStatus::Pending));
    }

    #[test]
    fn reservation_can_transition_from_in_use_to_completed() {
        let r = test_reservation(10, 12, ReservationStatus::InUse);
        assert!(r.can_transition_to(ReservationStatus::Completed));
    }

    #[test]
    fn reservation_cannot_transition_from_cancelled_to_anything() {
        let r = test_reservation(10, 12, ReservationStatus::Cancelled);
        assert!(!r.can_transition_to(ReservationStatus::Confirmed));
        assert!(!r.can_transition_to(ReservationStatus::Pending));
        assert!(!r.can_transition_to(ReservationStatus::InUse));
    }

    #[test]
    fn reservation_can_transition_to_same_status() {
        let r = test_reservation(10, 12, ReservationStatus::Pending);
        assert!(r.can_transition_to(ReservationStatus::Pending));
    }

    // CreateReservation validation tests
    #[test]
    fn create_reservation_validate_succeeds_with_valid_times() {
        let future_start = Utc::now() + chrono::Duration::hours(1);
        let future_end = future_start + chrono::Duration::hours(2);
        let cmd = CreateReservation {
            resource_type: ResourceType::Vehicle,
            resource_id: Uuid::new_v4(),
            site_id: None,
            start_time: future_start,
            end_time: future_end,
            notes: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_reservation_validate_fails_when_end_equals_start() {
        let future_time = Utc::now() + chrono::Duration::hours(1);
        let cmd = CreateReservation {
            resource_type: ResourceType::Vehicle,
            resource_id: Uuid::new_v4(),
            site_id: None,
            start_time: future_time,
            end_time: future_time,
            notes: None,
        };
        assert_eq!(
            cmd.validate(),
            Err("End time must be after start time".to_string())
        );
    }

    #[test]
    fn create_reservation_validate_fails_when_start_in_past() {
        let past_start = Utc::now() - chrono::Duration::hours(1);
        let future_end = Utc::now() + chrono::Duration::hours(1);
        let cmd = CreateReservation {
            resource_type: ResourceType::Vehicle,
            resource_id: Uuid::new_v4(),
            site_id: None,
            start_time: past_start,
            end_time: future_end,
            notes: None,
        };
        assert_eq!(
            cmd.validate(),
            Err("Start time cannot be in the past".to_string())
        );
    }

    // UpdateReservation validation tests
    #[test]
    fn update_reservation_validate_succeeds_with_valid_times() {
        let current = test_reservation(10, 12, ReservationStatus::Pending);
        let new_end = test_time(14);
        let update = UpdateReservation {
            start_time: None,
            end_time: Some(new_end),
            site_id: None,
            notes: None,
            status: None,
        };
        assert!(update.validate(&current).is_ok());
    }

    #[test]
    fn update_reservation_validate_fails_with_invalid_status_transition() {
        let current = test_reservation(10, 12, ReservationStatus::Pending);
        let update = UpdateReservation {
            start_time: None,
            end_time: None,
            site_id: None,
            notes: None,
            status: Some(ReservationStatus::InUse),
        };
        assert!(update.validate(&current).is_err());
    }
}
