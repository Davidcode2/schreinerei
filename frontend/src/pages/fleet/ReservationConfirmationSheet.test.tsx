import { describe, expect, it } from "vitest"
import { buildDefaultTimes } from "./ReservationConfirmationSheet"

describe("buildDefaultTimes", () => {
  it("moves same-day quick bookings into the future", () => {
    const now = new Date(2026, 4, 3, 14, 10, 0)

    const defaults = buildDefaultTimes("2026-05-03", "2026-05-03", now)

    expect(defaults.start).toBe("2026-05-03T14:30")
    expect(defaults.end).toBe("2026-05-03T17:00")
  })

  it("extends the end time when the workday default has already passed", () => {
    const now = new Date(2026, 4, 3, 18, 20, 0)

    const defaults = buildDefaultTimes("2026-05-03", "2026-05-03", now)

    expect(defaults.start).toBe("2026-05-03T18:30")
    expect(defaults.end).toBe("2026-05-03T19:30")
  })
})
