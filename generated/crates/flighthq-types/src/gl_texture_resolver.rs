// @generated from upstream/packages/types/src/GlTextureResolver.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlRenderState, TextureColorSpace, TextureLike};

// Source: upstream/packages/types/src/GlTextureResolver.ts:6 (sha256:2434f9a1fb2037a05dab7e3c814cd4f5aad43173de1d59711c7a9cb19d3a504f)
#[derive(Clone, Default)]
pub struct GlTextureRealization {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub straight_alpha: bool,
    pub texture: crate::OpaqueHostValue,
}
impl PartialEq for GlTextureRealization {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlTextureResolver.ts:14 (sha256:b81b0df7443167d6bc31c38121b1128fd68110782876016f020dcbd770080b67)
pub type GlTextureResolver = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    GlRenderState,
                    TextureLike,
                    bool,
                    TextureColorSpace,
                ) -> Option<GlTextureRealization>
                + Send
                + 'static,
        >,
    >,
>;
