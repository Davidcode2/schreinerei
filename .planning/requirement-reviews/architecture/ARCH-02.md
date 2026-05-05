# ARCH-02

Status: Partial
Fit: Very strong
Priority: Now
Decision: Keep

Current state: Tenant enforcement is strong at the API edge, but `tenant_id` is still threaded manually through service/repository calls.
Evidence: `src/auth/middleware.rs`, `src/modules/*/infrastructure/*repository.rs`

Implementation:
1. Introduce a shared request-scoped `TenantScope` abstraction.
2. Refactor repo APIs to depend on scoped context instead of raw `tenant_id` parameters.
3. Centralize tenant filtering in infrastructure.
4. Add real multi-tenant isolation tests.
