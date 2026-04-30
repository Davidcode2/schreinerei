# Testing Architecture Research

**Domain:** Rust Hexagonal Architecture Backend + React SPA Frontend
**Researched:** 2026-04-30
**Confidence:** HIGH

## Executive Summary

Testing a hexagonal architecture requires distinct strategies for each layer. Domain layer tests are pure unit tests with no dependencies. Application layer tests mock infrastructure using trait-based ports. Infrastructure layer tests are integration tests with a real database. Frontend tests use Vitest + React Testing Library + MSW for component isolation.

## Test Organization Structure

### Backend Test Organization

```
schreinerei/
├── src/
│   └── modules/
│       └── inventory/
│           ├── domain/
│           │   ├── material.rs
│           │   └── material.rs.tests  # Inline unit tests (cargo test)
│           ├── application/
│           │   ├── inventory_service.rs
│           │   └── inventory_service.rs.tests  # Unit tests with mocks
│           └── infrastructure/
│               └── material_repository.rs
├── tests/                               # Integration tests directory
│   ├── tenant_isolation_test.rs        # Existing multi-tenant tests
│   ├── inventory_integration_test.rs   # NEW: Module integration tests
│   ├── sites_integration_test.rs       # NEW
│   └── fleet_integration_test.rs       # NEW
└── migrations/                          # SQLx migrations (used by tests)
```

### Frontend Test Organization

```
frontend/
├── src/
│   ├── components/
│   │   ├── MaterialList.tsx
│   │   └── __tests__/
│   │       └── MaterialList.test.tsx    # Component tests
│   ├── pages/
│   │   ├── Inventory.tsx
│   │   └── __tests__/
│   │       └── Inventory.test.tsx       # Page tests
│   ├── hooks/
│   │   ├── useMaterials.ts
│   │   └── __tests__/
│   │       └── useMaterials.test.ts     # Hook tests
│   └── test/
│       ├── setup.ts                     # Vitest setup
│       ├── msw/                         # Mock Service Worker handlers
│       │   ├── handlers.ts
│       │   └── server.ts
│       └── utils.tsx                    # Test utilities
├── vitest.config.ts
└── playwright.config.ts                 # E2E tests (existing)
```

## Testing Strategies by Layer

### 1. Domain Layer Tests (Pure Unit Tests)

**What to test:**
- Entity business logic (validation, state transitions)
- Value object invariants
- Domain rules (no dependencies)

**Test pattern:**
```rust
// src/modules/inventory/domain/material.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::*;
    use chrono::Utc;

    fn create_test_material(quantity: i32, min_quantity: i32) -> Material {
        Material {
            id: MaterialId(Uuid::new_v4()),
            tenant_id: TenantId(Uuid::new_v4()),
            category_id: CategoryId(Uuid::new_v4()),
            name: "Test Material".to_string(),
            description: None,
            unit: Unit::Piece,
            quantity,
            min_quantity,
            location: None,
            qr_code: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_is_low_stock_when_at_threshold() {
        let material = create_test_material(5, 10);
        assert!(material.is_low_stock());
    }

    #[test]
    fn test_is_low_stock_when_above_threshold() {
        let material = create_test_material(15, 10);
        assert!(!material.is_low_stock());
    }

    #[test]
    fn test_can_withdraw_sufficient_stock() {
        let material = create_test_material(10, 5);
        assert!(material.can_withdraw(5));
        assert!(material.can_withdraw(10));
    }

    #[test]
    fn test_cannot_withdraw_insufficient_stock() {
        let material = create_test_material(5, 5);
        assert!(!material.can_withdraw(10));
    }

    #[test]
    fn test_create_material_validation() {
        let valid = CreateMaterial {
            category_id: CategoryId(Uuid::new_v4()),
            name: "Valid Name".to_string(),
            description: None,
            unit: Unit::Piece,
            quantity: 10,
            min_quantity: 5,
            location: None,
        };
        assert!(valid.validate().is_ok());

        let invalid_name = CreateMaterial {
            name: "   ".to_string(), // Empty name
            ..valid.clone()
        };
        assert!(invalid_name.validate().is_err());

        let invalid_quantity = CreateMaterial {
            quantity: -1, // Negative quantity
            ..valid
        };
        assert!(invalid_quantity.validate().is_err());
    }
}
```

