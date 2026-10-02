// @generated from upstream/packages/types/src/TextLayout.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, TextAutoSize, TextDirection, TextFormat, TextFormatRange, TextJustification,
    TextVerticalAlign,
};

// Source: upstream/packages/types/src/TextLayout.ts:9 (sha256:5cc4820d45dba5a294eaae15256981669731351902beb6cefe2dd00108a0293b)
pub type TextMeasureFunction =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, TextFormat) -> f64 + Send + 'static>>>;

// Source: upstream/packages/types/src/TextLayout.ts:11 (sha256:6e7eab8abfe4dd89b536307fee94cb0b4537d1c56ac67eadd4af3676475bc85f)
#[derive(Clone, Default)]
pub struct TextLayoutGroup {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub ascent: f64,
    pub descent: f64,
    pub end_index: f64,
    pub format: TextFormat,
    pub height: f64,
    pub leading: f64,
    pub line_index: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub positions: Vec<f64>,
    pub start_index: f64,
    pub width: f64,
}
impl PartialEq for TextLayoutGroup {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for TextLayoutGroup {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/TextLayout.ts:27 (sha256:1f2c95acb12ba7582d7411b09d418cf403f8a1cfd850662b185e4c344cecdd40)
#[derive(Clone)]
pub struct TextLayoutParams {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub auto_size: Option<TextAutoSize>,
    pub border: Option<bool>,
    pub direction: Option<TextDirection>,
    pub format_ranges: Vec<TextFormatRange>,
    pub height: f64,
    pub justification: Option<TextJustification>,
    pub max_lines: Option<f64>,
    pub measure: TextMeasureFunction,
    pub multiline: Option<bool>,
    pub text: String,
    pub truncation_character: Option<String>,
    pub vertical_align: Option<TextVerticalAlign>,
    pub width: f64,
    pub word_wrap: Option<bool>,
}
impl PartialEq for TextLayoutParams {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextLayout.ts:50 (sha256:5789a9f4e46e1d43c6d18e8d1e328ea00b0fa66b89757a1cb1754d82fa29d0d2)
#[derive(Clone, Default)]
pub struct TextLayoutResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub groups: Vec<TextLayoutGroup>,
    pub line_ascents: Vec<f64>,
    pub line_descents: Vec<f64>,
    pub line_heights: Vec<f64>,
    pub line_leadings: Vec<f64>,
    pub line_widths: Vec<f64>,
    pub num_lines: f64,
    pub text_height: f64,
    pub text_width: f64,
}
impl PartialEq for TextLayoutResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for TextLayoutResult {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}
