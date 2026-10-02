// @generated from upstream/packages/types/src/TextShaper.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FontMetrics, GlyphExtents, ShapedRun, TextDirection, TextFormat, TextMeasureFunction};

// Source: upstream/packages/types/src/TextShaper.ts:23 (sha256:5276a896b0d7cc1055fd89e424f1238e5721d9bbabc642c70f5be6c52e2739b2)
#[derive(Clone, Default)]
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

// Source: upstream/packages/types/src/TextShaper.ts:28 (sha256:6cc5f9c1d556b57c5971f914bcee51e7ae5581f4cca38f40d10ec14a58b67c25)
#[derive(Clone)]
pub struct HostTextShaperCapability {
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
impl PartialEq for HostTextShaperCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextShaper.ts:55 (sha256:1854f297cbd0f1d55b95c8b2ab7d5f507dfa2c67c0415517c7d9410eddfc6ece)
pub type TextShaperOperation = HostTextShaperCapability;