**Rationale:**
- Inline tests (same file) for domain logic - keeps tests close to implementation
- No mocking needed - domain has zero dependencies
- Focus on business rules and invariants
- Fast execution (no database, no network)

**Run with:**
```bash
cargo test --lib inventory::domain
```

### 2. Application Layer Tests (Unit Tests with Mocks)

**Pattern: Extract Repository Trait (Port)**

Currently, `InventoryService` depends on concrete `MaterialRepository`. To enable testing, extract a trait:

```rust
// src/modules/inventory/application/ports.rs (NEW)
use async_trait::async_trait;
use crate::common::error::AppError;
use crate::common::types::*;
use crate::modules::inventory::domain::*;

#[async_trait]
pub trait MaterialRepositoryPort: Send + Sync {
    async fn create_category(
        &self,
        create: &CreateCategory,
        tenant_id: TenantId,
    ) -> Result<Category, AppError>;

    async fn list_categories(
        &self,
        tenant_id: TenantId,
    ) -> Result<Vec<Category>, AppError>;

    async fn find_category_by_id(
        &self,
        id: CategoryId,
        tenant_id: TenantId,
    ) -> Result<Option<Category>, AppError>;

    async fn create_material(
        &self,
        create: &CreateMaterial,
        tenant_id: TenantId,
    ) -> Result<Material, AppError>;

    async fn find_material_by_id(
        &self,
        id: MaterialId,
        tenant_id: TenantId,
    ) -> Result<Option<Material>, AppError>;

    async fn withdraw_stock(
        &self,
        material_id: MaterialId,
        quantity: i32,
        user_id: UserId,
        notes: Option<String>,
        tenant_id: TenantId,
    ) -> Result<Material, AppError>;

    async fn publish_event(&self, event: &DomainEvent) -> Result<(), AppError>;

    // ... other methods
}
```

**Implement trait for concrete repository:**
```rust
// src/modules/inventory/infrastructure/material_repository.rs
#[async_trait]
impl MaterialRepositoryPort for MaterialRepository {
    async fn create_category(
        &self,
        create: &CreateCategory,
        tenant_id: TenantId,
    ) -> Result<Category, AppError> {
        self.create_category(create, tenant_id).await
    }

    // ... delegate all methods to self
}
```

**Update service to use trait:**
```rust
// src/modules/inventory/application/inventory_service.rs
use std::sync::Arc;
use crate::modules::inventory::application::ports::MaterialRepositoryPort;

pub struct InventoryService {
    material_repo: Arc<dyn MaterialRepositoryPort>,
    pool: PgPool,
}

impl InventoryService {
    pub fn new(material_repo: Arc<dyn MaterialRepositoryPort>, pool: PgPool) -> Self {
        Self { material_repo, pool }
    }

    // Methods now use material_repo trait object
}
```

**Generate mock with Mockall:**
```rust
// src/modules/inventory/application/inventory_service.rs
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait MaterialRepositoryPort: Send + Sync {
    // ... trait definition
}
```

