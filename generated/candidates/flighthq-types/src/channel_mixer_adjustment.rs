// @generated from upstream/packages/types/src/ChannelMixerAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ChannelMixerAdjustment.ts:3 (sha256:84da0f283936b20e51ba76eccd3f2f24d92874106d702c51515bd990ba2803fd)
#[derive(Clone)]
pub struct ChannelMixerAdjustment {
    pub kind: String,
    pub color_matrix: Vec<f64>,
    pub matrix: Vec<f64>,
}
