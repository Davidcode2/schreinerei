# Phase 24 Requirement Revalidation

## Matrix

| Requirement | Source Phase | Revalidated Status | Evidence | Notes |
|---|---|---|---|---|
| PREF-01 | 22 | pass | `.planning/phases/22-backend-foundation-user-preferences/22-VERIFICATION.md` (PREF-01 row; commits `9c918e8`, `b973b4a`) | Server-side user preference persistence confirmed. |
| PREF-02 | 22 | pass | `.planning/phases/22-backend-foundation-user-preferences/22-VERIFICATION.md` (PREF-02 row; commit `60b7644`) | Active-site validation covers existence, tenancy, and archived state checks. |
| PREF-03 | 22 | pass | `.planning/phases/22-backend-foundation-user-preferences/22-VERIFICATION.md` (PREF-03 row) | Invalid active-site preference is auto-cleared during validated reads. |
| DEDU-01 | 22 | pass | `.planning/phases/22-backend-foundation-user-preferences/22-VERIFICATION.md` (DEDU-01 row; commits `1e01ebe`, `624ac7d`) | Deduction records can carry Baustelle linkage via optional `site_id`. |
| DEDU-02 | 22 | pass | `.planning/phases/22-backend-foundation-user-preferences/22-VERIFICATION.md` (DEDU-02 row; commits `27396a2`, `9c087cd`) | Withdraw command/API accept optional `site_id` and keep backward compatibility. |

## Scope Boundary

- `DEDU-03` is intentionally excluded from Phase 24 completion and remains owned by **Phase 25** for frontend history consumer wiring and end-to-end verification.
