// @generated from upstream/packages/node/src/hasClip.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{ClipRegion, HasClip};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/node/src/hasClip.ts:3 (sha256:e2cd12ba22fcb8d40ef5c52e2b0747dd907f33dd09c170e266616d9165a452eb)
pub fn init_clip_trait(target: &mut HasClip, obj: Option<FlightPartialRecord1>) -> () {
    target.clip = obj.as_ref().and_then(|value| (value.clip).clone());
}
