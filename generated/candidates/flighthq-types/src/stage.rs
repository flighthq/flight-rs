// @generated from upstream/packages/types/src/Stage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DisplayObject, StageSignals, ViewportAlign, ViewportScaleMode};

// Source: upstream/packages/types/src/Stage.ts:12 (sha256:395313cd947e137c2d949635cc2332004f36e418c238cd420bc5beb08214657b)
#[derive(Clone, Default)]
pub struct Stage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub align: ViewportAlign,
    pub root: DisplayObject,
    pub scale_mode: ViewportScaleMode,
    pub color: Option<f64>,
    pub stage_height: f64,
    pub stage_width: f64,
}
impl PartialEq for Stage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Stage.ts:19 (sha256:2132e54a4e2c283ba473779d330f87294776820f1173f4f86d6fd606e9f39945)
#[derive(Clone, Default)]
pub struct StageRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub stage_signals: Option<StageSignals>,
}
impl PartialEq for StageRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
