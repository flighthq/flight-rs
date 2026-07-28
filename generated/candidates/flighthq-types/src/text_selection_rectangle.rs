// @generated from upstream/packages/types/src/TextSelectionRectangle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextSelectionRectangle.ts:1 (sha256:8b127f9af8b5c5c504869aff4b368a55038b3df041dcc04c392bae8aed708e39)
#[derive(Clone)]
pub struct TextSelectionRectangle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub line_index: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for TextSelectionRectangle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
