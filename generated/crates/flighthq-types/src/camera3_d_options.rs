// @generated from upstream/packages/types/src/Camera3DOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Projection;

// Source: upstream/packages/types/src/Camera3DOptions.ts:4 (sha256:1b6e166caabd6ba14eb054d0fc9df1efd93be3b2590e689bbb5cac95bf42c77f)
#[derive(Clone)]
pub struct Camera3DOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub far: f64,
    pub near: f64,
    pub projection: Projection,
}
impl PartialEq for Camera3DOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