**Write application layer tests:**
```rust
// src/modules/inventory/application/inventory_service.rs.tests
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use std::sync::Arc;

    fn create_test_context() -> TenantContext {
        TenantContext {
            tenant_id: TenantId(Uuid::new_v4()),
            user_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            roles: vec!["admin".to_string()],
        }
    }

    fn create_test_category() -> Category {
        Category {
            id: CategoryId(Uuid::new_v4()),
            tenant_id: TenantId(Uuid::new_v4()),
            name: "Test Category".to_string(),
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_material() -> Material {
        Material {
            id: MaterialId(Uuid::new_v4()),
            tenant_id: TenantId(Uuid::new_v4()),
            category_id: CategoryId(Uuid::new_v4()),
            name: "Test Material".to_string(),
            description: None,
            unit: Unit::Piece,
            quantity: 10,
            min_quantity: 5,
            location: None,
            qr_code: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_create_material_as_admin() {
        // Arrange
        let mut mock_repo = MockMaterialRepositoryPort::new();
        let category = create_test_category();
        let category_id = category.id;
        
        mock_repo
            .expect_find_category_by_id()
            .with(eq(category_id), eq(create_test_context().tenant_id))
            .times(1)
            .returning(|_, _| Ok(Some(create_test_category())));

        let expected_material = create_test_material();
        let expected_clone = expected_material.clone();
        
        mock_repo
            .expect_create_material()
            .times(1)
            .returning(move |_, _| Ok(expected_clone.clone()));

        mock_repo
            .expect_publish_event()
            .times(1)
            .returning(|_| Ok(()));

        let pool = PgPool::connect_lazy("postgres://localhost/test").unwrap();
        let service = InventoryService::new(Arc::new(mock_repo), pool);
        let ctx = create_test_context();

        // Act
        let result = service.create_material(
            CreateMaterial {
                category_id,
                name: "Test Material".to_string(),
                description: None,
                unit: Unit::Piece,
                quantity: 10,
                min_quantity: 5,
                location: None,
            },
            &ctx,
        ).await;

        // Assert
        assert!(result.is_ok());
        let material = result.unwrap();
        assert_eq!(material.name, "Test Material");
    }

    #[tokio::test]
    async fn test_create_material_as_employee_forbidden() {
        // Arrange
        let mock_repo = MockMaterialRepositoryPort::new();
        let pool = PgPool::connect_lazy("postgres://localhost/test").unwrap();
        let service = InventoryService::new(Arc::new(mock_repo), pool);
        
        let mut ctx = create_test_context();
        ctx.roles = vec!["employee".to_string()]; // Not admin

        // Act
        let result = service.create_material(
            CreateMaterial {
                category_id: CategoryId(Uuid::new_v4()),
                name: "Test Material".to_string(),
                description: None,
                unit: Unit::Piece,
                quantity: 10,
                min_quantity: 5,
                location: None,
            },
            &ctx,
        ).await;

        // Assert
        assert!(matches!(result, Err(AppError::Forbidden(_))));
    }

    #[tokio::test]
    async fn test_withdraw_material_emits_low_stock_event() {
        // Arrange
        let mut mock_repo = MockMaterialRepositoryPort::new();
        
        let material = create_test_material();
        let material_low_stock = Material {
            quantity: 3, // Below min_quantity of 5
            ..material.clone()
        };

        mock_repo
            .expect_withdraw_stock()
            .times(1)
            .returning(move |_, _, _, _, _| Ok(material_low_stock.clone()));

        // Expect both StockWithdrawn and StockLow events
        mock_repo
            .expect_publish_event()
            .times(2) // Two events
            .returning(|_| Ok(()));

        let pool = PgPool::connect_lazy("postgres://localhost/test").unwrap();
        let service = InventoryService::new(Arc::new(mock_repo), pool);
        let ctx = create_test_context();

        // Act
        let result = service.withdraw_material(
            WithdrawMaterial {
                material_id: material.id,
                quantity: 7,
                notes: None,
            },
            &ctx,
        ).await;

        // Assert
        assert!(result.is_ok());
    }
}
```

**Rationale:**
- Mock infrastructure to test application logic in isolation
- Test authorization rules (admin vs employee)
- Test business workflows (event emission)
- Fast tests - no database required
- Use Mockall's `automock` for zero boilerplate

**Run with:**
```bash
cargo test --lib inventory::application
```

### 3. Infrastructure Layer Tests (Integration Tests)

**Pattern: Use SQLx test macro with migrations**

