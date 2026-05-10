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
    const tokens = new Set<string>(resourceCalendarColorPalette.map((color) => color.token))
    const vehicleColor = getResourceCalendarColor("vehicle", "vehicle-1")
    const toolColor = getResourceCalendarColor("tool", "tool-99")

    expect(tokens.has(vehicleColor.token)).toBe(true)
    expect(tokens.has(toolColor.token)).toBe(true)
  })

  it("uses a persisted display color when the API provides a valid hex value", () => {
    const color = getResourceCalendarColor("vehicle", "vehicle-1", "#2F6F8F")

    expect(color.token).toBe("#2f6f8f")
    expect(color.markerStyle).toEqual({ backgroundColor: "#2f6f8f" })
    expect(color.labelStyle).toEqual({ color: "#2f6f8f" })
  })

  it("falls back to the palette when the API display color is invalid", () => {
    const fallbackColor = getResourceCalendarColor("vehicle", "vehicle-1")
    const invalidColor = getResourceCalendarColor("vehicle", "vehicle-1", "blue")

    expect(invalidColor).toEqual(fallbackColor)
  })
})
