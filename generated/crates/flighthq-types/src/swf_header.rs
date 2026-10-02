// @generated from upstream/packages/types/src/SwfHeader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SwfTagRectangle;

// Source: upstream/packages/types/src/SwfHeader.ts:10 (sha256:c9e8f3e5bd2af775c8544e5f901cf903198c251127da37acabc386ac0a9309fa)
#[derive(Clone, Default)]
pub struct SwfHeader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub file_length: f64,
    pub frame_rate: f64,
    pub stage_bounds: SwfTagRectangle,
    pub version: f64,
}
impl PartialEq for SwfHeader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
