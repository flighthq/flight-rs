/* tslint:disable */
/* eslint-disable */

export function apply_surface_curve_wasm(dest_data: Uint8Array, dest_descriptor: Float64Array, source_data: Uint8Array, source_descriptor: Float64Array, red_lut: Uint8Array, green_lut: Uint8Array, blue_lut: Uint8Array, alpha_lut: Uint8Array): void;

export function apply_surface_levels_wasm(dest_data: Uint8Array, dest_descriptor: Float64Array, source_data: Uint8Array, source_descriptor: Float64Array, black_point: number, white_point: number, gamma: number): void;

export function apply_surface_palette_map_wasm(dest_data: Uint8Array, dest_descriptor: Float64Array, source_data: Uint8Array, source_descriptor: Float64Array, red_map: Float64Array, green_map: Float64Array, blue_map: Float64Array, alpha_map: Float64Array): void;

export function build_surface_brightness_color_matrix_wasm(out: Float64Array, amount: number): void;

export function build_surface_contrast_color_matrix_wasm(out: Float64Array, amount: number): void;

export function build_surface_grayscale_color_matrix_wasm(out: Float64Array): void;

export function build_surface_hue_rotation_color_matrix_wasm(out: Float64Array, degrees: number): void;

export function build_surface_invert_color_matrix_wasm(out: Float64Array): void;

export function build_surface_saturation_color_matrix_wasm(out: Float64Array, amount: number): void;

export function build_surface_sepia_color_matrix_wasm(out: Float64Array): void;

export function color_matrix_surface_wasm(out: Uint8Array, source_data: Uint8Array, source_descriptor: Float64Array, matrix: Float64Array): void;

export function compare_surface_fingerprints_wasm(first_cells: Uint8Array, first_grid_size: number, second_cells: Uint8Array, second_grid_size: number): number;

export function concat_surface_color_matrix_wasm(out: Float64Array, first: Float64Array, second: Float64Array): void;

export function convolve_surface_wasm(out: Uint8Array, source_data: Uint8Array, source_descriptor: Float64Array, matrix: Float64Array, matrix_x: number, matrix_y: number, bias: number, edge: string, divisor: number, preserve_alpha: boolean): void;

export function copy_surface_alpha_wasm(dest_data: Uint8Array, dest_descriptor: Float64Array, source_data: Uint8Array, source_descriptor: Float64Array): void;

export function copy_surface_pixels_wasm(dest_data: Uint8Array, dest_descriptor: Float64Array, source_data: Uint8Array, source_descriptor: Float64Array, composite: boolean): void;

export function create_surface_fingerprint_wasm(out: Uint8Array, source_data: Uint8Array, source_width: number, source_height: number, grid_size: number): void;

export function dilate_surface_wasm(out: Uint8Array, source_data: Uint8Array, source_descriptor: Float64Array, radius: number): void;

export function erode_surface_wasm(out: Uint8Array, source_data: Uint8Array, source_descriptor: Float64Array, radius: number): void;

export function fill_surface_noise_wasm(data: Uint8Array, descriptor: Float64Array, seed: number, low: number, high: number, gray_scale: boolean): void;

export function fill_surface_perlin_noise_wasm(data: Uint8Array, descriptor: Float64Array, base_x: number, base_y: number, octaves: number, seed: number, gray_scale: boolean, stitch: boolean, channel_options: number): void;

export function fill_surface_rectangle_wasm(data: Uint8Array, descriptor: Float64Array, color: number): void;

export function fill_surface_turbulence_wasm(data: Uint8Array, descriptor: Float64Array, base_x: number, base_y: number, octaves: number, seed: number, gray_scale: boolean, stitch: boolean, channel_options: number): void;

export function get_surface_color_bounds_rectangle_wasm(out: Float64Array, data: Uint8Array, descriptor: Float64Array, mask: number, color: number, find_color: boolean): boolean;

export function get_surface_coverage_wasm(data: Uint8Array, width: number, height: number, background_color: number, channel_tolerance: number): number;

