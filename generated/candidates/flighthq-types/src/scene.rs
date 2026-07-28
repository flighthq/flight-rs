// @generated from upstream/packages/types/src/Scene.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, SceneMetadata, SceneNode};

// Source: upstream/packages/types/src/Scene.ts:18 (sha256:7adc62c6c88e27dc94837c206a741700dedd5c8a88e4b1213ee17ff202285c66)
#[derive(Clone)]
pub struct Scene {
    pub animations: crate::OpaqueHostValue,
    pub metadata: Option<SceneMetadata>,
    pub root: SceneNode,
}

// Source: upstream/packages/types/src/Scene.ts:24 (sha256:a12ec2b057d5fce567435f21acd5d9f22bbc5152fc445e41b62b2fd87aa70e3f)
pub type SceneRuntime = EntityRuntime;
