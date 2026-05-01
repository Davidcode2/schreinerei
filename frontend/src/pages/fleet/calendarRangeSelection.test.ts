import { describe, expect, it } from "vitest"
import {
  advanceRangeSelection,
  createPendingRangeSelection,
} from "./calendarRangeSelection"

describe("calendarRangeSelection", () => {
  it("creates a pending selection from the first tap", () => {
    expect(
      createPendingRangeSelection({
        resourceId: "vehicle-1",
        resourceType: "vehicle",
        date: "2026-05-06",
      })
    ).toEqual({
      resourceId: "vehicle-1",
      resourceType: "vehicle",
      firstDate: "2026-05-06",
    })
  })

  it("completes a same-resource range on the second tap", () => {
    const pendingSelection = createPendingRangeSelection({
      resourceId: "vehicle-1",
      resourceType: "vehicle",
      date: "2026-05-06",
    })

    expect(
      advanceRangeSelection(pendingSelection, {
        resourceId: "vehicle-1",
        resourceType: "vehicle",
        date: "2026-05-08",
      })
    ).toEqual({
      completedSelection: {
        resourceId: "vehicle-1",
        resourceType: "vehicle",
        startDate: "2026-05-06",
        endDate: "2026-05-08",
      },
    })
  })

  it("allows same-day bookings by completing the same date twice", () => {
    const pendingSelection = createPendingRangeSelection({
      resourceId: "vehicle-1",
      resourceType: "vehicle",
      date: "2026-05-06",
    })

    expect(
      advanceRangeSelection(pendingSelection, {
        resourceId: "vehicle-1",
        resourceType: "vehicle",
        date: "2026-05-06",
      })
    ).toEqual({
      completedSelection: {
        resourceId: "vehicle-1",
        resourceType: "vehicle",
        startDate: "2026-05-06",
        endDate: "2026-05-06",
      },
    })
  })

  it("normalizes reverse-order taps so the earlier date is the start", () => {
    const pendingSelection = createPendingRangeSelection({
      resourceId: "vehicle-1",
      resourceType: "vehicle",
      date: "2026-05-08",
    })

    expect(
      advanceRangeSelection(pendingSelection, {
        resourceId: "vehicle-1",
        resourceType: "vehicle",
        date: "2026-05-06",
      })
    ).toEqual({
      completedSelection: {
        resourceId: "vehicle-1",
        resourceType: "vehicle",
        startDate: "2026-05-06",
        endDate: "2026-05-08",
      },
    })
  })

  it("starts a new pending selection instead of creating a cross-resource range", () => {
    const pendingSelection = createPendingRangeSelection({
      resourceId: "vehicle-1",
      resourceType: "vehicle",
      date: "2026-05-06",
    })

    expect(
      advanceRangeSelection(pendingSelection, {
        resourceId: "tool-1",
        resourceType: "tool",
        date: "2026-05-07",
      })
    ).toEqual({
      pendingSelection: {
        resourceId: "tool-1",
        resourceType: "tool",
        firstDate: "2026-05-07",
      },
    })
  })
})
