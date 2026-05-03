import type { CalendarEntry, ReservationSummary, ResourceStatus } from "@/types/fleet"

function buildResourceKey(resourceType: string, resourceId: string): string {
  return `${resourceType}:${resourceId}`
}

function hasCurrentInUseReservation(reservations: ReservationSummary[], now: Date): boolean {
  return reservations.some((reservation) => {
    if (reservation.status !== "in_use") {
      return false
    }

    const start = new Date(reservation.start_time)
    const end = new Date(reservation.end_time)
    return start <= now && end >= now
  })
}

function hasUpcomingReservationToday(reservations: ReservationSummary[], now: Date): boolean {
  return reservations.some((reservation) => {
    if (!["pending", "confirmed", "in_use"].includes(reservation.status)) {
      return false
    }

    const end = new Date(reservation.end_time)
    return end >= now
  })
}

export function buildEffectiveStatusMap(
  resources: CalendarEntry[] | undefined,
  now: Date
): Map<string, ResourceStatus> {
  const statusMap = new Map<string, ResourceStatus>()

  for (const resource of resources ?? []) {
    const key = buildResourceKey(resource.resource_type, resource.resource_id)

    if (hasCurrentInUseReservation(resource.reservations, now)) {
      statusMap.set(key, "in_use")
      continue
    }

    if (hasUpcomingReservationToday(resource.reservations, now)) {
      statusMap.set(key, "reserved")
    }
  }

  return statusMap
}

export function getEffectiveResourceStatus(
  currentStatus: ResourceStatus,
  statusMap: Map<string, ResourceStatus>,
  resourceType: "vehicle" | "tool",
  resourceId: string
): ResourceStatus {
  if (currentStatus !== "available") {
    return currentStatus
  }

  return statusMap.get(buildResourceKey(resourceType, resourceId)) ?? currentStatus
}
