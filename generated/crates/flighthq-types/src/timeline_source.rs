// @generated from upstream/packages/types/src/TimelineSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DisplayObject, TimelineLabel};

// Source: upstream/packages/types/src/TimelineSource.ts:9 (sha256:ed9f85e1a3d8c220c97936c0f322a12da85eb73e0db48b10c730af3c87475db8)
#[derive(Clone)]
pub struct TimelineSource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub total_frames: f64,
    pub labels: Vec<TimelineLabel>,
    pub frame_rate: Option<f64>,
    pub construct_frame:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(DisplayObject, f64) -> () + Send + 'static>>>,
}
impl PartialEq for TimelineSource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
