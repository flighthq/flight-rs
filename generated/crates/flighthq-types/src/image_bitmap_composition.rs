// @generated from upstream/packages/types/src/ImageBitmapComposition.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Bitmap, DecodedImage};

// Source: upstream/packages/types/src/ImageBitmapComposition.ts:8 (sha256:a375fbc834f0605ee8364ac92dcd3571cf293e6ed1b7b9f0baaf2c9292070b34)
#[derive(Clone, Default)]
pub struct ImageBitmapComposition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub payload: Vec<u8>,
}
impl PartialEq for ImageBitmapComposition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ImageBitmapComposition.ts:13 (sha256:4952ea861a6abaed572a59b533d932a04d6f8bc7235e65d5f47c7679b5df896e)
pub type ImageBitmapComposer = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(Option<DecodedImage>, Vec<u8>) -> Option<Bitmap> + Send + 'static>,
    >,
>;
