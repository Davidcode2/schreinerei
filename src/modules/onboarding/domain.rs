use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateOnboardingSession {
    pub organization_name: String,
    pub admin_email: String,
    pub admin_name: Option<String>,
    pub selected_plan: String,
}

impl CreateOnboardingSession {
    pub fn validate(&self) -> Result<(), String> {
        if self.organization_name.trim().is_empty() {
            return Err("Organization name is required".to_string());
        }
        if self.organization_name.trim().len() > 255 {
            return Err("Organization name must be 255 characters or less".to_string());
        }
        if self.admin_email.trim().is_empty() {
            return Err("Admin email is required".to_string());
        }
        if !self.admin_email.contains('@') {
            return Err("Invalid admin email format".to_string());
        }
        if self.selected_plan.trim().is_empty() {
            return Err("Selected plan is required".to_string());
        }
        if self.selected_plan.trim().len() > 100 {
            return Err("Selected plan must be 100 characters or less".to_string());
        }

        Ok(())
    }

    pub fn normalized_admin_email(&self) -> String {
        self.admin_email.trim().to_lowercase()
    }

    pub fn normalized_admin_name(&self) -> Option<String> {
        self.admin_name
            .as_ref()
            .map(|name| name.trim().to_string())
            .filter(|name| !name.is_empty())
    }

    pub fn normalized_organization_name(&self) -> String {
        self.organization_name.trim().to_string()
    }

    pub fn normalized_selected_plan(&self) -> String {
        self.selected_plan.trim().to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingSession {
    pub id: Uuid,
    pub organization_slug: String,
    pub admin_email: String,
    pub selected_plan: String,
    pub status: OnboardingStatus,
    pub payment_provider: Option<String>,
    pub payment_id: Option<String>,
    pub checkout_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvisioningSession {
    pub id: Uuid,
    pub organization_name: String,
    pub organization_slug: String,
    pub admin_email: String,
    pub admin_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeycloakOrganization {
    pub id: Uuid,
    pub alias: String,
}

pub fn organization_slug(name: &str) -> String {
    let mut slug = String::with_capacity(name.len());
    let mut previous_was_separator = false;

    for character in name.trim().to_lowercase().chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character);
            previous_was_separator = false;
        } else if !previous_was_separator && !slug.is_empty() {
            slug.push('-');
            previous_was_separator = true;
        }
    }

    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        "organization".to_string()
    } else {
        slug.chars()
            .take(100)
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingStatus {
    PendingPayment,
    PaymentConfirmed,
    PaymentFailed,
    Provisioning,
    Completed,
    KeycloakFailed,
}

impl OnboardingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PendingPayment => "pending_payment",
            Self::PaymentConfirmed => "payment_confirmed",
            Self::PaymentFailed => "payment_failed",
            Self::Provisioning => "provisioning",
            Self::Completed => "completed",
            Self::KeycloakFailed => "keycloak_failed",
        }
    }
}

impl fmt::Display for OnboardingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for OnboardingStatus {
    type Err = String;

    fn from_str(status: &str) -> Result<Self, Self::Err> {
        match status {
            "pending_payment" => Ok(Self::PendingPayment),
            "payment_confirmed" => Ok(Self::PaymentConfirmed),
            "payment_failed" => Ok(Self::PaymentFailed),
            "provisioning" => Ok(Self::Provisioning),
            "completed" => Ok(Self::Completed),
            "keycloak_failed" => Ok(Self::KeycloakFailed),
            _ => Err(format!("Unknown onboarding status: {status}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderPaymentStatus {
    Open,
    Pending,
    Authorized,
    Paid,
    Failed,
    Canceled,
    Expired,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InviteStatus {
    Pending,
    Failed,
    Accepted,
    Expired,
}

impl InviteStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Failed => "failed",
            Self::Accepted => "accepted",
            Self::Expired => "expired",
        }
    }
}

impl fmt::Display for InviteStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for InviteStatus {
    type Err = String;

    fn from_str(status: &str) -> Result<Self, Self::Err> {
        match status {
            "pending" => Ok(Self::Pending),
            "failed" => Ok(Self::Failed),
            "accepted" => Ok(Self::Accepted),
            "expired" => Ok(Self::Expired),
            _ => Err(format!("Unknown invite status: {status}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationInvite {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub role: String,
    pub token: String,
    pub status: InviteStatus,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl ProviderPaymentStatus {
    pub fn should_confirm_payment(self) -> bool {
        matches!(self, Self::Paid)
    }

    pub fn should_fail_payment(self) -> bool {
        matches!(self, Self::Failed | Self::Canceled | Self::Expired)
    }
}

impl FromStr for ProviderPaymentStatus {
    type Err = String;

    fn from_str(status: &str) -> Result<Self, Self::Err> {
        match status {
            "open" => Ok(Self::Open),
            "pending" => Ok(Self::Pending),
            "authorized" => Ok(Self::Authorized),
            "paid" => Ok(Self::Paid),
            "failed" => Ok(Self::Failed),
            "canceled" => Ok(Self::Canceled),
            "expired" => Ok(Self::Expired),
            _ => Ok(Self::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InviteStatus, OnboardingStatus, ProviderPaymentStatus};
    use std::str::FromStr;

    #[test]
    fn only_provider_paid_status_confirms_payment() {
        assert!(ProviderPaymentStatus::Paid.should_confirm_payment());
        assert!(!ProviderPaymentStatus::Pending.should_confirm_payment());
        assert!(!ProviderPaymentStatus::Authorized.should_confirm_payment());
    }

    #[test]
    fn terminal_provider_failures_mark_payment_failed() {
        assert!(ProviderPaymentStatus::Failed.should_fail_payment());
        assert!(ProviderPaymentStatus::Canceled.should_fail_payment());
        assert!(ProviderPaymentStatus::Expired.should_fail_payment());
        assert!(!ProviderPaymentStatus::Open.should_fail_payment());
    }

    #[test]
    fn onboarding_status_round_trips_database_values() {
        let status = OnboardingStatus::from_str("payment_confirmed").unwrap();
        assert_eq!(status.as_str(), "payment_confirmed");
    }

    #[test]
    fn invite_status_round_trips_database_values() {
        let status = InviteStatus::from_str("pending").unwrap();
        assert_eq!(status.as_str(), "pending");
    }

    #[test]
    fn organization_slug_normalizes_for_database_aliases() {
        assert_eq!(
            super::organization_slug("  Schreinerei Mueller & Soehne GmbH  "),
            "schreinerei-mueller-soehne-gmbh"
        );
        assert_eq!(super::organization_slug("!!!"), "organization");
    }

    #[test]
    fn onboarding_session_requires_core_fields() {
        let command = super::CreateOnboardingSession {
            organization_name: "Schreinerei Beispiel".to_string(),
            admin_email: "admin@example.com".to_string(),
            admin_name: Some("Ada Admin".to_string()),
            selected_plan: "starter".to_string(),
        };

        assert!(command.validate().is_ok());
    }
}
