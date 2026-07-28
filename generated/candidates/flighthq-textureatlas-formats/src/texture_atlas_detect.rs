// @generated from upstream/packages/textureatlas-formats/src/textureAtlasDetect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    TEXTURE_ATLAS_FORMAT_KIND_ASEPRITE as texture_atlas_format_kind_aseprite_constant,
    TEXTURE_ATLAS_FORMAT_KIND_LIBGDX_ATLAS as texture_atlas_format_kind_libgdx_atlas_constant,
    TEXTURE_ATLAS_FORMAT_KIND_STARLING as texture_atlas_format_kind_starling_constant,
    TEXTURE_ATLAS_FORMAT_KIND_TEXTURE_PACKER as texture_atlas_format_kind_texture_packer_constant,
    TextureAtlasFormatKind,
};

// Source: upstream/packages/textureatlas-formats/src/textureAtlasDetect.ts:22 (sha256:7a16380baaf152b449e628d715865a8599a7301ba68ffdbd0d3f1c8a71a9cf4c)
#[derive(Clone)]
struct DetectTextureAtlasFormatRecord1 {
    __flight_identity: std::sync::Arc<()>,
    frames: Option<crate::OpaqueHostValue>,
    meta: Option<crate::OpaqueHostValue>,
}
impl PartialEq for DetectTextureAtlasFormatRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn detect_texture_atlas_format(content: String) -> Option<TextureAtlasFormatKind> {
    let trimmed = (content.trim_start)();
    if (trimmed == "") {
        return None;
    }
    if (trimmed.starts_with)("<") {
        return if (trimmed.includes)("<TextureAtlas") {
            Some(texture_atlas_format_kind_starling_constant)
        } else {
            None
        };
    }
    if (trimmed.starts_with)("{") {
        let mut raw: crate::OpaqueHostValue;
        let __flight_try_return: Option<Option<TextureAtlasFormatKind>> =
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || -> Option<Option<TextureAtlasFormatKind>> {
                    {
                        raw = (json.parse)(content);
                    }
                    None
                },
            )) {
                Ok(value) => value,
                Err(_) => (|| -> Option<Option<TextureAtlasFormatKind>> {
                    {
                        return Some(None);
                    }
                    None
                })(),
            };
        if let Some(__flight_return) = __flight_try_return {
            return __flight_return;
        }
        if (((raw).is_none()
            || (match &(raw) {
                crate::OpaqueHostValue::Undefined => "undefined",
                crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                crate::OpaqueHostValue::Bool(_) => "boolean",
                crate::OpaqueHostValue::Number(_) => "number",
                crate::OpaqueHostValue::String(_) => "string",
            } != "object"))
            || (array.is_array)(raw))
        {
            return None;
        }
        let obj = raw;
        if ((obj.frames).clone()).is_none() {
            return None;
        }
        let app = (read_meta_app(((obj.meta).clone()).unwrap())).to_lowercase();
        if (app).contains("aseprite") {
            return Some(texture_atlas_format_kind_aseprite_constant);
        }
        if ((app).contains("texturepacker") || (app).contains("codeandweb")) {
            return Some(texture_atlas_format_kind_texture_packer_constant);
        }
        return if has_frame_duration(((obj.frames).clone()).unwrap()) {
            Some(texture_atlas_format_kind_aseprite_constant)
        } else {
            Some(texture_atlas_format_kind_texture_packer_constant)
        };
    }
    if ((regex::RegexBuilder::new("^\\s*(size|format|filter|repeat)\\s*:")
        .case_insensitive(false)
        .multi_line(true)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .is_match(&(trimmed))
        && (regex::RegexBuilder::new("^\\s*(xy|orig)\\s*:")
            .case_insensitive(false)
            .multi_line(true)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .is_match(&(trimmed)))
    {
        return Some(texture_atlas_format_kind_libgdx_atlas_constant);
    }
    return None;
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasDetect.ts:53 (sha256:78e34bcf6415019109599e89407bc45b1a2d149171b3aeefbe5efad287ddcf01)
fn first_frame(frames: crate::OpaqueHostValue) -> crate::OpaqueHostValue {
    if (array.is_array)(frames) {
        return frames[0.0_f64 as usize].clone();
    }
    if ((frames).is_some()
        && (match &(frames) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } == "object"))
    {
        for value in (crate::host_value::<()>("host.values")).iter().cloned() {
            return value;
        }
    }
    return crate::OpaqueHostValue::Undefined;
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasDetect.ts:61 (sha256:53c600dbcc30e9bd5e3197373083463723e0c0d483151238ec3eca7036356bfd)
#[derive(Clone)]
struct HasFrameDurationRecord1 {
    __flight_identity: std::sync::Arc<()>,
    duration: Option<crate::OpaqueHostValue>,
}
impl PartialEq for HasFrameDurationRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn has_frame_duration(frames: crate::OpaqueHostValue) -> bool {
    let frame = first_frame((frames).clone());
    return (((frame).is_some()
        && (match &(frame) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } == "object"))
        && (match &((frame.duration).clone()) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } == "number"));
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasDetect.ts:66 (sha256:cc28b45ffa394bc305f890b8dbd710e9831f07abcdb6c59b9aa375b228736bcf)
#[derive(Clone)]
struct ReadMetaAppRecord1 {
    __flight_identity: std::sync::Arc<()>,
    app: Option<crate::OpaqueHostValue>,
}
impl PartialEq for ReadMetaAppRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn read_meta_app(meta: crate::OpaqueHostValue) -> String {
    if ((meta).is_none()
        || (match &(meta) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } != "object"))
    {
        return "".to_owned();
    }
    let app = (meta.app).clone();
    return if (match &(app) {
        crate::OpaqueHostValue::Undefined => "undefined",
        crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
        crate::OpaqueHostValue::Bool(_) => "boolean",
        crate::OpaqueHostValue::Number(_) => "number",
        crate::OpaqueHostValue::String(_) => "string",
    } == "string")
    {
        (app).clone().unwrap()
    } else {
        "".to_owned()
    };
}
