// @generated from upstream/packages/types/src/SceneAnimationTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{SceneAnimationPath, SceneNode};

// Source: upstream/packages/types/src/SceneAnimationTarget.ts:9 (sha256:01ab1226d7ebad39e82c031c642a554df71ce554250b32dd76289a8ee735cabc)
#[derive(Clone)]
pub struct SceneAnimationTarget {
    pub node: SceneNode,
    pub path: SceneAnimationPath,
}
