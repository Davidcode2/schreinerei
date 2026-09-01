use async_trait::async_trait;
use schreinerei::common::error::AppError;
use schreinerei::common::types::{Role, TenantId, UserId};
use schreinerei::modules::iam::api::routes::UserResponse;
use schreinerei::modules::iam::application::user_service::{
    KeycloakUserManager, TenantContext, UserService,
};
use schreinerei::modules::iam::infrastructure::user_repository::UserRepository;
use sqlx::PgPool;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[sqlx::test]
async fn user_repository_excludes_soft_deleted_management_targets(pool: PgPool) {
    let tenant_id = insert_tenant(&pool, "repository-filter").await;
    let active_id = insert_user(&pool, tenant_id, "active", false).await;
    let deleted_id = insert_user(&pool, tenant_id, "deleted", true).await;
    let repository = UserRepository::new(pool);

    let users = repository.list(TenantId(tenant_id)).await.unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].id, UserId(active_id));
    assert!(repository
        .find_by_id(UserId(deleted_id), TenantId(tenant_id))
        .await
        .unwrap()
        .is_none());
}

#[sqlx::test]
async fn original_admin_index_allows_only_one_active_original_per_tenant(pool: PgPool) {
    let tenant_id = insert_tenant(&pool, "original-index").await;
    insert_original_admin(&pool, tenant_id, "first")
        .await
        .unwrap();

    let duplicate = insert_original_admin(&pool, tenant_id, "second").await;

    assert!(duplicate.is_err());
}

#[sqlx::test]
async fn original_admin_marker_is_immutable(pool: PgPool) {
    let tenant_id = insert_tenant(&pool, "original-immutable").await;
    let original_id = insert_original_admin(&pool, tenant_id, "immutable")
        .await
        .unwrap();

    let result = sqlx::query("UPDATE users SET is_original_admin = FALSE WHERE id = $1")
        .bind(original_id)
        .execute(&pool)
        .await;

    assert!(result.is_err());
}

#[sqlx::test]
async fn original_admin_role_and_active_status_are_immutable(pool: PgPool) {
    let tenant_id = insert_tenant(&pool, "original-protected").await;
    let original_id = insert_original_admin(&pool, tenant_id, "protected")
        .await
        .unwrap();

    let demotion = sqlx::query("UPDATE users SET role = 'employee' WHERE id = $1")
        .bind(original_id)
        .execute(&pool)
        .await;
    let deletion = sqlx::query("UPDATE users SET deleted_at = NOW() WHERE id = $1")
        .bind(original_id)
        .execute(&pool)
        .await;

    assert!(demotion.is_err());
    assert!(deletion.is_err());
}

#[sqlx::test]
async fn admin_role_change_synchronizes_keycloak_before_local_role(pool: PgPool) {
    let tenant_id = insert_tenant_with_organization(&pool, "role-sync").await;
    let actor_subject = Uuid::new_v4();
    let target_id = insert_user(&pool, tenant_id, "target-role", false).await;
    let manager = Arc::new(FakeKeycloakManager::default());
    let service = managed_service(&pool, manager.clone());

    let user = service
        .update_role(
            UserId(target_id),
            Role::Admin,
            &admin_context(tenant_id, actor_subject),
        )
        .await
        .unwrap();

    assert_eq!(user.role, Role::Admin);
    assert_eq!(manager.calls(), vec!["role:target-role:admin"]);
}

