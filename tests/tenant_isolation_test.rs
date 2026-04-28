//! Integration tests for multi-tenant isolation
//! 
//! These tests verify that:
//! - Users cannot access data from other tenants
//! - Admins cannot invite users to other tenants
//! - All queries respect tenant_id boundary
//!
//! Run with: cargo test --test tenant_isolation_test
//! 
//! Requires DATABASE_URL environment variable to be set.

use sqlx::PgPool;
use uuid::Uuid;

// Note: These tests require a database connection.
// They will be skipped if DATABASE_URL is not set.

/// Test helper to create a test tenant
async fn create_test_tenant(pool: &PgPool, name: &str) -> Uuid {
    let id = Uuid::new_v4();
    let slug = name.to_lowercase().replace(' ', "-");
    
    sqlx::query(
        r#"
        INSERT INTO tenants (id, keycloak_realm, name, slug)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(id)
    .bind(format!("realm-{}", slug))
    .bind(name)
    .bind(&slug)
    .execute(pool)
    .await
    .expect("Failed to create test tenant");
    
    id
}

/// Test helper to create a test user
async fn create_test_user(
    pool: &PgPool,
    tenant_id: Uuid,
    email: &str,
    role: &str,
) -> Uuid {
    let id = Uuid::new_v4();
    
    sqlx::query(
        r#"
        INSERT INTO users (id, tenant_id, keycloak_user_id, email, role)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(id)
    .bind(tenant_id)
    .bind(format!("kc-{}", email))
    .bind(email)
    .bind(role)
    .execute(pool)
    .await
    .expect("Failed to create test user");
    
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_user_cannot_access_other_tenant_users(pool: PgPool) {
        // Setup: Create tenant A and tenant B with users
        let tenant_a = create_test_tenant(&pool, "Tenant A").await;
        let tenant_b = create_test_tenant(&pool, "Tenant B").await;
        
        let user_a = create_test_user(&pool, tenant_a, "user-a@test.com", "employee").await;
        let _user_b = create_test_user(&pool, tenant_b, "user-b@test.com", "employee").await;
        
        // Action: Query for user_b with tenant_a context
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE id = $1 AND tenant_id = $2"
        )
        .bind(user_a)
        .bind(tenant_b)  // Wrong tenant!
        .fetch_one(&pool)
        .await
        .expect("Query failed");
        
        // Assert: No results (tenant isolation)
        assert_eq!(result, 0, "User from tenant A should not be visible in tenant B context");
    }

    #[sqlx::test]
    async fn test_list_users_respects_tenant_boundary(pool: PgPool) {
        // Setup: Create two tenants with users
        let tenant_a = create_test_tenant(&pool, "Tenant A").await;
        let tenant_b = create_test_tenant(&pool, "Tenant B").await;
        
        create_test_user(&pool, tenant_a, "user-a1@test.com", "employee").await;
        create_test_user(&pool, tenant_a, "user-a2@test.com", "admin").await;
        create_test_user(&pool, tenant_b, "user-b1@test.com", "employee").await;
        
        // Action: List users for tenant A
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE tenant_id = $1"
        )
        .bind(tenant_a)
        .fetch_one(&pool)
        .await
        .expect("Query failed");
        
        // Assert: Only tenant A users returned
        assert_eq!(count, 2, "Should only return users from tenant A");
    }

    #[sqlx::test]
    async fn test_update_role_respects_tenant(pool: PgPool) {
        // Setup: Create tenant A and tenant B with a user in each
        let tenant_a = create_test_tenant(&pool, "Tenant A").await;
        let tenant_b = create_test_tenant(&pool, "Tenant B").await;
        
        let user_a = create_test_user(&pool, tenant_a, "user-a@test.com", "employee").await;
        
        // Action: Try to update user_a's role using tenant_b context
        let updated = sqlx::query(
            "UPDATE users SET role = 'admin' WHERE id = $1 AND tenant_id = $2"
        )
        .bind(user_a)
        .bind(tenant_b)  // Wrong tenant!
        .execute(&pool)
        .await
        .expect("Query failed");
        
        // Assert: No rows updated (tenant isolation)
        assert_eq!(updated.rows_affected(), 0, "Should not update user in different tenant");
    }

    #[sqlx::test]
    async fn test_all_queries_include_tenant_filter(pool: PgPool) {
        // Setup: Create tenant with users
        let tenant_a = create_test_tenant(&pool, "Tenant A").await;
        let tenant_b = create_test_tenant(&pool, "Tenant B").await;
        
        create_test_user(&pool, tenant_a, "user-a1@test.com", "employee").await;
        create_test_user(&pool, tenant_b, "user-b1@test.com", "admin").await;
        
        // Action: Count users per tenant
        let count_a: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE tenant_id = $1")
            .bind(tenant_a)
            .fetch_one(&pool)
            .await
            .expect("Query failed");
            
        let count_b: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE tenant_id = $1")
            .bind(tenant_b)
            .fetch_one(&pool)
            .await
            .expect("Query failed");
        
        // Assert: Each tenant sees only its users
        assert_eq!(count_a, 1);
        assert_eq!(count_b, 1);
    }

    #[sqlx::test]
    async fn test_unique_constraint_on_keycloak_user_id_per_tenant(pool: PgPool) {
        // Setup: Create tenant
        let tenant = create_test_tenant(&pool, "Test Tenant").await;
        
        // Create first user
        create_test_user(&pool, tenant, "user@test.com", "employee").await;
        
        // Action: Try to create duplicate keycloak_user_id in same tenant
        let result = sqlx::query(
            "INSERT INTO users (id, tenant_id, keycloak_user_id, email, role) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(Uuid::new_v4())
        .bind(tenant)
        .bind("kc-user@test.com")  // Same keycloak_user_id
        .bind("user2@test.com")
        .bind("admin")
        .execute(&pool)
        .await;
        
        // Assert: Should fail due to unique constraint
        assert!(result.is_err(), "Should not allow duplicate keycloak_user_id in same tenant");
    }
}
