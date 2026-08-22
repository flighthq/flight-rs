declare module '@flighthq/types' {
  export type Physics2DAbiWorldHandle = number;
  export type Physics2DAbiWorldStatus = 'Busy' | 'Ready' | 'Stale';
  export type Physics2DAbiExecutionStatus =
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
  export type Physics2DAbiStepStatus = 'BusyWorld' | 'Complete' | 'Declined' | 'InsufficientHookBuffer' | 'StaleWorld';
  export type Physics2DAbiContactSelection = 'All' | 'Began' | 'Ended';
  export interface Physics2DAbiExecutionResult {
    status: Physics2DAbiExecutionStatus;
    commandIndex: number;
    byteOffset: number;
    commandKind: number;
  }
  export interface Physics2DAbiCommandBuffer {
    readonly data: Uint8Array<ArrayBufferLike>;
    byteLength: number;
    commandCount: number;
  }
  export interface Physics2DAbiBodyBuffer {
    readonly ids: Uint32Array<ArrayBufferLike>;
    readonly flags: Uint32Array<ArrayBufferLike>;
    readonly values: Float64Array<ArrayBufferLike>;
    count: number;
    requiredCount: number;
  }
  export interface Physics2DAbiContactBuffer {
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
  export interface Physics2DAbiContactHooks {
    readonly buffer: Physics2DAbiContactBuffer;
    readonly preSolve: ((contact: Physics2DAbiContactBuffer) => void) | null;
    readonly postSolve: ((contact: Physics2DAbiContactBuffer) => void) | null;
  }
  export interface Physics2DAbiJointBuffer {
    readonly ids: Uint32Array<ArrayBufferLike>;
    readonly flags: Uint32Array<ArrayBufferLike>;
    readonly values: Float64Array<ArrayBufferLike>;
    count: number;
    requiredCount: number;
  }
  export interface Physics2DAbiQueryBuffer {
    readonly bodyIds: Uint32Array<ArrayBufferLike>;
    readonly colliderIds: Uint32Array<ArrayBufferLike>;
    readonly values: Float64Array<ArrayBufferLike>;
    count: number;
    requiredCount: number;
  }
  export interface Physics2DQueryFilter {
    readonly [name: string]: unknown;
  }
  export interface SpatialAabb2D {
    readonly minX: number;
    readonly minY: number;
    readonly maxX: number;
    readonly maxY: number;
  }
  export interface CollisionBuiltInShape2D {
    readonly kind: string;
    readonly [name: string]: unknown;
  }
  export interface Physics2DAbi {
    readonly version: number;
    readonly capabilities: number;
    createWorld(): Physics2DAbiWorldHandle;
    destroyWorld(world: Physics2DAbiWorldHandle): boolean;
    getWorldStatus(world: Physics2DAbiWorldHandle): Physics2DAbiWorldStatus;
    execute(
      world: Physics2DAbiWorldHandle,
      commands: Readonly<Physics2DAbiCommandBuffer>,
      out: Physics2DAbiExecutionResult,
    ): boolean;
    step(
      world: Physics2DAbiWorldHandle,
      dt: number,
      hooks: Readonly<Physics2DAbiContactHooks> | null,
    ): Physics2DAbiStepStatus;
    readBodies(
      world: Physics2DAbiWorldHandle,
      bodyIds: Readonly<Uint32Array<ArrayBufferLike>> | null,
      out: Physics2DAbiBodyBuffer,
    ): boolean;
    readContacts(
      world: Physics2DAbiWorldHandle,
      selection: Physics2DAbiContactSelection,
      out: Physics2DAbiContactBuffer,
    ): boolean;
    readJoints(world: Physics2DAbiWorldHandle, out: Physics2DAbiJointBuffer): boolean;
    queryPoint(
      world: Physics2DAbiWorldHandle,
      x: number,
      y: number,
      filter: Readonly<Physics2DQueryFilter> | null,
      out: Physics2DAbiQueryBuffer,
    ): boolean;
    queryRay(
      world: Physics2DAbiWorldHandle,
      originX: number,
      originY: number,
      directionX: number,
      directionY: number,
      maxFraction: number,
      closestOnly: boolean,
      filter: Readonly<Physics2DQueryFilter> | null,
      out: Physics2DAbiQueryBuffer,
    ): boolean;
    queryRegion(
      world: Physics2DAbiWorldHandle,
      region: Readonly<SpatialAabb2D>,
      filter: Readonly<Physics2DQueryFilter> | null,
      out: Physics2DAbiQueryBuffer,
    ): boolean;
    queryShapeCast(
      world: Physics2DAbiWorldHandle,
      shape: Readonly<CollisionBuiltInShape2D>,
      dx: number,
      dy: number,
      maxFraction: number,
      filter: Readonly<Physics2DQueryFilter> | null,
      out: Physics2DAbiQueryBuffer,
    ): boolean;
  }
}

declare module '@flighthq/physics2d-abi' {
  import type { Physics2DAbi } from '@flighthq/types';
  export function createPhysics2DAbi(): Physics2DAbi;
}
