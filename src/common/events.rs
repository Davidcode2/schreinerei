use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::TenantId;

/// Unique identifier for a domain event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventId(pub Uuid);

impl std::fmt::Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Types of domain events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    // Inventory events
    MaterialCreated,
    StockWithdrawn,
    StockLow,
    StockAdjusted,
    MaterialAdded,
    LocationChanged,
    MinQuantityChanged,
    OrderRequestCreated,
    // Sites events
    SiteCreated,
    SiteStatusChanged,
    UserAssignedToSite,
    TimeEntryCreated,
    ActivityAdded,
    // Fleet events
    VehicleCreated,
    ToolCreated,
    ReservationCreated,
    ReservationUpdated,
    ReservationCancelled,
    ResourceStatusChanged,
}

/// A domain event representing something that happened in the domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub id: EventId,
    pub event_type: EventType,
    pub tenant_id: TenantId,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub payload: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl DomainEvent {
    pub fn new(
        event_type: EventType,
        tenant_id: TenantId,
        aggregate_type: impl Into<String>,
        aggregate_id: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: EventId(Uuid::new_v4()),
            event_type,
            tenant_id,
            aggregate_type: aggregate_type.into(),
            aggregate_id: aggregate_id.into(),
            payload,
            occurred_at: now,
            created_at: now,
        }
    }
}

/// Trait for aggregates that can emit events
pub trait EmitsEvents {
    fn take_events(&mut self) -> Vec<DomainEvent>;
}

/// Event store trait for persistence
#[async_trait]
pub trait EventStore: Send + Sync {
    async fn append(&self, event: &DomainEvent) -> Result<(), AppError>;
    async fn get_events(
        &self,
        tenant_id: TenantId,
        event_type: Option<EventType>,
        limit: i32,
    ) -> Result<Vec<DomainEvent>, AppError>;
}

/// Simple event bus for V1
/// Events are stored in database and can be queried by handlers
pub struct EventBus {
    // V1: Just store events, handlers poll
    // V2: Add pub/sub with channels or external queue
}

impl EventBus {
    pub fn new() -> Self {
        Self {}
    }

    /// Publish an event (stores to database)
    pub async fn publish(
        &self,
        event: &DomainEvent,
        pool: &PgPool,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO domain_events (id, event_type, tenant_id, aggregate_type, aggregate_id, payload, occurred_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(event.id.0)
        .bind(serde_json::to_string(&event.event_type).unwrap())
        .bind(event.tenant_id.0)
        .bind(&event.aggregate_type)
        .bind(&event.aggregate_id)
        .bind(&event.payload)
        .bind(event.occurred_at)
        .bind(event.created_at)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        tracing::info!(
            "Event published: {} for {}:{}",
            serde_json::to_string(&event.event_type).unwrap_or_default(),
            event.aggregate_type,
            event.aggregate_id
        );

        Ok(())
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
