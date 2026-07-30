// @generated from upstream/packages/types/src/PosterizeEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/PosterizeEffect.ts:3 (sha256:10a155b5a8da3a8782430345e3ba7d32a9d58f22cbdd1d0099b3cc9e55057711)
#[derive(Clone, Default)]
pub struct PosterizeEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub levels: Option<f64>,
}
impl PartialEq for PosterizeEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
