// @generated from upstream/packages/spritesheet/src/spritesheet.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_spritesheet_frame;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    EntityConstruction, Spritesheet, SpritesheetAnimation, SpritesheetFrame, TextureAtlas,
};

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

// Source: upstream/packages/spritesheet/src/spritesheet.ts:6 (sha256:2896ca2da47569ecc5fd0851283319201ae8cb48d8b0f0a75e0559d59848245e)
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
    let mut out = allocate_entity();
    crate::host_set("host.atlas", (spritesheet.atlas).clone());
    crate::host_set(
        "host.animations",
        ((spritesheet.animations).clone()).clone(),
    );
    crate::host_set("host.frames", frames);
    return finish_entity((out).clone());
}

// Source: upstream/packages/spritesheet/src/spritesheet.ts:24 (sha256:812de98bdfec2da616f88a45de8cb8141d0e90c6a5ecd9755ee8f51cd3593aa0)
pub fn create_spritesheet(obj: Option<FlightPartialRecord2119237179>) -> Spritesheet {
    let mut out = allocate_entity();
    initialize_spritesheet(
        (out).clone(),
        ((obj).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            FlightPartialRecord2119237179 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                atlas: (__flight_source.atlas).clone(),
                animations: (__flight_source.animations).clone(),
                frames: (__flight_source.frames).clone(),
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/spritesheet/src/spritesheet.ts:30 (sha256:6f56609b9982389e44152ae36263dc8ad0723eaaa895a139b62ac7bbe26cf6c8)
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

// Source: upstream/packages/spritesheet/src/spritesheet.ts:34 (sha256:75f0086be6af5b7bf9247ecdcdf6cc3ccce342126446df68f09f6e7d0a28641f)
#[derive(Clone, Default)]
struct InitializeSpritesheetRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for InitializeSpritesheetRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn initialize_spritesheet(
    out: EntityConstruction<Spritesheet>,
    obj: Option<FlightPartialRecord2119237179>,
) -> () {
    crate::host_set(
        "host.atlas",
        obj.as_ref().and_then(|value| (value.atlas).clone()),
    );
    crate::host_set(
        "host.animations",
        (obj.as_ref().and_then(|value| (value.animations).clone())).unwrap_or({
            let mut __flight_record = Vec::new();
            __flight_record
        }),
    );
    crate::host_set(
        "host.frames",
        (obj.as_ref().and_then(|value| (value.frames).clone())).unwrap_or(vec![]),
    );
}
