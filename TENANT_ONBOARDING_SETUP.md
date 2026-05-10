# Tenant Onboarding Setup Plan

Bead: `sc-zhz7o`

## Goal And Scope

Build self-service customer onboarding without weakening the existing tenant isolation model.

The target flow is:

1. New customer starts from the product UI.
2. Customer enters organization/admin setup data.
3. Payment checkout is created and completed.
4. Backend creates the local tenant and Keycloak organization.
5. The signing user becomes organization admin.
6. A separate onboarding demo seed pre-fills the dashboard.
7. Admin can generate expiring organization invites from settings.
8. Invited users land on sign-up and join that organization.

This document is the implementation plan and setup reference. The first low-risk code slice adds a public `/signup` entry point and a separate demo seed file at `scripts/onboarding-demo-seed.sql`; it does not create live tenants, charge payments, or mutate Keycloak yet.

## Current Implementation Findings

- Auth is Keycloak OAuth2 PKCE in `frontend/src/lib/auth/keycloak.ts`; the frontend requests the `organization` scope and extracts the first organization claim into `user.tenant_id`.
- Backend JWT validation is in `src/auth/middleware.rs`; it resolves `claims.organization[0]` through `tenants.keycloak_organization_alias`.
- Tenant columns already exist: `tenants.keycloak_organization_id` and `tenants.keycloak_organization_alias`.
- IAM user sync is local and tenant-scoped in `src/modules/iam/application/user_service.rs`.
- `POST /api/v1/users/invite` currently creates a placeholder local user only. It does not create an expiring invite or call Keycloak.
- Settings already has a user-management surface in `frontend/src/pages/settings/UserManagementSection.tsx`, but the invite URL is a static Keycloak org URL derived from the current token claim.
- Normal dev/test data lives in `scripts/realistic-test-data.sql`; onboarding prefill needs a separate seed dataset.

## Requirements

- `SS-01`: public website/product UI with organization registration.
- `SS-02`: self-service organization creation flow.
- `SS-03`: organization admin dashboard.
- `SS-04`: member invitation via email.
- Cross-cutting: `ARCH-02` tenant-scoped request context on every query/mutation.

## Provider Recommendation

Use Mollie for the first paid onboarding slice.

Reasoning:

- Mollie has a simple hosted checkout flow: backend creates a payment, user is redirected to hosted checkout, and Mollie calls a webhook after payment status changes.
- Mollie supports subscriptions, but subscription lifecycle requires careful payment-webhook handling because subscription status changes are not reported through a dedicated subscription webhook.
- Adyen is stronger for complex/global enterprise payment orchestration, but its Checkout and notification setup is heavier for the first small German SaaS onboarding path.

References:

- Mollie payment flow: https://docs.mollie.com/docs/accepting-payments
- Mollie recurring payment webhook behavior: https://docs.mollie.com/docs/recurring-payments
- Adyen Checkout/webhook capabilities: https://docs.adyen.com/api-explorer

## Proposed Backend Design

Add a new bounded onboarding surface under `src/modules/onboarding/` using modern Rust module layout:

- `src/modules/onboarding.rs`
- `src/modules/onboarding/domain.rs`
- `src/modules/onboarding/application.rs`
- `src/modules/onboarding/infrastructure.rs`
- `src/modules/onboarding/api.rs`
- `src/modules/onboarding/api/routes.rs`
- `src/modules/onboarding/infrastructure/onboarding_repository.rs`
- `src/modules/onboarding/infrastructure/keycloak_admin_client.rs`
- `src/modules/onboarding/infrastructure/payment_provider.rs`

Initial API:

- `POST /api/v1/onboarding/sessions`
  - Public endpoint.
  - Input: organization name, admin email, admin name, selected plan.
  - Creates `onboarding_sessions` with status `pending_payment`.
  - Calls Mollie to create checkout.
  - Returns hosted checkout URL.
- `POST /api/v1/onboarding/webhooks/mollie`
  - Public, signature/IP/secret validated according to provider support.
  - Idempotently records event, fetches payment from Mollie by ID, and advances state only from provider-confirmed status.
- `GET /api/v1/onboarding/sessions/{id}`
  - Public with opaque session token, or authenticated once the user logs in.
  - Returns setup status for the frontend completion screen.
- `POST /api/v1/users/invites`
  - Authenticated admin endpoint.
  - Calls Keycloak organization invite API and returns invitation metadata/link if Keycloak exposes it for the configured flow.

## Data Model

Add migrations:

- `onboarding_sessions`
  - `id`, `organization_name`, `organization_slug`, `admin_email`, `admin_name`
  - `selected_plan`, `status`
  - `payment_provider`, `payment_id`, `checkout_url`
  - `tenant_id`, `keycloak_organization_id`, `keycloak_organization_alias`
  - `error_code`, `error_message`
  - `created_at`, `updated_at`, `completed_at`
- `payment_events`
  - `id`, `provider`, `provider_event_id/payment_id`, `raw_payload`, `processed_at`
  - unique provider event/payment key for idempotency
