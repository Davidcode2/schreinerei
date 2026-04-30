# Pitfalls Research: Active Project Context

**Domain:** Field Service / Construction App — Adding User-Scoped Active Project Context
**Researched:** 2026-04-30
**Confidence:** HIGH (based on Nielsen Norman heuristics, offline-first patterns, and WatermelonDB sync documentation)

## Critical Pitfalls

### Pitfall 1: Stale Active Project Assignment

**What goes wrong:**
User sets active project on Monday, works offline all week, then syncs on Friday. Meanwhile, the project was archived or deleted by another user. All material deductions and reservations created offline are now orphaned or rejected.

**Why it happens:**
Offline-first apps cannot validate referential integrity during offline operations. Developers assume "active project" is always valid because it was valid when set.

**How to avoid:**
- Store `active_project_set_at` timestamp alongside `active_project_id`
- On sync, validate active project still exists and is active status
- Show warning banner if active project became invalid during offline period
- Provide "resolve conflicts" UI showing affected records with reassignment options

**Warning signs:**
- Long offline periods before sync
- Users reporting "my deductions disappeared"
- Foreign key constraint violations on sync

**Phase to address:** Phase 1 (Backend & Sync Logic) — requires validation during pushChanges

---

### Pitfall 2: Context Blindness (Users Forget Active Project)

**What goes wrong:**
Users set active project, then navigate to other tasks. By the time they deduct materials, they've forgotten which project is active and create entries for the wrong project. The UI indicator becomes "wallpaper" that users stop noticing.

**Why it happens:**
Nielsen Norman Heuristic #1 (Visibility of System Status) requires persistent, meaningful feedback. Static indicators lose salience over time. Users develop "banner blindness" — same reason cookie banners are ignored.

**How to avoid:**
- Color-code ALL relevant UI elements when active project is set (not just indicator)
- Use the project's auto-assigned color as background tint for affected forms
- Show project name prominently in the opt-out dialog (not just "Current Project")
- Consider subtle visual "pulse" or badge when context affects current action
- Add project name to confirmation messages: "Material deducted to [Project Name]"

**Warning signs:**
- Support tickets about "wrong project assignments"
- Users asking "which project am I on?"
- High rate of deduction reassignments after the fact

**Phase to address:** Phase 2 (Frontend UI) — indicator visibility and color coding

---

### Pitfall 3: Auto-Assignment Without Easy Undo

**What goes wrong:**
User scans QR code to deduct material. System auto-assigns to active project. User realizes wrong project but has no quick way to fix it. Must navigate to separate screen, find the record, edit, and reassign. Many users just leave it wrong.

**Why it happens:**
Nielsen Norman Heuristic #3 (User Control and Freedom) requires "emergency exits." Auto-assignment is a convenience that becomes a trap without easy reversal. The 5-second opt-out dialog is pre-action, not post-action correction.

**How to avoid:**
- Add "Reassign" button immediately visible after auto-assignment
- Show toast notification with "Undo" button for 5-10 seconds after action
- Support inline project change on the deduction confirmation screen
- Log recent assignments to allow bulk reassignment if user realizes systematic error

**Warning signs:**
- High rate of edit operations on recently created records
- Users complaining "too many clicks to fix"
- Support requests for bulk reassignment

**Phase to address:** Phase 2 (Frontend UI) — undo/redo and reassignment UX

---

### Pitfall 4: Offline Context Desynchronization

**What goes wrong:**
User A sets active project to "Kitchen Renovation" while offline. User A's colleague (User B) also offline, sets active project to "Bathroom Renovation" on same device (shared tablet). Both users create records. On sync, context is overwritten or conflicts.

**Why it happens:**
Active project is user-scoped, but IndexedDB is device-scoped. Multi-user device scenarios create "last write wins" problems for user preferences.

**How to avoid:**
- Store active project per user in a `user_preferences` table with `user_id` FK
- Include `user_id` in IndexedDB storage key: `active_project:{user_id}`
- On login, load user's active project from server if available
- Handle logout by clearing active project from IndexedDB
- Consider: should active project persist across logout/login on same device?

**Warning signs:**
- Users on shared devices seeing wrong context
- "My settings changed" reports
- Context switching unexpectedly after sync

