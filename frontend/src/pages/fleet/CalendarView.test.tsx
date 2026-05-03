import { beforeEach, describe, expect, it, vi } from "vitest"
import userEvent from "@testing-library/user-event"
import { fireEvent, render, screen, waitFor } from "@/test/utils"
import CalendarView from "./CalendarView"

vi.mock("@/lib/api/hooks", () => ({
  useCalendar: vi.fn(),
  useCreateReservation: vi.fn(),
  usePreferences: vi.fn(),
  useSites: vi.fn(),
}))

import {
  useCalendar,
  useCreateReservation,
  usePreferences,
  useSites,
} from "@/lib/api/hooks"
import { getResourceCalendarColor } from "./resourceCalendarColor"

const mutateAsyncMock = vi.fn()

function weekDateIso(offsetDays: number, hour: number) {
  const now = new Date()
  now.setHours(12, 0, 0, 0)

  const weekStart = new Date(now)
  weekStart.setDate(weekStart.getDate() - weekStart.getDay() + 1)
  weekStart.setDate(weekStart.getDate() + offsetDays)
  weekStart.setHours(hour, 0, 0, 0)

  return weekStart.toISOString()
}

function getCalendarButtons() {
  return screen
    .getAllByRole("button")
    .filter((button) => button.getAttribute("aria-label")?.startsWith("Sprinter am "))
}

function requireElement<T>(value: T | undefined): T {
  expect(value).toBeDefined()
  return value as T
}

function setCalendarData(resources: Array<{
  resource_type: "vehicle" | "tool"
  resource_id: string
  resource_name: string
  reservations: Array<{
    id: string
    start_time: string
    end_time: string
    user_name: string
    site_name: string | null
    status: "pending" | "confirmed" | "in_use" | "completed" | "cancelled"
  }>
}>) {
  vi.mocked(useCalendar).mockReturnValue({
    data: { resources },
    isLoading: false,
    error: null,
  } as never)
}

