# Phase 22 Verification — Backend Foundation (User Preferences)

## Scope

This verification artifact covers Phase 22 backend requirements reassigned to Phase 24 revalidation:

- PREF-01
- PREF-02
- PREF-03
- DEDU-01
- DEDU-02

`DEDU-03` is intentionally excluded here because ownership moved to Phase 25 per milestone audit.

## Evidence Sources

- `.planning/phases/22-backend-foundation-user-preferences/22-01-SUMMARY.md`
- `.planning/phases/22-backend-foundation-user-preferences/22-02-SUMMARY.md`
- `.planning/phases/22-backend-foundation-user-preferences/22-03-SUMMARY.md`
- `.planning/phases/22-backend-foundation-user-preferences/22-04-SUMMARY.md`
- `.planning/v1.7-MILESTONE-AUDIT.md`

## Requirement Matrix

| Requirement | Status | Evidence | Rationale |
|---|---|---|---|
| PREF-01 | pass | `22-01-SUMMARY.md` Task commits `9c918e8`, `b973b4a`; repository/service created for persistent user preferences | Preferences are stored server-side through `UserPreferencesRepository` and `UserPreferencesService`, satisfying persistent backend storage. |
| PREF-02 | pass | `22-01-SUMMARY.md` lines 72-83 (site existence/tenant/status validation), `22-03-SUMMARY.md` lines 56-60 (PATCH endpoint validation path), commit `60b7644` | Active site updates are validated against existing, tenant-scoped, non-archived Baustellen before persistence. |
| PREF-03 | pass | `22-01-SUMMARY.md` lines 76-77 (`get_validated_preferences` auto-clear), `22-03-SUMMARY.md` lines 51-54 (GET returns validated prefs) | System automatically clears invalid active site preferences when the referenced site becomes invalid, matching requirement intent. |
| DEDU-01 | pass | `22-02-SUMMARY.md` Task 2 commit `1e01ebe`, Task 3 commit `624ac7d` (repository + service wiring), lines 80-84 | Material deduction storage now includes optional `site_id` linkage to Baustelle via backend persistence path. |
| DEDU-02 | pass | `22-02-SUMMARY.md` Task 1 commit `27396a2`, Task 4 commit `9c087cd`; lines 52-56 + API DTO update | `WithdrawMaterial` and API request contract accept optional `site_id`, enabling caller-provided assignment without breaking existing flows. |

## Verdict

Phase 22 verification blockers from the milestone audit are resolved for PREF-01/02/03 and DEDU-01/02. Each requirement now has explicit, evidence-backed status and traceable implementation references. `DEDU-03` remains out of scope for this verification file because Phase 25 owns the frontend history-consumer closure.
