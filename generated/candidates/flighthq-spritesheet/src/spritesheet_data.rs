// @generated from upstream/packages/spritesheet/src/spritesheetData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use flighthq_types::{SpritesheetAnimationData, SpritesheetData, SpritesheetFrameData};

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:7 (sha256:71f3c52b04d2a42ddc3fc08f427916f2820328dac2e6ecb102a84fdb2782a5a5)
pub fn create_spritesheet_animation_data(
    obj: Option<SpritesheetAnimationData>,
) -> SpritesheetAnimationData {
    return SpritesheetAnimationData {
        __flight_identity: std::sync::Arc::new(()),
        direction: (obj.as_ref().map(|value| (value.direction).clone()))
            .unwrap_or("forward".to_owned()),
        frame_duration: (obj.as_ref().map(|value| value.frame_duration)).unwrap_or(100.0_f64),
        frame_durations: obj
            .as_ref()
            .and_then(|value| (value.frame_durations).clone()),
        frame_names: (obj.as_ref().map(|value| (value.frame_names).clone())).unwrap_or(vec![]),
        loop_: (obj.as_ref().map(|value| value.loop_)).unwrap_or(true),
        name: (obj.as_ref().map(|value| (value.name).clone())).unwrap_or("".to_owned()),
        origin_x: (obj.as_ref().map(|value| value.origin_x)).unwrap_or(0.0_f64),
        origin_y: (obj.as_ref().map(|value| value.origin_y)).unwrap_or(0.0_f64),
    };
}

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:20 (sha256:c3f2395387d416958ff29d606d0e46ff9b0f75d1bb0a3433bf488022d426346b)
pub fn create_spritesheet_data(obj: Option<SpritesheetData>) -> SpritesheetData {
    return SpritesheetData {
        __flight_identity: std::sync::Arc::new(()),
        animations: (obj.as_ref().map(|value| (value.animations).clone())).unwrap_or(vec![]),
        frames: (obj.as_ref().map(|value| (value.frames).clone())).unwrap_or(vec![]),
        image_file: (obj.as_ref().map(|value| (value.image_file).clone())).unwrap_or("".to_owned()),
        image_height: (obj.as_ref().map(|value| value.image_height)).unwrap_or(0.0_f64),
        image_width: (obj.as_ref().map(|value| value.image_width)).unwrap_or(0.0_f64),
        scale: (obj.as_ref().map(|value| value.scale)).unwrap_or(1.0_f64),
    };
}

// Source: upstream/packages/spritesheet/src/spritesheetData.ts:31 (sha256:9d05562850066d821bd2a4150f42908eb93905565400bc28524d18c1d11af51b)
pub fn create_spritesheet_frame_data(obj: Option<SpritesheetFrameData>) -> SpritesheetFrameData {
    return SpritesheetFrameData {
        __flight_identity: std::sync::Arc::new(()),
        height: (obj.as_ref().map(|value| value.height)).unwrap_or(0.0_f64),
        name: (obj.as_ref().map(|value| (value.name).clone())).unwrap_or("".to_owned()),
        offset_x: (obj.as_ref().map(|value| value.offset_x)).unwrap_or(0.0_f64),
        offset_y: (obj.as_ref().map(|value| value.offset_y)).unwrap_or(0.0_f64),
        pivot_x: obj.as_ref().and_then(|value| value.pivot_x),
        pivot_y: obj.as_ref().and_then(|value| value.pivot_y),
        rotated: (obj.as_ref().map(|value| value.rotated)).unwrap_or(false),
        source_height: (obj.as_ref().map(|value| value.source_height)).unwrap_or(0.0_f64),
        source_width: (obj.as_ref().map(|value| value.source_width)).unwrap_or(0.0_f64),
        width: (obj.as_ref().map(|value| value.width)).unwrap_or(0.0_f64),
        x: (obj.as_ref().map(|value| value.x)).unwrap_or(0.0_f64),
        y: (obj.as_ref().map(|value| value.y)).unwrap_or(0.0_f64),
    };
}
