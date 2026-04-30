import type { Site, SiteStatus } from '@/types/sites';

let siteCounter = 0;

export function createSite(overrides: Partial<Site> = {}): Site {
  siteCounter++;
  return {
    id: crypto.randomUUID(),
    name: `Baustelle ${siteCounter}`,
    customer_name: `Kunde ${siteCounter}`,
    location: `Musterstraße ${siteCounter}, 12345 Berlin`,
    description: null,
    status: 'active' as SiteStatus,
    start_date: null,
    end_date: null,
    estimated_days: null,
    created_at: new Date().toISOString(),
    ...overrides,
  };
}
