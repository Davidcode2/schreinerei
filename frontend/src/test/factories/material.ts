import type { Material, Category } from '@/types/inventory';
import { createCategory } from './category';

let materialCounter = 0;

export function createMaterial(overrides: Partial<Material> = {}): Material {
  materialCounter++;
  const category = createCategory();
  return {
    id: crypto.randomUUID(),
    category_id: category.id,
    name: `Material ${materialCounter}`,
    description: null,
    unit: 'Stück',
    quantity: 100,
    min_quantity: 10,
    location: 'Regal A1',
    qr_code: null,
    is_low_stock: false,
    created_at: new Date().toISOString(),
    ...overrides,
  };
}

export function createMaterialWithCategory(
  category: Category,
  overrides: Partial<Material> = {}
): Material {
  return createMaterial({ category_id: category.id, ...overrides });
}
