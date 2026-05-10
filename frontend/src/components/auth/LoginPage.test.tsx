import { describe, expect, it } from "vitest"
import { screen } from "@testing-library/react"
import { render } from "@/test/utils"
import { LoginPage } from "./LoginPage"

describe("LoginPage", () => {
  it("shows the self-service signup entry point", () => {
    render(<LoginPage />)

    const signupLink = screen.getByRole("link", {
      name: /neue organisation erstellen/i,
    })

    expect(signupLink).toHaveAttribute("href", "/signup")
  })
})
