// @generated from upstream/packages/types/src/RenderBlendStateEntry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::BlendMode;

// Source: upstream/packages/types/src/RenderBlendStateEntry.ts:2 (sha256:2f3548c73cc3520a34ae971cf2758535e84b777be11d4e13d56379f04beb3827)
#[derive(Clone, Default)]
pub struct RenderBlendStateEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for RenderBlendStateEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
