# Project Research Summary

**Project:** Schreinerei — Construction Site Management SaaS
**Domain:** Field Service / Construction — Active Project Context Feature (v1.7)
**Researched:** 2026-04-30
**Confidence:** HIGH

## Executive Summary

This research covers adding a user-scoped "Active Baustelle" (construction site) context to an existing construction management SaaS. The feature enables workers to set their current project context, which then auto-assigns materials, tool reservations, and time entries to that project with minimal friction. Experts in this domain implement similar features using persistent UI indicators, opt-out dialogs with auto-confirm timers, and offline-first state management that syncs preferences across devices.

The recommended approach is a **two-phase implementation** using entirely existing libraries (no new dependencies). Store active site preference per-user in a dedicated `user_preferences` table (not in the User aggregate), cache locally with Zustand's persist middleware, and implement auto-assignment with a 5-second opt-out dialog. Key architectural decisions: keep TenantContext focused on auth, separate UserPreferences as its own aggregate, and use deterministic hash-based colors from site IDs rather than storing color values.

The key risks are **context blindness** (users forget their active project) and **stale active project assignments** (project archived/deleted while user is offline). Both are mitigated through prominent color-coded UI indicators, validation during sync operations, and undo functionality for auto-assigned records. The 5-second opt-out dialog must be designed carefully to avoid becoming "wallpaper" that users dismiss without reading.

## Key Findings

### Recommended Stack

**NO new dependencies required** — the feature can be implemented entirely with existing libraries already in the codebase. This significantly de-risks the implementation.

**Core technologies:**
- **Zustand 5.0.12 with persist middleware** — Active site state store — Already used for auth (`authStore.ts`), identical pattern applies
- **Tailwind CSS 4.2.4 color palette** — Deterministic site colors — Hash site ID → index into predefined palette (rose, orange, amber, emerald, teal, cyan, blue, indigo, violet)
- **TanStack Query 5.100.6** — API calls and cache invalidation — Already used, no changes needed
- **Axum 0.8 + SQLx 0.8** — Backend API endpoints — Standard patterns already established
- **ts-rs 12** — Type generation — Add DTOs for active site preference endpoints

### Expected Features

**Must have (table stakes):**
- **Persistent status indicator** — Badge/chip in header showing active Baustelle name + color (like Slack workspace indicator)
- **Easy context switch** — Toggle on overview page + dashboard; dropdown in header as alternative
- **Auto-assignment visibility** — Show "Wird gebucht auf: [Baustelle Name]" in dialogs; pre-fill field, allow change
- **Single active project per user** — User-scoped, not global; one user = one active Baustelle at a time
- **Opt-out capability** — Dialog shows pre-filled Baustelle with option to change or clear; 5-second auto-confirm

**Should have (competitive):**
- **Auto-assigned colors per Baustelle** — Visual distinction at a glance; hash-based from site ID
- **Context-aware dashboard** — Filter by active Baustelle (defer to v1.8)

**Defer (v2+):**
- **Smart context suggestions** — GPS/calendar-based suggestions require significant integration work
- **Context history** — Recently active Baustellen for quick switching (v1.8)
- **Cross-device sync** — Already supported via backend preferences; explicit sync indicator is v1.8

**Anti-features (avoid):**
- Multiple active Baustellen (cognitive load, confusing UX)
- Auto-switch based on GPS (GPS drift causes false positives)
- Forced context with no opt-out (user frustration, workarounds emerge)
- Global active Baustelle (different users work on different sites)

### Architecture Approach

The architecture follows a **modular monolith with DDD bounded contexts**. Active project context is a user preference (not request context), stored in a new `UserPreferences` aggregate within the IAM module. This separation prevents bloating the User aggregate and allows independent lifecycle management.

**Major components:**
1. **UserPreferences (IAM module)** — New aggregate storing `active_site_id`, `updated_at` per user/tenant; separate from User entity
2. **preferencesStore (Frontend)** — Zustand store with localStorage persistence; syncs to backend on change
3. **ActiveSiteIndicator component** — Persistent UI element showing current context with color; clickable for quick switch
4. **Auto-assignment injection** — Frontend hooks inject `activeSiteId` into WithdrawDialog, ReservationDialog, TimeEntryDialog
5. **Offline sync extension** — IndexedDB stores user-scoped preferences; sync queue includes site_id for pending actions

**Key integration points:**
- Reservation already has `site_id` field — just pre-fill from context
- WithdrawMaterial MISSING `site_id` — requires migration to add column
- TenantContext should NOT include active_site_id — it's preference, not auth context

### Critical Pitfalls

1. **Stale Active Project Assignment** — User sets active project, goes offline for days, project gets archived by another user. All offline records become orphaned on sync. **Prevention:** Store `active_project_set_at` timestamp; validate project status during sync; show warning banner if project became invalid.

2. **Context Blindness** — Users forget their active project and create entries for wrong project. Static indicators become "wallpaper." **Prevention:** Color-code ALL affected UI elements (not just indicator); show project name prominently in opt-out dialog; use color as background tint for affected forms.

3. **Auto-Assignment Without Easy Undo** — User realizes wrong project after deduction but has no quick way to fix. **Prevention:** Show toast with "Undo" button for 5-10 seconds after action; add "Reassign" button immediately visible; support inline project change.

4. **Offline Context Desynchronization** — Multiple users on shared device overwrite each other's context. **Prevention:** Store active project per user in IndexedDB with `user_id` key prefix; load user's preference on login; clear on logout.

5. **5-Second Dialog Fatigue** — Opt-out dialog appears on every deduction, users dismiss without reading. **Prevention:** Add "Don't ask again for this session" checkbox; track dismissal rate; consider showing opt-out only for suspicious patterns (different location than usual).

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Backend Foundation & User Preferences

