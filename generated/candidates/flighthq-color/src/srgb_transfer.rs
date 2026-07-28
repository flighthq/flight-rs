// @generated from upstream/packages/color/src/srgbTransfer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/color/src/srgbTransfer.ts:4 (sha256:b321930a6d7d019ea4c2e8d81ddec21c04008d6834eb2175236eec66cd296fca)
pub fn linear_channel_to_srgb(value: f64) -> f64 {
    return if (value <= 0.0031308_f64) {
        (value * 12.92_f64)
    } else {
        ((1.055_f64 * (value).powf((1.0_f64 / 2.4_f64))) - 0.055_f64)
    };
}

// Source: upstream/packages/color/src/srgbTransfer.ts:11 (sha256:4a8a5db67a732be0880930db2f88ef459e070a214da9767fb00ac92f8fbea596)
pub fn srgb_channel_to_linear(value: f64) -> f64 {
    return if (value <= 0.04045_f64) {
        (value / 12.92_f64)
    } else {
        ((value + 0.055_f64) / 1.055_f64).powf(2.4_f64)
    };
}