#[sqlx::test]
async fn role_change_rejects_original_admin_and_caller_self(pool: PgPool) {
    let tenant_id = insert_tenant_with_organization(&pool, "protected-users").await;
    let actor_subject = Uuid::new_v4();
    let original_id = insert_original_admin(&pool, tenant_id, "original")
        .await
        .unwrap();
    let self_id = insert_user(&pool, tenant_id, &actor_subject.to_string(), false).await;
    let manager = Arc::new(FakeKeycloakManager::default());
    let service = managed_service(&pool, manager.clone());
    let context = admin_context(tenant_id, actor_subject);

    let original_error = service
        .update_role(UserId(original_id), Role::Employee, &context)
        .await
        .unwrap_err();
    let self_error = service
        .update_role(UserId(self_id), Role::Employee, &context)
        .await
        .unwrap_err();

    assert!(matches!(original_error, AppError::Conflict(_)));
    assert!(matches!(self_error, AppError::Conflict(_)));
    assert!(manager.calls().is_empty());
}

#[sqlx::test]
async fn delete_removes_organization_member_then_soft_deletes_user(pool: PgPool) {
    let organization_id = Uuid::new_v4();
    let tenant_id =
        insert_tenant_with_specific_organization(&pool, "delete-user", organization_id).await;
    let target_id = insert_user(&pool, tenant_id, "target-delete", false).await;
    let manager = Arc::new(FakeKeycloakManager::default());
    let service = managed_service(&pool, manager.clone());

    service
        .delete_user(UserId(target_id), &admin_context(tenant_id, Uuid::new_v4()))
        .await
        .unwrap();

    assert_eq!(
        manager.calls(),
        vec![format!("remove:{organization_id}:target-delete")]
    );
    let deleted_at: Option<chrono::DateTime<chrono::Utc>> =
        sqlx::query_scalar("SELECT deleted_at FROM users WHERE id = $1")
            .bind(target_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(deleted_at.is_some());
}

#[sqlx::test]
async fn admin_can_demote_and_delete_a_later_admin(pool: PgPool) {
    let tenant_id = insert_tenant_with_organization(&pool, "later-admin").await;
    let actor_subject = Uuid::new_v4();
    let target_id = insert_user(&pool, tenant_id, "later-admin-target", false).await;
    sqlx::query("UPDATE users SET role = 'admin' WHERE id = $1")
        .bind(target_id)
        .execute(&pool)
        .await
        .unwrap();
    let service = managed_service(&pool, Arc::new(FakeKeycloakManager::default()));
    let context = admin_context(tenant_id, actor_subject);

    let demoted = service
        .update_role(UserId(target_id), Role::Employee, &context)
        .await
        .unwrap();
    service
        .delete_user(UserId(target_id), &context)
        .await
        .unwrap();

    assert_eq!(demoted.role, Role::Employee);
    let deleted_at: Option<chrono::DateTime<chrono::Utc>> =
        sqlx::query_scalar("SELECT deleted_at FROM users WHERE id = $1")
            .bind(target_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(deleted_at.is_some());
}

#[sqlx::test]
async fn failed_keycloak_change_leaves_local_user_unchanged(pool: PgPool) {
    let tenant_id = insert_tenant_with_organization(&pool, "keycloak-failure").await;
    let target_id = insert_user(&pool, tenant_id, "target-failure", false).await;
    let manager = Arc::new(FakeKeycloakManager::failing());
    let service = managed_service(&pool, manager);

    let error = service
        .update_role(
            UserId(target_id),
            Role::Admin,
            &admin_context(tenant_id, Uuid::new_v4()),
        )
        .await;

    assert!(error.is_err());
    let role: String = sqlx::query_scalar("SELECT role FROM users WHERE id = $1")
        .bind(target_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(role, "employee");
}

#[sqlx::test]
async fn user_response_derives_management_policy_from_caller_subject(pool: PgPool) {
    let tenant_id = insert_tenant(&pool, "response-policy").await;
    let caller_subject = Uuid::new_v4();
    let self_id = insert_user(&pool, tenant_id, &caller_subject.to_string(), false).await;
    let other_id = insert_user(&pool, tenant_id, "other-response", false).await;
    let repository = UserRepository::new(pool);

    let own_user = repository
        .find_by_id(UserId(self_id), TenantId(tenant_id))
        .await
        .unwrap()
        .unwrap();
    let other_user = repository
        .find_by_id(UserId(other_id), TenantId(tenant_id))
        .await
        .unwrap()
        .unwrap();

    assert!(!UserResponse::from_user(own_user, &caller_subject.to_string()).can_manage);
    assert!(UserResponse::from_user(other_user, &caller_subject.to_string()).can_manage);
}

async fn insert_tenant(pool: &PgPool, suffix: &str) -> Uuid {
    sqlx::query_scalar(
        "INSERT INTO tenants (keycloak_realm, name, slug) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(format!("realm-{suffix}"))
    .bind(suffix)
    .bind(suffix)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn insert_tenant_with_organization(pool: &PgPool, suffix: &str) -> Uuid {
    insert_tenant_with_specific_organization(pool, suffix, Uuid::new_v4()).await
}

async fn insert_tenant_with_specific_organization(
    pool: &PgPool,
    suffix: &str,
    organization_id: Uuid,
) -> Uuid {
    sqlx::query_scalar(
        r#"
        INSERT INTO tenants (keycloak_realm, name, slug, keycloak_organization_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind(format!("realm-{suffix}"))
    .bind(suffix)
    .bind(suffix)
    .bind(organization_id)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn insert_user(pool: &PgPool, tenant_id: Uuid, subject: &str, deleted: bool) -> Uuid {
    sqlx::query_scalar(
        r#"
        INSERT INTO users (tenant_id, keycloak_user_id, email, role, deleted_at)
        VALUES ($1, $2, $3, 'employee', CASE WHEN $4 THEN NOW() ELSE NULL END)
        RETURNING id
        "#,
    )
    .bind(tenant_id)
    .bind(subject)
    .bind(format!("{subject}@example.com"))
    .bind(deleted)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn insert_original_admin(
    pool: &PgPool,
    tenant_id: Uuid,
    subject: &str,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query_scalar(
        r#"
        INSERT INTO users (tenant_id, keycloak_user_id, email, role, is_original_admin)
        VALUES ($1, $2, $3, 'admin', TRUE)
        RETURNING id
        "#,
    )
    .bind(tenant_id)
    .bind(subject)
    .bind(format!("{subject}@example.com"))
    .fetch_one(pool)
    .await
}

fn admin_context(tenant_id: Uuid, subject: Uuid) -> TenantContext {
    TenantContext {
        tenant_id: TenantId(tenant_id),
        user_id: UserId(subject),
        email: "admin@example.com".to_string(),
        roles: vec![Role::Admin],
        token_roles: vec![Role::Admin],
    }
}

fn managed_service(pool: &PgPool, manager: Arc<FakeKeycloakManager>) -> UserService {
    UserService::new_with_keycloak_manager(UserRepository::new(pool.clone()), manager)
}

#[derive(Default)]
struct FakeKeycloakManager {
    calls: Mutex<Vec<String>>,
    fail: bool,
}

impl FakeKeycloakManager {
    fn failing() -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
            fail: true,
        }
    }

    fn calls(&self) -> Vec<String> {
        self.calls.lock().unwrap().clone()
    }
}

#[async_trait]
impl KeycloakUserManager for FakeKeycloakManager {
    async fn synchronize_realm_role(&self, user_id: &str, role: Role) -> Result<(), AppError> {
        self.calls
            .lock()
            .unwrap()
            .push(format!("role:{user_id}:{role}"));
        if self.fail {
            return Err(AppError::Internal("keycloak failed".to_string()));
        }
        Ok(())
    }

    async fn remove_organization_member(
        &self,
        organization_id: &str,
        user_id: &str,
    ) -> Result<(), AppError> {
        self.calls
            .lock()
            .unwrap()
            .push(format!("remove:{organization_id}:{user_id}"));
        if self.fail {
            return Err(AppError::Internal("keycloak failed".to_string()));
        }
        Ok(())
    }
}
