import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

function padTwoDigits(value: number) {
  return value.toString().padStart(2, "0")
}

export function formatLocalDateKey(date: Date): string {
  return [
    date.getFullYear(),
    padTwoDigits(date.getMonth() + 1),
    padTwoDigits(date.getDate()),
  ].join("-")
}

export function formatDateTimeLocalInput(dateLike: string | Date): string {
  const date = typeof dateLike === "string" ? new Date(dateLike) : dateLike

  return `${formatLocalDateKey(date)}T${padTwoDigits(date.getHours())}:${padTwoDigits(date.getMinutes())}`
}

export function startOfLocalWeek(date: Date): Date {
  const start = new Date(date)
  const mondayOffset = (start.getDay() + 6) % 7

  start.setHours(0, 0, 0, 0)
  start.setDate(start.getDate() - mondayOffset)

  return start
}
