// @generated from upstream/packages/types/src/ChannelMixerAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::AdjustmentKind;

// Source: upstream/packages/types/src/ChannelMixerAdjustment.ts:3 (sha256:84da0f283936b20e51ba76eccd3f2f24d92874106d702c51515bd990ba2803fd)
#[derive(Clone, Default)]
pub struct ChannelMixerAdjustment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: AdjustmentKind,
    pub color_matrix: Vec<f64>,
    pub matrix: Vec<f64>,
}
impl PartialEq for ChannelMixerAdjustment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
