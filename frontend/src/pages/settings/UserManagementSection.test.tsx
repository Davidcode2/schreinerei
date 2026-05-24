import { describe, expect, it, vi, beforeEach } from "vitest"

import { render, screen } from "@/test/utils"
import { UserManagementSection } from "./UserManagementSection"

const useUsersMock = vi.fn()
const usePendingInvitesMock = vi.fn()

vi.mock("@/lib/api/hooks", () => ({
  useUsers: () => useUsersMock(),
  usePendingInvites: () => usePendingInvitesMock(),
}))

vi.mock("@/lib/auth/authStore", () => ({
  useAuthStore: (selector: (state: { isAuthenticated: boolean }) => unknown) =>
    selector({ isAuthenticated: true }),
}))

vi.mock("@/components/settings/InviteUserDialog", () => ({
  InviteUserDialog: () => <div data-testid="invite-user-dialog" />,
}))

describe("UserManagementSection", () => {
  beforeEach(() => {
    useUsersMock.mockReturnValue({
      data: [
        {
          id: "user-1",
          email: "ada@example.com",
          name: "Ada Admin",
          role: "admin",
          created_at: "2026-05-24T10:00:00.000Z",
        },
      ],
      isLoading: false,
      error: null,
    })
    usePendingInvitesMock.mockReturnValue({
      data: [],
      isLoading: false,
      error: null,
    })
  })

  it("renders pending invites above the active user list", () => {
    usePendingInvitesMock.mockReturnValue({
      data: [
        {
          id: "invite-1",
          email: "new.user@example.com",
          role: "employee",
          status: "pending",
          expires_at: "2026-05-31T12:00:00.000Z",
          created_at: "2026-05-24T10:00:00.000Z",
        },
      ],
      isLoading: false,
      error: null,
    })

    render(<UserManagementSection isAdmin={true} />)

    expect(screen.getByText("Ausstehende Einladungen")).toBeInTheDocument()
    expect(screen.getByText("new.user@example.com")).toBeInTheDocument()
    expect(screen.getByText("Ada Admin")).toBeInTheDocument()
    expect(screen.getByText("Ausstehend")).toBeInTheDocument()
    expect(screen.getByText("1 offen")).toBeInTheDocument()
  })
})
