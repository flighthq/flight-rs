// @generated from upstream/packages/types/src/ImageDecodeOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ImageDecodeOptions.ts:4 (sha256:1ee8eaf9149c956934b05aa13ef390bdae7a9255bb5d323e7beb0417f7094467)
#[derive(Clone, Default)]
pub struct ImageDecodeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub premultiply_alpha: Option<bool>,
}
impl PartialEq for ImageDecodeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
