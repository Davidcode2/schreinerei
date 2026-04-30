import { Page } from '@playwright/test';

interface TestResources {
  materials: string[];
  sites: string[];
  vehicles: string[];
  tools: string[];
  timeEntries: string[];
  reservations: string[];
  categories: string[];
}

export function useCleanup(page: Page) {
  const resources: TestResources = {
    materials: [],
    sites: [],
    vehicles: [],
    tools: [],
    timeEntries: [],
    reservations: [],
    categories: [],
  };

  const cleanup = async () => {
    for (const id of resources.reservations) {
      await page.request
        .delete(`/api/v1/fleet/reservations/${id}`)
        .catch(() => {});
    }
    for (const id of resources.timeEntries) {
      await page.request
        .delete(`/api/v1/time-entries/${id}`)
        .catch(() => {});
    }
    for (const id of resources.materials) {
      await page.request
        .delete(`/api/v1/inventory/materials/${id}`)
        .catch(() => {});
    }
    for (const id of resources.sites) {
      await page.request.delete(`/api/v1/sites/${id}`).catch(() => {});
    }
    for (const id of resources.vehicles) {
      await page.request
        .delete(`/api/v1/fleet/vehicles/${id}`)
        .catch(() => {});
    }
    for (const id of resources.tools) {
      await page.request.delete(`/api/v1/fleet/tools/${id}`).catch(() => {});
    }
    for (const id of resources.categories) {
      await page.request
        .delete(`/api/v1/inventory/categories/${id}`)
        .catch(() => {});
    }
    resources.materials = [];
    resources.sites = [];
    resources.vehicles = [];
    resources.tools = [];
    resources.timeEntries = [];
    resources.reservations = [];
    resources.categories = [];
  };

  const track = {
    material: (id: string) => resources.materials.push(id),
    site: (id: string) => resources.sites.push(id),
    vehicle: (id: string) => resources.vehicles.push(id),
    tool: (id: string) => resources.tools.push(id),
    timeEntry: (id: string) => resources.timeEntries.push(id),
    reservation: (id: string) => resources.reservations.push(id),
    category: (id: string) => resources.categories.push(id),
  };

  return { cleanup, track };
}

let counter = 0;
export function uniqueName(prefix: string): string {
  return `${prefix}-e2e-${Date.now()}-${++counter}`;
}
