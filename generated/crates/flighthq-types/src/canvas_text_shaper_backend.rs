// @generated from upstream/packages/types/src/CanvasTextShaperBackend.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    FontMetrics, GlyphExtents, ShapeRunOptions, ShapedRun, TextFormat, TextMeasureFunction,
};

// Source: upstream/packages/types/src/CanvasTextShaperBackend.ts:10 (sha256:55e7bdd049de1dd446683b3d609a45e20979af963235f936f5c2a994128223d9)
#[derive(Clone)]
pub struct CanvasTextShaperBackend {
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
    pub clear_cache: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for CanvasTextShaperBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
