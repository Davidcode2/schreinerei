import { describe, expect, it, vi, beforeEach } from "vitest"
import userEvent from "@testing-library/user-event"

import { render, screen } from "@/test/utils"
import { InviteUserDialog } from "./InviteUserDialog"

const mutateAsync = vi.fn()
const reset = vi.fn()

vi.mock("@/lib/api/hooks", () => ({
  useInviteUser: () => ({
    mutateAsync,
    reset,
    isPending: false,
  }),
}))

describe("InviteUserDialog", () => {
  beforeEach(() => {
    mutateAsync.mockReset()
    reset.mockReset()
  })

  it("shows Keycloak email delivery copy after invite creation", async () => {
    const user = userEvent.setup()
    mutateAsync.mockResolvedValue({
      id: "invite-1",
      email: "new.user@example.com",
      role: "employee",
      status: "pending",
      invite_url: "http://localhost:5173/signup?invite=invite-1",
      organization_alias: "tenant-1",
      expires_at: "2026-05-31T12:00:00.000Z",
    })

    render(<InviteUserDialog open={true} onOpenChange={vi.fn()} />)

    await user.type(screen.getByLabelText(/e-mail-adresse/i), "new.user@example.com")
    await user.click(screen.getByRole("button", { name: /einladung erstellen/i }))

    expect(await screen.findByText("Einladung erstellt")).toBeInTheDocument()
    expect(
      screen.getByText(/keycloak versendet die einladung direkt an/i)
    ).toBeInTheDocument()
    expect(screen.queryByDisplayValue(/signup\?invite=/i)).not.toBeInTheDocument()
  })
})
