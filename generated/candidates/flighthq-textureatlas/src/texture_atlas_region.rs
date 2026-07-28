// @generated from upstream/packages/textureatlas/src/textureAtlasRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{RectangleLike, TextureAtlas, TextureAtlasRegion, Vector2Like};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
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
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas/src/textureAtlasRegion.ts:10 (sha256:6384834e1695a5c5882f451b2633bcb2bc793b9a41e02ced8aed76655899a3f7)
pub fn add_texture_atlas_region(
    target: &mut TextureAtlas,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    pivot_x: Option<f64>,
    pivot_y: Option<f64>,
    name: Option<String>,
) -> () {
    target
        .regions
        .push(create_texture_atlas_region(Some(FlightPartialRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            x: Some(x),
            y: Some(y),
            width: Some(width),
            height: Some(height),
            id: Some((target.regions.len() as f64)),
            pivot_x: pivot_x,
            pivot_y: pivot_y,
            name: name,
            original_height: None,
            original_width: None,
            rotated: None,
            source_x: None,
            source_y: None,
            trimmed: None,
        })));
}

// Source: upstream/packages/textureatlas/src/textureAtlasRegion.ts:34 (sha256:63fd86111148481264fdc6cd1ac2bfe70d5b87ed568df18384bac50814e64f36)
pub fn add_texture_atlas_region_rectangle(
    target: &mut TextureAtlas,
    rect: &RectangleLike,
    pivot: Option<Vector2Like>,
    name: Option<String>,
) -> () {
    add_texture_atlas_region(
        target,
        rect.x,
        rect.y,
        rect.width,
        rect.height,
        Some(if (pivot).is_some() {
            pivot.as_ref().unwrap().x
        } else {
            undefined
        }),
        Some(if (pivot).is_some() {
            pivot.as_ref().unwrap().y
        } else {
            undefined
        }),
        Some(((name).clone().unwrap()).clone()),
    );
}

// Source: upstream/packages/textureatlas/src/textureAtlasRegion.ts:52 (sha256:8c249bd88096c8bac0cc92ac48250e7b4037d7ce26d03333616f3f10f7651a75)
pub fn add_texture_atlas_region_rectangle_xy(
    target: &mut TextureAtlas,
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    pivot_x: Option<f64>,
    pivot_y: Option<f64>,
    name: Option<String>,
) -> () {
    add_texture_atlas_region(
        target,
        ax,
        ay,
        (bx - ax),
        (by - ay),
        Some((pivot_x).clone().unwrap()),
        Some((pivot_y).clone().unwrap()),
        Some(((name).clone().unwrap()).clone()),
    );
}

// Source: upstream/packages/textureatlas/src/textureAtlasRegion.ts:65 (sha256:be83bfa6e6a5fb251ab4cb2a639202a4d121784222c4615409f176a2c425a7e6)
pub fn add_texture_atlas_region_vector2(
    target: &mut TextureAtlas,
    a: &Vector2Like,
    b: &Vector2Like,
    pivot: Option<Vector2Like>,
    name: Option<String>,
) -> () {
    add_texture_atlas_region(
        target,
        a.x,
        a.y,
        (b.x - a.x),
        (b.y - a.y),
        Some(if (pivot).is_some() {
            pivot.as_ref().unwrap().x
        } else {
            undefined
        }),
        Some(if (pivot).is_some() {
            pivot.as_ref().unwrap().y
        } else {
            undefined
        }),
        Some(((name).clone().unwrap()).clone()),
    );
}

