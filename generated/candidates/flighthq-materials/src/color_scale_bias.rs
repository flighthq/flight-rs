// @generated from upstream/packages/materials/src/colorScaleBias.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{ColorScaleBias, ColorScaleBiasLike};

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

#[derive(Clone, Default)]
pub struct FlightPartialRecord2155237004 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_scale: Option<f64>,
    pub alpha_bias: Option<f64>,
    pub blue_scale: Option<f64>,
    pub blue_bias: Option<f64>,
    pub green_scale: Option<f64>,
    pub green_bias: Option<f64>,
    pub red_scale: Option<f64>,
    pub red_bias: Option<f64>,
}
impl PartialEq for FlightPartialRecord2155237004 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:4 (sha256:300ff59cddc7c7664bf9073551a533c27847abd7fe2c5e650983f307c7abe5fa)
pub fn clone_color_scale_bias(source: &ColorScaleBiasLike) -> ColorScaleBias {
    return create_color_scale_bias(Some({
        let __flight_source = &((*source).clone());
        FlightPartialRecord2155237004 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            alpha_scale: Some(__flight_source.alpha_scale),
            alpha_bias: Some(__flight_source.alpha_bias),
            blue_scale: Some(__flight_source.blue_scale),
            blue_bias: Some(__flight_source.blue_bias),
            green_scale: Some(__flight_source.green_scale),
            green_bias: Some(__flight_source.green_bias),
            red_scale: Some(__flight_source.red_scale),
            red_bias: Some(__flight_source.red_bias),
        }
    }));
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:8 (sha256:7f718e4370340702a83a4cebd2df8cb7dfb2aafe9475e6755aee88f40870b076)
pub fn concat_color_scale_bias(
    out: &mut ColorScaleBiasLike,
    source: &ColorScaleBiasLike,
    other: &ColorScaleBiasLike,
) -> () {
    out.red_bias = ((source.red_scale * other.red_bias) + source.red_bias);
    out.green_bias = ((source.green_scale * other.green_bias) + source.green_bias);
    out.blue_bias = ((source.blue_scale * other.blue_bias) + source.blue_bias);
    out.alpha_bias = ((source.alpha_scale * other.alpha_bias) + source.alpha_bias);
    out.red_scale = (source.red_scale * other.red_scale);
    out.green_scale = (source.green_scale * other.green_scale);
    out.blue_scale = (source.blue_scale * other.blue_scale);
    out.alpha_scale = (source.alpha_scale * other.alpha_scale);
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:23 (sha256:facfeb1cb1b49878f144d24183c396003989dbcc4f11d46c48b7f2dde11caaf5)
pub fn copy_color_scale_bias(out: &mut ColorScaleBiasLike, source: &ColorScaleBiasLike) -> () {
    out.red_scale = source.red_scale;
    out.green_scale = source.green_scale;
    out.blue_scale = source.blue_scale;
    out.alpha_scale = source.alpha_scale;
    out.red_bias = source.red_bias;
    out.green_bias = source.green_bias;
    out.blue_bias = source.blue_bias;
    out.alpha_bias = source.alpha_bias;
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:34 (sha256:367c47778e49484a68ffeca9b2eb18b912031129418bca99a10843553de243ea)
pub fn copy_color_scale_bias_to_arrays(
    out_color_scales: &mut Vec<f64>,
    out_color_biases: &mut Vec<f64>,
    source: &ColorScaleBiasLike,
) -> () {
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = source.red_scale;
        if __flight_index == out_color_scales.len() {
            out_color_scales.push(__flight_value);
        } else {
            out_color_scales[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = source.green_scale;
        if __flight_index == out_color_scales.len() {
            out_color_scales.push(__flight_value);
        } else {
            out_color_scales[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = source.blue_scale;
        if __flight_index == out_color_scales.len() {
            out_color_scales.push(__flight_value);
        } else {
            out_color_scales[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = source.alpha_scale;
        if __flight_index == out_color_scales.len() {
            out_color_scales.push(__flight_value);
        } else {
            out_color_scales[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = source.red_bias;
        if __flight_index == out_color_biases.len() {
            out_color_biases.push(__flight_value);
        } else {
            out_color_biases[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = source.green_bias;
        if __flight_index == out_color_biases.len() {
            out_color_biases.push(__flight_value);
        } else {
            out_color_biases[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = source.blue_bias;
        if __flight_index == out_color_biases.len() {
            out_color_biases.push(__flight_value);
        } else {
            out_color_biases[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = source.alpha_bias;
        if __flight_index == out_color_biases.len() {
            out_color_biases.push(__flight_value);
        } else {
            out_color_biases[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:49 (sha256:c4b5331c7b20f14f99a43fb9910b9181ae308655fe5dfa1b8a9cacdc1ca9b1dd)
pub fn create_color_scale_bias(opts: Option<FlightPartialRecord2155237004>) -> ColorScaleBias {
    return create_entity(Some(ColorScaleBias {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        red_scale: (opts.as_ref().and_then(|value| value.red_scale))
            .clone()
            .unwrap_or(1.0_f64),
        green_scale: (opts.as_ref().and_then(|value| value.green_scale))
            .clone()
            .unwrap_or(1.0_f64),
        blue_scale: (opts.as_ref().and_then(|value| value.blue_scale))
            .clone()
            .unwrap_or(1.0_f64),
        alpha_scale: (opts.as_ref().and_then(|value| value.alpha_scale))
            .clone()
            .unwrap_or(1.0_f64),
        red_bias: (opts.as_ref().and_then(|value| value.red_bias))
            .clone()
            .unwrap_or(0.0_f64),
        green_bias: (opts.as_ref().and_then(|value| value.green_bias))
            .clone()
            .unwrap_or(0.0_f64),
        blue_bias: (opts.as_ref().and_then(|value| value.blue_bias))
            .clone()
            .unwrap_or(0.0_f64),
        alpha_bias: (opts.as_ref().and_then(|value| value.alpha_bias))
            .clone()
            .unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:62 (sha256:603a5da1aa69a6ade166bb03d80147dc6cbaa201fd7e48c929ec497414f63279)
pub fn equals_color_scale_bias(a: &ColorScaleBiasLike, b: &ColorScaleBiasLike) -> bool {
    return (equals_color_scale_bias_biases(a, b, None))
        && (equals_color_scale_bias_scales(a, b, None));
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:66 (sha256:674a178c33bab2ec06f8bd61c34eb47b18f8cfa5294d4e287eea56960b8ff6b2)
pub fn equals_color_scale_bias_biases(
    a: &ColorScaleBiasLike,
    b: &ColorScaleBiasLike,
    compare_alpha: Option<bool>,
) -> bool {
    let compare_alpha = compare_alpha.unwrap_or(true);
    return (((a.red_bias == b.red_bias) && (a.green_bias == b.green_bias))
        && (a.blue_bias == b.blue_bias))
        && ((!compare_alpha) || (a.alpha_bias == b.alpha_bias));
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:79 (sha256:bb5176ee29d804d9b16018cbee12f94866f37048bfe48b5a7665f2272c8682be)
pub fn equals_color_scale_bias_scales(
    a: &ColorScaleBiasLike,
    b: &ColorScaleBiasLike,
    compare_alpha: Option<bool>,
) -> bool {
    let compare_alpha = compare_alpha.unwrap_or(true);
    return (((a.red_scale == b.red_scale) && (a.green_scale == b.green_scale))
        && (a.blue_scale == b.blue_scale))
        && ((!compare_alpha) || (a.alpha_scale == b.alpha_scale));
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:92 (sha256:204c23236166e260f511cc498c6b6edead2da84c00850722fd25042aa1bae50b)
pub fn get_color_scale_bias_bias_rgb(source: &ColorScaleBiasLike) -> f64 {
    return (__flight_js_to_i32(
        (__flight_js_to_i32(
            __flight_js_to_i32((source.red_bias * 255.0_f64).round())
                .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64,
        ) | __flight_js_to_i32(
            __flight_js_to_i32((source.green_bias * 255.0_f64).round())
                .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
        )) as f64,
    ) | __flight_js_to_i32((source.blue_bias * 255.0_f64).round())) as f64;
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:100 (sha256:c71b2144b4f5c2dc9fad870c0c42262b284a83a89cae5beed524962671011b71)
pub fn get_color_scale_bias_bias_rgba(source: &ColorScaleBiasLike) -> f64 {
    return (__flight_js_to_i32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                __flight_js_to_i32((source.red_bias * 255.0_f64).round())
                    .wrapping_shl((__flight_js_to_u32(24.0_f64) & 31)) as f64,
            ) | __flight_js_to_i32(
                __flight_js_to_i32((source.green_bias * 255.0_f64).round())
                    .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64,
            )) as f64,
        ) | __flight_js_to_i32(
            __flight_js_to_i32((source.blue_bias * 255.0_f64).round())
                .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
        )) as f64,
    ) | __flight_js_to_i32((source.alpha_bias * 255.0_f64).round())) as f64;
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:109 (sha256:64d500c7d0fd0993c8b7afbf3065d398b7ac20c631a8665647dcbaad8fa58017)
pub fn invert_color_scale_bias(out: &mut ColorScaleBiasLike, source: &ColorScaleBiasLike) -> () {
    out.red_scale = if (source.red_scale != 0.0_f64) {
        (1.0_f64 / source.red_scale)
    } else {
        1.0_f64
    };
    out.green_scale = if (source.green_scale != 0.0_f64) {
        (1.0_f64 / source.green_scale)
    } else {
        1.0_f64
    };
    out.blue_scale = if (source.blue_scale != 0.0_f64) {
        (1.0_f64 / source.blue_scale)
    } else {
        1.0_f64
    };
    out.alpha_scale = if (source.alpha_scale != 0.0_f64) {
        (1.0_f64 / source.alpha_scale)
    } else {
        1.0_f64
    };
    out.red_bias = (-source.red_bias);
    out.green_bias = (-source.green_bias);
    out.blue_bias = (-source.blue_bias);
    out.alpha_bias = (-source.alpha_bias);
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:120 (sha256:779bb7789589579f964efde0ed21f96a982370e0ebb791b072a3f86869e45119)
pub fn is_identity_color_scale_bias(
    source: &ColorScaleBiasLike,
    compare_alpha_scale: Option<bool>,
) -> bool {
    let compare_alpha_scale = compare_alpha_scale.unwrap_or(true);
    return (equals_color_scale_bias_biases(
        source,
        &{
            let __flight_source = &(_IDENTITY);
            ColorScaleBiasLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                alpha_scale: __flight_source.alpha_scale,
                alpha_bias: __flight_source.alpha_bias,
                blue_scale: __flight_source.blue_scale,
                blue_bias: __flight_source.blue_bias,
                green_scale: __flight_source.green_scale,
                green_bias: __flight_source.green_bias,
                red_scale: __flight_source.red_scale,
                red_bias: __flight_source.red_bias,
            }
        },
        None,
    )) && (equals_color_scale_bias_scales(
        source,
        &{
            let __flight_source = &(_IDENTITY);
            ColorScaleBiasLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                alpha_scale: __flight_source.alpha_scale,
                alpha_bias: __flight_source.alpha_bias,
                blue_scale: __flight_source.blue_scale,
                blue_bias: __flight_source.blue_bias,
                green_scale: __flight_source.green_scale,
                green_bias: __flight_source.green_bias,
                red_scale: __flight_source.red_scale,
                red_bias: __flight_source.red_bias,
            }
        },
        Some(compare_alpha_scale),
    ));
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:129 (sha256:d215e86aae4d1d0201568c2e695760587fad3248cd4a3739d0f0930d0d967b73)
pub fn set_color_scale_bias(
    out: &mut ColorScaleBiasLike,
    red_scale: f64,
    green_scale: f64,
    blue_scale: f64,
    alpha_scale: f64,
    red_bias: f64,
    green_bias: f64,
    blue_bias: f64,
    alpha_bias: f64,
) -> () {
    out.red_scale = red_scale;
    out.green_scale = green_scale;
    out.blue_scale = blue_scale;
    out.alpha_scale = alpha_scale;
    out.red_bias = red_bias;
    out.green_bias = green_bias;
    out.blue_bias = blue_bias;
    out.alpha_bias = alpha_bias;
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:150 (sha256:40006d44a2c4a35009ea8e3cdf2d04706310e89c63a83daedcafef6d83966ce5)
pub fn set_color_scale_bias_bias_rgb(out: &mut ColorScaleBiasLike, value: f64) -> () {
    out.red_bias = ((__flight_js_to_i32(
        (__flight_js_to_i32(value) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    out.green_bias = ((__flight_js_to_i32(
        (__flight_js_to_i32(value) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    out.blue_bias =
        ((__flight_js_to_i32(value) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64);
    out.alpha_bias = 0.0_f64;
    out.red_scale = 0.0_f64;
    out.green_scale = 0.0_f64;
    out.blue_scale = 0.0_f64;
    out.alpha_scale = 1.0_f64;
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:161 (sha256:4de3c71e0eaa6916459ef831c518657b0b2b9cb23a729df61511f711cc91cd43)
pub fn set_color_scale_bias_bias_rgba(out: &mut ColorScaleBiasLike, value: f64) -> () {
    out.red_bias = ((__flight_js_to_i32(
        (__flight_js_to_i32(value) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    out.green_bias = ((__flight_js_to_i32(
        (__flight_js_to_i32(value) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    out.blue_bias = ((__flight_js_to_i32(
        (__flight_js_to_i32(value) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    out.alpha_bias =
        ((__flight_js_to_i32(value) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64);
    out.red_scale = 0.0_f64;
    out.green_scale = 0.0_f64;
    out.blue_scale = 0.0_f64;
    out.alpha_scale = 0.0_f64;
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:172 (sha256:eec65f3b731660a3f6af839a364b180b37529dba0649340dff0c9d7ea026514a)
pub fn set_color_scale_bias_identity(out: &mut ColorScaleBias) -> () {
    (|| -> () {
        out.red_scale = 1.0_f64;
        out.green_scale = 1.0_f64;
        out.blue_scale = 1.0_f64;
        out.alpha_scale = 1.0_f64;
        out.red_bias = 0.0_f64;
        out.green_bias = 0.0_f64;
        out.blue_bias = 0.0_f64;
        out.alpha_bias = 0.0_f64;
    })();
}

// Source: upstream/packages/materials/src/colorScaleBias.ts:176 (sha256:446724a7984be7aed29fb9732c2e9edc32186a3f70c837ee71601ce9bb2148df)
static _IDENTITY: std::sync::LazyLock<ColorScaleBias> =
    std::sync::LazyLock::new(|| create_color_scale_bias(None));
