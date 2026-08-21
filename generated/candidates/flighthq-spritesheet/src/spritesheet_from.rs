// @generated from upstream/packages/spritesheet/src/spritesheetFrom.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    SpritesheetData, create_spritesheet, create_spritesheet_animation, create_spritesheet_frame,
};
use flighthq_textureatlas::create_texture_atlas_from_grid;
use flighthq_types::{
    GridSliceOptions, Spritesheet, SpritesheetAnimation, SpritesheetData, SpritesheetFrame,
    SpritesheetFrameData, TextureAtlas, TextureAtlasRegion,
};

// Source: upstream/packages/spritesheet/src/spritesheetFrom.ts:13 (sha256:9cc3160e9580a744dda95e4989ab4e490e45fb2d04436697e0d86da273625547)
#[derive(Clone, Default)]
struct CreateSpritesheetFromDataRecord7 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSpritesheetFromDataRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_spritesheet_from_data(data: &SpritesheetData, atlas: &TextureAtlas) -> Spritesheet {
    let name_to_region_id: std::sync::Arc<std::sync::Mutex<Vec<(String, f64)>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    for region in ((atlas.regions).clone()).iter().cloned() {
        if ((region.name).clone()).is_some() {
            {
                let __flight_key = ((region.name).clone()).unwrap();
                let __flight_value = region.id;
                if let Some((_, value)) = (*name_to_region_id.lock().unwrap())
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    (*name_to_region_id.lock().unwrap()).push((__flight_key, __flight_value));
                }
            };
        }
    }
    let frames = ((data.frames).clone())
        .iter()
        .cloned()
        .map(
            |fd: SpritesheetFrameData, index: crate::OpaqueHostValue| -> SpritesheetFrame {
                let region_id = if ((fd.name).clone() != "") {
                    ((*name_to_region_id.lock().unwrap())
                        .iter()
                        .find(|(entry_key, _)| entry_key == &(fd.name).clone())
                        .map(|(_, value)| value.clone()))
                    .clone()
                    .unwrap_or(index)
                } else {
                    index
                };
                return create_spritesheet_frame(Some(
                    crate::spritesheet_frame::FlightPartialRecord3721705896 {
                        __flight_identity: std::sync::Arc::new(()),
                        id: Some(region_id),
                        offset_x: Some(fd.offset_x),
                        offset_y: Some(fd.offset_y),
                        pivot_x: fd.pivot_x,
                        pivot_y: fd.pivot_y,
                        rotated: Some(fd.rotated),
                    },
                ));
            },
        )
        .collect::<Vec<_>>();
    let frame_name_to_index: std::sync::Arc<std::sync::Mutex<Vec<(String, f64)>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    {
        let mut i = 0.0_f64;
        while (i < (data.frames.len() as f64)) {
            let name = (data.frames[i as usize].name).clone();
            if ((name).clone() != "") {
                {
                    let __flight_key = (name).clone();
                    let __flight_value = i;
                    if let Some((_, value)) = (*frame_name_to_index.lock().unwrap())
                        .iter_mut()
                        .find(|(key, _)| key == &__flight_key)
                    {
                        *value = __flight_value;
                    } else {
                        (*frame_name_to_index.lock().unwrap()).push((__flight_key, __flight_value));
                    }
                };
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let mut animations: Vec<(String, SpritesheetAnimation)> = {
        let mut __flight_record = Vec::new();
        __flight_record
    };
    for ad in ((data.animations).clone()).iter().cloned() {
        let resolved_frames = if ((ad.frame_names.len() as f64) > 0.0_f64) {
            {
                let mut __flight_filter = |i: Option<f64>| -> bool { (i).is_some() };
                (((ad.frame_names).clone())
                    .iter()
                    .cloned()
                    .map(|n: String| -> Option<f64> {
                        (*frame_name_to_index.lock().unwrap())
                            .iter()
                            .find(|(entry_key, _)| entry_key == &(n).clone())
                            .map(|(_, value)| value.clone())
                    })
                    .collect::<Vec<_>>())
                .iter()
                .cloned()
                .filter(|value| __flight_filter(value.clone()))
                .collect::<Vec<_>>()
            }
        } else {
            crate::host_value::<Vec<Option<f64>>>("host.Array.from")
        };
        {
            let __flight_key = (ad.name).clone();
            let __flight_value = create_spritesheet_animation(Some(
                crate::spritesheet_animation::FlightPartialRecord1490253840 {
                    __flight_identity: std::sync::Arc::new(()),
                    direction: Some((ad.direction).clone()),
                    frame_duration: Some(ad.frame_duration),
                    frame_durations: (ad.frame_durations).clone(),
                    frames: Some((resolved_frames).clone()),
                    origin_x: Some(ad.origin_x),
                    origin_y: Some(ad.origin_y),
                    repeat_count: Some(ad.repeat_count),
                },
            ));
            if let Some((_, value)) = animations.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                animations.push((__flight_key, __flight_value));
            }
        };
    }
    return create_spritesheet(Some(crate::spritesheet::FlightPartialRecord2119237179 {
        __flight_identity: std::sync::Arc::new(()),
        animations: Some((animations).clone()),
        atlas: Some((*atlas).clone()),
        frames: Some((frames).clone()),
    }));
}

// Source: upstream/packages/spritesheet/src/spritesheetFrom.ts:65 (sha256:d5f6eebcd2ad9fafa9272512bc91515debe4c3c1016754261127971d71811789)
pub fn create_spritesheet_from_grid(options: &GridSliceOptions) -> Spritesheet {
    let atlas = create_texture_atlas_from_grid(options, None);
    let frames = ((atlas.regions).clone())
        .iter()
        .cloned()
        .map(|region: TextureAtlasRegion| -> SpritesheetFrame {
            create_spritesheet_frame(Some(
                crate::spritesheet_frame::FlightPartialRecord3721705896 {
                    __flight_identity: std::sync::Arc::new(()),
                    id: Some(region.id),
                    offset_x: None,
                    offset_y: None,
                    pivot_x: None,
                    pivot_y: None,
                    rotated: None,
                },
            ))
        })
        .collect::<Vec<_>>();
    return create_spritesheet(Some(crate::spritesheet::FlightPartialRecord2119237179 {
        __flight_identity: std::sync::Arc::new(()),
        atlas: Some((atlas).clone()),
        frames: Some((frames).clone()),
        animations: None,
    }));
}
