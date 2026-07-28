// @generated from upstream/packages/types/src/ImageResourceCompressed.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextureContainer;

// Source: upstream/packages/types/src/ImageResourceCompressed.ts:11 (sha256:0bbed5fa912595338b4bbf39d199d4f209dd50c8e10b1caafc708a883c0639e4)
#[derive(Clone)]
pub struct ImageResourceCompressed {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub container: TextureContainer,
    pub payload: Vec<u8>,
}
impl PartialEq for ImageResourceCompressed {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
