import type { Category } from '@/types/inventory';

let categoryCounter = 0;

export function createCategory(overrides: Partial<Category> = {}): Category {
  categoryCounter++;
  return {
    id: crypto.randomUUID(),
    name: `Category ${categoryCounter}`,
    description: null,
    created_at: new Date().toISOString(),
    ...overrides,
  };
}
