// @generated from upstream/packages/types/src/Scene2DRenderExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, Scene2DRenderBlankReason};

// Source: upstream/packages/types/src/Scene2DRenderExplanation.ts:8 (sha256:2bd81882dde1f622bb072765dbcd62757c0f35bd6dee16f96739ffb072a846a8)
#[derive(Clone, Default)]
pub struct Scene2DRenderExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub has_renderer: bool,
    pub prepared: bool,
    pub visible: bool,
    pub effective_alpha: f64,
    pub reason: Scene2DRenderBlankReason,
}
impl PartialEq for Scene2DRenderExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
