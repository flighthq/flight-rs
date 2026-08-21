// @generated from upstream/packages/spritesheet/src/spritesheet.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_spritesheet_frame;
use flighthq_entity::create_entity;
use flighthq_types::{Spritesheet, SpritesheetAnimation, SpritesheetFrame, TextureAtlas};

#[derive(Clone, Default)]
pub struct FlightPartialRecord2119237179 {
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: Option<TextureAtlas>,
    pub animations: Option<Vec<(String, SpritesheetAnimation)>>,
    pub frames: Option<Vec<SpritesheetFrame>>,
}
impl PartialEq for FlightPartialRecord2119237179 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/spritesheet/src/spritesheet.ts:6 (sha256:ece52e49ff2700a8b6a8e9102cc84379f04e6abb6dd08315e6d758b6db6d9718)
pub fn clone_spritesheet(spritesheet: &Spritesheet) -> Spritesheet {
    let frames = ((spritesheet.frames).clone())
        .iter()
        .cloned()
        .map(|f: SpritesheetFrame| -> SpritesheetFrame {
            create_spritesheet_frame(Some(
                crate::spritesheet_frame::FlightPartialRecord3721705896 {
                    __flight_identity: std::sync::Arc::new(()),
                    id: Some(f.id),
                    offset_x: Some(f.offset_x),
                    offset_y: Some(f.offset_y),
                    pivot_x: f.pivot_x,
                    pivot_y: f.pivot_y,
                    rotated: Some(f.rotated),
                },
            ))
        })
        .collect::<Vec<_>>();
    return create_entity(Some(Spritesheet {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        atlas: (spritesheet.atlas).clone(),
        animations: ((spritesheet.animations).clone()).clone(),
        frames: (frames).clone(),
    }));
}

// Source: upstream/packages/spritesheet/src/spritesheet.ts:24 (sha256:98a6a5f0626783039851024b48235bdace564ed9b1c823cf7f811c6c94568bb4)
#[derive(Clone, Default)]
struct CreateSpritesheetRecord6 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSpritesheetRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_spritesheet(obj: Option<FlightPartialRecord2119237179>) -> Spritesheet {
    return create_entity(Some(Spritesheet {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        atlas: obj.as_ref().and_then(|value| (value.atlas).clone()),
        animations: (obj.as_ref().and_then(|value| (value.animations).clone()))
            .clone()
            .unwrap_or({
                let mut __flight_record = Vec::new();
                __flight_record
            }),
        frames: (obj.as_ref().and_then(|value| (value.frames).clone()))
            .clone()
            .unwrap_or(vec![]),
    }));
}

// Source: upstream/packages/spritesheet/src/spritesheet.ts:32 (sha256:6f56609b9982389e44152ae36263dc8ad0723eaaa895a139b62ac7bbe26cf6c8)
pub fn get_spritesheet_animation(
    spritesheet: &Spritesheet,
    label: String,
) -> Option<SpritesheetAnimation> {
    return Some(
        (spritesheet
            .animations
            .iter()
            .find(|(entry_key, _)| entry_key == &(label).clone())
            .map(|(_, value)| value.clone()))
        .expect("TypeScript Record key was absent"),
    );
}
