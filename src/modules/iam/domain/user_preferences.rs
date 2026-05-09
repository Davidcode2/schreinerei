use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::common::types::{TenantId, UserId};

#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct UserPreferences {
    pub active_site_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferenceRecord {
    pub id: uuid::Uuid,
    pub tenant_id: TenantId,
    pub user_id: UserId,
    pub preferences: UserPreferences,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct UpdatePreferences {
    pub active_site_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_preferences_default_has_no_active_site() {
        let prefs = UserPreferences::default();
        assert!(prefs.active_site_id.is_none());
    }

    #[test]
    fn user_preferences_can_set_active_site() {
        let prefs = UserPreferences {
            active_site_id: Some("site-123".to_string()),
        };
        assert_eq!(prefs.active_site_id, Some("site-123".to_string()));
    }
}
