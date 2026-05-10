use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{ResourceStatus, TenantId, VehicleId, VehicleType};

/// Vehicle aggregate representing a vehicle (Fahrzeug)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: VehicleId,
    pub tenant_id: TenantId,
    pub name: String,
    pub license_plate: Option<String>,
    pub vehicle_type: VehicleType,
    pub description: Option<String>,
    pub status: ResourceStatus,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub display_color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Vehicle {
    /// Check if status transition is valid
    /// Transitions: Available → Reserved → InUse → Available, any → Maintenance
    pub fn can_transition_to(&self, new_status: ResourceStatus) -> bool {
        match (&self.status, &new_status) {
            // Available → Reserved
            (ResourceStatus::Available, ResourceStatus::Reserved) => true,
            // Available → InUse (direct use without reservation)
            (ResourceStatus::Available, ResourceStatus::InUse) => true,
            // Reserved → InUse
            (ResourceStatus::Reserved, ResourceStatus::InUse) => true,
            // Reserved → Available (cancellation)
            (ResourceStatus::Reserved, ResourceStatus::Available) => true,
            // InUse → Available
            (ResourceStatus::InUse, ResourceStatus::Available) => true,
            // Any → Maintenance
            (_, ResourceStatus::Maintenance) => true,
            // Maintenance → Available
            (ResourceStatus::Maintenance, ResourceStatus::Available) => true,
            // Same status (no change)
            _ if self.status == new_status => true,
            _ => false,
        }
    }
}

/// Command to create a new vehicle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVehicle {
    pub name: String,
    pub license_plate: Option<String>,
    pub vehicle_type: VehicleType,
    pub description: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub display_color: Option<String>,
}

impl CreateVehicle {
    /// Validate the create vehicle command
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Vehicle name is required".to_string());
        }
        // Validate license plate format if provided (basic check)
        if let Some(ref plate) = self.license_plate {
            if plate.trim().is_empty() {
                return Err("License plate cannot be empty if provided".to_string());
            }
        }
        if let Some(ref color) = self.display_color {
            validate_display_color(color)?;
        }
        Ok(())
    }
}

/// Command to update a vehicle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVehicle {
    pub name: Option<String>,
    pub license_plate: Option<String>,
    pub vehicle_type: Option<VehicleType>,
    pub description: Option<String>,
    pub status: Option<ResourceStatus>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub display_color: Option<String>,
}

pub fn validate_display_color(color: &str) -> Result<(), String> {
    let is_valid = color.len() == 7
        && color.starts_with('#')
        && color[1..]
            .chars()
            .all(|character| character.is_ascii_hexdigit());

    if is_valid {
        Ok(())
    } else {
        Err("Display color must be a hex color like #2563eb".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vehicle(status: ResourceStatus) -> Vehicle {
        Vehicle {
            id: VehicleId::new(),
            tenant_id: TenantId::new(),
            name: "Test Vehicle".to_string(),
            license_plate: Some("AB-123-CD".to_string()),
            vehicle_type: VehicleType::Van,
            description: None,
            status,
            location: None,
            qr_code: None,
            display_color: "#2563eb".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // State machine tests
    #[test]
    fn vehicle_can_transition_from_available_to_reserved() {
        let v = test_vehicle(ResourceStatus::Available);
        assert!(v.can_transition_to(ResourceStatus::Reserved));
    }

    #[test]
    fn vehicle_can_transition_from_available_to_in_use() {
        let v = test_vehicle(ResourceStatus::Available);
        assert!(v.can_transition_to(ResourceStatus::InUse));
    }

    #[test]
    fn vehicle_can_transition_from_available_to_maintenance() {
        let v = test_vehicle(ResourceStatus::Available);
        assert!(v.can_transition_to(ResourceStatus::Maintenance));
    }

    #[test]
    fn vehicle_can_transition_from_reserved_to_in_use() {
        let v = test_vehicle(ResourceStatus::Reserved);
        assert!(v.can_transition_to(ResourceStatus::InUse));
    }

    #[test]
    fn vehicle_can_transition_from_reserved_to_available() {
        let v = test_vehicle(ResourceStatus::Reserved);
        assert!(v.can_transition_to(ResourceStatus::Available));
    }

    #[test]
    fn vehicle_can_transition_from_in_use_to_available() {
        let v = test_vehicle(ResourceStatus::InUse);
        assert!(v.can_transition_to(ResourceStatus::Available));
    }

    #[test]
    fn vehicle_can_transition_from_maintenance_to_available() {
        let v = test_vehicle(ResourceStatus::Maintenance);
        assert!(v.can_transition_to(ResourceStatus::Available));
    }

    #[test]
    fn vehicle_cannot_transition_from_maintenance_to_in_use() {
        let v = test_vehicle(ResourceStatus::Maintenance);
        assert!(!v.can_transition_to(ResourceStatus::InUse));
    }

    #[test]
    fn vehicle_can_transition_to_same_status() {
        let v = test_vehicle(ResourceStatus::Available);
        assert!(v.can_transition_to(ResourceStatus::Available));
    }

    // CreateVehicle validation tests
    #[test]
    fn create_vehicle_validate_succeeds_with_valid_data() {
        let cmd = CreateVehicle {
            name: "Transporter".to_string(),
            license_plate: Some("AB-123-CD".to_string()),
            vehicle_type: VehicleType::Van,
            description: None,
            location: None,
            qr_code: None,
            display_color: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_vehicle_validate_fails_with_empty_name() {
        let cmd = CreateVehicle {
            name: "".to_string(),
            license_plate: None,
            vehicle_type: VehicleType::Van,
            description: None,
            location: None,
            qr_code: None,
            display_color: None,
        };
        assert_eq!(cmd.validate(), Err("Vehicle name is required".to_string()));
    }

    #[test]
    fn create_vehicle_validate_fails_with_empty_license_plate() {
        let cmd = CreateVehicle {
            name: "Transporter".to_string(),
            license_plate: Some("".to_string()),
            vehicle_type: VehicleType::Van,
            description: None,
            location: None,
            qr_code: None,
            display_color: None,
        };
        assert_eq!(
            cmd.validate(),
            Err("License plate cannot be empty if provided".to_string())
        );
    }

    #[test]
    fn create_vehicle_validate_fails_with_invalid_display_color() {
        let cmd = CreateVehicle {
            name: "Transporter".to_string(),
            license_plate: None,
            vehicle_type: VehicleType::Van,
            description: None,
            location: None,
            qr_code: None,
            display_color: Some("blue".to_string()),
        };

        assert_eq!(
            cmd.validate(),
            Err("Display color must be a hex color like #2563eb".to_string())
        );
    }
}