export function get_surface_histogram_wasm(out: Float64Array, data: Uint8Array, descriptor: Float64Array): void;

export function get_surface_mismatch_wasm(out: Float64Array, source_data: Uint8Array, source_width: number, source_height: number, other_data: Uint8Array, other_width: number, other_height: number, channel_tolerance: number): void;

export function merge_surface_channels_wasm(out_data: Uint8Array, out_descriptor: Float64Array, red_data: Uint8Array, red_descriptor: Float64Array, green_data: Uint8Array, green_descriptor: Float64Array, blue_data: Uint8Array, blue_descriptor: Float64Array, alpha_data: Uint8Array, alpha_descriptor: Float64Array): void;

export function multiply_surface_alpha_wasm(data: Uint8Array, descriptor: Float64Array, factor: number): void;

export function pixelate_surface_wasm(out: Uint8Array, source_data: Uint8Array, source_descriptor: Float64Array, block_size: number): void;

export function premultiply_surface_pixels_wasm(out: Uint8Array, source: Uint8Array, length: number): void;

export function set_surface_alpha_wasm(data: Uint8Array, descriptor: Float64Array, alpha: number): void;

export function set_surface_color_matrix_identity_wasm(out: Float64Array): void;

export function unpremultiply_surface_pixels_wasm(out: Uint8Array, source: Uint8Array, length: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly apply_surface_curve_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number) => void;
    readonly apply_surface_levels_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => void;
    readonly apply_surface_palette_map_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number) => void;
    readonly build_surface_brightness_color_matrix_wasm: (a: number, b: number, c: any, d: number) => void;
    readonly build_surface_contrast_color_matrix_wasm: (a: number, b: number, c: any, d: number) => void;
    readonly build_surface_grayscale_color_matrix_wasm: (a: number, b: number, c: any) => void;
    readonly build_surface_hue_rotation_color_matrix_wasm: (a: number, b: number, c: any, d: number) => void;
    readonly build_surface_invert_color_matrix_wasm: (a: number, b: number, c: any) => void;
    readonly build_surface_saturation_color_matrix_wasm: (a: number, b: number, c: any, d: number) => void;
    readonly build_surface_sepia_color_matrix_wasm: (a: number, b: number, c: any) => void;
    readonly color_matrix_surface_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number) => void;
    readonly compare_surface_fingerprints_wasm: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly concat_surface_color_matrix_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number) => void;
    readonly convolve_surface_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number) => void;
    readonly copy_surface_alpha_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number) => void;
    readonly copy_surface_pixels_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => void;
    readonly create_surface_fingerprint_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number) => void;
    readonly dilate_surface_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number) => void;
    readonly erode_surface_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number) => void;
    readonly fill_surface_noise_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number) => void;
    readonly fill_surface_perlin_noise_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => void;
    readonly fill_surface_rectangle_wasm: (a: number, b: number, c: any, d: number, e: number, f: number) => void;
    readonly fill_surface_turbulence_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => void;
    readonly get_surface_color_bounds_rectangle_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
    readonly get_surface_coverage_wasm: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly get_surface_histogram_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number) => void;
    readonly get_surface_mismatch_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => void;
    readonly merge_surface_channels_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number) => void;
    readonly multiply_surface_alpha_wasm: (a: number, b: number, c: any, d: number, e: number, f: number) => void;
    readonly pixelate_surface_wasm: (a: number, b: number, c: any, d: number, e: number, f: number, g: number, h: number) => void;
    readonly premultiply_surface_pixels_wasm: (a: number, b: number, c: any, d: number, e: number, f: number) => void;
    readonly set_surface_alpha_wasm: (a: number, b: number, c: any, d: number, e: number, f: number) => void;
    readonly set_surface_color_matrix_identity_wasm: (a: number, b: number, c: any) => void;
    readonly unpremultiply_surface_pixels_wasm: (a: number, b: number, c: any, d: number, e: number, f: number) => void;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
