import { describe, expect, it } from "vitest"
import { truncateQrCode } from "./truncateQrCode"

describe("truncateQrCode", () => {
  it("returns codes up to 20 characters unchanged", () => {
    expect(truncateQrCode("short-code")).toBe("short-code")
    expect(truncateQrCode("12345678901234567890")).toBe("12345678901234567890")
  })

  it("truncates longer codes to 20 characters including the ellipsis", () => {
    expect(truncateQrCode("1234567890123456789012345")).toBe("12345678901234567...")
  })

  it("never returns more than 20 characters", () => {
    const result = truncateQrCode("x".repeat(200))
    expect(result.length).toBe(20)
    expect(result.endsWith("...")).toBe(true)
  })
})
