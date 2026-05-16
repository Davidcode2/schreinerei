## Local Mollie Testing

For local signup testing with Mollie, keep the frontend local and expose only the backend through a public HTTPS tunnel.

Current local setup:

- Backend runs on `http://localhost:3009`
- Frontend runs on `http://localhost:5175`
- Public backend tunnel: `https://elaborate-atom-clutter.ngrok-free.dev`

Required `.env` values for this setup:

```env
PORT=3009
MOLLIE_API_BASE_URL=https://api.mollie.com
APP_PUBLIC_URL=https://elaborate-atom-clutter.ngrok-free.dev
FRONTEND_PUBLIC_URL=http://localhost:5175
```

Why this works:

- Mollie must be able to reach the webhook URL from the public internet.
- The app generates the webhook URL from `APP_PUBLIC_URL`.
- The app generates the return URL from `FRONTEND_PUBLIC_URL`.
- This lets Mollie call the backend through ngrok while the browser still returns to the local frontend.

Typical workflow:

1. Start the backend on port `3009`.
2. Start the frontend on port `5175`.
3. Start ngrok for the backend: `ngrok http 3009`.
4. Put the ngrok HTTPS URL into `APP_PUBLIC_URL`.
5. Submit the signup form.

If Mollie returns `422 Unprocessable Entity` with `field=webhookUrl`, the webhook URL is still not publicly reachable.
