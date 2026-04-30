import type { Tool, ResourceStatus } from '@/types/fleet';

let toolCounter = 0;

export function createTool(overrides: Partial<Tool> = {}): Tool {
  toolCounter++;
  return {
    id: crypto.randomUUID(),
    name: `Werkzeug ${toolCounter}`,
    category: 'Handwerkzeug',
    description: null,
    status: 'available' as ResourceStatus,
    location: 'Werkstatt',
    qr_code: null,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    ...overrides,
  };
}
