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

/// Metadata and blob keys for activity photo attachments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteActivityAttachment {
    pub id: uuid::Uuid,
    pub tenant_id: TenantId,
    pub activity_id: ActivityId,
    pub site_id: SiteId,
    pub storage_key: String,
    pub thumbnail_key: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub original_bytes: Option<Vec<u8>>,
    pub thumbnail_bytes: Option<Vec<u8>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_site_id() -> SiteId {
        SiteId::new()
    }

    #[test]
    fn attachment_struct_holds_uuid_storage_keys() {
        let attachment = SiteActivityAttachment {
            id: uuid::Uuid::new_v4(),
            tenant_id: TenantId::new(),
            activity_id: ActivityId::new(),
            site_id: SiteId::new(),
            storage_key: format!("{}.jpg", uuid::Uuid::new_v4()),
            thumbnail_key: format!("{}.jpg", uuid::Uuid::new_v4()),
            mime_type: "image/jpeg".to_string(),
            size_bytes: 12,
            original_bytes: Some(vec![1, 2, 3]),
            thumbnail_bytes: Some(vec![4, 5, 6]),
            created_at: Utc::now(),
        };

        assert!(attachment.storage_key.contains('-'));
        assert!(attachment.thumbnail_key.contains('-'));
    }

    #[test]
    fn activity_type_as_str_returns_correct_strings() {
        assert_eq!(ActivityType::Photo.as_str(), "photo");
        assert_eq!(ActivityType::Note.as_str(), "note");
        assert_eq!(ActivityType::StatusChange.as_str(), "status_change");
    }

    #[test]
    fn activity_type_from_str_parses_valid_strings() {
        let photo: Result<ActivityType, String> = "photo".parse();
        assert_eq!(photo, Ok(ActivityType::Photo));

        let note: Result<ActivityType, String> = "note".parse();
        assert_eq!(note, Ok(ActivityType::Note));

        let status_change: Result<ActivityType, String> = "status_change".parse();
        assert_eq!(status_change, Ok(ActivityType::StatusChange));
    }

    #[test]
    fn activity_type_from_str_fails_with_invalid_string() {
        let result: Result<ActivityType, String> = "invalid".parse();
        assert!(result.is_err());
    }

    #[test]
    fn create_activity_validate_succeeds_for_photo_with_url() {
        let cmd = CreateActivity {
            site_id: test_site_id(),
            activity_type: ActivityType::Photo,
            content: None,
            photo_url: Some("https://example.com/photo.jpg".to_string()),
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_activity_validate_fails_for_photo_without_url() {
        let cmd = CreateActivity {
            site_id: test_site_id(),
            activity_type: ActivityType::Photo,
            content: None,
            photo_url: None,
        };
        assert_eq!(cmd.validate(), Err("Photo URL is required for photo activity".to_string()));
    }

    #[test]
    fn create_activity_validate_succeeds_for_note_with_content() {
        let cmd = CreateActivity {
            site_id: test_site_id(),
            activity_type: ActivityType::Note,
            content: Some("This is a note".to_string()),
            photo_url: None,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_activity_validate_fails_for_note_without_content() {
        let cmd = CreateActivity {
            site_id: test_site_id(),
            activity_type: ActivityType::Note,
            content: None,
            photo_url: None,
        };
        assert_eq!(cmd.validate(), Err("Content is required for note activity".to_string()));
    }

    #[test]
    fn create_activity_validate_fails_for_note_with_empty_content() {
        let cmd = CreateActivity {
            site_id: test_site_id(),
            activity_type: ActivityType::Note,
            content: Some("   ".to_string()),
            photo_url: None,
        };
        assert_eq!(cmd.validate(), Err("Content is required for note activity".to_string()));
    }

    #[test]
    fn create_activity_validate_fails_for_status_change() {
        let cmd = CreateActivity {
            site_id: test_site_id(),
            activity_type: ActivityType::StatusChange,
            content: None,
            photo_url: None,
        };
        assert_eq!(cmd.validate(), Err("Cannot manually create status change activity".to_string()));
    }
}
