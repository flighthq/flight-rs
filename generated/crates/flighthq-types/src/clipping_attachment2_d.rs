// @generated from upstream/packages/types/src/ClippingAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Skin2D;

// Source: upstream/packages/types/src/ClippingAttachment2D.ts:16 (sha256:069b4df44a3486606cce30493977aebffc60f2fba30fa4e33ea784fbd06ba962)
#[derive(Clone, Default)]
pub struct ClippingAttachment2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub name: Option<String>,
    pub end_slot_index: f64,
    pub point_count: f64,
    pub skin: Option<Skin2D>,
    pub vertices: Option<Vec<f32>>,
}
impl PartialEq for ClippingAttachment2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ClippingAttachment2D.ts:25 (sha256:aa2e7c96f1a7faa09e9a2d5d247839c66c3c6d558cf638515792296c1cca265d)
pub const CLIPPING_ATTACHMENT2_D_KIND: &'static str = "ClippingAttachment2D";
