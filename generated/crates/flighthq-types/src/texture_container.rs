// @generated from upstream/packages/types/src/TextureContainer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{TextureContainerFormat, TextureContainerLevel, TextureContainerSupercompression};

// Source: upstream/packages/types/src/TextureContainer.ts:18 (sha256:8ec3670c4d9138ddabd2f31f44282b7a63234e7b28abfeaaa79fb46b60386ac4)
#[derive(Clone, Default)]
pub struct TextureContainer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: TextureContainerFormat,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub mip_levels: f64,
    pub layers: f64,
    pub faces: f64,
    pub supercompression: TextureContainerSupercompression,
    pub levels: Vec<TextureContainerLevel>,
}
impl PartialEq for TextureContainer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
