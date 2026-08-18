// @generated from upstream/packages/types/src/TextShaperCache.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ShapedRun;

// Source: upstream/packages/types/src/TextShaperCache.ts:3 (sha256:5d7a96c46bd25c288ba6c93b68f325d2de58903685861d31f41fb4f82c50fd92)
#[derive(Clone, Default)]
pub struct TextShaperCache {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub _entries: Vec<(String, ShapedRun)>,
}
impl PartialEq for TextShaperCache {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
