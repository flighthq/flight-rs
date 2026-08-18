// @generated from upstream/packages/types/src/Bone2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TransformInherit2D;

// Source: upstream/packages/types/src/Bone2D.ts:15 (sha256:3d10fda7097f7f264675ff2fbbe74d434ff166d3d06a571aa99cb9af4342d4f3)
#[derive(Clone, Default)]
pub struct Bone2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub length: f64,
    pub name: Option<String>,
    pub parent_index: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub shear_x: f64,
    pub shear_y: f64,
    pub transform_mode: TransformInherit2D,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for Bone2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
