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
    GridSliceOptions, Spritesheet, SpritesheetAnimation, SpritesheetAnimationDirection,
    SpritesheetData, SpritesheetFrame, SpritesheetFrameData, Texture2D, TextureAtlas,
    TextureAtlasRegion, TextureFilter, TextureWrap,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy: Option<f64>,
    pub mag_filter: Option<TextureFilter>,
    pub min_filter: Option<TextureFilter>,
    pub mipmaps: Option<bool>,
    pub wrap_u: Option<TextureWrap>,
    pub wrap_v: Option<TextureWrap>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: Option<TextureAtlas>,
    pub animations: Option<Vec<(String, SpritesheetAnimation)>>,
    pub frames: Option<Vec<SpritesheetFrame>>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Option<Vec<f64>>,
    pub frame_duration: Option<f64>,
    pub frame_durations: Option<Vec<f64>>,
    pub direction: Option<SpritesheetAnimationDirection>,
    pub repeat_count: Option<f64>,
    pub origin_x: Option<f64>,
    pub origin_y: Option<f64>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub id: Option<f64>,
    pub offset_x: Option<f64>,
    pub offset_y: Option<f64>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: Option<bool>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: Option<Texture2D>,
    pub regions: Option<Vec<TextureAtlasRegion>>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub height: Option<f64>,
    pub id: Option<f64>,
    pub name: Option<String>,
    pub original_height: Option<f64>,
    pub original_width: Option<f64>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: Option<bool>,
    pub source_x: Option<f64>,
    pub source_y: Option<f64>,
    pub trimmed: Option<bool>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

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
            |fd: SpritesheetFrameData, index: crate::OpaqueHostValue| -> crate::OpaqueHostValue {
                let region_id = if ((fd.name).clone() != "") {
                    ((*name_to_region_id.lock().unwrap())
                        .iter()
                        .find(|(key, _)| key == &(fd.name).clone())
                        .map(|(_, value)| value.clone()))
                    .unwrap_or(index)
                } else {
                    index
                };
                return {
                    let __flight_portable_source =
                        create_spritesheet_frame(Some(FlightPartialRecord4 {
                            __flight_identity: std::sync::Arc::new(()),
                            id: Some(region_id),
                            offset_x: Some(fd.offset_x),
                            offset_y: Some(fd.offset_y),
                            pivot_x: fd.pivot_x,
                            pivot_y: fd.pivot_y,
                            rotated: Some(fd.rotated),
                        }));
                    crate::FlightValue::Record({
                        let mut __flight_record = Vec::new();
                        __flight_record.push((
                            "id".to_owned(),
                            crate::FlightValue::Number(*(&((&__flight_portable_source).id)) as f64),
                        ));
                        __flight_record.push((
                            "offsetX".to_owned(),
                            crate::FlightValue::Number(
                                *(&((&__flight_portable_source).offset_x)) as f64,
                            ),
                        ));
                        __flight_record.push((
                            "offsetY".to_owned(),
                            crate::FlightValue::Number(
                                *(&((&__flight_portable_source).offset_y)) as f64,
                            ),
                        ));
                        __flight_record.push((
                            "pivotX".to_owned(),
                            match (&((&__flight_portable_source).pivot_x)).as_ref() {
                                Some(value) => crate::FlightValue::Number(*(value) as f64),
                                None => crate::FlightValue::Null,
                            },
                        ));
                        __flight_record.push((
                            "pivotY".to_owned(),
                            match (&((&__flight_portable_source).pivot_y)).as_ref() {
                                Some(value) => crate::FlightValue::Number(*(value) as f64),
                                None => crate::FlightValue::Null,
                            },
                        ));
                        __flight_record.push((
                            "rotated".to_owned(),
                            crate::FlightValue::Bool(*(&((&__flight_portable_source).rotated))),
                        ));
                        __flight_record
                    })
                };
            },
        )
        .collect();
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
            (((ad.frame_names).clone())
                .iter()
                .cloned()
                .map(|n: String| -> crate::OpaqueHostValue {
                    {
                        let __flight_portable_source = (*frame_name_to_index.lock().unwrap())
                            .iter()
                            .find(|(key, _)| key == &(n).clone())
                            .map(|(_, value)| value.clone());
                        match (&__flight_portable_source).as_ref() {
                            Some(value) => crate::FlightValue::Number(*(value) as f64),
                            None => crate::FlightValue::Null,
                        }
                    }
                })
                .collect()
                .filter)(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                move |i: crate::OpaqueHostValue| -> bool { (i).is_some() },
            )
                as Box<dyn FnMut(crate::OpaqueHostValue) -> bool + Send + 'static>)))
        } else {
            crate::host_value::<Vec<crate::OpaqueHostValue>>("host.Array.from")
        };
        animations
            .iter()
            .find(|(key, _)| key == &(ad.name).clone())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") =
            create_spritesheet_animation(Some(FlightPartialRecord3 {
                __flight_identity: std::sync::Arc::new(()),
                direction: Some((ad.direction).clone()),
                frame_duration: Some(ad.frame_duration),
                frame_durations: (ad.frame_durations).clone(),
                frames: Some((resolved_frames).clone()),
                origin_x: Some(ad.origin_x),
                origin_y: Some(ad.origin_y),
                repeat_count: Some(ad.repeat_count),
            }));
    }
    return create_spritesheet(Some(FlightPartialRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        animations: Some((animations).clone()),
        atlas: Some((*atlas).clone()),
        frames: Some(frames),
    }));
}

