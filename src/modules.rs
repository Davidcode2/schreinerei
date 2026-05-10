pub mod billing;
pub mod fleet;
pub mod iam;
pub mod inventory;
pub mod onboarding;
pub mod sites;

/// Transitional alias while `sites` evolves into the broader project context.
/// Runtime routes and storage remain site-based for now, but new architectural
/// planning can depend on the `projects` boundary instead of deepening the
/// Baustelle-specific naming further.
pub mod projects {
    pub use super::sites::{api, application, domain, infrastructure};
}
