import { describe, expect, it } from "vitest"
import {
  getResourceCalendarColor,
  resourceCalendarColorPalette,
} from "./resourceCalendarColor"

describe("getResourceCalendarColor", () => {
  it("returns the same token for the same resource identity", () => {
    const firstColor = getResourceCalendarColor("vehicle", "resource-1")
    const secondColor = getResourceCalendarColor("vehicle", "resource-1")

    expect(firstColor).toEqual(secondColor)
  })

  it("stays stable regardless of resource row order", () => {
    const resources = [
      { resourceType: "vehicle" as const, resourceId: "vehicle-1" },
      { resourceType: "tool" as const, resourceId: "tool-1" },
      { resourceType: "vehicle" as const, resourceId: "vehicle-2" },
    ]

    const orderedColors = new Map(
      resources.map((resource) => [
        `${resource.resourceType}:${resource.resourceId}`,
        getResourceCalendarColor(resource.resourceType, resource.resourceId).token,
      ])
    )

    const reversedColors = new Map(
      [...resources].reverse().map((resource) => [
        `${resource.resourceType}:${resource.resourceId}`,
        getResourceCalendarColor(resource.resourceType, resource.resourceId).token,
      ])
    )

    expect(reversedColors).toEqual(orderedColors)
  })

  it("maps different identities into the approved palette", () => {
    const tokens = new Set(resourceCalendarColorPalette.map((color) => color.token))
    const vehicleColor = getResourceCalendarColor("vehicle", "vehicle-1")
    const toolColor = getResourceCalendarColor("tool", "tool-99")

    expect(tokens.has(vehicleColor.token)).toBe(true)
    expect(tokens.has(toolColor.token)).toBe(true)
  })
})
