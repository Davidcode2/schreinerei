# Phase 8 Context: Bug Fixes

## Goal

Fix all frontend-accessible functionalities - ensure end-to-end flows work correctly.

## Issues Discovered

### 1. Dashboard Sites API 500 Error

**Error:**
```
Database error: error occurred while decoding column "total_hours": 
mismatched types; Rust type `core::option::Option<f64>` (as SQL type `FLOAT8`) 
is not compatible with SQL type `NUMERIC`
```

**Endpoint:** `GET /api/v1/dashboard/sites`

**Root Cause:** PostgreSQL NUMERIC type cannot be decoded as Rust f64 directly. Need to use `sqlx::types::BigDecimal` or change column type.

**Files to check:**
- `src/modules/sites/` - Dashboard site queries
- Database schema for `total_hours` column

---

### 2. Logout Button Non-Functional

**Symptom:** Clicking logout button does nothing

**Files to check:**
- `frontend/src/lib/auth/authStore.ts` - logout action
- `frontend/src/components/layout/Header.tsx` or similar - logout button handler

---

### 3. Vehicle Creation 400 Error

**Error:**
```json
{"error":"Invalid vehicle type: van"}
```

**Endpoint:** `POST /api/v1/fleet/vehicles`

**Request body example:**
```json
{
  "name": "VW Transporter",
  "license_plate": "B-AB 1234",
  "vehicle_type": "van"
}
```

**Root Cause:** Backend doesn't accept "van" as valid vehicle type, but frontend sends it.

**Files to check:**
- `frontend/src/types/fleet.ts` - VehicleType enum
- `src/modules/fleet/api/routes.rs` - vehicle_type validation
- `src/modules/fleet/domain/entities.rs` - VehicleType enum

---

### 4. Cannot Create Materials - No Category Selection

**Symptom:** AddMaterialDialog requires category_id but there's no way to select or create a category

**Files to check:**
- `frontend/src/pages/inventory/AddMaterialDialog.tsx` - category dropdown
- `frontend/src/lib/api/hooks/useInventory.ts` - useCategories hook
- Backend: `GET /api/v1/inventory/categories` endpoint

**Possible fixes:**
1. Add category creation dialog
2. Add "uncategorized" default option
3. Pre-populate categories on tenant creation

---

### 5. Calendar Reservation 400 Error

**Error:**
```json
{"error":"Invalid start_date format"}
```

**Symptom:** Clicking calendar button to reserve vehicle fails

**Files to check:**
- `frontend/src/components/qr/QrScanner.tsx` or reservation component
- Date format being sent vs. expected by backend

---

## Test Credentials

**Login:**
- Email: `schreiner@admin.test`
- Password: `T6&Mo2wnhypFEZ$P$8QqdWELZ3BP5Hhe`

## Test Approach

1. Use Playwright CLI to test each function end-to-end
2. Login with test credentials
3. Test each broken flow:
   - Navigate to dashboard → verify sites load without 500
   - Navigate to inventory → try to create material
   - Navigate to fleet → create vehicle
   - Navigate to fleet → reserve vehicle
   - Test logout button

## Requirements

No formal requirements IDs yet - bugs discovered during testing.

## Success Criteria

1. Dashboard sites API returns 200 with correct data
2. Logout button successfully logs user out
3. Vehicle creation accepts "van" type (or frontend sends valid type)
4. Material creation works with category selection
5. Calendar reservation accepts date format
