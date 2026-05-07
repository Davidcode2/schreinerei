# Phase 41: Mobile Modal Improvements — Research

**Gathered:** 2026-05-04
**Phase:** 41 - mobile-modal-improvements
**Status:** Complete

---

## Executive Summary

This research identifies tall modals in the Schreinerei codebase and determines the best approach for implementing step-based navigation with swipe gestures. The primary target for the proof of concept is `AddMaterialDialog`, which has 7-8 input fields depending on category selection.

---

## 1. Modal Inventory

### Dialog Components Found

| Dialog | Location | Input Fields | Height Assessment |
|--------|----------|--------------|-------------------|
| AddMaterialDialog | inventory | 7-8 | **TALL** - Primary POC target |
| AddVehicleDialog | fleet | 6-7 | TALL |
| AddToolDialog | fleet | ~5 | Medium |
| WithdrawDialog | inventory | 4-5 + conditional | Medium-Tall |
| StockInDialog | inventory | 3-4 | Medium |
| MaterialEditDialog | inventory | 3 | Short |
| AddSiteDialog | sites | 4 | Medium |
| TimeEntryDialog | sites | 3-4 | Medium |
| CategoryDialog | inventory | 3 | Short |
| ReservationDialog | fleet | 3-4 | Medium |
| InviteUserDialog | settings | 2-3 | Short |
| CreateNoteModal | sites | 2 | Short |
| StatusChangeModal | sites | 1-2 | Short |

### Base Dialog Component

**File:** `frontend/src/components/ui/dialog.tsx`

**Current close button implementation:**
```tsx
<DialogPrimitive.Close className="absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground">
  <X className="h-4 w-4" />
  <span className="sr-only">Close</span>
</DialogPrimitive.Close>
```

**Issues identified:**
- Close button is only `h-4 w-4` (16x16px) — well below the 44x44px touch target minimum
- No padding around the button — hard to tap on mobile
- Uses `rounded-sm` — should use `rounded-lg` for better visual feedback

---

## 2. Primary Target: AddMaterialDialog Analysis

### Current Field Structure

**Step 1 - Basisdaten (Basic Data):**
1. Kategorie (Category) - Select with "+ Neu" button for inline creation
2. Name - Text input
3. Menge (Quantity) - Number input
4. Einheit (Unit) - Select

**Step 2 - Details:**
5. Mindestbestand (Min Quantity) - Number input
6. MHD (Expiry Date) - Date input (conditional: shown only when category has `can_expire`)
7. Lagerort (Location) - Text input (optional)

### Splitting Strategy

Per UI-SPEC:
- **Step 1:** Category, Name, Quantity, Unit (4 fields, all required)
- **Step 2:** Min Quantity, MHD (conditional), Location (3 fields, 1-2 required)

This gives a balanced split with required fields on Step 1 and mostly-optional fields on Step 2.

---

## 3. Technical Approach

### Step Navigation Pattern

No existing wizard/stepper components found in codebase. Will need to create:

1. **`StepIndicator` component** - Dot pagination at bottom of modal
2. **`StepContainer` component** - Wrapper with swipe gesture handling
3. **`useSwipeGesture` hook** - Touch gesture detection

### Gesture Library Assessment

**Current state:** No gesture libraries installed.

**Options:**

| Option | Bundle Size | Complexity | Recommendation |
|--------|-------------|------------|----------------|
| Native touch events | 0 KB | Medium | **Recommended** - No dependency, full control |
| @use-gesture/react | ~8 KB | Low | Good alternative if more complex gestures needed |
| framer-motion | ~45 KB | High | Overkill for this use case |

**Recommended approach:** Use native touch events with a custom `useSwipeGesture` hook. This is:
- Zero-dependency
- Full control over behavior
- Simple for horizontal swipe detection (50px threshold per UI-SPEC)

### Swipe Detection Algorithm

```typescript
// useSwipeGesture hook structure
interface SwipeGestureOptions {
  onSwipeLeft?: () => void;
  onSwipeRight?: () => void;
  threshold?: number; // Default: 50px
}

function useSwipeGesture(options: SwipeGestureOptions) {
  const [touchStart, setTouchStart] = useState<number | null>(null);
  
  const onTouchStart = (e: React.TouchEvent) => {
    setTouchStart(e.touches[0].clientX);
  };
  
  const onTouchEnd = (e: React.TouchEvent) => {
    if (touchStart === null) return;
    const touchEnd = e.changedTouches[0].clientX;
    const diff = touchStart - touchEnd;
    
    if (Math.abs(diff) > (options.threshold ?? 50)) {
      if (diff > 0) options.onSwipeLeft?.();
      else options.onSwipeRight?.();
    }
    setTouchStart(null);
  };
  
  return { onTouchStart, onTouchEnd };
}
```

