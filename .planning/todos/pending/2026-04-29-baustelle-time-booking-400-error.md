---
created: 2026-04-29T00:00:00Z
title: Fix baustelle time booking 400 error
area: api
resolves_phase: 17
files:
  - src/components/sites/TimeBookingDialog.tsx
  - src/api/time_bookings.rs
---

## Problem

Time booking on a baustelle (construction site) fails with a 400 error. Users cannot log work hours on sites.

## Solution

Investigate the time booking API endpoint. Check request payload format, required fields, and validation logic.