```rust
// tests/inventory_integration_test.rs
use sqlx::PgPool;
use uuid::Uuid;
use schreinerei::modules::inventory::infrastructure::MaterialRepository;
use schreinerei::modules::inventory::domain::*;
use schreinerei::common::types::*;

/// Test helper: Create a test tenant
async fn create_test_tenant(pool: &PgPool) -> TenantId {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO tenants (id, keycloak_realm, name, slug) VALUES ($1, $2, $3, $4)"
    )
    .bind(id)
    .bind(format!("realm-{}", id))
    .bind("Test Tenant")
    .bind(format!("tenant-{}", id))
    .execute(pool)
    .await
    .expect("Failed to create test tenant");
    
    TenantId(id)
}

/// Test helper: Create a test category
async fn create_test_category(pool: &PgPool, tenant_id: TenantId) -> Category {
    let repo = MaterialRepository::new(pool.clone());
    repo.create_category(
        &CreateCategory {
            name: "Test Category".to_string(),
            description: None,
        },
        tenant_id,
    )
    .await
    .expect("Failed to create test category")
}

#[sqlx::test(migrations = "migrations")]
async fn test_create_and_find_material(pool: PgPool) {
    // Arrange
    let tenant_id = create_test_tenant(&pool).await;
    let category = create_test_category(&pool, tenant_id).await;
    let repo = MaterialRepository::new(pool.clone());

    // Act: Create material
    let create = CreateMaterial {
        category_id: category.id,
        name: "Test Material".to_string(),
        description: Some("A test material".to_string()),
        unit: Unit::Piece,
        quantity: 10,
        min_quantity: 5,
        location: Some("Shelf A".to_string()),
    };

    let created = repo.create_material(&create, tenant_id)
        .await
        .expect("Failed to create material");

    // Assert: Material was created correctly
    assert_eq!(created.name, "Test Material");
    assert_eq!(created.quantity, 10);

    // Act: Find the material
    let found = repo.find_material_by_id(created.id, tenant_id)
        .await
        .expect("Failed to query material");

    // Assert: Found matches created
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, created.id);
    assert_eq!(found.name, created.name);
}

#[sqlx::test(migrations = "migrations")]
async fn test_withdraw_stock_updates_quantity(pool: PgPool) {
    // Arrange
    let tenant_id = create_test_tenant(&pool).await;
    let category = create_test_category(&pool, tenant_id).await;
    let repo = MaterialRepository::new(pool.clone());

    let material = repo.create_material(
        &CreateMaterial {
            category_id: category.id,
            name: "Test Material".to_string(),
            description: None,
            unit: Unit::Piece,
            quantity: 10,
            min_quantity: 5,
            location: None,
        },
        tenant_id,
    )
    .await
    .unwrap();

    // Create test user
    let user_id = UserId(Uuid::new_v4());
    sqlx::query(
        "INSERT INTO users (id, tenant_id, keycloak_user_id, email, role) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(user_id.0)
    .bind(tenant_id.0)
    .bind(format!("kc-{}", user_id.0))
    .bind("test@example.com")
    .bind("employee")
    .execute(&pool)
    .await
    .unwrap();

    // Act: Withdraw 3 units
    let updated = repo.withdraw_stock(
        material.id,
        3,
        user_id,
        Some("Test withdrawal".to_string()),
        tenant_id,
    )
    .await
    .expect("Failed to withdraw stock");

    // Assert: Quantity updated
    assert_eq!(updated.quantity, 7);

    // Assert: Audit log created
    let audit_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM stock_entries WHERE material_id = $1"
    )
    .bind(material.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    
    assert_eq!(audit_count, 1);
}

#[sqlx::test(migrations = "migrations")]
async fn test_tenant_isolation_in_materials(pool: PgPool) {
    // Arrange: Create two tenants
    let tenant_a = create_test_tenant(&pool).await;
    let tenant_b = create_test_tenant(&pool).await;

    let category_a = create_test_category(&pool, tenant_a).await;
    let category_b = create_test_category(&pool, tenant_b).await;

    let repo = MaterialRepository::new(pool.clone());

    // Create material in tenant A
    let material_a = repo.create_material(
        &CreateMaterial {
            category_id: category_a.id,
            name: "Tenant A Material".to_string(),
            description: None,
            unit: Unit::Piece,
            quantity: 10,
            min_quantity: 5,
            location: None,
        },
        tenant_a,
    )
    .await
    .unwrap();

    // Act: Try to find tenant A's material with tenant B's context
    let found = repo.find_material_by_id(material_a.id, tenant_b)
        .await
        .expect("Query failed");

    // Assert: Material not visible across tenants
    assert!(found.is_none(), "Material should not be visible across tenants");

    // Act: List materials for tenant B
    let materials_b = repo.list_materials(tenant_b, None)
        .await
        .expect("Query failed");

    // Assert: Tenant B sees no materials from tenant A
    assert_eq!(materials_b.len(), 0);
}

#[sqlx::test(migrations = "migrations")]
async fn test_concurrent_withdrawals(pool: PgPool) {
    // Arrange
    let tenant_id = create_test_tenant(&pool).await;
    let category = create_test_category(&pool, tenant_id).await;
    let repo = MaterialRepository::new(pool.clone());

    let material = repo.create_material(
        &CreateMaterial {
            category_id: category.id,
            name: "Test Material".to_string(),
            description: None,
            unit: Unit::Piece,
            quantity: 10,
            min_quantity: 5,
            location: None,
        },
        tenant_id,
    )
    .await
    .unwrap();

    // Create test user
    let user_id = UserId(Uuid::new_v4());
    sqlx::query(
        "INSERT INTO users (id, tenant_id, keycloak_user_id, email, role) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(user_id.0)
    .bind(tenant_id.0)
    .bind(format!("kc-{}", user_id.0))
    .bind("test@example.com")
    .bind("employee")
    .execute(&pool)
    .await
    .unwrap();

    // Act: Attempt to withdraw more than available (should fail)
    let result = repo.withdraw_stock(
        material.id,
        15, // More than 10 available
        user_id,
        None,
        tenant_id,
    )
    .await;

    // Assert: Withdrawal failed
    assert!(result.is_err(), "Should not allow withdrawing more than available");
    
    // Verify quantity unchanged
    let material_after = repo.find_material_by_id(material.id, tenant_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(material_after.quantity, 10, "Quantity should remain unchanged after failed withdrawal");
}
```

