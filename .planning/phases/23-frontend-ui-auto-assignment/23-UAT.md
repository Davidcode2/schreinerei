---
status: testing
phase: 23-frontend-ui-auto-assignment
source: 23-01-SUMMARY.md, 23-02-SUMMARY.md, 23-03-SUMMARY.md
started: 2026-04-30T19:35:59Z
updated: 2026-04-30T20:53:39Z
---

## Current Test

number: 5
name: Time Entry Conditional Site Prefill
expected: |
  In time entry, site is prefilled from active site only when work type is site-related, and users can still override or clear it.
awaiting: user response

## Tests

### 1. Active Site Indicator Visibility
expected: On both desktop and mobile navigation, the currently active site is always visible as a persistent indicator with a stable color identity for that site.
result: pass

### 2. Toggle Active Site on Site Cards
expected: On Sites overview and Dashboard cards, selecting a site as active updates the active marker, and only one site is marked active at a time. Selecting the active one again clears it.
result: pass

### 3. Inventory Withdraw Site Prefill and Override
expected: In material withdrawal, site selection is prefilled from active site, can be overridden, and can be explicitly cleared when needed.
result: pass

### 4. Reservation Site Prefill and Override
expected: In reservation creation, site selection is prefilled from active site and users can override or clear it before submission.
result: pass

### 5. Time Entry Conditional Site Prefill
expected: In time entry, site is prefilled from active site only when work type is site-related, and users can still override or clear it.
result: [pending]

## Summary

total: 5
passed: 4
issues: 0
pending: 1
skipped: 0
blocked: 0

## Gaps

[none yet]
