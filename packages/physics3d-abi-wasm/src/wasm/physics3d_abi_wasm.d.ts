/* tslint:disable */
/* eslint-disable */

export function abi_capabilities(): number;

export function abi_version(): number;

export function create_abi(): number;

export function create_world(abi: number): number;

export function destroy_abi(abi: number): boolean;

export function destroy_world(abi: number, world: number): boolean;

export function execute(abi: number, world: number, commands: Uint8Array, byte_length: number, command_count: number, result: Uint32Array): boolean;

export function query(abi: number, world: number, counts: Uint32Array): boolean;

export function read_bodies(abi: number, world: number, selection: Uint32Array, has_selection: boolean, ids: Uint32Array, flags: Uint32Array, values: Float64Array, counts: Uint32Array): boolean;

export function read_contacts(abi: number, world: number, counts: Uint32Array): boolean;

export function read_joints(abi: number, world: number, ids: Uint32Array, flags: Uint32Array, values: Float64Array, counts: Uint32Array): boolean;

export function step(abi: number, world: number, dt: number, has_hooks: boolean): number;

export function world_status(abi: number, world: number): number;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly abi_capabilities: () => number;
    readonly abi_version: () => number;
    readonly destroy_abi: (a: number) => number;
    readonly destroy_world: (a: number, b: number) => number;
    readonly execute: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: any) => number;
    readonly query: (a: number, b: number, c: number, d: number, e: any) => number;
    readonly read_bodies: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: any, i: number, j: number, k: any, l: number, m: number, n: any, o: number, p: number, q: any) => number;
    readonly read_contacts: (a: number, b: number, c: number, d: number, e: any) => number;
    readonly read_joints: (a: number, b: number, c: number, d: number, e: any, f: number, g: number, h: any, i: number, j: number, k: any, l: number, m: number, n: any) => number;
    readonly step: (a: number, b: number, c: number, d: number) => number;
    readonly world_status: (a: number, b: number) => number;
    readonly create_abi: () => number;
    readonly create_world: (a: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
