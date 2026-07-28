// @generated from upstream/packages/types/src/TimelineLabel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TimelineLabel.ts:1 (sha256:f1bcd87631389ed349560d3d4adc78ee8835215d5e36b96236cc9169f128a773)
#[derive(Clone)]
pub struct TimelineLabel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: f64,
    pub name: String,
}
impl PartialEq for TimelineLabel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