**Phase to address:** Phase 1 (Backend & IndexedDB Schema) — user-scoped storage

---

### Pitfall 5: The 5-Second Opt-Out Dialog Becomes Annoyance

**What goes wrong:**
Opt-out dialog appears on every deduction. User dismisses it automatically without reading. Same problem as Pitfall 2 — automated behavior defeats the purpose. NN/G warns that overused confirmations "lose their power to prevent errors."

**Why it happens:**
Nielsen Norman research shows confirmation dialogs must be for "serious consequences" and "not routine actions." Material deduction is a routine action for field workers. Repeated confirmations become muscle memory.

**How to avoid:**
- Only show opt-out dialog if active project is set AND user hasn't dismissed in last N actions
- Add "Don't ask again for this session" checkbox
- Track dismissal rate — if >90%, reduce frequency
- Consider: show opt-out ONLY when context seems suspicious (different location than usual, new material type)
- Use progressive disclosure: show project name in form, dialog only on explicit "Change" action

**Warning signs:**
- Users clicking through without reading
- Support requests showing users didn't notice active project
- A/B testing shows no difference in error rate with/without dialog

**Phase to address:** Phase 2 (Frontend UI) — dialog UX refinement

---

### Pitfall 6: Cross-Device Context Inconsistency

**What goes wrong:**
User sets active project on phone, then uses tablet. Active project differs between devices. User creates records on both devices, leading to inconsistent project assignments. Sync merges records but context remains device-specific.

**Why it happens:**
Active project is stored locally (IndexedDB) but not synced as user preference. This is a deliberate offline-first choice, but creates inconsistency when users work across devices.

**How to avoid:**
- Sync active project as a user preference (not per-device)
- Show "Last synced from [device] at [time]" in UI
- On conflict (different active project on different devices), show resolution dialog
- Consider: should active project change propagate immediately, or only on next sync?
- Add "Active on other devices: [Project Name]" indicator

**Warning signs:**
- Users reporting inconsistent behavior across devices
- Records assigned to unexpected projects
- Confusion after switching devices mid-task

**Phase to address:** Phase 1 (Backend Sync) — add user preferences to sync protocol

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Store active project in localStorage only | Faster implementation, no IndexedDB schema change | Lost on logout, not offline-safe, no user-scoping | Never — use IndexedDB from start |
| Skip active project validation during deduction | Fewer API calls, faster UX | Orphaned records on invalid project | Never — always validate FK |
| Make active project global (not user-scoped) | Simpler state management | Breaks multi-user devices, multi-device scenarios | Never — must be user-scoped |
| Don't sync active project preference | Simpler sync protocol | Cross-device inconsistency | Acceptable for MVP if documented |
| Hardcode color assignments | Faster initial implementation | Can't change colors, conflicts with similar colors | Acceptable for MVP, move to config later |
| No undo for auto-assignment | Simpler state management | User frustration, wrong data | Never — must provide undo |

---

## Integration Gotchas

Common mistakes when connecting to existing services.

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| IndexedDB (Dexie.js) | Storing active project as single value without user_id FK | Use `user_preferences` table with compound key `[user_id+tenant_id]` |
| Sync Protocol | Not including user preferences in sync payload | Add `user_preferences` to sync schema, sync before other data |
| Material Deduction API | Assuming `site_id` is always provided | Default to active project, but require explicit confirmation if not set |
| Keycloak Organizations | Assuming organization context includes user preferences | Fetch user preferences from app backend, not Keycloak |
| QR Scanner | Auto-assigning without showing active project context | Display active project name prominently in scanner UI |

---

## Performance Traps

Patterns that work at small scale but fail as usage grows.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Fetching active project from server on every deduction | Latency, poor offline UX | Cache in IndexedDB, validate on sync only | Immediately — offline must work |
| Validating project existence on every form render | Slow form opens, laggy UI | Validate once on project set, trust until sync | 50+ active users |
| Storing full project object in active project state | Memory bloat, stale data | Store project_id only, fetch details as needed | 100+ projects per tenant |
| Color assignment via UUID hash (random) | Color collisions, hard to distinguish | Use predefined palette, assign sequentially | 10+ projects per tenant |
| No pagination on project selector | Slow load, UI freeze | Paginate or lazy-load project list | 100+ projects per tenant |

