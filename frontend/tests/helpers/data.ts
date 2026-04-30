import { Page } from '@playwright/test';

interface TestResources {
  materials: string[];
  sites: string[];
  vehicles: string[];
  tools: string[];
}

export function useCleanup(page: Page) {
  const resources: TestResources = {
    materials: [],
    sites: [],
    vehicles: [],
    tools: [],
  };

  const cleanup = async () => {
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
    resources.materials = [];
    resources.sites = [];
    resources.vehicles = [];
    resources.tools = [];
  };

  const track = {
    material: (id: string) => resources.materials.push(id),
    site: (id: string) => resources.sites.push(id),
    vehicle: (id: string) => resources.vehicles.push(id),
    tool: (id: string) => resources.tools.push(id),
  };

  return { cleanup, track };
}

let counter = 0;
export function uniqueName(prefix: string): string {
  return `${prefix}-e2e-${Date.now()}-${++counter}`;
}
