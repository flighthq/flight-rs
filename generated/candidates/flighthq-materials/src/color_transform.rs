// @generated from upstream/packages/materials/src/colorTransform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{ColorTransform, ColorTransformLike};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_multiplier: Option<f64>,
    pub alpha_offset: Option<f64>,
    pub blue_multiplier: Option<f64>,
    pub blue_offset: Option<f64>,
    pub green_multiplier: Option<f64>,
    pub green_offset: Option<f64>,
    pub red_multiplier: Option<f64>,
    pub red_offset: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/colorTransform.ts:4 (sha256:131b05762498a99e2691b50a18f3ac543088ddee563a8fab218827df95f28793)
pub fn clone_color_transform(source: &ColorTransformLike) -> ColorTransform {
    return create_color_transform(Some((source).clone()));
}

// Source: upstream/packages/materials/src/colorTransform.ts:8 (sha256:e8be109f418b7a17469cc502c92550118b7f1b60469b564549df0e812b34ae84)
pub fn concat_color_transform(
    out: &mut ColorTransformLike,
    source: &ColorTransformLike,
    other: &ColorTransformLike,
) -> () {
    out.red_offset = ((source.red_multiplier * other.red_offset) + source.red_offset);
    out.green_offset = ((source.green_multiplier * other.green_offset) + source.green_offset);
    out.blue_offset = ((source.blue_multiplier * other.blue_offset) + source.blue_offset);
    out.alpha_offset = ((source.alpha_multiplier * other.alpha_offset) + source.alpha_offset);
    out.red_multiplier = (source.red_multiplier * other.red_multiplier);
    out.green_multiplier = (source.green_multiplier * other.green_multiplier);
    out.blue_multiplier = (source.blue_multiplier * other.blue_multiplier);
    out.alpha_multiplier = (source.alpha_multiplier * other.alpha_multiplier);
}

// Source: upstream/packages/materials/src/colorTransform.ts:23 (sha256:28dd64ee3649b07e412d672c670e2e0a63b7e74c40c0442c6d114418bac5a007)
pub fn copy_color_transform(out: &mut ColorTransformLike, source: &ColorTransformLike) -> () {
    out.red_multiplier = source.red_multiplier;
    out.green_multiplier = source.green_multiplier;
    out.blue_multiplier = source.blue_multiplier;
    out.alpha_multiplier = source.alpha_multiplier;
    out.red_offset = source.red_offset;
    out.green_offset = source.green_offset;
    out.blue_offset = source.blue_offset;
    out.alpha_offset = source.alpha_offset;
}