---

## Security Mistakes

Domain-specific security issues beyond general web security.

| Mistake | Risk | Prevention |
|---------|------|------------|
| User can set active project from another tenant | Cross-tenant data leak | Validate tenant_id matches user's current organization on set |
| Active project not validated on API call | Data assigned to unauthorized project | Always validate user has access to project_id on write operations |
| Storing active project in URL params | User can manipulate, share wrong context | Use IndexedDB/localStorage, not URL state |
| No audit trail for context changes | Can't investigate data quality issues | Log active project changes with timestamp and user_id |

---

## UX Pitfalls

Common user experience mistakes in this domain.

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Indicator hidden in hamburger menu | Users forget active project, create wrong assignments | Persistent header bar with color indicator |
| No feedback after auto-assignment | User doesn't realize what happened | Toast notification with project name and undo option |
| Context switch requires navigation to settings | Too many clicks, users don't switch | Quick toggle in header or on dashboard |
| Same UI whether context is set or not | User doesn't notice absence of context | Grayed/inactive UI when no active project, prompt to set |
| Opt-out dialog blocks all interaction | Workflow interruption, frustration | Non-modal indicator with dismiss/change options |
| Project colors not visible on monochrome prints | Printouts lose context information | Include project name/code in all printed labels |

---

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **Active Project Indicator:** Often missing color coding beyond text — verify color tint appears on affected forms
- [ ] **Offline Support:** Often missing user-scoped storage — verify active project persists per user after logout/login
- [ ] **Undo Function:** Often missing reassignment flow — verify user can change project after auto-assignment
- [ ] **Opt-Out Dialog:** Often missing "don't ask again" option — verify session persistence of dismissal
- [ ] **Cross-Device Sync:** Often missing preference sync — verify active project syncs between devices
- [ ] **Validation:** Often missing on write operations — verify API rejects invalid project_id even if set locally
- [ ] **Audit Trail:** Often missing for context changes — verify changes are logged with timestamp

---

## Recovery Strategies

When pitfalls occur despite prevention, how to recover.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Stale Active Project | MEDIUM | Run script to find orphaned records, provide bulk reassignment UI |
| Context Blindness | LOW | Add more prominent indicators, retrain users |
| No Undo | LOW | Add undo feature retroactively, migrate recent records to include undo window |
| Offline Desync | HIGH | May require manual data reconciliation if multiple users affected |
| Dialog Fatigue | LOW | Adjust dialog frequency, add "don't ask again" option |
| Cross-Device Inconsistency | MEDIUM | Implement preference sync, may need manual cleanup of inconsistent records |

---

## Pitfall-to-Phase Mapping

How roadmap phases should address these pitfalls.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Stale Active Project | Phase 1 (Backend) | E2E test: create deductions offline, archive project, sync, verify warning |
| Context Blindness | Phase 2 (Frontend) | User test: ask users to identify active project after 5 minutes |
| No Undo | Phase 2 (Frontend) | E2E test: deduct material, undo, verify reverted |
| Offline Desync | Phase 1 (Backend) | E2E test: two users on shared device, verify separate contexts |
| Dialog Fatigue | Phase 2 (Frontend) | Analytics: track dismiss rate, verify <80% auto-dismiss |
| Cross-Device Sync | Phase 1 (Backend) | E2E test: set on device A, sync device B, verify consistency |

---

## Sources

- Nielsen Norman Group: "10 Usability Heuristics for User Interface Design" — Visibility of System Status, User Control and Freedom
- Nielsen Norman Group: "Confirmation Dialogs Can Prevent User Errors" — Guidelines on dialog overuse and specificity
- Nielsen Norman Group: "The Power of Defaults" — User tendency to stick with defaults
- WatermelonDB Sync Documentation — Limitations, conflict resolution patterns
- Offline First Community (offlinefirst.org) — Principles for offline-capable applications
- Local First Web (localfirstweb.dev) — Patterns for local-first state management

---

*Pitfalls research for: Active Project Context (Construction Field Service App)*
*Researched: 2026-04-30*
