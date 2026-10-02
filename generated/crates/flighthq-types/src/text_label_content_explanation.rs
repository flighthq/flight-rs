// @generated from upstream/packages/types/src/TextLabelContentExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextLabelContentExplanation.ts:1 (sha256:84aa77622f772746c3239ca262cfcd23b12ba939f7345ed396d28b52be0ada65)
#[derive(Clone, Default)]
pub struct TextLabelContentExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub agreement: bool,
    pub live_string: String,
    pub rasterized_string: Option<String>,
    pub revision: f64,
}
impl PartialEq for TextLabelContentExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
