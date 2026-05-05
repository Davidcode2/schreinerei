# MEAS-11

Status: Missing
Fit: Good after core measurement exists
Priority: Later
Decision: Keep

Current state: There is no Leica DISTO or Bluetooth integration, and PWA/browser compatibility risk is still unknown.
Evidence: `frontend/package.json`, `frontend/src/*`

Implementation:
1. Build manual structured measurement first.
2. Validate real device/browser support in field.
3. Add narrow field injection from device reading to selected dimension input.
4. Keep manual input first-class at all times.
