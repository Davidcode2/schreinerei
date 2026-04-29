---
created: 2026-04-29T00:00:00Z
title: Fix vehicle reservation submit failure
area: ui
files:
  - src/components/fleet/ReservationDialog.tsx
---

## Problem

When attempting to reserve a vehicle, the submit action fails. Users cannot create vehicle reservations through the UI.

## Solution

Investigate the reservation API call. Likely issues: date formatting, missing fields, or API endpoint mismatch.
