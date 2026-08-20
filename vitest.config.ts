import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json-summary'],
    },
    globals: true,
    include: ['tests/**/*.test.ts'],
    // The inventory test analyzes all 143 upstream packages through the TypeScript AST, which takes
    // ~28s uninstrumented — 92% of a 30s budget, so the suite was one slow machine away from
    // flaking. Under v8 coverage the same analysis takes ~100s. This ceiling covers the instrumented
    // path with room to spare; it is a bound on a genuinely long analysis, not a hidden slow test.
    testTimeout: 180_000,
  },
});
