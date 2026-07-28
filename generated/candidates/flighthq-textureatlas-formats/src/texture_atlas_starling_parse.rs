// @generated from upstream/packages/textureatlas-formats/src/textureAtlasStarlingParse.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_textureatlas::create_texture_atlas_region;
use flighthq_types::TextureAtlas;
use flighthq_xml::parse_xml_document;

#[derive(Clone)]
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

// Source: upstream/packages/textureatlas-formats/src/textureAtlasStarlingParse.ts:5 (sha256:2ac90c2c6866b87895d0c19c21f0b9cf34a4de68f1985c0d5623c3ea1f3cc309)
#[derive(Clone)]
pub struct TextureAtlasStarlingParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub image_width: Option<f64>,
    pub image_height: Option<f64>,
}
impl PartialEq for TextureAtlasStarlingParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasStarlingParse.ts:19 (sha256:74bfc7a3f13cd2e0db4a4251202ee98293241dbc8a0ab3f4225422d30e20ec52)
pub fn parse_texture_atlas_starling_xml(
    xml: String,
    atlas: &mut TextureAtlas,
    _options: Option<TextureAtlasStarlingParseOptions>,
) -> TextureAtlas {
    atlas.regions.clear();
    let root = parse_xml_document((xml).clone());
    if (root).is_none() {
        return atlas.clone();
    }
    let atlas_el = if ((root.as_ref().unwrap().name).clone() == "TextureAtlas") {
        (root).clone().unwrap()
    } else {
        (((root.as_ref().unwrap().children).clone())
            .iter()
            .find(|value| {
                (|c: XmlElement| -> bool { ((c.name).clone() == "TextureAtlas") })((*value).clone())
            })
            .cloned())
        .unwrap_or((root).clone().unwrap())
    };
    let mut id = 0.0_f64;
    for el in ((atlas_el.children).clone()).iter().cloned() {
        if ((el.name).clone() != "SubTexture") {
            continue;
        }
        if (!el
            .attributes
            .iter()
            .find(|(key, _)| key == &"name".to_owned())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone())
        {
            continue;
        }
        let x = crate::host_value::<()>("host.call");
        let y = crate::host_value::<()>("host.call");
        let width = crate::host_value::<()>("host.call");
        let height = crate::host_value::<()>("host.call");
        let frame_width = if (el
            .attributes
            .iter()
            .find(|(key, _)| key == &"frameWidth".to_owned())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone())
        .is_some()
        {
            crate::host_value::<()>("host.call")
        } else {
            None
        };
        let frame_height = if (el
            .attributes
            .iter()
            .find(|(key, _)| key == &"frameHeight".to_owned())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone())
        .is_some()
        {
            crate::host_value::<()>("host.call")
        } else {
            None
        };
        let trimmed = ((frame_width).is_some())
            || ((el
                .attributes
                .iter()
                .find(|(key, _)| key == &"frameX".to_owned())
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone())
            .is_some());
        let rotated = (el
            .attributes
            .iter()
            .find(|(key, _)| key == &"rotated".to_owned())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone()
            == "true");
        let pivot_x = if (el
            .attributes
            .iter()
            .find(|(key, _)| key == &"pivotX".to_owned())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone())
        .is_some()
        {
            crate::host_value::<()>("host.call")
        } else {
            None
        };
        let pivot_y = if (el
            .attributes
            .iter()
            .find(|(key, _)| key == &"pivotY".to_owned())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone())
        .is_some()
        {
            crate::host_value::<()>("host.call")
        } else {
            None
        };
        atlas
            .regions
            .push(create_texture_atlas_region(Some(FlightPartialRecord1 {
                __flight_identity: std::sync::Arc::new(()),
                height: Some(height),
                id: Some(id),
                name: Some(
                    el.attributes
                        .iter()
                        .find(|(key, _)| key == &"name".to_owned())
                        .map(|(_, value)| value)
                        .expect("TypeScript Record key was absent")
                        .clone(),
                ),
                original_height: Some(if trimmed { frame_height } else { None }),
                original_width: Some(if trimmed { frame_width } else { None }),
                pivot_x: Some(pivot_x),
                pivot_y: Some(pivot_y),
                rotated: Some(rotated),
                source_x: Some(
                    if (el
                        .attributes
                        .iter()
                        .find(|(key, _)| key == &"frameX".to_owned())
                        .map(|(_, value)| value)
                        .expect("TypeScript Record key was absent")
                        .clone())
                    .is_some()
                    {
                        (-crate::host_value::<()>("host.call"))
                    } else {
                        0.0_f64
                    },
                ),
                source_y: Some(
                    if (el
                        .attributes
                        .iter()
                        .find(|(key, _)| key == &"frameY".to_owned())
                        .map(|(_, value)| value)
                        .expect("TypeScript Record key was absent")
                        .clone())
                    .is_some()
                    {
                        (-crate::host_value::<()>("host.call"))
                    } else {
                        0.0_f64
                    },
                ),
                trimmed: Some(trimmed),
                width: Some(width),
                x: Some(x),
                y: Some(y),
            })));
        {
            id += 1.0;
            id
        };
    }
    return atlas.clone();
}
