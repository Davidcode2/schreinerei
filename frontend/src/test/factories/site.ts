import type { ProjectType, Site, SiteStatus } from '@/types/sites';

let siteCounter = 0;

export function createSite(overrides: Partial<Site> = {}): Site {
  siteCounter++;
  return {
    id: crypto.randomUUID(),
    project_type: 'external_site' as ProjectType,
    name: `Baustelle ${siteCounter}`,
    customer_name: `Kunde ${siteCounter}`,
    location: `Musterstraße ${siteCounter}, 12345 Berlin`,
    description: null,
    status: 'active' as SiteStatus,
    start_date: null,
    end_date: null,
    estimated_days: null,
    budget_amount_cents: null,
    billing_reference: null,
    billing_notes: null,
    quote_reference: null,
    created_at: new Date().toISOString(),
    ...overrides,
  };
}
