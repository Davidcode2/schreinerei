# Requirement Review Summary

This summary indexes the per-requirement reviews in `.planning/requirement-reviews/` and captures the main product-direction decisions.

## Core Calls

- Keep the app mobile-first and project-centric.
- Evolve `Baustelle` into `Project`; do not create a second competing model.
- Make project-linked material/time booking the basis for analytics and billing.
- Keep inventory focused on stockable things; move serviceable assets into Assets/Maintenance.
- Build measurement structure and plausibility before AI automation.
- Keep AI narrow and supportive, not workflow-defining.
- Delay customer intake, RFID, GPS tracking, and deep CAD/CNC automation.

## Highest Priority Requirements

- `ARCH-01`, `ARCH-02`, `ARCH-04`
- `PROJ-10` to `PROJ-14`, `PROJ-18`, `PROJ-19`
- `RPT-12`
- `INV-11` to `INV-16`
- `AST-11`, `AST-12`, `AST-14`, `AST-15`
- `MEAS-10`, `MEAS-14`

## Strong Near-Term Candidates

1. Project model + project feed completion
2. Project-linked material/time analytics
3. Expiry-aware inventory and replenishment
4. Asset maintenance and project-linked reservations
5. Measurement foundation with plausibility checks

## Deferred by Design

- `PROJ-21`
- `AST-18`
- `RPT-13`
- `MFG-11`
- `MEAS-11` to `MEAS-13`, `MEAS-15`
- `COM-11`, `AI-11`, `FB-11`
- `FIN-11`, `FIN-12`
- `PRIV-10` implementation work beyond policy/guardrails
