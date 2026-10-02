// @generated from upstream/packages/types/src/OpenTypeFontExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/OpenTypeFontExplanation.ts:6 (sha256:8e01a2562e09b92b998944396a03cee2159e792f6c7e9611026f04a2ff213931)
#[derive(Clone, Default)]
pub struct OpenTypeFontExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub accepted: bool,
    pub reason: OpenTypeFontExplanationReason,
    pub format: String,
    pub table: String,
    pub table_count: f64,
    pub readable_table_count: f64,
}
impl PartialEq for OpenTypeFontExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/OpenTypeFontExplanation.ts:51 (sha256:903343f5d236cf994f204b7cff8bd7803f8ef6a0313a0adca5d4b3ce61b4e7a9)
pub type OpenTypeFontExplanationReason = String;
