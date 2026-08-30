use crate::common::error::AppError;
use crate::modules::iam::application::user_service::{TenantContext, UserService};
use crate::modules::iam::infrastructure::test_data_repository::TestDataRepository;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestDataStatus {
    pub installed: bool,
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
        let installed = self.repository.is_installed(context.tenant_id).await?;
        Ok(TestDataStatus { installed })
    }

    pub async fn install(&self, context: &TenantContext) -> Result<TestDataStatus, AppError> {
        require_admin(context)?;
        let admin_id = self
            .user_service
            .get_or_create_user_id_from_ctx(context)
            .await?;
        self.repository.install(context.tenant_id, admin_id).await?;
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
