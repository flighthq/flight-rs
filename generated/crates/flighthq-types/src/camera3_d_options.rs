// @generated from upstream/packages/types/src/Camera3DOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Plane, Projection};

// Source: upstream/packages/types/src/Camera3DOptions.ts:5 (sha256:8a3e02c24c3425cd71675f3fb34813a3da9b4ec1c78630de376c56cbf2b5ec15)
#[derive(Clone)]
pub struct Camera3DOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub far: f64,
    pub near: f64,
    pub near_clip_plane: Option<Plane>,
    pub projection: Projection,
}
impl PartialEq for Camera3DOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