**Rationale:**
- Use `#[sqlx::test]` macro for automatic test database management
- Tests run in parallel with isolated databases
- Migrations applied automatically before each test
- Test real database interactions, not mocks
- Focus on infrastructure concerns: transactions, constraints, isolation

**Run with:**
```bash
cargo test --test inventory_integration_test
```

**Database setup:**
```bash
# Ensure DATABASE_URL is set
export DATABASE_URL="postgres://user:pass@localhost/schreinerei_test"

# SQLx will create separate test databases:
# - schreinerei_test-1
# - schreinerei_test-2
# - etc. (one per parallel test)
```

### 4. Frontend Tests (React + Vitest + MSW)

**Setup Vitest configuration:**
```typescript
// frontend/vitest.config.ts
import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
});
```

**Setup MSW (Mock Service Worker):**
```typescript
// frontend/src/test/msw/handlers.ts
import { http, HttpResponse } from 'msw';
import { v4 as uuid } from 'uuid';

const materials = new Map();
const categories = new Map();

export const handlers = [
  // Categories
  http.get('/api/categories', () => {
    return HttpResponse.json(Array.from(categories.values()));
  }),

  http.post('/api/categories', async ({ request }) => {
    const body = await request.json();
    const id = uuid();
    const category = { id, ...body };
    categories.set(id, category);
    return HttpResponse.json(category, { status: 201 });
  }),

  // Materials
  http.get('/api/materials', ({ request }) => {
    const url = new URL(request.url);
    const categoryId = url.searchParams.get('category_id');
    
    let filtered = Array.from(materials.values());
    if (categoryId) {
      filtered = filtered.filter(m => m.category_id === categoryId);
    }
    
    return HttpResponse.json(filtered);
  }),

  http.get('/api/materials/:id', ({ params }) => {
    const material = materials.get(params.id);
    if (!material) {
      return new HttpResponse(null, { status: 404 });
    }
    return HttpResponse.json(material);
  }),

  http.post('/api/materials', async ({ request }) => {
    const body = await request.json();
    const id = uuid();
    const material = {
      id,
      ...body,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
    materials.set(id, material);
    return HttpResponse.json(material, { status: 201 });
  }),

  // Stock operations
  http.post('/api/materials/:id/withdraw', async ({ params, request }) => {
    const material = materials.get(params.id);
    if (!material) {
      return new HttpResponse(null, { status: 404 });
    }

    const body = await request.json();
    material.quantity -= body.quantity;
    material.updated_at = new Date().toISOString();
    materials.set(params.id, material);

    return HttpResponse.json(material);
  }),
];
```

```typescript
// frontend/src/test/msw/server.ts
import { setupServer } from 'msw/node';
import { handlers } from './handlers';

export const server = setupServer(...handlers);
```

```typescript
// frontend/src/test/setup.ts
import '@testing-library/jest-dom/vitest';
import { server } from './msw/server';
import { afterAll, afterEach, beforeAll } from 'vitest';

beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());
```

