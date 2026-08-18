// @generated from upstream/packages/types/src/WgpuTextureResolver.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{TextureColorSpace, TextureLike, WgpuRenderState, WgpuTextureEntry};

// Source: upstream/packages/types/src/WgpuTextureResolver.ts:6 (sha256:956362770e2a0439d647805fa1992d8e2bf49ab4b80f38c21cf487bdd2805771)
pub type WgpuTextureResolver = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    WgpuRenderState,
                    TextureLike,
                    bool,
                    TextureColorSpace,
                ) -> Option<WgpuTextureEntry>
                + Send
                + 'static,
        >,
    >,
>;
