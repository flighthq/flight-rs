// @generated from upstream/packages/types/src/MorphTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MorphTarget.ts:10 (sha256:a188aff29a2a3c80860716f2e22624be4888a384001af449f8290b49adbda94e)
#[derive(Clone, Default)]
pub struct MorphTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub normal_deltas: Option<Vec<f32>>,
    pub position_deltas: Vec<f32>,
    pub tangent_deltas: Option<Vec<f32>>,
}
impl PartialEq for MorphTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MorphTarget.ts:22 (sha256:71b4483bdbf0b4a6fc8b0b13a251315a59fc167b5ba49f7629fa1662b1cd429d)
#[derive(Clone, Default)]
pub struct MeshMorph {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub targets: Vec<MorphTarget>,
    pub weights: Vec<f32>,
}
impl PartialEq for MeshMorph {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