describe("CalendarView", () => {
  beforeEach(() => {
    mutateAsyncMock.mockReset()
    mutateAsyncMock.mockResolvedValue({ id: "reservation-1" })

    vi.mocked(useCreateReservation).mockReturnValue({
      mutateAsync: mutateAsyncMock,
      isPending: false,
    } as never)
    vi.mocked(usePreferences).mockReturnValue({
      data: { active_site_id: "site-1" },
    } as never)
    vi.mocked(useSites).mockReturnValue({
      data: [{ id: "site-1", name: "Baustelle Nord" }],
    } as never)
    setCalendarData([
      {
        resource_type: "vehicle",
        resource_id: "vehicle-1",
        resource_name: "Sprinter",
        reservations: [],
      },
    ])
  })

  it("marks the first empty-day tap as pending without opening confirmation", async () => {
    const user = userEvent.setup()
    setCalendarData([
      {
        resource_type: "vehicle",
        resource_id: "vehicle-1",
        resource_name: "Sprinter",
        reservations: [
          {
            id: "existing-1",
            start_time: weekDateIso(0, 8),
            end_time: weekDateIso(0, 17),
            user_name: "Alex",
            site_name: null,
            status: "confirmed",
          },
        ],
      },
    ])
    render(<CalendarView embedded />)

    const [dayButton] = getCalendarButtons()

    await user.click(requireElement(dayButton))

    expect(dayButton).toHaveAttribute("data-selection-state", "pending")
    expect(screen.queryByTestId("reservation-confirmation-sheet")).not.toBeInTheDocument()
    expect(screen.getByText("Alex")).toBeInTheDocument()
  })

  it("keeps existing reservation chips visible while a new selection is in progress", async () => {
    const user = userEvent.setup()
    setCalendarData([
      {
        resource_type: "vehicle",
        resource_id: "vehicle-1",
        resource_name: "Sprinter",
        reservations: [
          {
            id: "existing-1",
            start_time: weekDateIso(0, 8),
            end_time: weekDateIso(0, 17),
            user_name: "Alex",
            site_name: null,
            status: "confirmed",
          },
        ],
      },
      {
        resource_type: "tool",
        resource_id: "tool-1",
        resource_name: "Bohrhammer",
        reservations: [],
      },
    ])

    render(<CalendarView embedded />)

    const toolButtons = screen
      .getAllByRole("button")
      .filter((button) => button.getAttribute("aria-label")?.startsWith("Bohrhammer am "))

    await user.click(requireElement(toolButtons[0]))
    expect(screen.getByText("Alex")).toBeInTheDocument()

    await user.click(requireElement(toolButtons[2]))
    expect(await screen.findByTestId("reservation-confirmation-sheet")).toBeInTheDocument()
    expect(screen.getByText("Alex")).toBeInTheDocument()
  })

  it("keeps the same derived resource color markers across rerenders", () => {
    setCalendarData([
      {
        resource_type: "vehicle",
        resource_id: "vehicle-1",
        resource_name: "Sprinter",
        reservations: [
          {
            id: "existing-1",
            start_time: weekDateIso(0, 8),
            end_time: weekDateIso(0, 17),
            user_name: "Alex",
            site_name: null,
            status: "confirmed",
          },
        ],
      },
    ])

    const expectedColor = getResourceCalendarColor("vehicle", "vehicle-1").token
    const { rerender } = render(<CalendarView embedded />)

    const rowHeader = screen.getByText("Sprinter").closest("div[data-resource-color]")
    const reservationChip = screen.getByText("Alex").closest("div[data-resource-color]")

    expect(rowHeader).toHaveAttribute("data-resource-color", expectedColor)
    expect(reservationChip).toHaveAttribute("data-resource-color", expectedColor)

    rerender(<CalendarView embedded />)

    const rerenderedRowHeader = screen.getByText("Sprinter").closest("div[data-resource-color]")
    const rerenderedReservationChip = screen.getByText("Alex").closest("div[data-resource-color]")

    expect(rerenderedRowHeader).toHaveAttribute("data-resource-color", expectedColor)
    expect(rerenderedReservationChip).toHaveAttribute("data-resource-color", expectedColor)
  })

  it("opens confirmation with a sorted date range on the second tap", async () => {
    const user = userEvent.setup()
    render(<CalendarView embedded />)

    const buttons = getCalendarButtons()
    const laterButton = buttons[4]
    const earlierButton = buttons[2]

    await user.click(requireElement(laterButton))
    await user.click(requireElement(earlierButton))

    expect(await screen.findByTestId("reservation-confirmation-sheet")).toBeInTheDocument()
    expect(screen.getAllByText(/bis/).length).toBeGreaterThan(0)
  })

  it("supports same-day double tap selections", async () => {
    const user = userEvent.setup()
    render(<CalendarView embedded />)

    const [dayButton] = getCalendarButtons()

    await user.click(requireElement(dayButton))
    await user.click(requireElement(dayButton))

    expect(await screen.findByTestId("reservation-confirmation-sheet")).toBeInTheDocument()
    expect(screen.queryByText(/bis/)).not.toBeInTheDocument()
  })

  it("clears the selection when the user cancels confirmation", async () => {
    const user = userEvent.setup()
    render(<CalendarView embedded />)

    const buttons = getCalendarButtons()
    const startButton = buttons[1]
    const endButton = buttons[3]

    await user.click(requireElement(startButton))
    await user.click(requireElement(endButton))
    await user.click(screen.getByRole("button", { name: /abbrechen/i }))

    await waitFor(() => {
      expect(screen.queryByTestId("reservation-confirmation-sheet")).not.toBeInTheDocument()
    })
    expect(startButton).toHaveAttribute("data-selection-state", "idle")
    expect(endButton).toHaveAttribute("data-selection-state", "idle")
  })

  it("hides custom time inputs until the user enables them", async () => {
    const user = userEvent.setup()
    render(<CalendarView embedded />)

    const buttons = getCalendarButtons()
    const startButton = buttons[1]
    const endButton = buttons[3]

    await user.click(requireElement(startButton))
    await user.click(requireElement(endButton))

    const toggle = screen.getByRole("checkbox", { name: /zeitangaben anpassen/i })

    expect(document.querySelectorAll('input[type="datetime-local"]').length).toBe(0)
    fireEvent.click(toggle)

    await waitFor(() => {
      expect(toggle).toBeChecked()
      expect(document.querySelectorAll('input[type="datetime-local"]').length).toBe(2)
    })
  })

  it("submits a reservation with sorted start and end datetimes", async () => {
    const user = userEvent.setup()
    render(<CalendarView embedded />)

    const buttons = getCalendarButtons()
    const laterButton = buttons[4]
    const earlierButton = buttons[2]

    await user.click(requireElement(laterButton))
    await user.click(requireElement(earlierButton))
    await user.click(screen.getByRole("button", { name: /reservierung bestaetigen/i }))

    await waitFor(() => {
      const payload = mutateAsyncMock.mock.calls[0]?.[0]

      expect(payload).toBeDefined()
      if (!payload) {
        return
      }

      expect(mutateAsyncMock).toHaveBeenCalledWith({
        resource_id: "vehicle-1",
        resource_type: "vehicle",
        site_id: "site-1",
        start_time: payload.start_time,
        end_time: payload.end_time,
      })
      expect(new Date(payload.start_time).getTime()).toBeLessThan(new Date(payload.end_time).getTime())
    })
  })
})
