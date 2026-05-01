import type { ResourceType } from "@/types/fleet"

export interface PendingRangeSelection {
  resourceId: string
  resourceType: ResourceType
  firstDate: string
}

export interface CompletedRangeSelection {
  resourceId: string
  resourceType: ResourceType
  startDate: string
  endDate: string
}

interface SelectionTap {
  resourceId: string
  resourceType: ResourceType
  date: string
}

export function createPendingRangeSelection({
  resourceId,
  resourceType,
  date,
}: SelectionTap): PendingRangeSelection {
  return {
    resourceId,
    resourceType,
    firstDate: date,
  }
}

export function advanceRangeSelection(
  pendingSelection: PendingRangeSelection | null,
  tap: SelectionTap
):
  | { pendingSelection: PendingRangeSelection }
  | { completedSelection: CompletedRangeSelection } {
  if (
    !pendingSelection ||
    pendingSelection.resourceId !== tap.resourceId ||
    pendingSelection.resourceType !== tap.resourceType
  ) {
    return {
      pendingSelection: createPendingRangeSelection(tap),
    }
  }

  const [startDate, endDate] = [pendingSelection.firstDate, tap.date].toSorted()

  return {
    completedSelection: {
      resourceId: tap.resourceId,
      resourceType: tap.resourceType,
      startDate,
      endDate,
    },
  }
}
