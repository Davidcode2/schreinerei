# Schreinerei Landing Page

Standalone Astro landing page for the customer-facing marketing site.

## Stack

- Astro
- Tailwind CSS v4 via `@tailwindcss/vite`
- Shared visual direction from the main frontend:
  - `DM Sans`
  - `DM Serif Display`
  - violet primary palette

## Commands

```bash
npm install
npm run dev
npm run build
```

## App Links

The CTA buttons point to the existing frontend app.

- Default target: `http://localhost:5173`
- Override with `PUBLIC_APP_URL`

Example:

```bash
PUBLIC_APP_URL=https://app.example.de npm run dev
```

## Replace Before Go-Live

- `src/pages/impressum.astro` still contains legal placeholders for the business details.
- `public/screenshots/*.svg` are intentionally placeholders until real screenshots are captured.
