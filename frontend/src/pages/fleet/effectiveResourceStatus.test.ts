import { describe, expect, it } from "vitest"
import { buildEffectiveStatusMap, getEffectiveResourceStatus } from "./effectiveResourceStatus"
import type { CalendarEntry, ResourceStatus } from "@/types/fleet"

function resourceWithReservations(reservations: CalendarEntry["reservations"]): CalendarEntry {
  return {
    resource_type: "vehicle",
    resource_id: "vehicle-1",
    resource_name: "Sprinter",
    resource_display_color: "#2563eb",
    reservations,
  }
}

describe("effectiveResourceStatus", () => {
  it("marks resources with future same-day reservations as reserved", () => {
    const now = new Date("2026-05-03T10:00:00.000Z")
    const map = buildEffectiveStatusMap([
      resourceWithReservations([
        {
          id: "reservation-1",
          start_time: "2026-05-03T12:00:00.000Z",
          end_time: "2026-05-03T15:00:00.000Z",
          user_name: "Alex",
          site_id: null,
          site_name: null,
          status: "confirmed",
        },
      ]),
    ], now)

    expect(getEffectiveResourceStatus("available", map, "vehicle", "vehicle-1")).toBe("reserved")
  })

  it("marks resources with active in-use reservations as in_use", () => {
    const now = new Date("2026-05-03T10:00:00.000Z")
    const map = buildEffectiveStatusMap([
      resourceWithReservations([
        {
          id: "reservation-1",
          start_time: "2026-05-03T09:00:00.000Z",
          end_time: "2026-05-03T11:00:00.000Z",
          user_name: "Alex",
          site_id: null,
          site_name: null,
          status: "in_use",
        },
      ]),
    ], now)

    expect(getEffectiveResourceStatus("available", map, "vehicle", "vehicle-1")).toBe("in_use")
  })

  it("preserves non-available persisted statuses", () => {
    const map = new Map<string, ResourceStatus>([["vehicle:vehicle-1", "reserved"]])

    expect(getEffectiveResourceStatus("maintenance", map, "vehicle", "vehicle-1")).toBe("maintenance")
  })
})
