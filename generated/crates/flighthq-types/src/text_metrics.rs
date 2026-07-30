// @generated from upstream/packages/types/src/TextMetrics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextMetrics.ts:5 (sha256:7f31b4bdfeab17b6c516914031a3edd6f4ba0572631550ebbcd7b90d2e985680)
#[derive(Clone, Default)]
pub struct TextMetrics {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub width: f64,
    pub height: f64,
    pub num_lines: f64,
}
impl PartialEq for TextMetrics {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
