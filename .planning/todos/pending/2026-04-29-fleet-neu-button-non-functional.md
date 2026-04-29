---
created: 2026-04-29T00:00:00Z
title: Fix fleet page "Neu" button
area: ui
files:
  - src/components/fleet/FleetPage.tsx
---

## Problem

The "Neu" (New) button on the fleet page is non-functional. Users cannot add new vehicles through the UI.

## Solution

Connect the "Neu" button to the AddVehicleDialog component. Ensure onClick handler opens the dialog.
