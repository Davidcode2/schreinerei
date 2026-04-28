use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{TenantId, VehicleId, VehicleType, ResourceStatus};

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
}
