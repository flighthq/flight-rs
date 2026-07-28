// @generated from upstream/packages/types/src/Camera.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Matrix4, Vector2};

// Source: upstream/packages/types/src/Camera.ts:12 (sha256:5b25d9f44809b168101850d099cc8f5e2ff9f30356638216b00fbd21e08fce43)
#[derive(Clone)]
pub struct Camera {
    pub far: f64,
    pub inverse_view_projection: Matrix4,
    pub jitter: Vector2,
    pub near: f64,
    pub projection: Projection,
    pub view: Matrix4,
}

// Source: upstream/packages/types/src/Camera.ts:21 (sha256:b7d45d66db148544041c5acf5f092944f4890be57ce36e2b8b2cce26d507c557)
pub type CameraLike = Camera;

// Source: upstream/packages/types/src/Camera.ts:24 (sha256:21f9a9a587e79b904172f7c12a21872323b14399b691065d3e5e2227b3d88e20)
pub type Projection = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Camera.ts:28 (sha256:e3adb55f5918c927447f09149b2a6a9d9abee406e4abe3295085b6e634017e2a)
#[derive(Clone)]
pub struct PerspectiveProjection {
    pub aspect: f64,
    pub fov_y: f64,
    pub kind: String,
}

// Source: upstream/packages/types/src/Camera.ts:36 (sha256:e14f2b29ed5efd13226a5e9ca3bfc45e4904ee84f8ff0c7839a3de1b4e006497)
#[derive(Clone)]
pub struct OrthographicProjection {
    pub half_height: f64,
    pub half_width: f64,
    pub kind: String,
}
