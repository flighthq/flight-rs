import { defineFacadeRuntimeSlot } from '@flighthq/runtime-rs';

export interface SurfaceRsRuntime {
  initialized: boolean;
}

export const surfaceRsRuntime = defineFacadeRuntimeSlot<SurfaceRsRuntime>('@flighthq/surface-rs', () => ({
  initialized: false,
}));
