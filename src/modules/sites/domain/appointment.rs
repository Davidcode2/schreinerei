use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::common::types::{SiteAppointmentId, SiteId, TenantId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SiteAppointmentKind {
    CustomerAppointment,
    WorkerDeployment,
    Milestone,
}

impl SiteAppointmentKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            SiteAppointmentKind::CustomerAppointment => "customer_appointment",
            SiteAppointmentKind::WorkerDeployment => "worker_deployment",
            SiteAppointmentKind::Milestone => "milestone",
        }
    }
}

impl FromStr for SiteAppointmentKind {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "customer_appointment" => Ok(Self::CustomerAppointment),
            "worker_deployment" => Ok(Self::WorkerDeployment),
            "milestone" => Ok(Self::Milestone),
            _ => Err(format!("Invalid appointment kind: {value}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteAppointment {
    pub id: SiteAppointmentId,
    pub tenant_id: TenantId,
    pub site_id: SiteId,
    pub title: String,
    pub appointment_kind: SiteAppointmentKind,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub notes: Option<String>,
    pub assigned_user_ids: Vec<UserId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSiteAppointment {
    pub site_id: SiteId,
    pub title: String,
    pub appointment_kind: SiteAppointmentKind,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub notes: Option<String>,
    pub assigned_user_ids: Vec<UserId>,
}

impl CreateSiteAppointment {
    pub fn validate(&self) -> Result<(), String> {
        validate_title_and_range(&self.title, self.starts_at, self.ends_at)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSiteAppointment {
    pub title: Option<String>,
    pub appointment_kind: Option<SiteAppointmentKind>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub notes: Option<Option<String>>,
    pub assigned_user_ids: Option<Vec<UserId>>,
}

impl UpdateSiteAppointment {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(title) = &self.title {
            if title.trim().is_empty() {
                return Err("Appointment title cannot be empty".to_string());
            }
        }

        match (self.starts_at, self.ends_at) {
            (Some(start), Some(end)) if end <= start => {
                Err("Appointment end must be after start".to_string())
            }
            _ => Ok(()),
        }
    }
}

fn validate_title_and_range(
    title: &str,
    starts_at: DateTime<Utc>,
    ends_at: DateTime<Utc>,
) -> Result<(), String> {
    if title.trim().is_empty() {
        return Err("Appointment title is required".to_string());
    }
    if ends_at <= starts_at {
        return Err("Appointment end must be after start".to_string());
    }
    Ok(())
}
