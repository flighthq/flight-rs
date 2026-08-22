declare module '@flighthq/types' {
  export type Physics3DAbiWorldHandle = number;
  export type Physics3DAbiWorldStatus = 'Busy' | 'Ready' | 'Stale';
  export type Physics3DAbiExecutionStatus =
    | 'BusyWorld'
    | 'Complete'
    | 'InvalidBuffer'
    | 'InvalidCommand'
    | 'MissingBody'
    | 'MissingCollider'
    | 'MissingJoint'
    | 'RejectedMutation'
    | 'StaleWorld'
    | 'UnsupportedJoint'
    | 'UnsupportedShape';
  export type Physics3DAbiStepStatus = 'BusyWorld' | 'Complete' | 'Declined' | 'InsufficientHookBuffer' | 'StaleWorld';
  export type Physics3DAbiContactSelection = 'All' | 'Began' | 'Ended';
  export interface Physics3DAbiExecutionResult {
    status: Physics3DAbiExecutionStatus;
    commandIndex: number;
    byteOffset: number;
    commandKind: number;
  }
  export interface Physics3DAbiCommandBuffer {
    readonly data: Uint8Array<ArrayBufferLike>;
    byteLength: number;
    commandCount: number;
  }
  export interface Physics3DAbiBodyBuffer {
    readonly ids: Uint32Array<ArrayBufferLike>;
    readonly flags: Uint32Array<ArrayBufferLike>;
    readonly values: Float64Array<ArrayBufferLike>;
    count: number;
    requiredCount: number;
  }
  export interface Physics3DAbiContactBuffer {
    readonly ids: Uint32Array<ArrayBufferLike>;
    readonly flags: Uint32Array<ArrayBufferLike>;
    readonly pointStarts: Uint32Array<ArrayBufferLike>;
    readonly pointCounts: Uint32Array<ArrayBufferLike>;
    readonly values: Float64Array<ArrayBufferLike>;
    readonly pointFeatureIds: Uint32Array<ArrayBufferLike>;
    readonly pointValues: Float64Array<ArrayBufferLike>;
    count: number;
    pointCount: number;
    requiredCount: number;
    requiredPointCount: number;
  }
  export interface Physics3DAbiContactHooks {
    readonly buffer: Physics3DAbiContactBuffer;
    readonly preSolve: ((contact: Physics3DAbiContactBuffer) => void) | null;
    readonly postSolve: ((contact: Physics3DAbiContactBuffer) => void) | null;
  }
  export interface Physics3DAbiJointBuffer {
    readonly ids: Uint32Array<ArrayBufferLike>;
    readonly flags: Uint32Array<ArrayBufferLike>;
    readonly values: Float64Array<ArrayBufferLike>;
    count: number;
    requiredCount: number;
  }
  export interface Physics3DAbiQueryBuffer {
    readonly bodyIds: Uint32Array<ArrayBufferLike>;
    readonly colliderIds: Uint32Array<ArrayBufferLike>;
    readonly values: Float64Array<ArrayBufferLike>;
    count: number;
    requiredCount: number;
  }
  export interface Physics3DQueryFilter {
    readonly [name: string]: unknown;
  }
  export interface SpatialAabb3D {
    readonly minX: number;
    readonly minY: number;
    readonly minZ: number;
    readonly maxX: number;
    readonly maxY: number;
    readonly maxZ: number;
  }
  export interface CollisionBuiltInShape3D {
    readonly kind: string;
    readonly [name: string]: unknown;
  }
  export interface Physics3DAbi {
    readonly version: number;
    readonly capabilities: number;
    createWorld(): Physics3DAbiWorldHandle;
    destroyWorld(world: Physics3DAbiWorldHandle): boolean;
    getWorldStatus(world: Physics3DAbiWorldHandle): Physics3DAbiWorldStatus;
    execute(
      world: Physics3DAbiWorldHandle,
      commands: Readonly<Physics3DAbiCommandBuffer>,
      out: Physics3DAbiExecutionResult,
    ): boolean;
    step(
      world: Physics3DAbiWorldHandle,
      dt: number,
      hooks: Readonly<Physics3DAbiContactHooks> | null,
    ): Physics3DAbiStepStatus;
    readBodies(
      world: Physics3DAbiWorldHandle,
      bodyIds: Readonly<Uint32Array<ArrayBufferLike>> | null,
      out: Physics3DAbiBodyBuffer,
    ): boolean;
    readContacts(
      world: Physics3DAbiWorldHandle,
      selection: Physics3DAbiContactSelection,
      out: Physics3DAbiContactBuffer,
    ): boolean;
    readJoints(world: Physics3DAbiWorldHandle, out: Physics3DAbiJointBuffer): boolean;
    queryPoint(
      world: Physics3DAbiWorldHandle,
      x: number,
      y: number,
      z: number,
      filter: Readonly<Physics3DQueryFilter> | null,
      out: Physics3DAbiQueryBuffer,
    ): boolean;
    queryRay(
      world: Physics3DAbiWorldHandle,
      originX: number,
      originY: number,
      originZ: number,
      directionX: number,
      directionY: number,
      directionZ: number,
      maxFraction: number,
      closestOnly: boolean,
      filter: Readonly<Physics3DQueryFilter> | null,
      out: Physics3DAbiQueryBuffer,
    ): boolean;
    queryRegion(
      world: Physics3DAbiWorldHandle,
      region: Readonly<SpatialAabb3D>,
      filter: Readonly<Physics3DQueryFilter> | null,
      out: Physics3DAbiQueryBuffer,
    ): boolean;
    queryShapeCast(
      world: Physics3DAbiWorldHandle,
      shape: Readonly<CollisionBuiltInShape3D>,
      dx: number,
      dy: number,
      dz: number,
      maxFraction: number,
      filter: Readonly<Physics3DQueryFilter> | null,
      out: Physics3DAbiQueryBuffer,
    ): boolean;
  }
}

declare module '@flighthq/physics3d-abi' {
  import type { Physics3DAbi } from '@flighthq/types';
  export function createPhysics3DAbi(): Physics3DAbi;
}
