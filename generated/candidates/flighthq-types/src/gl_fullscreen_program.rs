// @generated from upstream/packages/types/src/GlFullscreenProgram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlFullscreenProgram.ts:5 (sha256:4a6e4eb3b93808f0ccd4b27918297fc8be16015058c903937fdf0a3c7bf69ae4)
#[derive(Clone)]
pub struct GlFullscreenProgram {
    pub program: crate::OpaqueHostValue,
    pub loc_position: f64,
    pub loc_tex_coord: f64,
    pub texture: crate::OpaqueHostValue,
    pub textures: Vec<crate::OpaqueHostValue>,
}
