// @generated from upstream/packages/types/src/Transform3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Quaternion, Vector3};

// Source: upstream/packages/types/src/Transform3D.ts:10 (sha256:aa17fd420e35972ab44fa60b948ff2ace8dfbdda06715a1b77ca9104b708378f)
#[derive(Clone)]
pub struct Transform3D {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

// Source: upstream/packages/types/src/Transform3D.ts:16 (sha256:01c7da482785274cce44537c94c6df42d29ffcf4ebcc14ef58e3ec0807d8cd9f)
pub type Transform3DLike = Transform3D;
