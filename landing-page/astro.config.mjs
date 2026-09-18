import { defineConfig } from "astro/config"
import tailwindcss from "@tailwindcss/vite"

export default defineConfig({
  site: "https://schreinerei-app.jakob-lingel.dev",
  vite: {
    plugins: [tailwindcss()],
  },
})
