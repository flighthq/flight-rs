// @generated from upstream/packages/types/src/Spritesheet.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{SpritesheetFrame, TextureAtlas};

// Source: upstream/packages/types/src/Spritesheet.ts:6 (sha256:da9195606c447ce0db19f20527419fc6269a289464dc737abd92885efe9e69d3)
#[derive(Clone)]
pub struct Spritesheet {
    pub atlas: Option<TextureAtlas>,
    pub animations: crate::OpaqueHostValue,
    pub frames: Vec<SpritesheetFrame>,
}