**Component test example:**
```typescript
// frontend/src/components/__tests__/MaterialList.test.tsx
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, it, expect, vi } from 'vitest';
import { MaterialList } from '../MaterialList';
import { server } from '@/test/msw/server';
import { http, HttpResponse } from 'msw';

describe('MaterialList', () => {
  it('displays loading state initially', () => {
    render(<MaterialList />);
    expect(screen.getByText(/loading/i)).toBeInTheDocument();
  });

  it('displays materials after loading', async () => {
    render(<MaterialList />);

    await waitFor(() => {
      expect(screen.queryByText(/loading/i)).not.toBeInTheDocument();
    });

    // Verify materials are displayed
    expect(screen.getByText('Test Material')).toBeInTheDocument();
  });

  it('displays error message on fetch failure', async () => {
    server.use(
      http.get('/api/materials', () => {
        return new HttpResponse(null, { status: 500 });
      })
    );

    render(<MaterialList />);

    await waitFor(() => {
      expect(screen.getByText(/error/i)).toBeInTheDocument();
    });
  });

  it('filters materials by category', async () => {
    const user = userEvent.setup();
    render(<MaterialList />);

    await waitFor(() => {
      expect(screen.getByText('Test Material')).toBeInTheDocument();
    });

    // Select category filter
    const categorySelect = screen.getByLabelText(/category/i);
    await user.selectOptions(categorySelect, 'category-1');

    // Verify filtered results
    await waitFor(() => {
      const displayedMaterials = screen.getAllByTestId('material-item');
      expect(displayedMaterials.length).toBeGreaterThan(0);
    });
  });

  it('calls onWithdraw when withdraw button clicked', async () => {
    const onWithdraw = vi.fn();
    const user = userEvent.setup();
    
    render(<MaterialList onWithdraw={onWithdraw} />);

    await waitFor(() => {
      expect(screen.getByText('Test Material')).toBeInTheDocument();
    });

    const withdrawButton = screen.getByRole('button', { name: /withdraw/i });
    await user.click(withdrawButton);

    expect(onWithdraw).toHaveBeenCalled();
  });
});
```

**Hook test example:**
```typescript
// frontend/src/hooks/__tests__/useMaterials.test.ts
import { renderHook, waitFor } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { useMaterials } from '../useMaterials';
import { server } from '@/test/msw/server';
import { http, HttpResponse } from 'msw';

describe('useMaterials', () => {
  it('fetches materials successfully', async () => {
    const { result } = renderHook(() => useMaterials());

    expect(result.current.loading).toBe(true);
    expect(result.current.materials).toEqual([]);

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.materials.length).toBeGreaterThan(0);
    expect(result.current.error).toBeNull();
  });

  it('handles fetch errors', async () => {
    server.use(
      http.get('/api/materials', () => {
        return new HttpResponse(null, { status: 500 });
      })
    );

    const { result } = renderHook(() => useMaterials());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.error).not.toBeNull();
    expect(result.current.materials).toEqual([]);
  });

  it('filters by category', async () => {
    const { result } = renderHook(() => useMaterials({ categoryId: 'cat-1' }));

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    result.current.materials.forEach(material => {
      expect(material.category_id).toBe('cat-1');
    });
  });
});
```

**Rationale:**
- MSW intercepts HTTP requests at the network level - no need to mock fetch/axios
- Tests real component behavior, not implementation details
- User-centric queries (getByRole, getByLabelText) match how users interact
- Async handling with waitFor for loading states
- Test both happy path and error scenarios

**Run with:**
```bash
cd frontend
npm test                    # Run all tests
npm test -- --watch        # Watch mode
npm run test:coverage      # With coverage
```

## Testing Strategy Matrix

| Layer | Test Type | Speed | Dependencies | Focus |
|-------|-----------|-------|--------------|-------|
| **Domain** | Unit (inline) | Very Fast (ms) | None | Business logic, validation |
| **Application** | Unit (mocked) | Fast (ms) | Mocked infrastructure | Orchestration, authorization |
| **Infrastructure** | Integration | Slow (seconds) | Real database | SQL queries, transactions |
| **API Routes** | Integration | Slow (seconds) | Real database + app | HTTP handling, serialization |
| **Frontend Components** | Unit (jsdom) | Fast (ms) | Mocked API | UI rendering, interactions |
| **Frontend Hooks** | Unit (jsdom) | Fast (ms) | Mocked API | State management, data fetching |
| **E2E** | End-to-end | Very Slow (minutes) | Full stack | User workflows |

