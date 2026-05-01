---
status: complete
phase: 28-material-history-tab
source: [28-material-history-tab-01-SUMMARY.md, 28-material-history-tab-02-SUMMARY.md, 28-material-history-tab-03-SUMMARY.md]
started: 2026-05-01T12:00:00Z
updated: 2026-05-01T12:05:00Z
---

## Current Test

[testing complete]

## Tests

### 1. Material tab shows stock entries
expected: Navigate to a Baustelle detail page. Click the "Material" tab. The tab displays all stock entries linked to the Baustelle — each entry shows material name, category, extractor, and timestamp.
result: pass

### 2. Baustelle name links to site detail
expected: In the Material tab, click a Baustelle name. Browser navigates to that site's detail page (/sites/:id).
result: pass

### 3. Category name displayed alongside material
expected: Each material entry in the Material tab shows both the material name and its category name (e.g. "Holz" with category "Baustoffe").
result: pass

### 4. Extracted-by shows user name
expected: Each material entry shows who extracted the material — displaying the user's name (or email if name is unavailable).
result: pass

### 5. Material history loads without N+1 lag
expected: Switch to the Material tab and observe that entries load promptly without noticeable per-row delay. The backend uses a single eager-loaded query (no N+1).
result: pass

## Summary

total: 5
passed: 5
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]