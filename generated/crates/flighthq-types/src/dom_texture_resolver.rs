// @generated from upstream/packages/types/src/DomTextureResolver.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DomRenderState, Texture};

// Source: upstream/packages/types/src/DomTextureResolver.ts:4 (sha256:68cb0b3617d4e172e85b7d951994a056999e1f31a4d4359407da2aec5ef251e2)
pub type DomTextureResolver = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(DomRenderState, Texture) -> Option<crate::OpaqueHostValue> + Send + 'static>,
    >,
>;
