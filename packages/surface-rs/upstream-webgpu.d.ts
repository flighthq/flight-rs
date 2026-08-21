// Ambient WebGPU names for `tsconfig.upstream.json` only.
//
// That configuration compiles this facade against the real `upstream/packages/*` sources, and
// importing `@flighthq/types` pulls in its whole index — including the `Wgpu*.ts` modules, which
// reference the browser's WebGPU globals. Without a declaration for those names the check drowns in
// ~150 TS2304 errors from files the facade never touches.
//
// The obvious fix is a `@webgpu/types` devDependency, which is what upstream's own `@flighthq/types`
// does. It is the wrong fix HERE: it makes an offline checkout unable to run `npm run check` at all
// (TS2688, "cannot find type definition file"), which is how this check broke for another agent
// working without registry access.
//
// So the names are declared locally. Nothing is lost: this check exists to prove the facade's
// imports still match upstream's declarations, and no WebGPU type is reachable from the bitmap
// surface it wraps. Typing them faithfully would add a dependency to describe values this program
// never inspects.
//
// The split below is load-bearing rather than cosmetic. Handles must be object types because
// upstream keys a `WeakMap` on one (`WeakMap<GPUShaderModule, …>` in WgpuQuadBatchResources), and a
// WeakMap key is constrained to `object` — declaring them `unknown` fails with TS2344. The format
// and preference names are string unions in the real API, so `string` is the honest widening.

// Opaque device handles.
declare type GPUBindGroup = object;
declare type GPUBindGroupLayout = object;
declare type GPUBuffer = object;
declare type GPUCanvasContext = object;
declare type GPUCommandEncoder = object;
declare type GPUDevice = object;
declare type GPUDeviceLostInfo = object;
declare type GPUPipelineLayout = object;
declare type GPURenderPassEncoder = object;
declare type GPURenderPipeline = object;
declare type GPUSampler = object;
declare type GPUShaderModule = object;
declare type GPUTexture = object;
declare type GPUTextureView = object;
declare type GPUVertexAttribute = object;
declare type GPUVertexBufferLayout = object;

// String unions in the real API.
declare type GPUIndexFormat = string;
declare type GPUPowerPreference = string;
declare type GPUTextureFormat = string;
