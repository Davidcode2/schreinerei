use crate::common::error::AppError;
use crate::modules::iam::application::user_service::{TenantContext, UserService};
use crate::modules::iam::infrastructure::test_data_repository::{
    TestDataRepository, COMPLETE_TEST_DATA_RECORD_COUNT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestDataState {
    Complete,
    Partial,
    Absent,
}

impl TestDataState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Absent => "absent",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestDataStatus {
    pub installed: bool,
    pub state: TestDataState,
    pub removed_records: i64,
    pub retained_records: i64,
}

pub struct TestDataService {
    repository: TestDataRepository,
    user_service: UserService,
}

impl TestDataService {
    pub fn new(repository: TestDataRepository, user_service: UserService) -> Self {
        Self {
            repository,
            user_service,
        }
    }

    pub async fn status(&self, context: &TenantContext) -> Result<TestDataStatus, AppError> {
        require_admin(context)?;
        let retained_records = self
            .repository
            .seeded_record_count(context.tenant_id)
            .await?;
        Ok(status_from_counts(0, retained_records))
    }

    pub async fn install(&self, context: &TenantContext) -> Result<TestDataStatus, AppError> {
        require_admin(context)?;
        let admin_id = self
            .user_service
            .get_or_create_user_id_from_ctx(context)
            .await?;
        self.repository.install(context.tenant_id, admin_id).await?;
        let retained_records = self
            .repository
            .seeded_record_count(context.tenant_id)
            .await?;
        Ok(status_from_counts(0, retained_records))
    }

    pub async fn remove(&self, context: &TenantContext) -> Result<TestDataStatus, AppError> {
        require_admin(context)?;
        let removal = self.repository.remove(context.tenant_id).await?;
        Ok(status_from_counts(
            removal.removed_records,
            removal.retained_records,
        ))
    }
}

fn status_from_counts(removed_records: i64, retained_records: i64) -> TestDataStatus {
    let state = match retained_records {
        0 => TestDataState::Absent,
        COMPLETE_TEST_DATA_RECORD_COUNT => TestDataState::Complete,
        _ => TestDataState::Partial,
    };
    TestDataStatus {
        installed: retained_records > 0,
        state,
        removed_records,
        retained_records,
    }
}

fn require_admin(context: &TenantContext) -> Result<(), AppError> {
    if context.is_admin() {
        return Ok(());
    }
    Err(AppError::Forbidden("Admin access required".to_string()))
}
