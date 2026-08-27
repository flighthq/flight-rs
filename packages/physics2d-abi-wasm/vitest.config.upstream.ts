import path from 'node:path';

import { defineConfig, type Plugin } from 'vitest/config';

const upstreamPackages = path.resolve(import.meta.dirname, '../../upstream/packages');
const upstreamSource = path.join(upstreamPackages, 'physics2d-abi/src');
const facade = path.resolve(import.meta.dirname, 'src/index.ts');

function substituteWasmBackend(): Plugin {
  return {
    name: 'physics2d-abi-wasm-upstream-conformance',
    enforce: 'pre',
    resolveId(source, importer) {
      if (source !== './physics2DAbi' || importer === undefined) return null;
      if (!importer.startsWith(upstreamSource) || !importer.endsWith('.test.ts')) return null;
      return facade;
    },
  };
}

export default defineConfig({
  root: import.meta.dirname,
  plugins: [substituteWasmBackend()],
  resolve: {
    alias: [
      { find: /^@flighthq\/([^/]+)$/u, replacement: `${upstreamPackages}/$1/src/index.ts` },
      { find: /^@flighthq\/([^/]+)\/(.+)$/u, replacement: `${upstreamPackages}/$1/src/$2` },
    ],
  },
  test: {
    environment: 'node',
    globals: true,
    include: ['../../upstream/packages/physics2d-abi/src/**/*.test.ts'],
  },
});