## Test Execution Commands

```bash
# Backend
cargo test                              # All tests
cargo test --lib                        # Unit tests only
cargo test --test tenant_isolation      # Specific integration test
cargo test inventory::domain           # Domain tests for inventory module
cargo test -- --nocapture              # Show println! output

# Frontend
npm test                               # All tests
npm test -- MaterialList              # Run specific test file
npm run test:coverage                  # With coverage report

# E2E (existing)
npx playwright test                    # All E2E tests
npx playwright test --ui              # Interactive mode
```

## Test Data Management

### Backend Test Data

**Option 1: SQLx test fixtures (Recommended)**
```rust
// tests/fixtures/mod.rs
pub async fn create_test_tenant(pool: &PgPool, name: &str) -> TenantId {
    let id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO tenants (id, keycloak_realm, name, slug) VALUES ($1, $2, $3, $4)",
        id,
        format!("realm-{}", name.to_lowercase()),
        name,
        name.to_lowercase().replace(' ', "-")
    )
    .execute(pool)
    .await
    .expect("Failed to create tenant");
    
    TenantId(id)
}

pub async fn create_test_material(
    pool: &PgPool,
    tenant_id: TenantId,
    category_id: CategoryId,
    name: &str,
) -> Material {
    let repo = MaterialRepository::new(pool.clone());
    repo.create_material(
        &CreateMaterial {
            category_id,
            name: name.to_string(),
            description: None,
            unit: Unit::Piece,
            quantity: 10,
            min_quantity: 5,
            location: None,
        },
        tenant_id,
    )
    .await
    .expect("Failed to create material")
}
```

**Option 2: Factory pattern**
```rust
// tests/factories/material_factory.rs
pub struct MaterialFactory {
    pool: PgPool,
}

impl MaterialFactory {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, tenant_id: TenantId) -> MaterialBuilder {
        MaterialBuilder::new(self.pool.clone(), tenant_id)
    }
}

pub struct MaterialBuilder {
    pool: PgPool,
    tenant_id: TenantId,
    category_id: Option<CategoryId>,
    name: String,
    quantity: i32,
    min_quantity: i32,
}

impl MaterialBuilder {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_quantity(mut self, quantity: i32) -> Self {
        self.quantity = quantity;
        self
    }

    pub async fn build(self) -> Material {
        // Create material with configured values
    }
}
```

### Frontend Test Data

**MSW with test fixtures:**
```typescript
// frontend/src/test/fixtures/materials.ts
export const testMaterials = [
  {
    id: 'mat-1',
    name: 'Pine Wood',
    category_id: 'cat-1',
    quantity: 50,
    min_quantity: 10,
    unit: 'piece',
  },
  {
    id: 'mat-2',
    name: 'Screws M6',
    category_id: 'cat-2',
    quantity: 200,
    min_quantity: 50,
    unit: 'piece',
  },
];

export const testCategories = [
  { id: 'cat-1', name: 'Wood' },
  { id: 'cat-2', name: 'Hardware' },
];
```

```typescript
// frontend/src/test/msw/handlers.ts
import { testMaterials, testCategories } from '../fixtures/materials';

export const handlers = [
  http.get('/api/materials', () => {
    return HttpResponse.json(testMaterials);
  }),
  
  http.get('/api/categories', () => {
    return HttpResponse.json(testCategories);
  }),
];
```

## Coverage Goals

| Layer | Target Coverage | Rationale |
|-------|----------------|-----------|
| **Domain** | 90%+ | Critical business logic, easy to test |
| **Application** | 80%+ | Authorization + orchestration |
| **Infrastructure** | 70%+ | Database interactions, hard to test edge cases |
| **API Routes** | 60%+ | Focus on authentication/authorization |
| **Frontend** | 70%+ | Key components and hooks |

**Measure coverage:**
```bash
# Backend
cargo tarpaulin --out Html --output-dir coverage

# Frontend
npm run test:coverage
```