### Step Transition Animation

Per UI-SPEC, use CSS transitions:
- **Duration:** 200ms ease-out
- **Enter:** `translateX(100%)` → `translateX(0)`
- **Exit:** `translateX(0)` → `translateX(-100%)`

No JavaScript animation library needed — CSS transitions are sufficient.

---

## 4. Close Button Fix

### Current Implementation (Problematic)

```tsx
<X className="h-4 w-4" />
// Total size: 16x16px
```

### Required Implementation (per UI-SPEC)

```tsx
<DialogPrimitive.Close className="absolute right-3 top-3 rounded-lg p-2.5 opacity-70 ring-offset-background transition-all hover:opacity-100 hover:bg-accent focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none">
  <X className="h-5 w-5" />
  <span className="sr-only">Schließen</span>
</DialogPrimitive.Close>
```

**Changes:**
- `p-2.5` (10px padding) = 20px + 20px icon = 40x40px touch target
- Icon size increased from `h-4 w-4` to `h-5 w-5`
- `rounded-sm` → `rounded-lg` for better visual feedback
- Added `hover:bg-accent` for touch feedback
- `transition-opacity` → `transition-all` for hover state

---

## 5. Modal Height Constraints

Per UI-SPEC:

| Screen Height | Max Height | Behavior |
|---------------|------------|----------|
| < 667px (iPhone SE) | 85vh | Scroll within step content |
| 667px - 812px (iPhone 8-X) | 90vh | Scroll within step content |
| > 812px | 90vh | Natural height |

**Implementation:** Add `max-h-[85vh] sm:max-h-[90vh]` to DialogContent and `overflow-y-auto` to the form content area.

---

## 6. Component Architecture

### New Components

```
frontend/src/components/ui/
├── dialog.tsx (modified - close button fix)
├── step-indicator.tsx (new)
└── step-container.tsx (new)

frontend/src/hooks/
└── useSwipeGesture.ts (new)
```

### StepIndicator Component

```tsx
interface StepIndicatorProps {
  currentStep: number;
  totalSteps: number;
  onStepClick?: (step: number) => void;
}

// Render: dots with active/completed states
// Touch target: 44x44px per dot
```

### StepContainer Component

```tsx
interface StepContainerProps {
  currentStep: number;
  onStepChange: (step: number) => void;
  children: React.ReactNode;
}

// Handles swipe gestures
// Manages enter/exit animations
// Prevents body scroll during gesture
```

---

## 7. Implementation Order

Per CONTEXT.md scope: **Proof of Concept First**

1. **Phase 41 Plan 1: Infrastructure**
   - Fix close button in base Dialog component
   - Create StepIndicator component
   - Create useSwipeGesture hook
   - Create StepContainer component

2. **Phase 41 Plan 2: Apply to AddMaterialDialog**
   - Refactor AddMaterialDialog to use step-based layout
   - Add step navigation (buttons + swipe)
   - Add dot indicators
   - Test on mobile viewport

3. **Future (post-approval):** Apply pattern to other tall modals

---

## 8. Testing Strategy

### Manual Testing
- Test on iPhone SE (375x667) - smallest target device
- Test on iPhone 12 Pro (390x844) - common device
- Verify swipe gestures work smoothly
- Verify touch targets are 44x44px minimum

### Automated Testing
- Add unit tests for useSwipeGesture hook
- Add integration tests for AddMaterialDialog step navigation
- Test step validation (can't proceed without required fields)

---

## 9. Dependencies

### Required (Existing)
- `@radix-ui/react-dialog` - Already in use
- `lucide-react` - Already in use (X, ChevronLeft, ChevronRight icons)

### Required (New)
None - using native touch events

### Optional (Future)
- `@use-gesture/react` - If gesture handling becomes more complex

---

## 10. Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Swipe conflicts with form scrolling | Use horizontal threshold; prevent swipe when vertical scroll detected |
| Form state lost on step change | Keep all state in parent component; steps are just UI |
| Accessibility for keyboard users | Add Tab navigation between steps; focus management |
| Validation timing | Validate on step advance; show errors before proceeding |

---

## Summary

**Primary target:** AddMaterialDialog (7-8 fields)
**Approach:** Split into 2 steps with dot indicators and swipe navigation
**Close button:** Increase touch target from 16x16px to 44x44px
**Gestures:** Native touch events (no library needed)
**Animation:** CSS transitions (200ms ease-out)

This is a focused, low-risk improvement that addresses a real mobile UX pain point without adding heavy dependencies.

---

*Research complete: 2026-05-04*
