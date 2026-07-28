// @generated from upstream/packages/node/src/hasBoundsRectangle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{BoundsNodeAny, HasBoundsRectangle, HasBoundsRectangleRuntime, Rectangle};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub bounds_rectangle: Option<Rectangle>,
    pub compute_local_bounds_rectangle: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>>,
        >,
    >,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_rectangle: Option<Rectangle>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/node/src/hasBoundsRectangle.ts:9 (sha256:0a6fbecbed7383cf4426dbf1a642f6682020f47c74e087613d23ea6d7817f52a)
pub fn default_compute_local_bounds_rectangle(_out: &Rectangle, _source: &BoundsNodeAny) -> () {}

// Source: upstream/packages/node/src/hasBoundsRectangle.ts:11 (sha256:af4df04f5896c85229e434e242cf2be38af99a1a2d8347b1f6ff67f403cc7dab)
pub fn init_bounds_rectangle_runtime_trait(
    target: &mut HasBoundsRectangleRuntime,
    methods: Option<FlightPartialRecord1>,
) -> () {
    target.bounds_rectangle = None;
    target.local_bounds_rectangle = None;
    target.world_bounds_rectangle = None;
    target.compute_local_bounds_rectangle = (methods
        .as_ref()
        .and_then(|value| (value.compute_local_bounds_rectangle).clone()))
    .unwrap_or(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |__flight_argument_0: Rectangle, __flight_argument_1: BoundsNodeAny| -> () {
            default_compute_local_bounds_rectangle(&__flight_argument_0, &__flight_argument_1)
        },
    )
        as Box<
            dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static,
        >)));
}

// Source: upstream/packages/node/src/hasBoundsRectangle.ts:21 (sha256:1ccd3f52d80184fb2202c6f71d2541f57e192b89702ee883c017c80c113fb07c)
pub fn init_bounds_rectangle_trait(
    _target: &HasBoundsRectangle,
    _obj: Option<FlightPartialRecord2>,
) -> () {
}
