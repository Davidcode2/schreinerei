import { http, HttpResponse, delay } from 'msw';

const API_BASE = '/api/v1';
const apiRoute = (path: string) => `*${API_BASE}${path}`;
type MockRecord = Record<string, unknown>;

// Mock data stores (can be modified in tests)
export const mockData = {
  materials: [] as MockRecord[],
  categories: [] as MockRecord[],
  sites: [] as MockRecord[],
  vehicles: [] as MockRecord[],
  tools: [] as MockRecord[],
  reservations: [] as MockRecord[],
  timeEntries: [] as MockRecord[],
};

export const handlers = [
  // Materials (inventory module)
  http.get(apiRoute('/inventory/materials'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.materials);
  }),

  http.post(apiRoute('/inventory/materials'), async ({ request }) => {
    const body = await request.json() as Record<string, unknown>;
    const newMaterial = {
      id: crypto.randomUUID(),
      created_at: new Date().toISOString(),
      is_low_stock: false,
      qr_code: null,
      description: null,
      ...body,
    };
    mockData.materials.push(newMaterial);
    return HttpResponse.json(newMaterial, { status: 201 });
  }),

  // Categories (inventory module)
  http.get(apiRoute('/inventory/categories'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.categories);
  }),

  http.post(apiRoute('/inventory/categories'), async ({ request }) => {
    const body = await request.json() as MockRecord;
    const newCategory = {
      id: crypto.randomUUID(),
      name: body.name,
      description: body.description || null,
      created_at: new Date().toISOString(),
    };
    mockData.categories.push(newCategory);
    return HttpResponse.json(newCategory, { status: 201 });
  }),

  // Sites
  http.get(apiRoute('/sites'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.sites);
  }),

  http.post(apiRoute('/sites'), async ({ request }) => {
    const body = await request.json() as Record<string, unknown>;
    const newSite = {
      id: crypto.randomUUID(),
      status: 'planned',
      description: null,
      location: null,
      start_date: null,
      end_date: null,
      estimated_days: null,
      created_at: new Date().toISOString(),
      ...body,
    };
    mockData.sites.push(newSite);
    return HttpResponse.json(newSite, { status: 201 });
  }),

  // Vehicles (fleet module)
  http.get(apiRoute('/fleet/vehicles'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.vehicles);
  }),

  http.post(apiRoute('/fleet/vehicles'), async ({ request }) => {
    const body = await request.json() as Record<string, unknown>;
    const newVehicle = {
      id: crypto.randomUUID(),
      license_plate: null,
      description: null,
      status: 'available',
      location: null,
      qr_code: null,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      ...body,
    };
    mockData.vehicles.push(newVehicle);
    return HttpResponse.json(newVehicle, { status: 201 });
  }),

  // Tools (fleet module)
  http.get(apiRoute('/fleet/tools'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.tools);
  }),

  http.post(apiRoute('/fleet/tools'), async ({ request }) => {
    const body = await request.json() as Record<string, unknown>;
    const newTool = {
      id: crypto.randomUUID(),
      category: null,
      description: null,
      status: 'available',
      location: null,
      qr_code: null,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      ...body,
    };
    mockData.tools.push(newTool);
    return HttpResponse.json(newTool, { status: 201 });
  }),

  // Reservations (fleet module)
  http.get(apiRoute('/fleet/reservations'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.reservations);
  }),

  http.post(apiRoute('/fleet/reservations'), async ({ request }) => {
    const body = await request.json() as Record<string, unknown>;
    const newReservation = {
      id: crypto.randomUUID(),
      site_id: null,
      site_name: null,
      notes: null,
      status: 'confirmed',
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      ...body,
    };
    mockData.reservations.push(newReservation);
    return HttpResponse.json(newReservation, { status: 201 });
  }),

  // Time Entries
  http.get(apiRoute('/time-entries'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.timeEntries);
  }),

  http.get(apiRoute('/time-entries/my'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.timeEntries);
  }),

  http.post(apiRoute('/time-entries'), async ({ request }) => {
    const body = await request.json() as Record<string, unknown>;
    const newTimeEntry = {
      id: crypto.randomUUID(),
      site_id: null,
      notes: null,
      created_at: new Date().toISOString(),
      ...body,
    };
    mockData.timeEntries.push(newTimeEntry);
    return HttpResponse.json(newTimeEntry, { status: 201 });
  }),
];
