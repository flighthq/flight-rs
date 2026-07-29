// @generated from upstream/packages/tileset/src/tileset.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_textureatlas::{create_texture_atlas_region, set_texture_atlas_region};
use flighthq_types::{TextureAtlas, Tileset};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: Option<TextureAtlas>,
    pub columns: Option<f64>,
    pub margin: Option<f64>,
    pub rows: Option<f64>,
    pub spacing: Option<f64>,
    pub tile_height: Option<f64>,
    pub tile_width: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
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
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/tileset/src/tileset.ts:12 (sha256:933ed9fd99426ced216b4ab9e54a8d846eb130d462e954f7107262e06224c4b3)
pub fn build_tileset_regions(target: &mut Tileset) -> () {
    let mut atlas = (target.atlas).clone();
    let rows = target.rows;
    let columns = target.columns;
    let tile_width = target.tile_width;
    let tile_height = target.tile_height;
    let margin = target.margin;
    let spacing = target.spacing;
    if (atlas).is_none() {
        return;
    }
    let count = (rows * columns);
    if ((atlas.as_mut().unwrap().regions.len() as f64) > count) {
        atlas.as_mut().unwrap().regions.truncate((count) as usize);
    }
    let mut i = 0.0_f64;
    {
        let mut row = 0.0_f64;
        while (row < rows) {
            {
                let mut column = 0.0_f64;
                while (column < columns) {
                    if (i >= (atlas.as_mut().unwrap().regions.len() as f64)) {
                        atlas
                            .as_mut()
                            .unwrap()
                            .regions
                            .push(create_texture_atlas_region(None));
                    }
                    let mut region = atlas.as_mut().unwrap().regions[i as usize].clone();
                    let x = (margin + (column * (tile_width + spacing)));
                    let y = (margin + (row * (tile_height + spacing)));
                    set_texture_atlas_region(
                        &mut region,
                        x,
                        Some(y),
                        Some(tile_width),
                        Some(tile_height),
                        None,
                        None,
                    );
                    region.id = i;
                    region.name = None;
                    region.rotated = false;
                    region.trimmed = false;
                    {
                        i += 1.0;
                        i
                    };
                    {
                        column += 1.0;
                        column
                    };
                }
            }
            {
                row += 1.0;
                row
            };
        }
    }
}

// Source: upstream/packages/tileset/src/tileset.ts:35 (sha256:d052ece170ca2fbce28cd6d280bf658a38bbb378c81d1ca58ab8b2c7a468c711)
pub fn create_tileset(obj: Option<FlightPartialRecord1>) -> Tileset {
    return create_entity(Some(Tileset {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        atlas: obj.as_ref().and_then(|value| (value.atlas).clone()),
        columns: (obj.as_ref().and_then(|value| value.columns)).unwrap_or(0.0_f64),
        margin: (obj.as_ref().and_then(|value| value.margin)).unwrap_or(0.0_f64),
        rows: (obj.as_ref().and_then(|value| value.rows)).unwrap_or(0.0_f64),
        spacing: (obj.as_ref().and_then(|value| value.spacing)).unwrap_or(0.0_f64),
        tile_height: (obj.as_ref().and_then(|value| value.tile_height)).unwrap_or(0.0_f64),
        tile_width: (obj.as_ref().and_then(|value| value.tile_width)).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/tileset/src/tileset.ts:52 (sha256:dc85937ecd5f7e40d5a045c4b2c1deeb69d85840096035a26562567e06c58dd3)
pub fn dispose_tileset(tileset: &mut Tileset) -> () {
    tileset.atlas = None;
}
