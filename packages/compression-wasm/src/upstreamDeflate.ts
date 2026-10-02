// What upstream's `deflate.test.ts` sees in place of `./deflate.ts` when the conformance lane runs.
//
// A lane shim exports exactly the names the upstream module it stands in for exports, and nothing more. The
// restraint is the point: if a shim exported a superset, a test importing a name upstream had REMOVED would
// still resolve, and the lane would pass on an API that no longer exists.
export { decompressDeflate, sdkHostDecompressDeflate } from './compressionWasm';
