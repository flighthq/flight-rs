// @generated from upstream/packages/types/src/GlTextureResolver.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlRenderState, TextureColorSpace, TextureLike};

// Source: upstream/packages/types/src/GlTextureResolver.ts:6 (sha256:b87e5dc116fda44a01c31c803494e46cb7ac1f684a3ab654293a97a1d36ad692)
pub type GlTextureResolver = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    GlRenderState,
                    TextureLike,
                    bool,
                    TextureColorSpace,
                ) -> Option<crate::OpaqueHostValue>
                + Send
                + 'static,
        >,
    >,
>;
