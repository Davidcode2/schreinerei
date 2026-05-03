import { describe, expect, it, vi } from "vitest"
import userEvent from "@testing-library/user-event"
import { render, screen } from "@/test/utils"
import FleetPage from "./FleetPage"

const reservationDialogMock = vi.fn<(props: unknown) => void>()

vi.mock("./CalendarView", () => ({
  default: ({ embedded }: { embedded?: boolean }) => (
    <div data-testid="fleet-calendar">{embedded ? "embedded-calendar" : "standalone-calendar"}</div>
  ),
}))

vi.mock("./VehiclesList", () => ({
  VehiclesList: ({ onReserve }: { onReserve: (id: string, type: "vehicle" | "tool") => void }) => (
    <div data-testid="vehicles-list">
      <button type="button" onClick={() => onReserve("vehicle-1", "vehicle")}>
        reserve-vehicle
      </button>
    </div>
  ),
}))

vi.mock("./ToolsList", () => ({
  ToolsList: () => <div data-testid="tools-list">tools-list</div>,
}))

vi.mock("./ReservationsList", () => ({
  ReservationsList: () => <div data-testid="reservations-list">reservations-list</div>,
}))

vi.mock("./ReservationDialog", () => ({
  ReservationDialog: (props: unknown) => {
    reservationDialogMock(props)
    return null
  },
}))

vi.mock("./AddVehicleDialog", () => ({
  AddVehicleDialog: () => null,
}))

vi.mock("./AddToolDialog", () => ({
  AddToolDialog: () => null,
}))

describe("FleetPage", () => {
  it("renders the vehicle calendar section above the vehicle list", () => {
    reservationDialogMock.mockClear()
    render(<FleetPage />)

    const calendarSection = screen.getByTestId("fleet-calendar")
    const pageHeading = screen.getByRole("heading", { name: "Fuhrpark" })
    const vehiclesList = screen.getByTestId("vehicles-list")

    expect(calendarSection).toHaveTextContent("embedded-calendar")
    expect(pageHeading).toBeInTheDocument()
    expect(calendarSection.compareDocumentPosition(vehiclesList)).toBe(Node.DOCUMENT_POSITION_FOLLOWING)
  })

  it("removes the old standalone calendar call-to-action", () => {
    reservationDialogMock.mockClear()
    render(<FleetPage />)

    expect(screen.queryByRole("button", { name: /kalenderansicht öffnen/i })).not.toBeInTheDocument()
  })

  it("keeps the list-based reservation dialog path available", async () => {
    reservationDialogMock.mockClear()
    const user = userEvent.setup()

    render(<FleetPage />)
    await user.click(screen.getByRole("button", { name: "reserve-vehicle" }))

    expect(reservationDialogMock).toHaveBeenLastCalledWith(
      expect.objectContaining({
        open: true,
        resourceId: "vehicle-1",
        resourceType: "vehicle",
      })
    )
  })
})
