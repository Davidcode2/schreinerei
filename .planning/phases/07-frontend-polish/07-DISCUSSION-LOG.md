# Phase 7: Frontend Polish - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-29
**Phase:** 07-Frontend-Polish
**Areas discussed:** Dialog UX flow, Form validation, QR scanner retry, User invitation

---

## Dialog UX Flow

### Success Flow

| Option | Description | Selected |
|--------|-------------|----------|
| Close + Toast | Dialog closes immediately, shows success toast. User sees updated list. Clean, fast, consistent with WithdrawDialog pattern. | ✓ |
| Keep open + Success state | Dialog stays open showing 'Created successfully' message. User must click Close. Good for creating multiple items in a row. | |
| Close + Navigate | Dialog closes, navigates to newly created item's detail page. More clicks but immediate deep access. | |

**User's choice:** Close + Toast (Recommended)

### Toast Position

| Option | Description | Selected |
|--------|-------------|----------|
| Bottom-right | Standard shadcn/ui Sonner position. Unobtrusive, familiar pattern. Already used in app if Sonner installed. | ✓ |
| Top-center | More visible, good for important confirmations. Can feel intrusive if overused. | |

**User's choice:** Bottom-right (Recommended)

### Form Reset

| Option | Description | Selected |
|--------|-------------|----------|
| Reset to empty | Fresh form each time. User won't accidentally submit old data. Standard pattern. | ✓ |
| Preserve last values | Remember previous entries. Useful for creating similar items repeatedly. Risk of stale data. | |

**User's choice:** Reset to empty (Recommended)

---

## Form Validation

**User's choice:** Agent discretion
**Notes:** User asked agent to make decisions for all remaining questions

**Agent decisions:**
- Client-side validation matching backend rules
- Inline error messages below each field
- Required fields marked with asterisk
- Submit button disabled until form is valid

---

## QR Scanner Retry

**User's choice:** Agent discretion
**Notes:** User asked agent to make decisions for all remaining questions

**Agent decisions:**
- Retry button appears when camera access denied
- Manual code entry text field as fallback option
- Friendly German error message with instructions

---

## User Invitation

**User's choice:** Agent discretion
**Notes:** User asked agent to make decisions for all remaining questions

**Agent decisions:**
- Show Keycloak organization invite link for admins to copy and share
- No email sending from frontend — uses Keycloak's built-in invitation URL
- Display "Organisation beitreten" section with copyable link

---

## the agent's Discretion

- Form field ordering and layout (follow existing patterns)
- Exact toast message wording (German, concise)
- Error boundary behavior for failed API calls

## Deferred Ideas

None — discussion stayed within phase scope.
