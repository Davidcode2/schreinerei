import type { Vehicle, VehicleType, ResourceStatus } from '@/types/fleet';

let vehicleCounter = 0;

export function createVehicle(overrides: Partial<Vehicle> = {}): Vehicle {
  vehicleCounter++;
  return {
    id: crypto.randomUUID(),
    name: `Fahrzeug ${vehicleCounter}`,
    license_plate: `B-AB ${1000 + vehicleCounter}`,
    vehicle_type: 'van' as VehicleType,
    description: null,
    status: 'available' as ResourceStatus,
    location: 'Hof',
    qr_code: null,
    display_color: '#2563eb',
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    ...overrides,
  };
}
