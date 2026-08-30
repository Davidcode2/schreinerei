use crate::common::error::AppError;
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::iam::infrastructure::test_data_repository::TestDataRepository;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestDataStatus {
    pub installed: bool,
}

pub struct TestDataService {
    repository: TestDataRepository,
}

impl TestDataService {
    pub fn new(repository: TestDataRepository) -> Self {
        Self { repository }
    }

    pub async fn status(&self, context: &TenantContext) -> Result<TestDataStatus, AppError> {
        require_admin(context)?;
        let installed = self.repository.is_installed(context.tenant_id).await?;
        Ok(TestDataStatus { installed })
    }

    pub async fn install(&self, context: &TenantContext) -> Result<TestDataStatus, AppError> {
        require_admin(context)?;
        self.repository
            .install(context.tenant_id, context.user_id)
            .await?;
        Ok(TestDataStatus { installed: true })
    }

    pub async fn remove(&self, context: &TenantContext) -> Result<TestDataStatus, AppError> {
        require_admin(context)?;
        self.repository.remove(context.tenant_id).await?;
        Ok(TestDataStatus { installed: false })
    }
}

fn require_admin(context: &TenantContext) -> Result<(), AppError> {
    if context.is_admin() {
        return Ok(());
    }
    Err(AppError::Forbidden("Admin access required".to_string()))
}