- `organization_invites`
  - `id`, `tenant_id`, `email`, `role`, `keycloak_invitation_id`, `expires_at`, `status`
  - no user-controlled expiry field
  - unique active invite per `tenant_id/email`

All post-tenant queries and mutations must include `tenant_id`. Public onboarding tables are not tenant-scoped until a tenant exists, so they must use opaque IDs/tokens and never expose cross-session lookup by email or slug.

## Keycloak Integration

Use a backend service account with minimum admin permissions for organization creation and invitations.

Keycloak APIs to use:

- Create organization: `POST /admin/realms/{realm}/organizations`.
- Add existing member: `POST /admin/realms/{realm}/organizations/{org-id}/members`.
- Invite by email: `POST /admin/realms/{realm}/organizations/{org-id}/members/invite-user`.
- List/manage invitations: organization invitation endpoints.

Reference: https://www.keycloak.org/docs-api/latest/rest-api/index.html

Required local config:

- `KEYCLOAK_ADMIN_CLIENT_ID`
- `KEYCLOAK_ADMIN_CLIENT_SECRET`
- `KEYCLOAK_ADMIN_REALM` if service account lives outside the app realm
- `KEYCLOAK_ORGANIZATION_INVITE_TTL_SECONDS`

Failure handling:

- If payment is not confirmed, do not create tenant or Keycloak organization.
- If local tenant insert succeeds but Keycloak organization fails, mark session `keycloak_failed` and allow retry by session ID.
- If Keycloak organization succeeds but local update fails, retry local update by Keycloak org ID/alias.
- Do not delete a paid Keycloak organization automatically unless a compensating admin operation is explicitly implemented and audited.
- Make every step idempotent by onboarding session ID and provider payment ID.

## Demo Seed Prefill

Use `scripts/onboarding-demo-seed.sql` after local tenant and admin user creation:

```bash
psql "$DATABASE_URL" \
  -v ON_ERROR_STOP=1 \
  -v tenant_id="<tenant uuid>" \
  -v admin_user_id="<local admin user uuid>" \
  -f scripts/onboarding-demo-seed.sql
```

The seed inserts a small set of categories, materials, projects, assignments, and assets using deterministic tenant-derived UUIDs. It is separate from `scripts/realistic-test-data.sql` and should only run for newly onboarded or explicitly reset demo tenants.

## Frontend Plan

Affected files:

- `frontend/src/components/auth/LoginPage.tsx`
- `frontend/src/components/auth/SignupPage.tsx`
- `frontend/src/App.tsx`
- future `frontend/src/lib/api/hooks/useOnboarding.ts`
- future `frontend/src/pages/settings/UserManagementSection.tsx`
- future `frontend/src/components/settings/InviteUserDialog.tsx`

Implementation approach:

- Keep `/signup` public.
- Replace the scaffold with a form that posts to `/api/v1/onboarding/sessions`.
- Redirect to Mollie hosted checkout.
- Add `/onboarding/complete?session=...` to poll session status, then start Keycloak login/registration.
- Update settings invites to call the backend invite endpoint, not construct static Keycloak URLs in the browser.
- For invite links, route `/signup?invite=...` and let the backend resolve invite status before sending the user to Keycloak.

## Test Plan

Backend:

- Unit-test slug generation, onboarding state transitions, and idempotency.
- Repository tests for unique slug/payment event constraints.
- Mock Mollie client tests for payment creation and webhook status fetch.
- Mock Keycloak admin client tests for create organization, add admin, invite user, and retry behavior.
- Tenant isolation regression tests for all invite reads/writes.

Frontend:

- Vitest for login signup link and signup form states.
- MSW tests for onboarding session creation success/failure.
- Settings invite dialog tests for generated invite, expiry display, copy link, and error states.
- E2E happy path with mocked payment webhook and Keycloak admin client.

Manual:

- Payment sandbox checkout.
- Webhook replay/idempotency.
- Keycloak organization claim appears in token.
- New tenant dashboard shows demo data only for the new tenant.

## Follow-Up Slices

1. Backend onboarding session + Mollie checkout abstraction.
2. Mollie webhook processing + idempotent state machine.
3. Keycloak admin client + tenant creation + admin assignment.
4. Onboarding demo seed execution from service layer.
5. Generated expiring org invites in settings.
6. Invite landing flow on `/signup?invite=...`.
7. End-to-end onboarding verification with sandbox providers.

## Open Questions And Risks

- Confirm whether the deployed Keycloak version has organizations and invitation APIs enabled in the current realm.
- Confirm whether user registration should happen through Keycloak self-registration, Keycloak invitation email, or an app-owned invite landing page followed by Keycloak.
- Decide whether payment is required before account creation, or whether trial tenants can be created before paid conversion.
- Mollie subscription failure behavior needs an explicit dunning/suspension policy.
- Keycloak organization alias must be unique, stable, and stored in `tenants.keycloak_organization_alias`; changing aliases would break tenant resolution for existing JWTs.
- Existing frontend `user.tenant_id` is actually the organization alias, not the internal UUID. Avoid using it as a database tenant ID in future UI logic.
