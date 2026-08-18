// @generated from upstream/packages/types/src/StarlingSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpritesheetData;

// Source: upstream/packages/types/src/StarlingSchema.ts:8 (sha256:0a07450aff4443b4f3ab1a91e0e1098a4acdb91feebaf00239502cc0073c0998)
#[derive(Clone, Default)]
pub struct StarlingSubTexture {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame_height: Option<f64>,
    pub frame_width: Option<f64>,
    pub frame_x: Option<f64>,
    pub frame_y: Option<f64>,
    pub height: f64,
    pub name: String,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: Option<bool>,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for StarlingSubTexture {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StarlingSchema.ts:35 (sha256:e0df2105fda2a70c007bca131e90b5ac26013a2d8bac56db0debdf215ab259c9)
#[derive(Clone, Default)]
pub struct StarlingDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub image_path: String,
    pub sub_textures: Vec<StarlingSubTexture>,
}
impl PartialEq for StarlingDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StarlingSchema.ts:42 (sha256:3191678c492309c55574326e7ed6e7ab1d4723c72a1ac4027140287412045688)
#[derive(Clone, Default)]
pub struct StarlingParsed {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: SpritesheetData,
    pub document: StarlingDocument,
}
impl PartialEq for StarlingParsed {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StarlingSchema.ts:47 (sha256:878671683654d4e4592e4659c258df47e41b82d8726517fd74edc8316540449f)
#[derive(Clone, Default)]
pub struct StarlingParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame_duration: Option<f64>,
}
impl PartialEq for StarlingParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
