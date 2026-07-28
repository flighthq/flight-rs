// @generated from upstream/packages/types/src/TextShaperOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FontVariation, TextDirection, TextFeature};

// Source: upstream/packages/types/src/TextShaperOptions.ts:4 (sha256:f41ba551f122e03a7501256d4ed0a1391ee63f6eee999ff52f9a2bf455ef110e)
#[derive(Clone, Default)]
pub struct TextShaperOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub direction: Option<TextDirection>,
    pub features: Option<Vec<TextFeature>>,
    pub language: Option<String>,
    pub script: Option<String>,
    pub variations: Option<Vec<FontVariation>>,
}
impl PartialEq for TextShaperOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
