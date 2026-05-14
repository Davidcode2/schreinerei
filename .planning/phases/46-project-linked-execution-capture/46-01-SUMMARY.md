# 46-01 Summary

## Outcome

Real material withdrawals are now project-linked by rule, while disposal remains a valid no-project exception.

## Changes

- Added backend validation so non-disposal `WithdrawMaterial` commands require `site_id`
- Preserved disposal as a null-project path
- Updated `WithdrawDialog` to require a project for real withdrawals and show project-type cues
- Wired `InventoryDetailPage` to default the withdrawal dialog from query-string project or active project preference
- Added targeted withdrawal dialog and inventory detail tests

## Verification

- `cargo test withdraw_material_validate --lib`
- `npm --prefix frontend run test -- WithdrawDialog.test.tsx InventoryDetailPage.test.tsx`
- `npm --prefix frontend run build`

## Notes

- This plan enforces project linkage for material consumption only. Stock correction remains a separate flow and disposal remains exempt.
