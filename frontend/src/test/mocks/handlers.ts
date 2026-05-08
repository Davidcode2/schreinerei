import { http, HttpResponse, delay } from 'msw';

const API_BASE = '/api/v1';
const apiRoute = (path: string) => `*${API_BASE}${path}`;
type MockRecord = Record<string, unknown>;

// Mock data stores (can be modified in tests)
export const mockData = {
  materials: [] as MockRecord[],
  categories: [] as MockRecord[],
  sites: [] as MockRecord[],
  users: [] as MockRecord[],
  preferences: { active_site_id: null as string | null },
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
      can_expire: false,
      legacy_quantity: body.quantity ?? 0,
      expired_quantity: 0,
      expiring_soon_quantity: 0,
      next_expiry_on: null,
      expiry_batches: [],
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
      can_expire: Boolean(body.can_expire),
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

  http.get(apiRoute('/sites/:id'), async ({ params }) => {
    await delay(10);
    const site = mockData.sites.find((entry) => entry.id === params.id);
    return site
      ? HttpResponse.json(site)
      : HttpResponse.json({ error: 'Not found' }, { status: 404 });
  }),

  http.post(apiRoute('/sites'), async ({ request }) => {
    const body = await request.json() as Record<string, unknown>;
    const newSite = {
      id: crypto.randomUUID(),
      project_type: 'external_site',
      status: 'planned',
      description: null,
      location: null,
      start_date: null,
      end_date: null,
      estimated_days: null,
      budget_amount_cents: null,
      billing_reference: null,
      billing_notes: null,
      quote_reference: null,
      created_at: new Date().toISOString(),
      ...body,
    };
    mockData.sites.push(newSite);
    return HttpResponse.json(newSite, { status: 201 });
  }),

  http.get(apiRoute('/users'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.users);
  }),

  http.get(apiRoute('/preferences'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.preferences);
  }),

  http.patch(apiRoute('/preferences'), async ({ request }) => {
    const body = await request.json() as Record<string, unknown>;
    mockData.preferences = {
      ...mockData.preferences,
      active_site_id: (body.active_site_id as string | null | undefined) ?? null,
    };
    return HttpResponse.json(mockData.preferences);
  }),

  http.patch(apiRoute('/sites/:id'), async ({ params, request }) => {
    const body = await request.json() as Record<string, unknown>;
    const index = mockData.sites.findIndex((entry) => entry.id === params.id);
    if (index === -1) {
      return HttpResponse.json({ error: 'Not found' }, { status: 404 });
    }
    mockData.sites[index] = { ...mockData.sites[index], ...body };
    return HttpResponse.json(mockData.sites[index]);
  }),

  http.get(apiRoute('/sites/:id/assignments'), async () => {
    await delay(10);
    return HttpResponse.json([]);
  }),

  http.get(apiRoute('/sites/history-report'), async () => {
    await delay(10);
    return HttpResponse.json([]);
  }),

  http.get(apiRoute('/sites/:id/summary'), async () => {
    await delay(10);
    return HttpResponse.json({
      labor: {
        total_hours: 0,
        entry_count: 0,
        site_hours: 0,
        workshop_hours: 0,
        last_work_date: null,
      },
      materials: {
        distinct_material_count: 0,
        withdrawal_count: 0,
        lines: [],
      },
    });
  }),

  http.post(apiRoute('/sites/:id/assign'), async ({ params, request }) => {
    const body = await request.json() as Record<string, unknown>;
    return HttpResponse.json(
      {
        id: crypto.randomUUID(),
        site_id: params.id,
        created_at: new Date().toISOString(),
        ...body,
      },
      { status: 200 }
    );
  }),

  http.delete(apiRoute('/sites/:id/assign/:userId'), async () => {
    return HttpResponse.json({ success: true }, { status: 200 });
  }),

  http.get(apiRoute('/sites/:id/time-entries'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.timeEntries);
  }),

  http.get(apiRoute('/sites/:id/activities'), async () => {
    await delay(10);
    return HttpResponse.json([]);
  }),

  http.get(apiRoute('/dashboard/sites'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.sites.map((site) => ({
      assigned_users: 0,
      total_hours: 0,
      ...site,
    })));
  }),

  http.get(apiRoute('/inventory/low-stock'), async () => {
    await delay(10);
    return HttpResponse.json([]);
  }),

  http.get(apiRoute('/inventory/alerts'), async () => {
    await delay(10);
    return HttpResponse.json(
      mockData.materials.filter((entry) => {
        const expired = Number(entry.expired_quantity ?? 0);
        const soon = Number(entry.expiring_soon_quantity ?? 0);
        return expired > 0 || soon > 0;
      })
    );
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

  http.get(apiRoute('/fleet/reservations/my'), async () => {
    await delay(10);
    return HttpResponse.json(mockData.reservations);
  }),

  http.get(apiRoute('/fleet/reservations/:id'), async ({ params }) => {
    await delay(10);
    const reservation = mockData.reservations.find((entry) => entry.id === params.id);
    return reservation
      ? HttpResponse.json(reservation)
      : HttpResponse.json({ error: 'Not found' }, { status: 404 });
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
