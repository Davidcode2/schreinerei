import userEvent from "@testing-library/user-event"
import { toast } from "sonner"
import { beforeEach, describe, expect, it, vi } from "vitest"

import { render, screen, waitFor } from "@/test/utils"
import { UserManagementSection } from "./UserManagementSection"

const useUsersMock = vi.fn()
const usePendingInvitesMock = vi.fn()
const updateUserRoleMock = vi.fn()
const deleteUserMock = vi.fn()

vi.mock("@/lib/api/hooks", () => ({
  useUsers: () => useUsersMock(),
  usePendingInvites: () => usePendingInvitesMock(),
  useUpdateUserRole: () => ({ mutate: updateUserRoleMock, isPending: false }),
  useDeleteUser: () => ({ mutate: deleteUserMock, isPending: false }),
}))

vi.mock("sonner", async () => {
  const actual = await vi.importActual<typeof import("sonner")>("sonner")
  return {
    ...actual,
    toast: { ...actual.toast, success: vi.fn(), error: vi.fn() },
  }
})

vi.mock("@/lib/auth/authStore", () => ({
  useAuthStore: (selector: (state: { isAuthenticated: boolean }) => unknown) =>
    selector({ isAuthenticated: true }),
}))

vi.mock("@/components/settings/InviteUserDialog", () => ({
  InviteUserDialog: () => <div data-testid="invite-user-dialog" />,
}))

describe("UserManagementSection", () => {
  beforeEach(() => {
    vi.clearAllMocks()
    useUsersMock.mockReturnValue({
      data: [
        {
          id: "user-1",
          email: "ada@example.com",
          name: "Ada Admin",
          role: "admin",
          is_original_admin: true,
          can_manage: false,
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

  it("renders edit actions for every user and disables protected users", () => {
    useUsersMock.mockReturnValue({
      data: [
        {
          id: "user-1",
          email: "ada@example.com",
          name: "Ada Admin",
          role: "admin",
          is_original_admin: true,
          can_manage: false,
          created_at: "2026-05-24T10:00:00.000Z",
        },
        {
          id: "user-2",
          email: "max@example.com",
          name: "Max Mitarbeiter",
          role: "employee",
          is_original_admin: false,
          can_manage: true,
          created_at: "2026-05-25T10:00:00.000Z",
        },
      ],
      isLoading: false,
      error: null,
    })

    render(<UserManagementSection isAdmin={true} />)

    expect(screen.getByRole("button", { name: "Ada Admin kann nicht bearbeitet werden" })).toBeDisabled()
    expect(screen.getByRole("button", { name: "Max Mitarbeiter bearbeiten" })).toBeEnabled()
  })

  it("updates a manageable user's role and closes the dialog on success", async () => {
    const user = userEvent.setup()
    useManageableUser()
    updateUserRoleMock.mockImplementation((_input, options) => options.onSuccess())

    render(<UserManagementSection isAdmin={true} />)

    await user.click(screen.getByRole("button", { name: "Max Mitarbeiter bearbeiten" }))
    const adminRole = screen.getByRole("radio", { name: "Admin" })
    await user.click(adminRole)
    expect(adminRole).toBeChecked()
    expect(screen.getByRole("button", { name: "Änderungen speichern" })).toBeEnabled()
    await user.click(screen.getByRole("button", { name: "Änderungen speichern" }))

    expect(updateUserRoleMock).toHaveBeenCalledWith(
      { id: "user-2", role: "admin" },
      expect.any(Object)
    )
    expect(toast.success).toHaveBeenCalledWith("Benutzerrolle aktualisiert")
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument()
  })

  it("keeps the edit dialog open and toasts when a role update fails", async () => {
    const user = userEvent.setup()
    useManageableUser()
    updateUserRoleMock.mockImplementation((_input, options) =>
      options.onError(new Error("Role update failed"))
    )

    render(<UserManagementSection isAdmin={true} />)

    await user.click(screen.getByRole("button", { name: "Max Mitarbeiter bearbeiten" }))
    await user.click(screen.getByRole("radio", { name: "Admin" }))
    await user.click(screen.getByRole("button", { name: "Änderungen speichern" }))

    expect(toast.error).toHaveBeenCalledWith("Role update failed")
    expect(screen.getByRole("dialog")).toBeInTheDocument()
  })

  it("requires confirmation before deleting and closes both dialogs on success", async () => {
    const user = userEvent.setup()
    useManageableUser()
    deleteUserMock.mockImplementation((_id, options) => options.onSuccess())

    render(<UserManagementSection isAdmin={true} />)

    await user.click(screen.getByRole("button", { name: "Max Mitarbeiter bearbeiten" }))
    await user.click(screen.getByRole("button", { name: "Benutzer löschen" }))

    expect(screen.getByRole("alertdialog")).toBeInTheDocument()
    expect(deleteUserMock).not.toHaveBeenCalled()

    await user.click(screen.getByRole("button", { name: /^löschen$/i }))

    expect(deleteUserMock).toHaveBeenCalledWith("user-2", expect.any(Object))
    expect(toast.success).toHaveBeenCalledWith("Benutzer gelöscht")
    expect(screen.queryByRole("alertdialog")).not.toBeInTheDocument()
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument()
  })

  it("keeps dialogs open and toasts when deletion fails", async () => {
    const user = userEvent.setup()
    useManageableUser()
    deleteUserMock.mockImplementation((_id, options) =>
      options.onError(new Error("Original admin is protected"))
    )

    render(<UserManagementSection isAdmin={true} />)

    await user.click(screen.getByRole("button", { name: "Max Mitarbeiter bearbeiten" }))
    await user.click(screen.getByRole("button", { name: "Benutzer löschen" }))
    await user.click(screen.getByRole("button", { name: /^löschen$/i }))

    await waitFor(() => {
      expect(toast.error).toHaveBeenCalledWith("Original admin is protected")
    })
    expect(screen.getByRole("alertdialog")).toBeInTheDocument()
    expect(screen.getByRole("dialog", { hidden: true })).toBeInTheDocument()
  })
})

function useManageableUser() {
  useUsersMock.mockReturnValue({
    data: [
      {
        id: "user-2",
        email: "max@example.com",
        name: "Max Mitarbeiter",
        role: "employee",
        is_original_admin: false,
        can_manage: true,
        created_at: "2026-05-25T10:00:00.000Z",
      },
    ],
    isLoading: false,
    error: null,
  })
}