## CI/CD Integration

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  backend-tests:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_DB: schreinerei_test
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run backend tests
        run: cargo test --all
        env:
          DATABASE_URL: postgres://postgres:postgres@localhost/schreinerei_test
      
      - name: Generate coverage
        run: cargo tarpaulin --out Xml
        
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: cobertura.xml
          flags: backend

  frontend-tests:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: '20'
          cache: 'npm'
          cache-dependency-path: frontend/package-lock.json
      
      - name: Install dependencies
        working-directory: frontend
        run: npm ci
      
      - name: Run tests
        working-directory: frontend
        run: npm run test:coverage
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: frontend/coverage/coverage-final.json
          flags: frontend

  e2e-tests:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_DB: schreinerei_e2e
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
        ports:
          - 5432:5432
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Run E2E tests
        run: npx playwright test
        env:
          DATABASE_URL: postgres://postgres:postgres@localhost/schreinerei_e2e
```

## Anti-Patterns to Avoid

### 1. Testing Private Methods Directly

**Bad:**
```rust
#[test]
fn test_private_helper() {
    let service = InventoryService::new(/* ... */);
    let result = service.some_private_method(); // Won't compile
}
```

**Good:**
```rust
#[test]
fn test_public_api_that_uses_private_helper() {
    let service = InventoryService::new(/* ... */);
    let result = service.create_material(/* ... */);
    assert!(result.is_ok());
}
```

### 2. Over-Mocking in Domain Tests

**Bad:**
```rust
#[test]
fn test_domain_entity_with_mocks() {
    let mut mock_validator = MockValidator::new();
    // Domain should not need mocks!
}
```

**Good:**
```rust
#[test]
fn test_domain_entity() {
    let material = Material { /* ... */ };
    assert!(material.is_low_stock());
    // Pure business logic, no mocks needed
}
```

### 3. Database State Leaking Between Tests

**Bad:**
```rust
#[test]
async fn test_1() {
    // Inserts data but doesn't clean up
}

#[test]
async fn test_2() {
    // Fails because of data from test_1
}
```

**Good:**
```rust
#[sqlx::test]  // Creates isolated database for each test
async fn test_1(pool: PgPool) {
    // Test runs in isolated database
}

#[sqlx::test]  // Separate isolated database
async fn test_2(pool: PgPool) {
    // Test runs in isolated database
}
```

### 4. Testing Implementation Details in Frontend

**Bad:**
```typescript
it('calls setState correctly', () => {
  const { result } = renderHook(() => useMaterials());
  expect(result.current.internalState).toBe('loading');
  // Testing implementation detail
});
```

**Good:**
```typescript
it('displays loading state initially', () => {
  render(<MaterialList />);
  expect(screen.getByText('Loading...')).toBeInTheDocument();
  // Testing user-visible behavior
});
```

### 5. Integration Tests for Pure Logic

**Bad:**
```rust
#[sqlx::test]
async fn test_validation_logic(pool: PgPool) {
    // Wasteful - validation has no database dependencies
    let create = CreateMaterial { /* ... */ };
    create.validate().unwrap();
}
```

**Good:**
```rust
#[test]  // Fast unit test
fn test_validation_logic() {
    let create = CreateMaterial { /* ... */ };
    create.validate().unwrap();
}
```

## Summary

### Test Organization

1. **Domain tests**: Inline in domain files, test pure business logic
2. **Application tests**: Mock infrastructure via trait ports, test orchestration
3. **Infrastructure tests**: Integration tests with real database, test SQL and transactions
4. **Frontend tests**: Vitest + React Testing Library + MSW, test user-visible behavior

### Key Patterns

- Extract repository traits for testability
- Use Mockall's `automock` for zero-boilerplate mocking
- Use `#[sqlx::test]` for automatic test database management
- Use MSW for API mocking in frontend tests
- Test behavior, not implementation details

### Execution Strategy

- Run domain and application unit tests on every commit (fast)
- Run integration tests on pull requests (slower)
- Run E2E tests before deployment (slowest)
- Measure coverage, but focus on critical paths

## Sources

- Mockall documentation: https://docs.rs/mockall
- SQLx testing: https://docs.rs/sqlx/latest/sqlx/attr.test.html
- React Testing Library: https://testing-library.com/docs/react-testing-library/intro
- Vitest: https://vitest.dev/guide/
- MSW: https://mswjs.io/docs/
- Context7 Rust documentation: /rust-lang/rust
- Context7 Vitest documentation: /vitest-dev/vitest
- Context7 Mockall documentation: /asomers/mockall

---
*Architecture research for: Testing & Quality Foundation milestone*
*Researched: 2026-04-30*
