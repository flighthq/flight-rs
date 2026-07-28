// @generated from upstream/packages/textureatlas-formats/src/textureAtlasLibgdxParse.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_textureatlas::create_texture_atlas_region;
use flighthq_types::TextureAtlas;

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

// Source: upstream/packages/textureatlas-formats/src/textureAtlasLibgdxParse.ts:7 (sha256:4faabc73c9e92c2778296a829dc5b55531388c78336554d7edf85fac92342677)
pub fn parse_texture_atlas_libgdx_atlas(text: String, atlas: &mut TextureAtlas) -> TextureAtlas {
    atlas.regions.clear();
    let lines = (regex::RegexBuilder::new("\\r?\\n")
        .case_insensitive(false)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .split(&(text))
    .map(|part| part.to_owned())
    .collect::<Vec<_>>();
    let mut i = 0.0_f64;
    let mut id = 0.0_f64;
    while (i < (lines.len() as f64)) {
        while (i < (lines.len() as f64)) && ((lines[i as usize].clone()).trim().to_owned() == "") {
            {
                i += 1.0;
                i
            };
        }
        if (i >= (lines.len() as f64)) {
            break;
        }
        let maybe_image = (lines[i as usize].clone()).trim().to_owned();
        if (!(maybe_image).contains(":")) {
            {
                i += 1.0;
                i
            };
            while (i < (lines.len() as f64))
                && ((lines[i as usize].clone()).trim().to_owned() != "")
            {
                if ((lines[i as usize].clone()).trim().to_owned()).contains(":") {
                    {
                        i += 1.0;
                        i
                    };
                } else {
                    break;
                }
            }
        }
        while (i < (lines.len() as f64)) && ((lines[i as usize].clone()).trim().to_owned() != "") {
            let line = (lines[i as usize].clone()).trim().to_owned();
            if (!(line).contains(":")) {
                let region_name = line;
                {
                    i += 1.0;
                    i
                };
                let mut atlas_x = 0.0_f64;
                let mut atlas_y = 0.0_f64;
                let mut atlas_w = 0.0_f64;
                let mut atlas_h = 0.0_f64;
                let mut orig_w = 0.0_f64;
                let mut orig_h = 0.0_f64;
                let mut offset_x = 0.0_f64;
                let mut offset_y = 0.0_f64;
                let mut rotated = false;
                let mut index = (-1.0_f64);
                while (i < (lines.len() as f64)) {
                    let kv = (lines[i as usize].clone()).trim().to_owned();
                    if (kv == "") || (!(kv).contains(":")) {
                        break;
                    }
                    let colon = (kv.index_of)(":");
                    let key = ((kv.slice)(0.0_f64, colon).trim)();
                    let value = ((kv.slice)((colon + 1.0_f64)).trim)();
                    {
                        i += 1.0;
                        i
                    };
                    {
                        let __switch_value = key;
                        let __flight_case = if __switch_value == "rotate" {
                            0_usize
                        } else if __switch_value == "xy" {
                            1_usize
                        } else if __switch_value == "size" {
                            2_usize
                        } else if __switch_value == "orig" {
                            3_usize
                        } else if __switch_value == "offset" {
                            4_usize
                        } else if __switch_value == "index" {
                            5_usize
                        } else {
                            6_usize
                        };
                        '__flight_switch: {
                            if __flight_case <= 0_usize {
                                rotated = (value == "true");
                                break '__flight_switch;
                            }
                            if __flight_case <= 1_usize {
                                {
                                    let parts = (value.split)(",");
                                    atlas_x = ((parts[0.0_f64 as usize].trim)())
                                        .trim()
                                        .parse::<f64>()
                                        .unwrap_or(f64::NAN);
                                    atlas_y = ((parts[1.0_f64 as usize].trim)())
                                        .trim()
                                        .parse::<f64>()
                                        .unwrap_or(f64::NAN);
                                    break '__flight_switch;
                                }
                            }
                            if __flight_case <= 2_usize {
                                {
                                    let parts = (value.split)(",");
                                    atlas_w = ((parts[0.0_f64 as usize].trim)())
                                        .trim()
                                        .parse::<f64>()
                                        .unwrap_or(f64::NAN);
                                    atlas_h = ((parts[1.0_f64 as usize].trim)())
                                        .trim()
                                        .parse::<f64>()
                                        .unwrap_or(f64::NAN);
                                    break '__flight_switch;
                                }
                            }
                            if __flight_case <= 3_usize {
                                {
                                    let parts = (value.split)(",");
                                    orig_w = ((parts[0.0_f64 as usize].trim)())
                                        .trim()
                                        .parse::<f64>()
                                        .unwrap_or(f64::NAN);
                                    orig_h = ((parts[1.0_f64 as usize].trim)())
                                        .trim()
                                        .parse::<f64>()
                                        .unwrap_or(f64::NAN);
                                    break '__flight_switch;
                                }
                            }
                            if __flight_case <= 4_usize {
                                {
                                    let parts = (value.split)(",");
                                    offset_x = ((parts[0.0_f64 as usize].trim)())
                                        .trim()
                                        .parse::<f64>()
                                        .unwrap_or(f64::NAN);
                                    offset_y = ((parts[1.0_f64 as usize].trim)())
                                        .trim()
                                        .parse::<f64>()
                                        .unwrap_or(f64::NAN);
                                    break '__flight_switch;
                                }
                            }
                            if __flight_case <= 5_usize {
                                index = {
                                    let __flight_value = value;
                                    let __flight_radix = (10.0_f64) as u32;
                                    i64::from_str_radix(__flight_value.trim(), __flight_radix)
                                        .map_or(f64::NAN, |value| value as f64)
                                };
                                break '__flight_switch;
                            }
                        }
                    }
                }
                let name = if (index >= 0.0_f64) {
                    format!("{}_{}", region_name, index)
                } else {
                    (region_name).clone()
                };
                let trimmed = ((orig_w > 0.0_f64) && (orig_h > 0.0_f64))
                    && ((orig_w != atlas_w) || (orig_h != atlas_h));
                atlas
                    .regions
                    .push(create_texture_atlas_region(Some(FlightPartialRecord1 {
                        __flight_identity: std::sync::Arc::new(()),
                        height: Some(atlas_h),
                        id: Some(id),
                        name: Some((name).clone()),
                        original_height: if trimmed { Some(orig_h) } else { None },
                        original_width: if trimmed { Some(orig_w) } else { None },
                        pivot_x: None,
                        pivot_y: None,
                        rotated: Some(rotated),
                        source_x: Some(offset_x),
                        source_y: Some(offset_y),
                        trimmed: Some(trimmed),
                        width: Some(atlas_w),
                        x: Some(atlas_x),
                        y: Some(atlas_y),
                    })));
                {
                    id += 1.0;
                    id
                };
            } else {
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    return atlas.clone();
}
