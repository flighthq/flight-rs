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
use flighthq_textureatlas::{create_texture_atlas, create_texture_atlas_region};
use flighthq_types::{
    GridSliceOptions, ImageResource, Spritesheet, SpritesheetAnimation,
    SpritesheetAnimationDirection, SpritesheetData, SpritesheetFrame, SpritesheetFrameData,
    TextureAtlas, TextureAtlasRegion, Tileset,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: Option<TextureAtlas>,
    pub animations: Option<Vec<(String, SpritesheetAnimation)>>,
    pub frames: Option<Vec<SpritesheetFrame>>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Option<Vec<f64>>,
    pub frame_duration: Option<f64>,
    pub frame_durations: Option<Vec<f64>>,
    pub direction: Option<SpritesheetAnimationDirection>,
    pub loop_: Option<bool>,
    pub origin_x: Option<f64>,
    pub origin_y: Option<f64>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub id: Option<f64>,
    pub offset_x: Option<f64>,
    pub offset_y: Option<f64>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: Option<bool>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub image: Option<ImageResource>,
    pub regions: Option<Vec<TextureAtlasRegion>>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord5 {
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
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/spritesheet/src/spritesheetFrom.ts:13 (sha256:33ef5d8ef476265f6ac3a7ddd8dca509de6033265694e4ca7d508c9745be29cc)
#[derive(Clone)]
struct CreateSpritesheetFromDataRecord6 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSpritesheetFromDataRecord6 {
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
                return create_spritesheet_frame(Some(FlightPartialRecord3 {
                    __flight_identity: std::sync::Arc::new(()),
                    id: Some(region_id),
                    offset_x: Some(fd.offset_x),
                    offset_y: Some(fd.offset_y),
                    pivot_x: fd.pivot_x,
                    pivot_y: fd.pivot_y,
                    rotated: Some(fd.rotated),
                }));
            },
        )
        .collect();
    let frame_name_to_index: std::sync::Arc<std::sync::Mutex<Vec<(String, f64)>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    {
        let mut i = 0.0_f64;
        while (i < (data.frames.len() as f64)) {
            let name = (data.frames[i as usize].name).clone();
            if (name != "") {
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
                    (*frame_name_to_index.lock().unwrap())
                        .iter()
                        .find(|(key, _)| key == &(n).clone())
                        .map(|(_, value)| value.clone())
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
            create_spritesheet_animation(Some(FlightPartialRecord2 {
                __flight_identity: std::sync::Arc::new(()),
                direction: Some((ad.direction).clone()),
                frame_duration: Some(ad.frame_duration),
                frame_durations: (ad.frame_durations).clone(),
                frames: Some(resolved_frames),
                loop_: Some(ad.loop_),
                origin_x: Some(ad.origin_x),
                origin_y: Some(ad.origin_y),
            }));
    }
    return create_spritesheet(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        animations: Some((animations).clone()),
        atlas: Some((*atlas).clone()),
        frames: Some(frames),
    }));
}

// Source: upstream/packages/spritesheet/src/spritesheetFrom.ts:65 (sha256:63f470e577dd9d26a96610dd29e78f9e9fb428cdfe347acf07b86fbff6a1c55a)
pub fn create_spritesheet_from_grid(options: &GridSliceOptions) -> Spritesheet {
    let columns = options.columns;
    let rows = options.rows;
    let image_width = options.image_width;
    let image_height = options.image_height;
    let margin_x = (options.margin_x).unwrap_or(0.0_f64);
    let margin_y = (options.margin_y).unwrap_or(0.0_f64);
    let spacing_x = (options.spacing_x).unwrap_or(0.0_f64);
    let spacing_y = (options.spacing_y).unwrap_or(0.0_f64);
    let name_prefix = ((options.name_prefix).clone()).unwrap_or("frame_".to_owned());
    let frame_width = (options.frame_width).unwrap_or(
        (((image_width - (2.0_f64 * margin_x)) - (spacing_x * (columns - 1.0_f64))) / columns)
            .floor(),
    );
    let frame_height = (options.frame_height).unwrap_or(
        (((image_height - (2.0_f64 * margin_y)) - (spacing_y * (rows - 1.0_f64))) / rows).floor(),
    );
    let mut atlas = create_texture_atlas(None);
    let mut frames = vec![];
    let mut id = 0.0_f64;
    {
        let mut row = 0.0_f64;
        while (row < rows) {
            {
                let mut col = 0.0_f64;
                while (col < columns) {
                    let x = (margin_x + (col * (frame_width + spacing_x)));
                    let y = (margin_y + (row * (frame_height + spacing_y)));
                    let mut region = create_texture_atlas_region(Some(FlightPartialRecord5 {
                        __flight_identity: std::sync::Arc::new(()),
                        height: Some(frame_height),
                        id: Some(id),
                        name: Some(format!("{}{}", name_prefix, id)),
                        width: Some(frame_width),
                        x: Some(x),
                        y: Some(y),
                        original_height: None,
                        original_width: None,
                        pivot_x: None,
                        pivot_y: None,
                        rotated: None,
                        source_x: None,
                        source_y: None,
                        trimmed: None,
                    }));
                    atlas.regions.push(((region).clone()).clone());
                    (frames.push)(create_spritesheet_frame(Some(FlightPartialRecord3 {
                        __flight_identity: std::sync::Arc::new(()),
                        id: Some(region.id),
                        offset_x: None,
                        offset_y: None,
                        pivot_x: None,
                        pivot_y: None,
                        rotated: None,
                    })));
                    {
                        region.id += 1.0;
                        region.id
                    };
                    {
                        col += 1.0;
                        col
                    };
                }
            }
            {
                row += 1.0;
                row
            };
        }
    }
    return create_spritesheet(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        atlas: Some((atlas).clone()),
        frames: Some(frames),
        animations: None,
    }));
}

// Source: upstream/packages/spritesheet/src/spritesheetFrom.ts:102 (sha256:ac4f880dbddc40075f81e54111ba93b4a3947acb3b611a763b63dfc5bc6e2d6c)
pub fn create_spritesheet_from_tileset(tileset: &Tileset) -> Spritesheet {
    let atlas = (tileset.atlas).clone();
    let frames = ((atlas.as_ref().map(|value| (value.regions).clone())).unwrap_or(vec![]))
        .iter()
        .cloned()
        .map(|region: TextureAtlasRegion| -> crate::OpaqueHostValue {
            create_spritesheet_frame(Some(FlightPartialRecord3 {
                __flight_identity: std::sync::Arc::new(()),
                id: Some(region.id),
                offset_x: None,
                offset_y: None,
                pivot_x: None,
                pivot_y: None,
                rotated: None,
            }))
        })
        .collect();
    return create_spritesheet(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        atlas: (atlas).clone(),
        frames: Some(frames),
        animations: None,
    }));
}
