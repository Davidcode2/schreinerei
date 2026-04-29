use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Tenant identifier - wraps UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TenantId(pub Uuid);

impl TenantId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl Default for TenantId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TenantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// User identifier - wraps UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// User role in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    #[default]
    Employee,
}

impl Role {
    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin)
    }

    pub fn is_employee(&self) -> bool {
        matches!(self, Role::Employee)
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Employee => write!(f, "employee"),
        }
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(Role::Admin),
            "employee" | "mitarbeiter" => Ok(Role::Employee),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}

/// Material identifier - wraps UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MaterialId(pub Uuid);

impl MaterialId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl Default for MaterialId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MaterialId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Category identifier - wraps UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CategoryId(pub Uuid);

impl CategoryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl Default for CategoryId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CategoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Measurement unit for materials
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    Piece,
    Meter,
    SquareMeter,
    Liter,
    Kilogram,
}

impl Unit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Unit::Piece => "Stück",
            Unit::Meter => "Meter",
            Unit::SquareMeter => "Quadratmeter",
            Unit::Liter => "Liter",
            Unit::Kilogram => "Kilogramm",
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Unit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stück" | "piece" | "stk" => Ok(Unit::Piece),
            "meter" | "m" => Ok(Unit::Meter),
            "quadratmeter" | "m²" | "sqm" | "quadrat-meter" => Ok(Unit::SquareMeter),
            "liter" | "l" => Ok(Unit::Liter),
            "kilogram" | "kilogramm" | "kg" => Ok(Unit::Kilogram),
            _ => Err(format!("Unknown unit: {}", s)),
        }
    }
}

/// Order Request identifier - wraps UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrderRequestId(pub Uuid);

impl OrderRequestId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl Default for OrderRequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for OrderRequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Site identifier - wraps UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SiteId(pub Uuid);

impl SiteId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl Default for SiteId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SiteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Time Entry identifier - wraps UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TimeEntryId(pub Uuid);

impl TimeEntryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl Default for TimeEntryId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TimeEntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Activity identifier - wraps UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActivityId(pub Uuid);

impl ActivityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl Default for ActivityId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ActivityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Site status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SiteStatus {
    #[default]
    Planned,
    Active,
    Completed,
    Archived,
}

impl SiteStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SiteStatus::Planned => "planned",
            SiteStatus::Active => "active",
            SiteStatus::Completed => "completed",
            SiteStatus::Archived => "archived",
        }
    }
}

impl fmt::Display for SiteStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for SiteStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "planned" => Ok(SiteStatus::Planned),
            "active" => Ok(SiteStatus::Active),
            "completed" => Ok(SiteStatus::Completed),
            "archived" => Ok(SiteStatus::Archived),
            _ => Err(format!("Invalid site status: {}", s)),
        }
    }
}

/// Work type for time entries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkType {
    SiteWork,
    Workshop,
    Cnc,
    Delivery,
}

impl WorkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkType::SiteWork => "site_work",
            WorkType::Workshop => "workshop",
            WorkType::Cnc => "cnc",
            WorkType::Delivery => "delivery",
        }
    }
}

impl fmt::Display for WorkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for WorkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "site_work" | "sitework" => Ok(WorkType::SiteWork),
            "workshop" => Ok(WorkType::Workshop),
            "cnc" => Ok(WorkType::Cnc),
            "delivery" => Ok(WorkType::Delivery),
            _ => Err(format!("Invalid work type: {}", s)),
        }
    }
}

/// Assignment role for site assignments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AssignmentRole {
    Lead,
    #[default]
    Worker,
}

impl AssignmentRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            AssignmentRole::Lead => "lead",
            AssignmentRole::Worker => "worker",
        }
    }
}

impl fmt::Display for AssignmentRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for AssignmentRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lead" => Ok(AssignmentRole::Lead),
            "worker" => Ok(AssignmentRole::Worker),
            _ => Err(format!("Invalid assignment role: {}", s)),
        }
    }
}

/// Vehicle identifier - wraps UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VehicleId(pub Uuid);

impl VehicleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl Default for VehicleId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for VehicleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Tool identifier - wraps UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ToolId(pub Uuid);

impl ToolId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl Default for ToolId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ToolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Reservation identifier - wraps UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReservationId(pub Uuid);

impl ReservationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl Default for ReservationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ReservationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Resource type for vehicles and tools
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceType {
    Vehicle,
    Tool,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Vehicle => "vehicle",
            ResourceType::Tool => "tool",
        }
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ResourceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vehicle" => Ok(ResourceType::Vehicle),
            "tool" => Ok(ResourceType::Tool),
            _ => Err(format!("Invalid resource type: {}", s)),
        }
    }
}

/// Vehicle type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VehicleType {
    Car,
    Van,
    Truck,
    Trailer,
    Other,
}

impl VehicleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            VehicleType::Car => "car",
            VehicleType::Van => "van",
            VehicleType::Truck => "truck",
            VehicleType::Trailer => "trailer",
            VehicleType::Other => "other",
        }
    }
}

impl fmt::Display for VehicleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for VehicleType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "car" => Ok(VehicleType::Car),
            "van" => Ok(VehicleType::Van),
            "truck" => Ok(VehicleType::Truck),
            "trailer" => Ok(VehicleType::Trailer),
            "other" => Ok(VehicleType::Other),
            _ => Err(format!("Invalid vehicle type: {}", s)),
        }
    }
}

/// Resource status for vehicles and tools
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ResourceStatus {
    #[default]
    Available,
    Reserved,
    InUse,
    Maintenance,
}

impl ResourceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceStatus::Available => "available",
            ResourceStatus::Reserved => "reserved",
            ResourceStatus::InUse => "in_use",
            ResourceStatus::Maintenance => "maintenance",
        }
    }
}

impl fmt::Display for ResourceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ResourceStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "available" => Ok(ResourceStatus::Available),
            "reserved" => Ok(ResourceStatus::Reserved),
            "in_use" | "inuse" => Ok(ResourceStatus::InUse),
            "maintenance" => Ok(ResourceStatus::Maintenance),
            _ => Err(format!("Invalid resource status: {}", s)),
        }
    }
}

/// Reservation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ReservationStatus {
    #[default]
    Pending,
    Confirmed,
    InUse,
    Completed,
    Cancelled,
}

impl ReservationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReservationStatus::Pending => "pending",
            ReservationStatus::Confirmed => "confirmed",
            ReservationStatus::InUse => "in_use",
            ReservationStatus::Completed => "completed",
            ReservationStatus::Cancelled => "cancelled",
        }
    }
}

impl fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ReservationStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(ReservationStatus::Pending),
            "confirmed" => Ok(ReservationStatus::Confirmed),
            "in_use" | "inuse" => Ok(ReservationStatus::InUse),
            "completed" => Ok(ReservationStatus::Completed),
            "cancelled" => Ok(ReservationStatus::Cancelled),
            _ => Err(format!("Invalid reservation status: {}", s)),
        }
    }
}
