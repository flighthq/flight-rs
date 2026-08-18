// @generated from upstream/packages/types/src/TimelineSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Node2D, TimelineCue, TimelineLabel};

// Source: upstream/packages/types/src/TimelineSource.ts:10 (sha256:17712e4a87bf614b85d2bb419f840da5621e57c5138477cc68f719c95afd2ccb)
#[derive(Clone)]
pub struct TimelineSource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub total_frames: f64,
    pub labels: Vec<TimelineLabel>,
    pub cues: Vec<TimelineCue>,
    pub frame_rate: Option<f64>,
    pub construct_frame:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node2D, f64) -> () + Send + 'static>>>,
}
impl PartialEq for TimelineSource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
