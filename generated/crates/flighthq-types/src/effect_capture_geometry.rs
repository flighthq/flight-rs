// @generated from upstream/packages/types/src/EffectCaptureGeometry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EffectPadding, MatrixLike, RectangleLike};

// Source: upstream/packages/types/src/EffectCaptureGeometry.ts:5 (sha256:f3fe3ea6115ca4a36d1c6913cc66bcac827c188d35e482d18637043a6ac597d0)
#[derive(Clone, Default)]
pub struct EffectCaptureGeometry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bounds: RectangleLike,
    pub capture_transform: MatrixLike,
    pub padding: EffectPadding,
    pub target_height: f64,
    pub target_width: f64,
}
impl PartialEq for EffectCaptureGeometry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
