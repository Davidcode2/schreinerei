# 23-01 Summary

- Added preferences hooks in `frontend/src/lib/api/hooks/usePreferences.ts` and exported them via `frontend/src/lib/api/hooks/index.ts`.
- Added deterministic active-site color utility in `frontend/src/lib/active-site/siteColor.ts`.
- Added persistent `ActiveSiteIndicator` in `frontend/src/components/layout/ActiveSiteIndicator.tsx`.
- Wired indicator into desktop and mobile shells:
  - `frontend/src/components/layout/DesktopSidebar.tsx`
  - `frontend/src/components/layout/MobileNav.tsx`

Verification:
- Ran `npm run build` in `frontend/`.
- Build currently fails due to unrelated pre-existing TypeScript/test setup issues outside this plan scope.
