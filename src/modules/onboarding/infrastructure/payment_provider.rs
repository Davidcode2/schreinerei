use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::common::error::AppError;
use crate::modules::onboarding::domain::ProviderPaymentStatus;

#[derive(Clone)]
pub struct MolliePaymentProvider {
    api_key: String,
    base_url: String,
    client: Client,
}

impl MolliePaymentProvider {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            base_url,
            client: Client::new(),
        }
    }

    pub async fn fetch_payment(&self, payment_id: &str) -> Result<ProviderPayment, AppError> {
        let response = self
            .client
            .get(format!(
                "{}/v2/payments/{}",
                self.base_url.trim_end_matches('/'),
                payment_id
            ))
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|error| {
                AppError::Internal(format!("Failed to fetch Mollie payment: {error}"))
            })?;

        if !response.status().is_success() {
            return Err(AppError::Validation(format!(
                "Mollie payment lookup failed with status {}",
                response.status()
            )));
        }

        response
            .json::<ProviderPayment>()
            .await
            .map_err(|error| AppError::Internal(format!("Failed to parse Mollie payment: {error}")))
    }

    pub async fn create_checkout(
        &self,
        request: CreateMolliePaymentRequest,
    ) -> Result<ProviderCheckout, AppError> {
        let response = self
            .client
            .post(format!(
                "{}/v2/payments",
                self.base_url.trim_end_matches('/')
            ))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|error| {
                AppError::Internal(format!("Failed to create Mollie payment: {error}"))
            })?;

        if !response.status().is_success() {
            return Err(AppError::Validation(format!(
                "Mollie payment creation failed with status {}",
                response.status()
            )));
        }

        let payment = response
            .json::<CreatedMolliePayment>()
            .await
            .map_err(|error| {
                AppError::Internal(format!("Failed to parse Mollie payment: {error}"))
            })?;

        let checkout_url = payment
            .links
            .checkout
            .map(|checkout| checkout.href)
            .filter(|href| !href.trim().is_empty())
            .ok_or_else(|| {
                AppError::Internal("Mollie payment did not include checkout URL".to_string())
            })?;

        Ok(ProviderCheckout {
            payment_id: payment.id,
            checkout_url,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateMolliePaymentRequest {
    pub amount: MollieAmount,
    pub description: String,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    #[serde(rename = "webhookUrl")]
    pub webhook_url: String,
    pub metadata: MolliePaymentMetadata,
}

#[derive(Debug, Clone, Serialize)]
pub struct MollieAmount {
    pub currency: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MolliePaymentMetadata {
    pub onboarding_session_id: String,
    pub organization_slug: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCheckout {
    pub payment_id: String,
    pub checkout_url: String,
}

#[derive(Debug, Deserialize)]
struct CreatedMolliePayment {
    id: String,
    #[serde(rename = "_links")]
    links: CreatedMolliePaymentLinks,
}

#[derive(Debug, Deserialize)]
struct CreatedMolliePaymentLinks {
    checkout: Option<MollieLink>,
}

#[derive(Debug, Deserialize)]
struct MollieLink {
    href: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderPayment {
    pub id: String,
    pub status: String,
}

impl ProviderPayment {
    pub fn payment_status(&self) -> ProviderPaymentStatus {
        ProviderPaymentStatus::from_str(&self.status).unwrap_or(ProviderPaymentStatus::Unknown)
    }
}