**Rationale:** Database schema changes and backend services must exist before frontend can store or retrieve active site preference. Validating site ownership and tenant scope requires backend logic first.

**Delivers:** 
- `user_preferences` table with user-scoped `active_site_id`
- `site_id` column added to `material_deductions` table
- API endpoints: `GET/PATCH /api/v1/preferences/active-site`
- Validation logic for site ownership and tenant scope

**Addresses:** 
- Features: Single active project per user, Backend preference storage
- Pitfalls: Stale active project (validation during set), Offline desync (user-scoped storage), Cross-device inconsistency (backend sync)

**Avoids:** Anti-pattern of adding active_site to TenantContext

### Phase 2: Frontend UI & Auto-Assignment

**Rationale:** With backend in place, frontend can implement state management, UI indicators, and auto-assignment hooks. This phase depends on Phase 1 API endpoints.

**Delivers:**
- Zustand preferencesStore with localStorage persistence
- ActiveSiteIndicator component (header chip with color)
- "Als aktiv setzen" button on Baustellen list
- Auto-assignment to WithdrawDialog, ReservationDialog, TimeEntryDialog
- 5-second opt-out dialog with auto-confirm timer
- Color generation utility (hash-based from site ID)
- Toast notifications with undo option

**Uses:** Zustand persist, TanStack Query for API calls, Tailwind color palette

**Implements:** 
- Architecture: preferencesStore, ActiveSiteIndicator, auto-assignment injection
- Features: Persistent status indicator, Context switch toggle, Auto-assignment visibility, Opt-out capability, Color per Baustelle

**Avoids:** Pitfalls of context blindness (color-coded UI), no undo (toast with undo), dialog fatigue (session-persisted dismissal)

### Phase 3: Offline Enhancement & Validation (v1.8)

**Rationale:** Offline scenarios and cross-device sync are important but not MVP-blocking. Phase 3 adds robustness after core functionality is validated.

**Delivers:**
- IndexedDB extension for user-scoped preferences cache
- Sync protocol extension for preference synchronization
- Validation during sync for stale active projects
- Context-aware dashboard (filter by active Baustelle)
- Context history (recently active Baustellen)

**Uses:** IndexedDB (Dexie), existing sync infrastructure

**Avoids:** Pitfall of cross-device context inconsistency

### Phase Ordering Rationale

- **Phase 1 first:** Backend schema changes are foundational; cannot test frontend without API
- **Phase 2 second:** Depends on Phase 1 endpoints; delivers user-visible functionality
- **Phase 3 later:** Offline robustness can be added incrementally; MVP works without it
- **Grouping by layer:** Backend changes together, frontend changes together — cleaner commits and testing
- **Color generation in Phase 2:** Trivial utility, no external dependencies, adds immediate user value

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 1:** IndexedDB schema for user-scoped preferences — need to verify Dexie versioning and migration strategy
- **Phase 3:** Sync protocol extension — need to research WatermelonDB sync customization for preferences

Phases with standard patterns (skip research-phase):
- **Phase 1:** Backend CRUD for preferences — standard Axum + SQLx patterns, well-documented
- **Phase 2:** Zustand store with persist — identical to existing authStore.ts pattern

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | No new dependencies; all technologies already in use with established patterns |
| Features | MEDIUM | Based on codebase analysis + UX patterns from similar apps (Slack, GitHub, Jira); needs user validation |
| Architecture | HIGH | Based on thorough codebase analysis; existing integration points well-documented |
| Pitfalls | HIGH | Based on Nielsen Norman heuristics, offline-first patterns, and WatermelonDB sync documentation |

**Overall confidence:** HIGH

### Gaps to Address

- **Dialog fatigue validation:** Dismissal rate and optimal frequency need real-world testing. Plan A/B test during Phase 2.
- **Color collision handling:** With 9 colors, collisions occur at 10+ active Baustellen. Need strategy for larger tenants (consider sequential assignment or user override).
- **Work type edge case:** Time entries with `work_type: "travel"` shouldn't auto-assign to Baustelle. Logic is defined but needs E2E testing.
- **Multi-device conflict resolution:** If user sets different active sites on different devices before sync, need explicit resolution strategy. Document in Phase 1 planning.

## Sources

### Primary (HIGH confidence)
- **Context7 `/pmndrs/zustand`** — persist middleware with custom storage, hydration patterns
- **Existing codebase** — `frontend/src/lib/auth/authStore.ts` (Zustand persist pattern), `src/modules/*/domain/*.rs` (domain models), `frontend/src/lib/offline/queue.ts` (sync queue)
- **Tailwind CSS docs** — https://tailwindcss.com/docs/customizing-colors — Default color palette
- **Nielsen Norman Group** — "10 Usability Heuristics for User Interface Design" — Visibility of System Status, User Control and Freedom

### Secondary (MEDIUM confidence)
- **Nielsen Norman Group** — "Confirmation Dialogs Can Prevent User Errors" — Guidelines on dialog overuse
- **WatermelonDB Sync Documentation** — Limitations, conflict resolution patterns
- **Offline First Community** (offlinefirst.org) — Principles for offline-capable applications
- **UX patterns** — Slack workspace indicator, GitHub repository context, Jira project selector, Notion workspace switcher

### Tertiary (LOW confidence)
- **GPS auto-switch estimates** — Not viable for MVP due to GPS drift; defer to v2+ research
- **Color collision frequency** — Theoretical estimate based on palette size; needs real-world validation

---
*Research completed: 2026-04-30*
*Ready for roadmap: yes*
