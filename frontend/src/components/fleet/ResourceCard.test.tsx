import { describe, expect, it, vi } from "vitest"

import { render, screen } from "@/test/utils"
import { createVehicle } from "@/test/factories"
import { ResourceCard } from "./ResourceCard"

vi.mock("@/lib/api/hooks", async () => {
  const actual = await vi.importActual<typeof import("@/lib/api/hooks")>("@/lib/api/hooks")
  return {
    ...actual,
    useDeleteVehicle: () => ({ mutate: vi.fn(), isPending: false }),
    useDeleteTool: () => ({ mutate: vi.fn(), isPending: false }),
  }
})

const onReserve = vi.fn()

describe("ResourceCard", () => {
  it("shows qr codes up to 20 characters in full", () => {
    const vehicle = createVehicle({ qr_code: "fleet-van-01" })

    render(<ResourceCard resource={vehicle} type="vehicle" onReserve={onReserve} />)

    expect(screen.getByText("fleet-van-01")).toBeInTheDocument()
  })

  it("truncates long qr codes to 20 characters including the ellipsis", () => {
    const vehicle = createVehicle({ qr_code: "qr-vehicle-0123456789-abcd" })

    render(<ResourceCard resource={vehicle} type="vehicle" onReserve={onReserve} />)

    expect(screen.getByText("qr-vehicle-012345...")).toBeInTheDocument()
    expect(screen.queryByText(vehicle.qr_code!)).not.toBeInTheDocument()
  })

  it("hides the qr badge when the vehicle has no qr code", () => {
    const vehicle = createVehicle({ qr_code: null })

    render(<ResourceCard resource={vehicle} type="vehicle" onReserve={onReserve} />)

    expect(screen.queryByTestId("qr-code-label")).not.toBeInTheDocument()
  })
})
