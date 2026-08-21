/* @ts-self-types="./bitmap_wasm.d.ts" */

/**
 * @param {Uint8Array} dest_data
 * @param {Float64Array} dest_descriptor
 * @param {Uint8Array} source_data
 * @param {Float64Array} source_descriptor
 * @param {Uint8Array} red_lut
 * @param {Uint8Array} green_lut
 * @param {Uint8Array} blue_lut
 * @param {Uint8Array} alpha_lut
 */
export function apply_bitmap_curve_wasm(dest_data, dest_descriptor, source_data, source_descriptor, red_lut, green_lut, blue_lut, alpha_lut) {
    var ptr0 = passArray8ToWasm0(dest_data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(dest_descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(source_descriptor, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passArray8ToWasm0(red_lut, wasm.__wbindgen_malloc);
    const len4 = WASM_VECTOR_LEN;
    const ptr5 = passArray8ToWasm0(green_lut, wasm.__wbindgen_malloc);
    const len5 = WASM_VECTOR_LEN;
    const ptr6 = passArray8ToWasm0(blue_lut, wasm.__wbindgen_malloc);
    const len6 = WASM_VECTOR_LEN;
    const ptr7 = passArray8ToWasm0(alpha_lut, wasm.__wbindgen_malloc);
    const len7 = WASM_VECTOR_LEN;
    wasm.apply_bitmap_curve_wasm(ptr0, len0, dest_data, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4, ptr5, len5, ptr6, len6, ptr7, len7);
}

/**
 * @param {Uint8Array} dest_data
 * @param {Float64Array} dest_descriptor
 * @param {Uint8Array} source_data
 * @param {Float64Array} source_descriptor
 * @param {number} black_point
 * @param {number} white_point
 * @param {number} gamma
 */
export function apply_bitmap_levels_wasm(dest_data, dest_descriptor, source_data, source_descriptor, black_point, white_point, gamma) {
    var ptr0 = passArray8ToWasm0(dest_data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(dest_descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(source_descriptor, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    wasm.apply_bitmap_levels_wasm(ptr0, len0, dest_data, ptr1, len1, ptr2, len2, ptr3, len3, black_point, white_point, gamma);
}

/**
 * @param {Uint8Array} dest_data
 * @param {Float64Array} dest_descriptor
 * @param {Uint8Array} source_data
 * @param {Float64Array} source_descriptor
 * @param {Float64Array} red_map
 * @param {Float64Array} green_map
 * @param {Float64Array} blue_map
 * @param {Float64Array} alpha_map
 */
export function apply_bitmap_palette_map_wasm(dest_data, dest_descriptor, source_data, source_descriptor, red_map, green_map, blue_map, alpha_map) {
    var ptr0 = passArray8ToWasm0(dest_data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(dest_descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(source_descriptor, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passArrayF64ToWasm0(red_map, wasm.__wbindgen_malloc);
    const len4 = WASM_VECTOR_LEN;
    const ptr5 = passArrayF64ToWasm0(green_map, wasm.__wbindgen_malloc);
    const len5 = WASM_VECTOR_LEN;
    const ptr6 = passArrayF64ToWasm0(blue_map, wasm.__wbindgen_malloc);
    const len6 = WASM_VECTOR_LEN;
    const ptr7 = passArrayF64ToWasm0(alpha_map, wasm.__wbindgen_malloc);
    const len7 = WASM_VECTOR_LEN;
    wasm.apply_bitmap_palette_map_wasm(ptr0, len0, dest_data, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4, ptr5, len5, ptr6, len6, ptr7, len7);
}

/**
 * @param {Float64Array} out
 * @param {number} amount
 */
export function build_bitmap_brightness_color_matrix_wasm(out, amount) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.build_bitmap_brightness_color_matrix_wasm(ptr0, len0, out, amount);
}

/**
 * @param {Float64Array} out
 * @param {number} amount
 */
export function build_bitmap_contrast_color_matrix_wasm(out, amount) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.build_bitmap_contrast_color_matrix_wasm(ptr0, len0, out, amount);
}

/**
 * @param {Float64Array} out
 */
export function build_bitmap_grayscale_color_matrix_wasm(out) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.build_bitmap_grayscale_color_matrix_wasm(ptr0, len0, out);
}

/**
 * @param {Float64Array} out
 * @param {number} degrees
 */
export function build_bitmap_hue_rotation_color_matrix_wasm(out, degrees) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.build_bitmap_hue_rotation_color_matrix_wasm(ptr0, len0, out, degrees);
}

/**
 * @param {Float64Array} out
 */
export function build_bitmap_invert_color_matrix_wasm(out) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.build_bitmap_invert_color_matrix_wasm(ptr0, len0, out);
}

/**
 * @param {Float64Array} out
 * @param {number} amount
 */
export function build_bitmap_saturation_color_matrix_wasm(out, amount) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.build_bitmap_saturation_color_matrix_wasm(ptr0, len0, out, amount);
}

/**
 * @param {Float64Array} out
 */
export function build_bitmap_sepia_color_matrix_wasm(out) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.build_bitmap_sepia_color_matrix_wasm(ptr0, len0, out);
}

/**
 * @param {Uint8Array} out
 * @param {Uint8Array} source_data
 * @param {Float64Array} source_descriptor
 * @param {Float64Array} matrix
 */
export function color_matrix_bitmap_wasm(out, source_data, source_descriptor, matrix) {
    var ptr0 = passArray8ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayF64ToWasm0(source_descriptor, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(matrix, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    wasm.color_matrix_bitmap_wasm(ptr0, len0, out, ptr1, len1, ptr2, len2, ptr3, len3);
}

/**
 * @param {Uint8Array} first_cells
 * @param {number} first_grid_size
 * @param {Uint8Array} second_cells
 * @param {number} second_grid_size
 * @returns {number}
 */
export function compare_bitmap_fingerprints_wasm(first_cells, first_grid_size, second_cells, second_grid_size) {
    const ptr0 = passArray8ToWasm0(first_cells, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(second_cells, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.compare_bitmap_fingerprints_wasm(ptr0, len0, first_grid_size, ptr1, len1, second_grid_size);
    return ret;
}

/**
 * @param {Float64Array} out
 * @param {Float64Array} first
 * @param {Float64Array} second
 */
export function concat_bitmap_color_matrix_wasm(out, first, second) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(first, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayF64ToWasm0(second, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    wasm.concat_bitmap_color_matrix_wasm(ptr0, len0, out, ptr1, len1, ptr2, len2);
}

/**
 * @param {Uint8Array} out
 * @param {Uint8Array} source_data
 * @param {Float64Array} source_descriptor
 * @param {Float64Array} matrix
 * @param {number} matrix_x
 * @param {number} matrix_y
 * @param {number} bias
 * @param {string} edge
 * @param {number} divisor
 * @param {boolean} preserve_alpha
 */
export function convolve_bitmap_wasm(out, source_data, source_descriptor, matrix, matrix_x, matrix_y, bias, edge, divisor, preserve_alpha) {
    var ptr0 = passArray8ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayF64ToWasm0(source_descriptor, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(matrix, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passStringToWasm0(edge, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len4 = WASM_VECTOR_LEN;
    wasm.convolve_bitmap_wasm(ptr0, len0, out, ptr1, len1, ptr2, len2, ptr3, len3, matrix_x, matrix_y, bias, ptr4, len4, divisor, preserve_alpha);
}

/**
 * @param {Uint8Array} dest_data
 * @param {Float64Array} dest_descriptor
 * @param {Uint8Array} source_data
 * @param {Float64Array} source_descriptor
 */
export function copy_bitmap_alpha_wasm(dest_data, dest_descriptor, source_data, source_descriptor) {
    var ptr0 = passArray8ToWasm0(dest_data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(dest_descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(source_descriptor, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    wasm.copy_bitmap_alpha_wasm(ptr0, len0, dest_data, ptr1, len1, ptr2, len2, ptr3, len3);
}

/**
 * @param {Uint8Array} dest_data
 * @param {Float64Array} dest_descriptor
 * @param {Uint8Array} source_data
 * @param {Float64Array} source_descriptor
 * @param {boolean} composite
 */
export function copy_bitmap_pixels_wasm(dest_data, dest_descriptor, source_data, source_descriptor, composite) {
    var ptr0 = passArray8ToWasm0(dest_data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(dest_descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(source_descriptor, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    wasm.copy_bitmap_pixels_wasm(ptr0, len0, dest_data, ptr1, len1, ptr2, len2, ptr3, len3, composite);
}

/**
 * @param {Uint8Array} out
 * @param {Uint8Array} source_data
 * @param {number} source_width
 * @param {number} source_height
 * @param {number} grid_size
 */
export function create_bitmap_fingerprint_wasm(out, source_data, source_width, source_height, grid_size) {
    var ptr0 = passArray8ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    wasm.create_bitmap_fingerprint_wasm(ptr0, len0, out, ptr1, len1, source_width, source_height, grid_size);
}

/**
 * @param {Uint8Array} out
 * @param {Uint8Array} source_data
 * @param {Float64Array} source_descriptor
 * @param {number} radius
 */
export function dilate_bitmap_wasm(out, source_data, source_descriptor, radius) {
    var ptr0 = passArray8ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayF64ToWasm0(source_descriptor, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    wasm.dilate_bitmap_wasm(ptr0, len0, out, ptr1, len1, ptr2, len2, radius);
}

/**
 * @param {Uint8Array} out
 * @param {Uint8Array} source_data
 * @param {Float64Array} source_descriptor
 * @param {number} radius
 */
export function erode_bitmap_wasm(out, source_data, source_descriptor, radius) {
    var ptr0 = passArray8ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayF64ToWasm0(source_descriptor, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    wasm.erode_bitmap_wasm(ptr0, len0, out, ptr1, len1, ptr2, len2, radius);
}

/**
 * @param {Uint8Array} data
 * @param {Float64Array} descriptor
 * @param {number} seed
 * @param {number} low
 * @param {number} high
 * @param {boolean} gray_scale
 */
export function fill_bitmap_noise_wasm(data, descriptor, seed, low, high, gray_scale) {
    var ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    wasm.fill_bitmap_noise_wasm(ptr0, len0, data, ptr1, len1, seed, low, high, gray_scale);
}

/**
 * @param {Uint8Array} data
 * @param {Float64Array} descriptor
 * @param {number} base_x
 * @param {number} base_y
 * @param {number} octaves
 * @param {number} seed
 * @param {boolean} gray_scale
 * @param {boolean} stitch
 * @param {number} channel_options
 */
export function fill_bitmap_perlin_noise_wasm(data, descriptor, base_x, base_y, octaves, seed, gray_scale, stitch, channel_options) {
    var ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    wasm.fill_bitmap_perlin_noise_wasm(ptr0, len0, data, ptr1, len1, base_x, base_y, octaves, seed, gray_scale, stitch, channel_options);
}

/**
 * @param {Uint8Array} data
 * @param {Float64Array} descriptor
 * @param {number} color
 */
export function fill_bitmap_rectangle_wasm(data, descriptor, color) {
    var ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    wasm.fill_bitmap_rectangle_wasm(ptr0, len0, data, ptr1, len1, color);
}

/**
 * @param {Uint8Array} data
 * @param {Float64Array} descriptor
 * @param {number} base_x
 * @param {number} base_y
 * @param {number} octaves
 * @param {number} seed
 * @param {boolean} gray_scale
 * @param {boolean} stitch
 * @param {number} channel_options
 */
export function fill_bitmap_turbulence_wasm(data, descriptor, base_x, base_y, octaves, seed, gray_scale, stitch, channel_options) {
    var ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    wasm.fill_bitmap_turbulence_wasm(ptr0, len0, data, ptr1, len1, base_x, base_y, octaves, seed, gray_scale, stitch, channel_options);
}

/**
 * @param {Float64Array} out
 * @param {Uint8Array} data
 * @param {Float64Array} descriptor
 * @param {number} mask
 * @param {number} color
 * @param {boolean} find_color
 * @returns {boolean}
 */
export function get_bitmap_color_bounds_rectangle_wasm(out, data, descriptor, mask, color, find_color) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayF64ToWasm0(descriptor, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.get_bitmap_color_bounds_rectangle_wasm(ptr0, len0, out, ptr1, len1, ptr2, len2, mask, color, find_color);
    return ret !== 0;
}

/**
 * @param {Uint8Array} data
 * @param {number} width
 * @param {number} height
 * @param {number} background_color
 * @param {number} channel_tolerance
 * @returns {number}
 */
export function get_bitmap_coverage_wasm(data, width, height, background_color, channel_tolerance) {
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.get_bitmap_coverage_wasm(ptr0, len0, width, height, background_color, channel_tolerance);
    return ret;
}

/**
 * @param {Float64Array} out
 * @param {Uint8Array} data
 * @param {Float64Array} descriptor
 */
export function get_bitmap_histogram_wasm(out, data, descriptor) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayF64ToWasm0(descriptor, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    wasm.get_bitmap_histogram_wasm(ptr0, len0, out, ptr1, len1, ptr2, len2);
}

/**
 * @param {Float64Array} out
 * @param {Uint8Array} source_data
 * @param {number} source_width
 * @param {number} source_height
 * @param {Uint8Array} other_data
 * @param {number} other_width
 * @param {number} other_height
 * @param {number} channel_tolerance
 */
export function get_bitmap_mismatch_wasm(out, source_data, source_width, source_height, other_data, other_width, other_height, channel_tolerance) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArray8ToWasm0(other_data, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    wasm.get_bitmap_mismatch_wasm(ptr0, len0, out, ptr1, len1, source_width, source_height, ptr2, len2, other_width, other_height, channel_tolerance);
}

/**
 * @param {Uint8Array} out_data
 * @param {Float64Array} out_descriptor
 * @param {Uint8Array} red_data
 * @param {Float64Array} red_descriptor
 * @param {Uint8Array} green_data
 * @param {Float64Array} green_descriptor
 * @param {Uint8Array} blue_data
 * @param {Float64Array} blue_descriptor
 * @param {Uint8Array} alpha_data
 * @param {Float64Array} alpha_descriptor
 */
export function merge_bitmap_channels_wasm(out_data, out_descriptor, red_data, red_descriptor, green_data, green_descriptor, blue_data, blue_descriptor, alpha_data, alpha_descriptor) {
    var ptr0 = passArray8ToWasm0(out_data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(out_descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArray8ToWasm0(red_data, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(red_descriptor, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passArray8ToWasm0(green_data, wasm.__wbindgen_malloc);
    const len4 = WASM_VECTOR_LEN;
    const ptr5 = passArrayF64ToWasm0(green_descriptor, wasm.__wbindgen_malloc);
    const len5 = WASM_VECTOR_LEN;
    const ptr6 = passArray8ToWasm0(blue_data, wasm.__wbindgen_malloc);
    const len6 = WASM_VECTOR_LEN;
    const ptr7 = passArrayF64ToWasm0(blue_descriptor, wasm.__wbindgen_malloc);
    const len7 = WASM_VECTOR_LEN;
    const ptr8 = passArray8ToWasm0(alpha_data, wasm.__wbindgen_malloc);
    const len8 = WASM_VECTOR_LEN;
    const ptr9 = passArrayF64ToWasm0(alpha_descriptor, wasm.__wbindgen_malloc);
    const len9 = WASM_VECTOR_LEN;
    wasm.merge_bitmap_channels_wasm(ptr0, len0, out_data, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4, ptr5, len5, ptr6, len6, ptr7, len7, ptr8, len8, ptr9, len9);
}

/**
 * @param {Uint8Array} data
 * @param {Float64Array} descriptor
 * @param {number} factor
 */
export function multiply_bitmap_alpha_wasm(data, descriptor, factor) {
    var ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    wasm.multiply_bitmap_alpha_wasm(ptr0, len0, data, ptr1, len1, factor);
}

/**
 * @param {Uint8Array} out
 * @param {Uint8Array} source_data
 * @param {Float64Array} source_descriptor
 * @param {number} block_size
 */
export function pixelate_bitmap_wasm(out, source_data, source_descriptor, block_size) {
    var ptr0 = passArray8ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(source_data, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayF64ToWasm0(source_descriptor, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    wasm.pixelate_bitmap_wasm(ptr0, len0, out, ptr1, len1, ptr2, len2, block_size);
}

/**
 * @param {Uint8Array} out
 * @param {Uint8Array} source
 * @param {number} length
 */
export function premultiply_bitmap_pixels_wasm(out, source, length) {
    var ptr0 = passArray8ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(source, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    wasm.premultiply_bitmap_pixels_wasm(ptr0, len0, out, ptr1, len1, length);
}

/**
 * @param {Uint8Array} data
 * @param {Float64Array} descriptor
 * @param {number} alpha
 */
export function set_bitmap_alpha_wasm(data, descriptor, alpha) {
    var ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(descriptor, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    wasm.set_bitmap_alpha_wasm(ptr0, len0, data, ptr1, len1, alpha);
}

/**
 * @param {Float64Array} out
 */
export function set_bitmap_color_matrix_identity_wasm(out) {
    var ptr0 = passArrayF64ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.set_bitmap_color_matrix_identity_wasm(ptr0, len0, out);
}

/**
 * @param {Uint8Array} out
 * @param {Uint8Array} source
 * @param {number} length
 */
export function unpremultiply_bitmap_pixels_wasm(out, source, length) {
    var ptr0 = passArray8ToWasm0(out, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(source, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    wasm.unpremultiply_bitmap_pixels_wasm(ptr0, len0, out, ptr1, len1, length);
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_copy_to_typed_array_4db0cbe2cc60dbee: function(arg0, arg1, arg2) {
            new Uint8Array(arg2.buffer, arg2.byteOffset, arg2.byteLength).set(getArrayU8FromWasm0(arg0, arg1));
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./bitmap_wasm_bg.js": import0,
    };
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getFloat64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedFloat64ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('bitmap_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
