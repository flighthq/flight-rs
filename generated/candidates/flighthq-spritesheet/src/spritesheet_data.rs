// @generated from upstream/packages/spritesheet/src/spritesheetData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{EntityConstruction, SpritesheetAnimationDirection};
pub use flighthq_types::{SpritesheetAnimationData, SpritesheetData, SpritesheetFrameData};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1910057007 {
    pub __flight_identity: std::sync::Arc<()>,
    pub direction: Option<SpritesheetAnimationDirection>,
    pub frame_duration: Option<f64>,
    pub frame_durations: Option<Vec<f64>>,
    pub frame_names: Option<Vec<String>>,
    pub repeat_count: Option<f64>,
    pub name: Option<String>,
    pub origin_x: Option<f64>,
    pub origin_y: Option<f64>,
}
impl PartialEq for FlightPartialRecord1910057007 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1317866477 {
    pub __flight_identity: std::sync::Arc<()>,
    pub animations: Option<Vec<SpritesheetAnimationData>>,
    pub frames: Option<Vec<SpritesheetFrameData>>,
    pub image_file: Option<String>,
    pub image_height: Option<f64>,
    pub image_width: Option<f64>,
    pub scale: Option<f64>,
}
impl PartialEq for FlightPartialRecord1317866477 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3557983918 {
    pub __flight_identity: std::sync::Arc<()>,
    pub height: Option<f64>,
    pub name: Option<String>,
    pub offset_x: Option<f64>,
    pub offset_y: Option<f64>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: Option<bool>,
    pub source_height: Option<f64>,
    pub source_width: Option<f64>,
    pub width: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}
impl PartialEq for FlightPartialRecord3557983918 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:13 (sha256:3c49b7ad2a8c43fca569d2e24b0a324ea9f75f094bf3aaadb64d32b843242a77)
pub fn create_spritesheet_animation_data(
    obj: Option<FlightPartialRecord1910057007>,
) -> SpritesheetAnimationData {
    let mut out = allocate_entity();
    initialize_spritesheet_animation_data(
        (out).clone(),
        ((obj).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            FlightPartialRecord1910057007 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                direction: (__flight_source.direction).clone(),
                frame_duration: __flight_source.frame_duration,
                frame_durations: (__flight_source.frame_durations).clone(),
                frame_names: (__flight_source.frame_names).clone(),
                repeat_count: __flight_source.repeat_count,
                name: (__flight_source.name).clone(),
                origin_x: __flight_source.origin_x,
                origin_y: __flight_source.origin_y,
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:19 (sha256:0310526df5e3727aa3ad5101b0fc56d5f94f9f09fbca851d3ea341b468bfcf0d)
pub fn create_spritesheet_data(obj: Option<FlightPartialRecord1317866477>) -> SpritesheetData {
    let mut out = allocate_entity();
    initialize_spritesheet_data(
        (out).clone(),
        ((obj).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            FlightPartialRecord1317866477 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                animations: (__flight_source.animations).clone(),
                frames: (__flight_source.frames).clone(),
                image_file: (__flight_source.image_file).clone(),
                image_height: __flight_source.image_height,
                image_width: __flight_source.image_width,
                scale: __flight_source.scale,
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:25 (sha256:9aaac90432fab2a3789f8253c832abbb8a7327638cdfd78547ccfaeaa3adc856)
pub fn create_spritesheet_frame_data(
    obj: Option<FlightPartialRecord3557983918>,
) -> SpritesheetFrameData {
    let mut out = allocate_entity();
    initialize_spritesheet_frame_data(
        (out).clone(),
        ((obj).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            FlightPartialRecord3557983918 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                height: __flight_source.height,
                name: (__flight_source.name).clone(),
                offset_x: __flight_source.offset_x,
                offset_y: __flight_source.offset_y,
                pivot_x: __flight_source.pivot_x,
                pivot_y: __flight_source.pivot_y,
                rotated: __flight_source.rotated,
                source_height: __flight_source.source_height,
                source_width: __flight_source.source_width,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:31 (sha256:d98b0e4e0ab3a0568d44b6d634d4492d45239e09c2f134656959813c51d1088c)
pub fn initialize_spritesheet_animation_data(
    out: EntityConstruction<SpritesheetAnimationData>,
    obj: Option<FlightPartialRecord1910057007>,
) -> () {
    crate::host_set(
        "host.direction",
        (obj.as_ref().and_then(|value| (value.direction).clone())).unwrap_or("forward".to_owned()),
    );
    crate::host_set(
        "host.frameDuration",
        (obj.as_ref().and_then(|value| value.frame_duration)).unwrap_or(100.0_f64),
    );
    crate::host_set(
        "host.frameDurations",
        obj.as_ref()
            .and_then(|value| (value.frame_durations).clone()),
    );
    crate::host_set(
        "host.frameNames",
        (obj.as_ref().and_then(|value| (value.frame_names).clone())).unwrap_or(vec![]),
    );
    crate::host_set(
        "host.name",
        (obj.as_ref().and_then(|value| (value.name).clone())).unwrap_or("".to_owned()),
    );
    crate::host_set(
        "host.originX",
        (obj.as_ref().and_then(|value| value.origin_x)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.originY",
        (obj.as_ref().and_then(|value| value.origin_y)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.repeatCount",
        (obj.as_ref().and_then(|value| value.repeat_count)).unwrap_or((-1.0_f64)),
    );
}

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:45 (sha256:3a6f40268492a251ac898e78d62c5410728b3f21d550bdd34dc2d7c612c6e342)
pub fn initialize_spritesheet_data(
    out: EntityConstruction<SpritesheetData>,
    obj: Option<FlightPartialRecord1317866477>,
) -> () {
    crate::host_set(
        "host.animations",
        (obj.as_ref().and_then(|value| (value.animations).clone())).unwrap_or(vec![]),
    );
    crate::host_set(
        "host.frames",
        (obj.as_ref().and_then(|value| (value.frames).clone())).unwrap_or(vec![]),
    );
    crate::host_set(
        "host.imageFile",
        (obj.as_ref().and_then(|value| (value.image_file).clone())).unwrap_or("".to_owned()),
    );
    crate::host_set(
        "host.imageHeight",
        (obj.as_ref().and_then(|value| value.image_height)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.imageWidth",
        (obj.as_ref().and_then(|value| value.image_width)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.scale",
        (obj.as_ref().and_then(|value| value.scale)).unwrap_or(1.0_f64),
    );
}

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:57 (sha256:7b09c19c17b38d4b126ac42705287085cbdb3d00650910a787bd1938c7cd77e7)
pub fn initialize_spritesheet_frame_data(
    out: EntityConstruction<SpritesheetFrameData>,
    obj: Option<FlightPartialRecord3557983918>,
) -> () {
    crate::host_set(
        "host.height",
        (obj.as_ref().and_then(|value| value.height)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.name",
        (obj.as_ref().and_then(|value| (value.name).clone())).unwrap_or("".to_owned()),
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
    crate::host_set(
        "host.sourceHeight",
        (obj.as_ref().and_then(|value| value.source_height)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.sourceWidth",
        (obj.as_ref().and_then(|value| value.source_width)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.width",
        (obj.as_ref().and_then(|value| value.width)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.x",
        (obj.as_ref().and_then(|value| value.x)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.y",
        (obj.as_ref().and_then(|value| value.y)).unwrap_or(0.0_f64),
    );
}
