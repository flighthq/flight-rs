// @generated from upstream/packages/textlayout/src/textBounds.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TEXT_LAYOUT_GUTTER as text_layout_gutter_constant;
use flighthq_types::{RectangleLike, TextAutoSize, TextLayoutResult};

// Source: upstream/packages/textlayout/src/textBounds.ts:9 (sha256:22ed1f7a16012f2378cd9af1f778a4fb162eded85e201f8d1ffe4d26c4d6dcde)
#[derive(Clone)]
pub struct TextBoundsSpec {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub auto_size: TextAutoSize,
    pub height: f64,
    pub width: f64,
    pub word_wrap: Option<bool>,
}
impl PartialEq for TextBoundsSpec {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textlayout/src/textBounds.ts:14 (sha256:0a465566c529ecc09afc3f4a09be8e63899d6ee588f91693e0706b8e0b0994af)
pub const TEXT_BOUNDS_GUTTER: f64 = text_layout_gutter_constant;

// Source: upstream/packages/textlayout/src/textBounds.ts:18 (sha256:e54c19db0044d6e7ca3686e825f5632db29a4bb434a89327cc8f557112a18008)
pub fn compute_text_bounds_height(spec: &TextBoundsSpec, layout: &TextLayoutResult) -> f64 {
    if ((spec.auto_size).clone() == "none") {
        return spec.height;
    }
    return (layout.text_height + (text_layout_gutter_constant * 2.0_f64)).ceil();
}

// Source: upstream/packages/textlayout/src/textBounds.ts:25 (sha256:2b8dae1d099182f1702736e16b2d4ec5b16f798b0805acdd65a00b9b0b9a2e54)
pub fn compute_text_bounds_offset_x(spec: &TextBoundsSpec, layout: &TextLayoutResult) -> f64 {
    let slack = (spec.width - compute_text_bounds_width(spec, layout));
    if ((spec.auto_size).clone() == "right") {
        return slack;
    }
    if ((spec.auto_size).clone() == "center") {
        return (slack / 2.0_f64);
    }
    return 0.0_f64;
}

// Source: upstream/packages/textlayout/src/textBounds.ts:36 (sha256:418c44aa408ccbdb3f214e33a963e231cd9f2fbd69e40cc9a626f0cceba505a6)
pub fn compute_text_bounds_rectangle(
    out: &mut RectangleLike,
    spec: &TextBoundsSpec,
    layout: &TextLayoutResult,
) -> () {
    let width = compute_text_bounds_width(spec, layout);
    let slack = (spec.width - width);
    out.x = if ((spec.auto_size).clone() == "right") {
        slack
    } else {
        if ((spec.auto_size).clone() == "center") {
            (slack / 2.0_f64)
        } else {
            0.0_f64
        }
    };
    out.y = 0.0_f64;
    out.width = width;
    out.height = compute_text_bounds_height(spec, layout);
}

// Source: upstream/packages/textlayout/src/textBounds.ts:51 (sha256:e5b271e3c1ab9d01f53bf4eb111f0cffbe8c9c504e7686dedc973fbb2375b7ae)
pub fn compute_text_bounds_width(spec: &TextBoundsSpec, layout: &TextLayoutResult) -> f64 {
    if ((spec.auto_size).clone() == "none") || ((spec.word_wrap).unwrap_or(false)) {
        return spec.width;
    }
    return (layout.text_width + (text_layout_gutter_constant * 2.0_f64)).ceil();
}
