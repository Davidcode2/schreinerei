---
name: access-schreinerei-frontend
description: Helps an agent open, authenticate, and inspect the Schreinerei frontend using `.agents/local/frontend-credentials.env` when working on UI flows, screenshots, or manual verification in this project.
---

<objective>
Open the local Schreinerei frontend reliably, authenticate through Keycloak, and navigate to the dashboard fast enough to support manual checks, screenshots, and UI debugging.
</objective>

<quick_start>
Use this skill when you need the running frontend quickly.

Project-specific starting points:
- Frontend URL: `http://localhost:5175`
- Login page: `http://localhost:5175/login`
- Dashboard route after auth: `http://localhost:5175/`
- Frontend env file: `frontend/.env`
- Local credentials file: `.agents/local/frontend-credentials.env`

Useful existing repo material:
- `frontend/tests/helpers/auth.ts`: current Playwright login flow for Keycloak
- `frontend/playwright.config.ts`: assumes frontend base URL `http://localhost:5175`
- `frontend/src/lib/auth/keycloak.ts`: PKCE login, callback, token refresh, logout URLs
- `frontend/src/App.tsx`: route map for public and protected pages
</quick_start>

<process>
1. Confirm the local frontend and backend are up.

   Frontend expectations:
   - `http://localhost:5175` serves the Vite app.
   - The app redirects unauthenticated users to `/login`.

   Backend expectations:
   - The checked-in `frontend/.env` currently points `VITE_API_URL` to `http://localhost:3009`.
   - The old `frontend/tests/README.md` still mentions port `3000`; treat that as stale documentation.

2. Use the repo-local credential file instead of pasting credentials into prompts or scripts.

   Source it in shell before one-off automation:

   ```bash
   set -a
   source .agents/local/frontend-credentials.env
   set +a
   ```

3. Start auth from the app login page, not from Keycloak directly.

   Current flow learned from the live app and source:
   - Visit `http://localhost:5175/login`
   - Click `Mit Keycloak anmelden`
   - Keycloak opens at `https://auth.jakob-lingel.dev/...`
   - Step 1: fill `Username or email`
   - Click `Sign In`
   - Step 2: fill `Password`
   - Click `Sign In` again
   - App redirects back to `http://localhost:5175/auth/callback`, then to `/`

4. For automation, prefer reusing the repo's existing login assumptions before inventing a new flow.

   Current automation anchors:
   - Login button text in app: `Mit Keycloak anmelden`
   - Keycloak username selector: `input#username` / `#username`
   - Keycloak password selector: `input#password` / `#password`
   - Submit button selector: `button:has-text("Sign In")` or `#kc-login`
   - Auth persistence key in local storage: `auth-storage`
   - PKCE state lives in session storage during login

5. Once authenticated, use the dashboard as the primary verification landing page.

   Dashboard characteristics:
   - Page heading: `Dashboard`
   - Route: `/`
   - Desktop shows a fixed left sidebar
   - Mobile hides sidebar and uses a top bar with a hamburger menu

6. Know the responsive navigation split before exploring.

   Desktop (`frontend/src/components/layout/DesktopSidebar.tsx`):
   - Left sidebar contains navigation and account/logout area
   - QR action button is in the sidebar footer

   Mobile (`frontend/src/components/layout/MobileNav.tsx`):
   - Top bar contains menu button, active site indicator, sync controls, QR button
   - Main navigation is inside the left sheet opened by the hamburger button

7. When you need screenshots, capture both form factors separately.

   Verified working output paths from this session:
   - Desktop dashboard screenshot: `/tmp/opencode/schreinerei-dashboard-desktop.png`
   - Mobile dashboard screenshot: `/tmp/opencode/schreinerei-dashboard-mobile.png`

8. If login automation fails, check the real page before changing code.

   Fast checks:
   - Confirm the app still serves `/login`
   - Confirm Keycloak still uses the two-step username-then-password flow
   - Confirm redirect URI still returns to `localhost:5175`
   - Confirm `frontend/.env` still points at a working backend
   - Compare live behavior against `frontend/tests/helpers/auth.ts`
</process>

<learned_behavior>
Learned during this session:
- No existing project-local skill for frontend access was present in `.agents/skills/` or `.claude/skills/`.
- The repo already had relevant access knowledge in `frontend/tests/helpers/auth.ts`, but it is embedded inside the e2e helper rather than documented as a reusable local skill.
- The frontend and backend were already running locally in this workspace.
- Live auth currently works against hosted Keycloak at `https://auth.jakob-lingel.dev`.
- The dashboard is a good post-login smoke target because it renders differently on desktop and mobile and confirms both auth and core layout.
</learned_behavior>

<related_tools>
Global skills that were useful while building this skill:
- `create-skill`: project-local skill structure and authoring rules
- `playwright-cli`: browser automation and page inspection
- `agent-browser`: alternative browser automation patterns
</related_tools>

<success_criteria>
This skill is successful when an agent can:
- Find the correct local frontend URL without searching the repo again
- Use the ignored credentials file instead of retyping credentials
- Reproduce the current Keycloak login flow
- Land on the dashboard after auth
- Understand the first-level desktop/mobile navigation split
- Capture or verify the dashboard on both desktop and mobile
</success_criteria>
