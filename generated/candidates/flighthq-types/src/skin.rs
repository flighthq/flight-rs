// @generated from upstream/packages/types/src/Skin.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{SceneNode, Skeleton3D};

// Source: upstream/packages/types/src/Skin.ts:13 (sha256:4237493d27e6444829b2009db82e94785c1ed46e5e9d134ec1b288566e759151)
#[derive(Clone)]
pub struct Skin {
    pub skeleton: Skeleton3D,
    pub skeleton_root: Option<Option<SceneNode>>,
}
