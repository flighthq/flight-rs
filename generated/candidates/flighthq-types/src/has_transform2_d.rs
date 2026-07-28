// @generated from upstream/packages/types/src/HasTransform2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, Matrix, NodeData};

// Source: upstream/packages/types/src/HasTransform2D.ts:5 (sha256:c9b731b05f4576401a528e7854ebe8e3994a7f1b690b6ff43de196725926a3de)
#[derive(Clone)]
pub struct HasTransform2D {
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub skew_x: f64,
    pub skew_y: f64,
    pub x: f64,
    pub y: f64,
}

// Source: upstream/packages/types/src/HasTransform2D.ts:22 (sha256:96c70f861965d4d1f68717a0a39a14317f7304603f79b6048c18a9291671a96a)
#[derive(Clone)]
pub struct HasTransform2DRuntime {
    pub binding: Option<crate::OpaqueHostValue>,
    pub local_matrix: Option<Matrix>,
    pub rotation_angle: f64,
    pub rotation_cosine: f64,
    pub rotation_sine: f64,
    pub world_matrix: Option<Matrix>,
}

// Source: upstream/packages/types/src/HasTransform2D.ts:30 (sha256:c744a508e3181c3c31e026d800f8f5928e2793b375bf32ecd637815510d9ab88)
#[derive(Clone)]
pub struct Transform2DNode {
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub skew_x: f64,
    pub skew_y: f64,
    pub x: f64,
    pub y: f64,
}
