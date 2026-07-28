// @generated from upstream/packages/types/src/TextShaper.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FontMetrics, GlyphExtents, ShapedRun, TextDirection, TextFormat, TextMeasureFunction};

// Source: upstream/packages/types/src/TextShaper.ts:21 (sha256:5276a896b0d7cc1055fd89e424f1238e5721d9bbabc642c70f5be6c52e2739b2)
#[derive(Clone)]
pub struct ShapeRunOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub direction: Option<TextDirection>,
    pub script: Option<String>,
}
impl PartialEq for ShapeRunOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextShaper.ts:26 (sha256:a6d76855c342cc710304eff6c3034f16a3853a751ee704d881de32f764d3c047)
#[derive(Clone)]
pub struct TextShaperBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_code_point_for_glyph:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>>,
    pub get_font_metrics: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(TextFormat) -> Option<FontMetrics> + Send + 'static>>,
        >,
    >,
    pub get_glyph_extents: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(f64) -> Option<GlyphExtents> + Send + 'static>>,
        >,
    >,
    pub get_glyph_index_for_code_point:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>>,
    pub get_glyph_name:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> String + Send + 'static>>>>,
    pub measure_text: TextMeasureFunction,
    pub shape_run: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(String, TextFormat, Option<ShapeRunOptions>) -> ShapedRun
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for TextShaperBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
