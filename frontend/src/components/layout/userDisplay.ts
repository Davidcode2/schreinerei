export function getDisplayName(name: string | null, email: string | undefined): string {
  return name?.trim() || email || "Benutzer"
}

export function getRoleLabel(role: string | undefined): string {
  return role === "admin" ? "Admin" : "Mitarbeiter"
}
