// @generated from upstream/packages/types/src/UnlitMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode, Texture, VideoTexture};

// Source: upstream/packages/types/src/UnlitMaterial.ts:14 (sha256:6df48f0e486e10bfb7ea85f40498e60724ef3d803a7b97615b6626f222268f4a)
#[derive(Clone)]
pub struct UnlitMaterial {
    pub kind: Kind,
    pub name: Option<Option<String>>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub base_color: f64,
    pub base_color_map: Option<Texture>,
    pub base_color_video_map: Option<VideoTexture>,
}

// Source: upstream/packages/types/src/UnlitMaterial.ts:20 (sha256:109f384517d58d45451b8b52b524915150578f99c842ef3011020beedefaf39d)
pub const UNLIT_MATERIAL_KIND: &'static str = "UnlitMaterial";