// Source: upstream/packages/spritesheet/src/spritesheetFrom.ts:65 (sha256:d5f6eebcd2ad9fafa9272512bc91515debe4c3c1016754261127971d71811789)
pub fn create_spritesheet_from_grid(options: &GridSliceOptions) -> Spritesheet {
    let atlas = create_texture_atlas_from_grid(options, None);
    let frames = ((atlas.regions).clone())
        .iter()
        .cloned()
        .map(|region: TextureAtlasRegion| -> crate::OpaqueHostValue {
            {
                let __flight_portable_source =
                    create_spritesheet_frame(Some(FlightPartialRecord4 {
                        __flight_identity: std::sync::Arc::new(()),
                        id: Some(region.id),
                        offset_x: None,
                        offset_y: None,
                        pivot_x: None,
                        pivot_y: None,
                        rotated: None,
                    }));
                crate::FlightValue::Record({
                    let mut __flight_record = Vec::new();
                    __flight_record.push((
                        "id".to_owned(),
                        crate::FlightValue::Number(*(&((&__flight_portable_source).id)) as f64),
                    ));
                    __flight_record.push((
                        "offsetX".to_owned(),
                        crate::FlightValue::Number(
                            *(&((&__flight_portable_source).offset_x)) as f64,
                        ),
                    ));
                    __flight_record.push((
                        "offsetY".to_owned(),
                        crate::FlightValue::Number(
                            *(&((&__flight_portable_source).offset_y)) as f64,
                        ),
                    ));
                    __flight_record.push((
                        "pivotX".to_owned(),
                        match (&((&__flight_portable_source).pivot_x)).as_ref() {
                            Some(value) => crate::FlightValue::Number(*(value) as f64),
                            None => crate::FlightValue::Null,
                        },
                    ));
                    __flight_record.push((
                        "pivotY".to_owned(),
                        match (&((&__flight_portable_source).pivot_y)).as_ref() {
                            Some(value) => crate::FlightValue::Number(*(value) as f64),
                            None => crate::FlightValue::Null,
                        },
                    ));
                    __flight_record.push((
                        "rotated".to_owned(),
                        crate::FlightValue::Bool(*(&((&__flight_portable_source).rotated))),
                    ));
                    __flight_record
                })
            }
        })
        .collect();
    return create_spritesheet(Some(FlightPartialRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        atlas: Some((atlas).clone()),
        frames: Some(frames),
        animations: None,
    }));
}
