// @generated from upstream/packages/types/src/Frustum.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Plane;

// Source: upstream/packages/types/src/Frustum.ts:8 (sha256:fa6d32fec091d67a461443a09bc7c418115ac3c6a4b4c8bdb1abd75272621e41)
#[derive(Clone)]
pub struct Frustum {
    pub bottom: Plane,
    pub far: Plane,
    pub left: Plane,
    pub near: Plane,
    pub right: Plane,
    pub top: Plane,
}

// Source: upstream/packages/types/src/Frustum.ts:17 (sha256:1494fa0d0979eeca28f09bd1a2d895bfb79f2463ca45ae2e81d3935f178b90b1)
pub type FrustumLike = Frustum;
