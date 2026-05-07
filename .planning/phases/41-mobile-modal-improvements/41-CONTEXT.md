# Phase 41: Mobile Modal Improvements

## User Context

On mobile, some of the modals for user interaction are too tall to fit on the screen. Scrolling is currently not possible.

## Goal

Find all modals which may be affected and implement a solution to prevent modals from being too tall.

## Approach

1. **Identify tall modals** - Find all modals that may be affected by height issues on mobile
2. **Split large modals** - For large modals with many inputs, split the inputs over two or more screens
3. **Add dot indicators** - The modal should contain dot indicators at the bottom showing on which screen of the modal the user is
4. **Swipe navigation** - The user should be able to swipe left and right to navigate to the next part of the input
5. **Larger close button** - Ensure the close button in the top right is a bit larger. Currently it is hard to tap due its small size.

## Scope

**Proof of Concept First:** Implement for ONE modal deemed suitable. After approval, apply pattern to other tall modals.

## Benefits

- Prevents modals from being too tall
- Better UX with fewer inputs per screen
- Easier to tap close button

## Constraints

- Must work on mobile devices
- Should maintain existing modal functionality
- Swipe navigation should feel natural
