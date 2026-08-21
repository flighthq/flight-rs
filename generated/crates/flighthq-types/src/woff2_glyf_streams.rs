// @generated from upstream/packages/types/src/Woff2GlyfStreams.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Woff2GlyfStreams.ts:5 (sha256:a464d3b4df6257129e226fa1fadfafb5d4a0c32b02aac84a101f84c5fc5ab615)
#[derive(Clone, Default)]
pub struct Woff2GlyfStreams {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bbox_stream: Vec<u8>,
    pub composite_stream: Vec<u8>,
    pub flag_stream: Vec<u8>,
    pub glyph_count: f64,
    pub glyph_stream: Vec<u8>,
    pub index_format: f64,
    pub instruction_stream: Vec<u8>,
    pub n_contour_stream: Vec<u8>,
    pub n_points_stream: Vec<u8>,
}
impl PartialEq for Woff2GlyfStreams {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