// Source: upstream/packages/materials/src/colorTransform.ts:34 (sha256:45011f608fa8e734d8644cab7ec1e70dcbe0df6431c0871386d7948d23dbb62a)
pub fn copy_color_transform_to_arrays(
    out_color_multipliers: &mut Vec<f64>,
    out_color_offsets: &mut Vec<f64>,
    source: &ColorTransformLike,
) -> () {
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = source.red_multiplier;
        if __flight_index == out_color_multipliers.len() {
            out_color_multipliers.push(__flight_value);
        } else {
            out_color_multipliers[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = source.green_multiplier;
        if __flight_index == out_color_multipliers.len() {
            out_color_multipliers.push(__flight_value);
        } else {
            out_color_multipliers[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = source.blue_multiplier;
        if __flight_index == out_color_multipliers.len() {
            out_color_multipliers.push(__flight_value);
        } else {
            out_color_multipliers[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = source.alpha_multiplier;
        if __flight_index == out_color_multipliers.len() {
            out_color_multipliers.push(__flight_value);
        } else {
            out_color_multipliers[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = source.red_offset;
        if __flight_index == out_color_offsets.len() {
            out_color_offsets.push(__flight_value);
        } else {
            out_color_offsets[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = source.green_offset;
        if __flight_index == out_color_offsets.len() {
            out_color_offsets.push(__flight_value);
        } else {
            out_color_offsets[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = source.blue_offset;
        if __flight_index == out_color_offsets.len() {
            out_color_offsets.push(__flight_value);
        } else {
            out_color_offsets[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = source.alpha_offset;
        if __flight_index == out_color_offsets.len() {
            out_color_offsets.push(__flight_value);
        } else {
            out_color_offsets[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/materials/src/colorTransform.ts:49 (sha256:bb63464767abcaf6ad3cc2ba5a5a0ae0bd9968b4078038f1ceed7083ec2dd411)
pub fn create_color_transform(opts: Option<FlightPartialRecord1>) -> ColorTransform {
    return create_entity(Some(ColorTransform {
        __flight_identity: std::sync::Arc::new(()),
        red_multiplier: (opts.as_ref().and_then(|value| value.red_multiplier)).unwrap_or(1.0_f64),
        green_multiplier: (opts.as_ref().and_then(|value| value.green_multiplier))
            .unwrap_or(1.0_f64),
        blue_multiplier: (opts.as_ref().and_then(|value| value.blue_multiplier)).unwrap_or(1.0_f64),
        alpha_multiplier: (opts.as_ref().and_then(|value| value.alpha_multiplier))
            .unwrap_or(1.0_f64),
        red_offset: (opts.as_ref().and_then(|value| value.red_offset)).unwrap_or(0.0_f64),
        green_offset: (opts.as_ref().and_then(|value| value.green_offset)).unwrap_or(0.0_f64),
        blue_offset: (opts.as_ref().and_then(|value| value.blue_offset)).unwrap_or(0.0_f64),
        alpha_offset: (opts.as_ref().and_then(|value| value.alpha_offset)).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/materials/src/colorTransform.ts:62 (sha256:d0d6931e6ec6cf96c032d8fa3f8cc1e9870ca05cf9280e9568cbc28e5f1458aa)
pub fn equals_color_transform(a: &ColorTransformLike, b: &ColorTransformLike) -> bool {
    return (equals_color_transform_offsets(a, b, None))
        && (equals_color_transform_multipliers(a, b, None));
}

// Source: upstream/packages/materials/src/colorTransform.ts:66 (sha256:1c44ed042a2834aa5d00b3d778e881bc3c070a2846a1582e6a458b4106943286)
pub fn equals_color_transform_multipliers(
    a: &ColorTransformLike,
    b: &ColorTransformLike,
    compare_alpha: Option<bool>,
) -> bool {
    let compare_alpha = compare_alpha.unwrap_or(true);
    return (((a.red_multiplier == b.red_multiplier)
        && (a.green_multiplier == b.green_multiplier))
        && (a.blue_multiplier == b.blue_multiplier))
        && ((!compare_alpha) || (a.alpha_multiplier == b.alpha_multiplier));
}

// Source: upstream/packages/materials/src/colorTransform.ts:79 (sha256:c170886f1f4859eb796ffce84da5d185c45ff44c0396940b37cb263630e5b27c)
pub fn equals_color_transform_offsets(
    a: &ColorTransformLike,
    b: &ColorTransformLike,
    compare_alpha: Option<bool>,
) -> bool {
    let compare_alpha = compare_alpha.unwrap_or(true);
    return (((a.red_offset == b.red_offset) && (a.green_offset == b.green_offset))
        && (a.blue_offset == b.blue_offset))
        && ((!compare_alpha) || (a.alpha_offset == b.alpha_offset));
}

// Source: upstream/packages/materials/src/colorTransform.ts:92 (sha256:5d6275b80dc2038e3d37243737c328b8514e47d8ecd043ab2fd67e1af7019306)
pub fn get_color_transform_offset_rgb(source: &ColorTransformLike) -> f64 {
    return (__flight_js_to_i32(
        (__flight_js_to_i32(
            __flight_js_to_i32(((source.red_offset) as f32) as f64)
                .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64,
        ) | __flight_js_to_i32(
            __flight_js_to_i32(((source.green_offset) as f32) as f64)
                .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
        )) as f64,
    ) | __flight_js_to_i32(((source.blue_offset) as f32) as f64)) as f64;
}

// Source: upstream/packages/materials/src/colorTransform.ts:98 (sha256:3b4cfadc391b956d26a06a0e237e45dc678fe3100de71e6e191531d37a6e4e48)
pub fn get_color_transform_offset_rgba(source: &ColorTransformLike) -> f64 {
    return (__flight_js_to_i32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                __flight_js_to_i32(((source.red_offset) as f32) as f64)
                    .wrapping_shl((__flight_js_to_u32(24.0_f64) & 31)) as f64,
            ) | __flight_js_to_i32(
                __flight_js_to_i32(((source.green_offset) as f32) as f64)
                    .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64,
            )) as f64,
        ) | __flight_js_to_i32(
            __flight_js_to_i32(((source.blue_offset) as f32) as f64)
                .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
        )) as f64,
    ) | __flight_js_to_i32(((source.alpha_offset) as f32) as f64)) as f64;
}

// Source: upstream/packages/materials/src/colorTransform.ts:107 (sha256:cff29e013b3c797ddd8c2c9869314172ad0ed420e2476168cddc8193762e4010)
pub fn invert_color_transform(out: &mut ColorTransformLike, source: &ColorTransformLike) -> () {
    out.red_multiplier = if (source.red_multiplier != 0.0_f64) {
        (1.0_f64 / source.red_multiplier)
    } else {
        1.0_f64
    };
    out.green_multiplier = if (source.green_multiplier != 0.0_f64) {
        (1.0_f64 / source.green_multiplier)
    } else {
        1.0_f64
    };
    out.blue_multiplier = if (source.blue_multiplier != 0.0_f64) {
        (1.0_f64 / source.blue_multiplier)
    } else {
        1.0_f64
    };
    out.alpha_multiplier = if (source.alpha_multiplier != 0.0_f64) {
        (1.0_f64 / source.alpha_multiplier)
    } else {
        1.0_f64
    };
    out.red_offset = (-source.red_offset);
    out.green_offset = (-source.green_offset);
    out.blue_offset = (-source.blue_offset);
    out.alpha_offset = (-source.alpha_offset);
}

// Source: upstream/packages/materials/src/colorTransform.ts:118 (sha256:cf016f376e8c42525b04891691bdcd4d95622bfc63718399e298f32865479ccf)
pub fn is_identity_color_transform(
    source: &ColorTransformLike,
    compare_alpha_multiplier: Option<bool>,
) -> bool {
    let compare_alpha_multiplier = compare_alpha_multiplier.unwrap_or(true);
    return (equals_color_transform_offsets(source, &_IDENTITY, None))
        && (equals_color_transform_multipliers(
            source,
            &_IDENTITY,
            Some(compare_alpha_multiplier),
        ));
}

// Source: upstream/packages/materials/src/colorTransform.ts:128 (sha256:3c0c37559a8abc8f2ac1c5e21a776728ab89d487453ebe3bed9133468a7f4b69)
pub fn set_color_transform(
    out: &mut ColorTransformLike,
    red_multiplier: f64,
    green_multiplier: f64,
    blue_multiplier: f64,
    alpha_multiplier: f64,
    red_offset: f64,
    green_offset: f64,
    blue_offset: f64,
    alpha_offset: f64,
) -> () {
    out.red_multiplier = red_multiplier;
    out.green_multiplier = green_multiplier;
    out.blue_multiplier = blue_multiplier;
    out.alpha_multiplier = alpha_multiplier;
    out.red_offset = red_offset;
    out.green_offset = green_offset;
    out.blue_offset = blue_offset;
    out.alpha_offset = alpha_offset;
}

// Source: upstream/packages/materials/src/colorTransform.ts:149 (sha256:4c11f8166272a843a273708815e22e8c8aa4557a7c5a45c6ed3f18751277c667)
pub fn set_color_transform_identity(out: &mut ColorTransform) -> () {
    set_color_transform(
        out, 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
    );
}

// Source: upstream/packages/materials/src/colorTransform.ts:153 (sha256:2e027bb94a203f2f8bad20fb4e80283bd2a774de9ca079c219149c40f799a4cb)
pub fn set_color_transform_offset_rgb(out: &mut ColorTransformLike, value: f64) -> () {
    out.red_offset = (__flight_js_to_i32(
        (__flight_js_to_i32(value) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    out.green_offset = (__flight_js_to_i32(
        (__flight_js_to_i32(value) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    out.blue_offset = (__flight_js_to_i32(value) & __flight_js_to_i32(255.0_f64)) as f64;
    out.alpha_offset = 0.0_f64;
    out.red_multiplier = 0.0_f64;
    out.green_multiplier = 0.0_f64;
    out.blue_multiplier = 0.0_f64;
    out.alpha_multiplier = 1.0_f64;
}

// Source: upstream/packages/materials/src/colorTransform.ts:164 (sha256:d59639087ae0097d6307df1840e9d87b69ed10c3b8e3a561b971365040b38ba7)
pub fn set_color_transform_offset_rgba(out: &mut ColorTransformLike, value: f64) -> () {
    out.red_offset = (__flight_js_to_i32(
        (__flight_js_to_i32(value) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    out.green_offset = (__flight_js_to_i32(
        (__flight_js_to_i32(value) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    out.blue_offset = (__flight_js_to_i32(
        (__flight_js_to_i32(value) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    out.alpha_offset = (__flight_js_to_i32(value) & __flight_js_to_i32(255.0_f64)) as f64;
    out.red_multiplier = 0.0_f64;
    out.green_multiplier = 0.0_f64;
    out.blue_multiplier = 0.0_f64;
    out.alpha_multiplier = 0.0_f64;
}

// Source: upstream/packages/materials/src/colorTransform.ts:175 (sha256:055845fe8f0a4d1fa90c5f19e578e4b6ac83567da20a0378ca699ef6d725c397)
static _IDENTITY: std::sync::LazyLock<ColorTransform> =
    std::sync::LazyLock::new(|| create_color_transform(None));
