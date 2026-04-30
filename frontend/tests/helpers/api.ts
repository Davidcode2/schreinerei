import { Page } from '@playwright/test';

interface MaterialData {
  category_id: string;
  name: string;
  description?: string;
  unit: string;
  quantity: number;
  min_quantity: number;
  location?: string;
}

interface MaterialResponse {
  id: string;
  category_id: string;
  name: string;
  description?: string;
  unit: string;
  quantity: number;
  min_quantity: number;
  location?: string;
  qr_code?: string;
  is_low_stock: boolean;
  created_at: string;
}

interface SiteData {
  name: string;
  customer_name: string;
  location?: string;
  description?: string;
  start_date?: string;
  end_date?: string;
  estimated_days?: number;
}

interface SiteResponse {
  id: string;
  name: string;
  customer_name: string;
  location?: string;
  description?: string;
  status: string;
  start_date?: string;
  end_date?: string;
  estimated_days?: number;
  created_at: string;
}

interface VehicleData {
  name: string;
  license_plate?: string;
  vehicle_type: string;
  description?: string;
  location?: string;
  qr_code?: string;
}

interface VehicleResponse {
  id: string;
  name: string;
  license_plate?: string;
  vehicle_type: string;
  description?: string;
  status: string;
  location?: string;
  qr_code?: string;
  created_at: string;
  updated_at: string;
}

interface ToolData {
  name: string;
  category?: string;
  description?: string;
  location?: string;
  qr_code?: string;
}

interface ToolResponse {
  id: string;
  name: string;
  category?: string;
  description?: string;
  status: string;
  location?: string;
  qr_code?: string;
  created_at: string;
  updated_at: string;
}

interface CategoryResponse {
  id: string;
  name: string;
  description?: string;
  created_at: string;
}

export async function createMaterial(
  page: Page,
  data: MaterialData
): Promise<MaterialResponse> {
  const response = await page.request.post('/api/v1/inventory/materials', {
    data,
  });
  if (!response.ok()) {
    throw new Error(`Failed to create material: ${response.status()}`);
  }
  return response.json();
}

export async function getMaterial(page: Page, id: string): Promise<MaterialResponse> {
  const response = await page.request.get(`/api/v1/inventory/materials/${id}`);
  return response.json();
}

export async function listMaterials(page: Page): Promise<MaterialResponse[]> {
  const response = await page.request.get('/api/v1/inventory/materials');
  return response.json();
}

export async function deleteMaterial(page: Page, id: string): Promise<void> {
  await page.request.delete(`/api/v1/inventory/materials/${id}`).catch(() => {});
}

export async function createSite(page: Page, data: SiteData): Promise<SiteResponse> {
  const response = await page.request.post('/api/v1/sites', { data });
  if (!response.ok()) {
    throw new Error(`Failed to create site: ${response.status()}`);
  }
  return response.json();
}

export async function getSite(page: Page, id: string): Promise<SiteResponse> {
  const response = await page.request.get(`/api/v1/sites/${id}`);
  return response.json();
}

export async function listSites(page: Page): Promise<SiteResponse[]> {
  const response = await page.request.get('/api/v1/sites');
  return response.json();
}

export async function deleteSite(page: Page, id: string): Promise<void> {
  await page.request.delete(`/api/v1/sites/${id}`).catch(() => {});
}

export async function createVehicle(
  page: Page,
  data: VehicleData
): Promise<VehicleResponse> {
  const response = await page.request.post('/api/v1/fleet/vehicles', { data });
  if (!response.ok()) {
    throw new Error(`Failed to create vehicle: ${response.status()}`);
  }
  return response.json();
}

export async function getVehicle(page: Page, id: string): Promise<VehicleResponse> {
  const response = await page.request.get(`/api/v1/fleet/vehicles/${id}`);
  return response.json();
}

export async function listVehicles(page: Page): Promise<VehicleResponse[]> {
  const response = await page.request.get('/api/v1/fleet/vehicles');
  return response.json();
}

export async function deleteVehicle(page: Page, id: string): Promise<void> {
  await page.request.delete(`/api/v1/fleet/vehicles/${id}`).catch(() => {});
}

export async function createTool(page: Page, data: ToolData): Promise<ToolResponse> {
  const response = await page.request.post('/api/v1/fleet/tools', { data });
  if (!response.ok()) {
    throw new Error(`Failed to create tool: ${response.status()}`);
  }
  return response.json();
}

export async function listTools(page: Page): Promise<ToolResponse[]> {
  const response = await page.request.get('/api/v1/fleet/tools');
  return response.json();
}

export async function deleteTool(page: Page, id: string): Promise<void> {
  await page.request.delete(`/api/v1/fleet/tools/${id}`).catch(() => {});
}

export async function listCategories(page: Page): Promise<CategoryResponse[]> {
  const response = await page.request.get('/api/v1/inventory/categories');
  return response.json();
}

interface CategoryData {
  name: string;
  description?: string;
}

export async function createCategory(
  page: Page,
  data: CategoryData
): Promise<CategoryResponse> {
  const response = await page.request.post('/api/v1/inventory/categories', {
    data,
  });
  if (!response.ok()) {
    throw new Error(`Failed to create category: ${response.status()}`);
  }
  return response.json();
}
