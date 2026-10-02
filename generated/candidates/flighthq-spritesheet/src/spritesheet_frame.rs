// @generated from upstream/packages/spritesheet/src/spritesheetFrame.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{EntityConstruction, SpritesheetFrame};

#[derive(Clone, Default)]
pub struct FlightPartialRecord3721705896 {
    pub __flight_identity: std::sync::Arc<()>,
    pub id: Option<f64>,
    pub offset_x: Option<f64>,
    pub offset_y: Option<f64>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: Option<bool>,
}
impl PartialEq for FlightPartialRecord3721705896 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/spritesheet/src/spritesheetFrame.ts:4 (sha256:aee6fb489203d07e5fd58bf8236e4e0844de6125c59f5530f17e23542a0d7d1e)
pub fn create_spritesheet_frame(obj: Option<FlightPartialRecord3721705896>) -> SpritesheetFrame {
    let mut out = allocate_entity();
    initialize_spritesheet_frame(
        (out).clone(),
        ((obj).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            FlightPartialRecord3721705896 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                id: __flight_source.id,
                offset_x: __flight_source.offset_x,
                offset_y: __flight_source.offset_y,
                pivot_x: __flight_source.pivot_x,
                pivot_y: __flight_source.pivot_y,
                rotated: __flight_source.rotated,
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/spritesheet/src/spritesheetFrame.ts:10 (sha256:877574a58b88e305f2ad7d9b61600502b898311994847d3df472bf06f21e6d00)
pub fn initialize_spritesheet_frame(
    out: EntityConstruction<SpritesheetFrame>,
    obj: Option<FlightPartialRecord3721705896>,
) -> () {
    crate::host_set(
        "host.id",
        (obj.as_ref().and_then(|value| value.id)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.offsetX",
        (obj.as_ref().and_then(|value| value.offset_x)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.offsetY",
        (obj.as_ref().and_then(|value| value.offset_y)).unwrap_or(0.0_f64),
    );
    crate::host_set("host.pivotX", obj.as_ref().and_then(|value| value.pivot_x));
    crate::host_set("host.pivotY", obj.as_ref().and_then(|value| value.pivot_y));
    crate::host_set(
        "host.rotated",
        (obj.as_ref().and_then(|value| value.rotated)).unwrap_or(false),
    );
}