// Source: upstream/packages/textureatlas/src/textureAtlasRegion.ts:84 (sha256:a7289d328150915a633fbbb307c32bfd2233b6fc5282919531b4141f372d7b2e)
pub fn create_texture_atlas_region(obj: Option<FlightPartialRecord1>) -> TextureAtlasRegion {
    return create_entity(Some(TextureAtlasRegion {
        __flight_identity: std::sync::Arc::new(()),
        x: (obj.as_ref().and_then(|value| value.x)).unwrap_or(0.0_f64),
        y: (obj.as_ref().and_then(|value| value.y)).unwrap_or(0.0_f64),
        width: (obj.as_ref().and_then(|value| value.width)).unwrap_or(0.0_f64),
        height: (obj.as_ref().and_then(|value| value.height)).unwrap_or(0.0_f64),
        id: (obj.as_ref().and_then(|value| value.id)).unwrap_or((-1.0_f64)),
        name: obj.as_ref().and_then(|value| (value.name).clone()),
        original_height: obj.as_ref().and_then(|value| value.original_height),
        original_width: obj.as_ref().and_then(|value| value.original_width),
        pivot_x: obj.as_ref().and_then(|value| value.pivot_x),
        pivot_y: obj.as_ref().and_then(|value| value.pivot_y),
        rotated: (obj.as_ref().and_then(|value| value.rotated)).unwrap_or(false),
        source_x: (obj.as_ref().and_then(|value| value.source_x)).unwrap_or(0.0_f64),
        source_y: (obj.as_ref().and_then(|value| value.source_y)).unwrap_or(0.0_f64),
        trimmed: (obj.as_ref().and_then(|value| value.trimmed)).unwrap_or(false),
    }));
}

// Source: upstream/packages/textureatlas/src/textureAtlasRegion.ts:104 (sha256:47dc2d7a23491545212a417ba353cda782811a39cc90467168232a43f9a0dd50)
pub fn get_texture_atlas_region_by_id(atlas: &TextureAtlas, id: f64) -> Option<TextureAtlasRegion> {
    for region in ((atlas.regions).clone()).iter().cloned() {
        if (region.id == id) {
            return Some((region).clone());
        }
    }
    return None;
}

// Source: upstream/packages/textureatlas/src/textureAtlasRegion.ts:113 (sha256:c5e777fd9b38bd0fbaa327494687ac3454dd7307714fb8af703fb44093227fbf)
pub fn get_texture_atlas_region_by_name(
    atlas: &TextureAtlas,
    name: String,
) -> Option<TextureAtlasRegion> {
    for region in ((atlas.regions).clone()).iter().cloned() {
        if ((region.name).clone()) == Some((name).clone()) {
            return Some((region).clone());
        }
    }
    return None;
}

// Source: upstream/packages/textureatlas/src/textureAtlasRegion.ts:123 (sha256:a8ca9f92a33a8b29b9e89357bb8321c0a4dfce7660ed60443e997a9fede904fa)
pub fn get_texture_atlas_region_sequence(
    atlas: &TextureAtlas,
    prefix: String,
) -> Vec<TextureAtlasRegion> {
    let mut result: Vec<TextureAtlasRegion> = vec![];
    for region in ((atlas.regions).clone()).iter().cloned() {
        if (((region.name).clone()).is_some()) && (((region.name).clone()).starts_with(prefix)) {
            result.push(((region).clone()).clone());
        }
    }
    return result;
}

// Source: upstream/packages/textureatlas/src/textureAtlasRegion.ts:137 (sha256:e00ab22591e79624d6a00a3264d3bd8d4fa7fcbe65dcbecbf85832f6c3c7b0e2)
pub fn get_texture_atlas_region_uv(
    region: &TextureAtlasRegion,
    image_width: f64,
    image_height: f64,
    out: &mut RectangleLike,
) -> RectangleLike {
    if (image_width <= 0.0_f64) || (image_height <= 0.0_f64) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.width = 0.0_f64;
        out.height = 0.0_f64;
        return out.clone();
    }
    let rx = region.x;
    let ry = region.y;
    let rw = region.width;
    let rh = region.height;
    out.x = (rx / image_width);
    out.y = (ry / image_height);
    out.width = (rw / image_width);
    out.height = (rh / image_height);
    return out.clone();
}

// Source: upstream/packages/textureatlas/src/textureAtlasRegion.ts:162 (sha256:b839f902274aa3b7a974b2601d698432bc1309bf6a7cceae31dd2bec6a0ceabf)
pub fn set_texture_atlas_region(
    out: &mut TextureAtlasRegion,
    x: f64,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    pivot_x: Option<f64>,
    pivot_y: Option<f64>,
) -> () {
    let y = y.unwrap_or(0.0_f64);
    let width = width.unwrap_or(0.0_f64);
    let height = height.unwrap_or(0.0_f64);
    let pivot_x = pivot_x.unwrap_or(0.0_f64);
    let pivot_y = pivot_y.unwrap_or(0.0_f64);
    out.x = x;
    out.y = y;
    out.width = width;
    out.height = height;
    out.pivot_x = Some(pivot_x);
    out.pivot_y = Some(pivot_y);
}
