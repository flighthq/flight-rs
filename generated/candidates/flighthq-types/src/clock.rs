// @generated from upstream/packages/types/src/Clock.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Clock.ts:8 (sha256:e6e95909db1bea0affe3369897e0632ad4f455db8211a678d4d881f01d456a9b)
#[derive(Clone, Default)]
pub struct Clock {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub scale: f64,
    pub paused: bool,
    pub delta_time: f64,
    pub elapsed: f64,
    pub parent: Option<Box<Clock>>,
    pub children: Vec<Clock>,
    pub on_tick: Option<
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    >,
}
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
