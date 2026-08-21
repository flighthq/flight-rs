// @generated from upstream/packages/spritesheet/src/spritesheetData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SpritesheetAnimationDirection;
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

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:7 (sha256:52dd45a91a3f0addb7b0e0830cb137c47c6d9a52dfa97b3b50aa4791a1e70c8f)
pub fn create_spritesheet_animation_data(
    obj: Option<FlightPartialRecord1910057007>,
) -> SpritesheetAnimationData {
    return SpritesheetAnimationData {
        __flight_identity: std::sync::Arc::new(()),
        direction: (obj.as_ref().and_then(|value| (value.direction).clone()))
            .clone()
            .unwrap_or("forward".to_owned()),
        frame_duration: (obj.as_ref().and_then(|value| value.frame_duration))
            .clone()
            .unwrap_or(100.0_f64),
        frame_durations: obj
            .as_ref()
            .and_then(|value| (value.frame_durations).clone()),
        frame_names: (obj.as_ref().and_then(|value| (value.frame_names).clone()))
            .clone()
            .unwrap_or(vec![]),
        name: (obj.as_ref().and_then(|value| (value.name).clone()))
            .clone()
            .unwrap_or("".to_owned()),
        origin_x: (obj.as_ref().and_then(|value| value.origin_x))
            .clone()
            .unwrap_or(0.0_f64),
        origin_y: (obj.as_ref().and_then(|value| value.origin_y))
            .clone()
            .unwrap_or(0.0_f64),
        repeat_count: (obj.as_ref().and_then(|value| value.repeat_count))
            .clone()
            .unwrap_or((-1.0_f64)),
    };
}

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:20 (sha256:c3f2395387d416958ff29d606d0e46ff9b0f75d1bb0a3433bf488022d426346b)
pub fn create_spritesheet_data(obj: Option<FlightPartialRecord1317866477>) -> SpritesheetData {
    return SpritesheetData {
        __flight_identity: std::sync::Arc::new(()),
        animations: (obj.as_ref().and_then(|value| (value.animations).clone()))
            .clone()
            .unwrap_or(vec![]),
        frames: (obj.as_ref().and_then(|value| (value.frames).clone()))
            .clone()
            .unwrap_or(vec![]),
        image_file: (obj.as_ref().and_then(|value| (value.image_file).clone()))
            .clone()
            .unwrap_or("".to_owned()),
        image_height: (obj.as_ref().and_then(|value| value.image_height))
            .clone()
            .unwrap_or(0.0_f64),
        image_width: (obj.as_ref().and_then(|value| value.image_width))
            .clone()
            .unwrap_or(0.0_f64),
        scale: (obj.as_ref().and_then(|value| value.scale))
            .clone()
            .unwrap_or(1.0_f64),
    };
}

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:31 (sha256:9d05562850066d821bd2a4150f42908eb93905565400bc28524d18c1d11af51b)
pub fn create_spritesheet_frame_data(
    obj: Option<FlightPartialRecord3557983918>,
) -> SpritesheetFrameData {
    return SpritesheetFrameData {
        __flight_identity: std::sync::Arc::new(()),
        height: (obj.as_ref().and_then(|value| value.height))
            .clone()
            .unwrap_or(0.0_f64),
        name: (obj.as_ref().and_then(|value| (value.name).clone()))
            .clone()
            .unwrap_or("".to_owned()),
        offset_x: (obj.as_ref().and_then(|value| value.offset_x))
            .clone()
            .unwrap_or(0.0_f64),
        offset_y: (obj.as_ref().and_then(|value| value.offset_y))
            .clone()
            .unwrap_or(0.0_f64),
        pivot_x: obj.as_ref().and_then(|value| value.pivot_x),
        pivot_y: obj.as_ref().and_then(|value| value.pivot_y),
        rotated: (obj.as_ref().and_then(|value| value.rotated))
            .clone()
            .unwrap_or(false),
        source_height: (obj.as_ref().and_then(|value| value.source_height))
            .clone()
            .unwrap_or(0.0_f64),
        source_width: (obj.as_ref().and_then(|value| value.source_width))
            .clone()
            .unwrap_or(0.0_f64),
        width: (obj.as_ref().and_then(|value| value.width))
            .clone()
            .unwrap_or(0.0_f64),
        x: (obj.as_ref().and_then(|value| value.x))
            .clone()
            .unwrap_or(0.0_f64),
        y: (obj.as_ref().and_then(|value| value.y))
            .clone()
            .unwrap_or(0.0_f64),
    };
}
