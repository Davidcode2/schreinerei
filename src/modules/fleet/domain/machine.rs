use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{MachineId, ResourceStatus, TenantId};

/// Machine aggregate representing a workshop machine (Maschine)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Machine {
    pub id: MachineId,
    pub tenant_id: TenantId,
    pub name: String,
    pub machine_type: Option<String>,
    pub description: Option<String>,
    pub status: ResourceStatus,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Machine {
    /// Check if status transition is valid
    /// Same transitions as Vehicle/Tool: Available → Reserved → InUse → Available, any → Maintenance
    pub fn can_transition_to(&self, new_status: ResourceStatus) -> bool {
        match (&self.status, &new_status) {
            (ResourceStatus::Available, ResourceStatus::Reserved) => true,
            (ResourceStatus::Available, ResourceStatus::InUse) => true,
            (ResourceStatus::Reserved, ResourceStatus::InUse) => true,
            (ResourceStatus::Reserved, ResourceStatus::Available) => true,
            (ResourceStatus::InUse, ResourceStatus::Available) => true,
            (_, ResourceStatus::Maintenance) => true,
            (ResourceStatus::Maintenance, ResourceStatus::Available) => true,
            _ if self.status == new_status => true,
            _ => false,
        }
    }
}

/// Command to create a new machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMachine {
    pub name: String,
    pub machine_type: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

impl CreateMachine {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Machine name is required".to_string());
        }
        Ok(())
    }
}

/// Command to update a machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMachine {
    pub name: Option<String>,
    pub machine_type: Option<String>,
    pub description: Option<String>,
    pub status: Option<ResourceStatus>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_machine(status: ResourceStatus) -> Machine {
        Machine {
            id: MachineId::new(),
            tenant_id: TenantId::new(),
            name: "Test Machine".to_string(),
            machine_type: Some("CNC".to_string()),
            description: None,
            status,
            location: None,
            qr_code: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn machine_can_transition_from_available_to_reserved() {
        let m = test_machine(ResourceStatus::Available);
        assert!(m.can_transition_to(ResourceStatus::Reserved));
    }

    #[test]
    fn machine_can_transition_from_available_to_in_use() {
        let m = test_machine(ResourceStatus::Available);
        assert!(m.can_transition_to(ResourceStatus::InUse));
    }

    #[test]
    fn machine_can_transition_from_available_to_maintenance() {
        let m = test_machine(ResourceStatus::Available);
        assert!(m.can_transition_to(ResourceStatus::Maintenance));
    }

    #[test]
    fn machine_can_transition_from_reserved_to_in_use() {
        let m = test_machine(ResourceStatus::Reserved);
        assert!(m.can_transition_to(ResourceStatus::InUse));
    }

    #[test]
    fn machine_can_transition_from_reserved_to_available() {
        let m = test_machine(ResourceStatus::Reserved);
        assert!(m.can_transition_to(ResourceStatus::Available));
    }

    #[test]
    fn machine_can_transition_from_in_use_to_available() {
        let m = test_machine(ResourceStatus::InUse);
        assert!(m.can_transition_to(ResourceStatus::Available));
    }

    #[test]
    fn machine_can_transition_from_maintenance_to_available() {
        let m = test_machine(ResourceStatus::Maintenance);
        assert!(m.can_transition_to(ResourceStatus::Available));
    }

    #[test]
    fn machine_cannot_transition_from_maintenance_to_in_use() {
        let m = test_machine(ResourceStatus::Maintenance);
        assert!(!m.can_transition_to(ResourceStatus::InUse));
    }

    #[test]
    fn machine_can_transition_to_same_status() {
        let m = test_machine(ResourceStatus::Available);
        assert!(m.can_transition_to(ResourceStatus::Available));
    }

    #[test]
    fn create_machine_validate_succeeds_with_valid_data() {
        let cmd = CreateMachine {
            name: "CNC Fräse".to_string(),
            machine_type: Some("CNC".to_string()),
            description: None,
            location: None,
            qr_code: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_machine_validate_fails_with_empty_name() {
        let cmd = CreateMachine {
            name: "".to_string(),
            machine_type: None,
            description: None,
            location: None,
            qr_code: None,
        };
        assert_eq!(cmd.validate(), Err("Machine name is required".to_string()));
    }
}
