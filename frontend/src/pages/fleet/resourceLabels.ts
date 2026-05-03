export function getVehicleTypeLabel(vehicleType: string): string {
  const labels: Record<string, string> = {
    car: "PKW",
    van: "Transporter",
    truck: "LKW",
    trailer: "Anhänger",
    other: "Sonstige",
  }

  return labels[vehicleType] ?? vehicleType
}

export function formatReservationDateTime(dateString: string): string {
  return new Date(dateString).toLocaleDateString("de-DE", {
    day: "2-digit",
    month: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  })
}
