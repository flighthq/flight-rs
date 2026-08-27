import path from 'node:path';

import { defineConfig, type Plugin } from 'vitest/config';

const upstreamPackages = path.resolve(import.meta.dirname, '../../upstream/packages');
const upstreamSource = path.join(upstreamPackages, 'physics3d-abi/src');
const facade = path.resolve(import.meta.dirname, 'src/index.ts');
const referenceFacade = path.resolve(import.meta.dirname, 'src/upstreamReferencePhysics3DAbi.ts');

function substituteWasmBackend(): Plugin {
  return {
    name: 'physics3d-abi-wasm-upstream-conformance',
    enforce: 'pre',
    resolveId(source, importer) {
      if (importer === undefined || !importer.startsWith(upstreamSource) || !importer.endsWith('.test.ts')) return null;
      if (source === './physics3DAbi') return facade;
      if (source === './referencePhysics3DAbi') return referenceFacade;
      return null;
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
    include: ['../../upstream/packages/physics3d-abi/src/**/*.test.ts'],
  },
});
