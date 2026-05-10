import { describe, expect, it } from "vitest"
import { screen } from "@testing-library/react"
import { render } from "@/test/utils"
import { SignupPage } from "./SignupPage"

describe("SignupPage", () => {
  it("renders the tenant onboarding form scaffold", () => {
    render(<SignupPage />)

    expect(screen.getByRole("heading", { name: /organisation erstellen/i })).toBeInTheDocument()
    expect(screen.getByLabelText(/organisationsname/i)).toBeDisabled()
    expect(screen.getByLabelText(/e-mail des admins/i)).toBeDisabled()
    expect(screen.getByRole("button", { name: /onboarding starten/i })).toBeDisabled()
  })
})
