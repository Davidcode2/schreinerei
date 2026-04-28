use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{ActivityId, SiteId, TenantId, UserId};

/// Activity type for the feed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    Photo,
    Note,
    StatusChange,
}

impl ActivityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActivityType::Photo => "photo",
            ActivityType::Note => "note",
            ActivityType::StatusChange => "status_change",
        }
    }
}

impl std::str::FromStr for ActivityType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "photo" => Ok(ActivityType::Photo),
            "note" => Ok(ActivityType::Note),
            "status_change" => Ok(ActivityType::StatusChange),
            _ => Err(format!("Invalid activity type: {}", s)),
        }
    }
}

/// Activity aggregate for site timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: ActivityId,
    pub tenant_id: TenantId,
    pub site_id: SiteId,
    pub user_id: UserId,
    pub activity_type: ActivityType,
    pub content: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Command to create an activity (photo or note)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateActivity {
    pub site_id: SiteId,
    pub activity_type: ActivityType,
    pub content: Option<String>,
    pub photo_url: Option<String>,
}

impl CreateActivity {
    pub fn validate(&self) -> Result<(), String> {
        match self.activity_type {
            ActivityType::Photo => {
                if self.photo_url.is_none() {
                    return Err("Photo URL is required for photo activity".to_string());
                }
            }
            ActivityType::Note => {
                if self.content.is_none() || self.content.as_ref().map(|c| c.trim().is_empty()).unwrap_or(true) {
                    return Err("Content is required for note activity".to_string());
                }
            }
            ActivityType::StatusChange => {
                // Status changes are created by system, not user
                return Err("Cannot manually create status change activity".to_string());
            }
        }
        Ok(())
    }
}
