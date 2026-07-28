// @generated from upstream/packages/node/src/hasBoundsRectangle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{BoundsNodeAny, HasBoundsRectangle, HasBoundsRectangleRuntime, Rectangle};

// Source: upstream/packages/node/src/hasBoundsRectangle.ts:9 (sha256:0a6fbecbed7383cf4426dbf1a642f6682020f47c74e087613d23ea6d7817f52a)
pub fn default_compute_local_bounds_rectangle(_out: &Rectangle, _source: &BoundsNodeAny) -> () {}

// Source: upstream/packages/node/src/hasBoundsRectangle.ts:11 (sha256:af4df04f5896c85229e434e242cf2be38af99a1a2d8347b1f6ff67f403cc7dab)
pub fn init_bounds_rectangle_runtime_trait(
    target: &mut HasBoundsRectangleRuntime,
    methods: Option<HasBoundsRectangleRuntime>,
) -> () {
    target.bounds_rectangle = None;
    target.local_bounds_rectangle = None;
    target.world_bounds_rectangle = None;
    target.compute_local_bounds_rectangle = (methods
        .as_ref()
        .map(|value| (value.compute_local_bounds_rectangle).clone()))
    .unwrap_or(default_compute_local_bounds_rectangle);
}

// Source: upstream/packages/node/src/hasBoundsRectangle.ts:21 (sha256:1ccd3f52d80184fb2202c6f71d2541f57e192b89702ee883c017c80c113fb07c)
pub fn init_bounds_rectangle_trait(
    _target: &HasBoundsRectangle,
    _obj: Option<HasBoundsRectangle>,
) -> () {
}
