# 46-02 Summary

## Outcome

Productive time booking is now project-linked for both on-site and workshop work, while travel and other overhead remain intentionally unlinked.

## Changes

- Added productive-time validation for `site` and `workshop` work types in `time_entry.rs`
- Enforced effective merged-state validation in `SiteService::update_time_entry`
- Updated `TimeEntryDialog` so productive work requires a project selector and preserves defaults
- Kept overhead work types (`travel`, `other`) as no-project exceptions
- Added targeted tests for productive-site defaults, workshop edit flow, and overhead behavior

## Verification

- `cargo test project_linked_time_entry --lib`
- `npm --prefix frontend run test -- TimeEntryDialog.test.tsx`
- `npm --prefix frontend run build`

## Notes

- The database still allows null `site_id` because overhead entries need that path.
