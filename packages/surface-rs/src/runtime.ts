import { defineFacadeWasmModuleSlot } from '@flighthq/runtime-rs';

export interface SurfaceRsRuntime {
  initialized: boolean;
}

export const surfaceRsRuntime = defineFacadeWasmModuleSlot<SurfaceRsRuntime>('@flighthq/surface-rs', () => ({
  initialized: false,
}));
