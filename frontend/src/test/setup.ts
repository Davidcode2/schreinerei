import '@testing-library/jest-dom';
import { server } from './mocks/server';
import { vi } from 'vitest';

// Polyfill for Radix UI components in jsdom
// Radix UI uses pointer events which are not fully supported in jsdom
class MockPointerEvent extends Event {
  pointerId: number;
  clientX: number;
  clientY: number;
  pointerType: string;
  isPrimary: boolean;

  constructor(type: string, props: PointerEventInit = {}) {
    super(type, props);
    this.pointerId = props.pointerId ?? 0;
    this.clientX = props.clientX ?? 0;
    this.clientY = props.clientY ?? 0;
    this.pointerType = props.pointerType ?? 'mouse';
    this.isPrimary = props.isPrimary ?? true;
  }
}

window.PointerEvent = MockPointerEvent as unknown as typeof PointerEvent;
window.HTMLElement.prototype.hasPointerCapture = () => false;
window.HTMLElement.prototype.setPointerCapture = () => {};
window.HTMLElement.prototype.releasePointerCapture = () => {};

// Polyfill scrollIntoView for Radix UI
window.HTMLElement.prototype.scrollIntoView = () => {};

// Polyfill ResizeObserver for Radix UI
class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}
window.ResizeObserver = ResizeObserverMock as unknown as typeof ResizeObserver;

// Mock matchMedia for responsive components
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

// Establish API mocking before all tests
beforeAll(() => server.listen({ onUnhandledRequest: 'error' }));

// Reset any request handlers that are declared during the tests
afterEach(() => server.resetHandlers());

// Clean up after the tests are finished
afterAll(() => server.close());
