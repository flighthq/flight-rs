// @generated from upstream/packages/texture/src/videoTexture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{clone_sampler, copy_sampler, create_sampler};
use flighthq_entity::create_entity;
use flighthq_geometry::{clone_vector2, copy_vector2, create_vector2, inverse_matrix3};
use flighthq_types::{
    Matrix3Like, Sampler, SamplerLike, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
    Vector2Like, VideoResource, VideoTexture, VideoTextureLike,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub uv_offset: Option<Vector2>,
    pub uv_rotation: Option<f64>,
    pub uv_scale: Option<Vector2>,
    pub color_space: Option<TextureColorSpace>,
    pub frame_id: Option<f64>,
    pub sampler: Option<Sampler>,
    pub source: Option<VideoResource>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy: Option<f64>,
    pub mag_filter: Option<TextureFilter>,
    pub min_filter: Option<TextureFilter>,
    pub mipmaps: Option<bool>,
    pub wrap_u: Option<TextureWrap>,
    pub wrap_v: Option<TextureWrap>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/texture/src/videoTexture.ts:11 (sha256:46518ded4490655f81f5e120629edf77a561064dc9f199a00ffe4b580bc2c2a6)
pub fn advance_video_texture(video_texture: &mut VideoTextureLike) -> f64 {
    video_texture.frame_id += 1.0_f64;
    return video_texture.frame_id;
}

// Source: upstream/packages/texture/src/videoTexture.ts:20 (sha256:5784b2d0542745d3c1bed7375520f8fbf14cf176af1561abd978df1407678c67)
pub fn clone_video_texture(source: &VideoTextureLike) -> VideoTexture {
    return create_entity(Some(VideoTexture {
        __flight_identity: std::sync::Arc::new(()),
        color_space: (source.color_space).clone(),
        frame_id: (-1.0_f64),
        sampler: clone_sampler(&{
            let __flight_source = &(source.sampler);
            SamplerLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                anisotropy: __flight_source.anisotropy,
                mag_filter: (__flight_source.mag_filter).clone(),
                min_filter: (__flight_source.min_filter).clone(),
                mipmaps: __flight_source.mipmaps,
                wrap_u: (__flight_source.wrap_u).clone(),
                wrap_v: (__flight_source.wrap_v).clone(),
            }
        }),
        source: (source.source).clone(),
        uv_offset: clone_vector2(&{
            let __flight_source = &(source.uv_offset);
            Vector2Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                x: __flight_source.x,
                y: __flight_source.y,
            }
        }),
        uv_rotation: source.uv_rotation,
        uv_scale: clone_vector2(&{
            let __flight_source = &(source.uv_scale);
            Vector2Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                x: __flight_source.x,
                y: __flight_source.y,
            }
        }),
    }));
}

// Source: upstream/packages/texture/src/videoTexture.ts:35 (sha256:812f3c1fc8d0822279f10cfff347bc9af685b42d7d10f2309f4dc9f669adb039)
pub fn copy_video_texture(out: &mut VideoTextureLike, source: &VideoTextureLike) -> () {
    let color_space = (source.color_space).clone();
    let frame_id = source.frame_id;
    let uv_rotation = source.uv_rotation;
    copy_sampler(&mut out.sampler, &{
        let __flight_source = &(source.sampler);
        SamplerLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            anisotropy: __flight_source.anisotropy,
            mag_filter: (__flight_source.mag_filter).clone(),
            min_filter: (__flight_source.min_filter).clone(),
            mipmaps: __flight_source.mipmaps,
            wrap_u: (__flight_source.wrap_u).clone(),
            wrap_v: (__flight_source.wrap_v).clone(),
        }
    });
    copy_vector2(&mut out.uv_offset, &{
        let __flight_source = &(source.uv_offset);
        Vector2Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            x: __flight_source.x,
            y: __flight_source.y,
        }
    });
    copy_vector2(&mut out.uv_scale, &{
        let __flight_source = &(source.uv_scale);
        Vector2Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            x: __flight_source.x,
            y: __flight_source.y,
        }
    });
    out.color_space = (color_space).clone();
    out.frame_id = frame_id;
    out.source = (source.source).clone();
    out.uv_rotation = uv_rotation;
}

// Source: upstream/packages/texture/src/videoTexture.ts:52 (sha256:4d0bca75d5af5c18c050d48f50d025d724c02579b10b603320f65c8bac77594e)
pub fn create_video_texture(
    source: &VideoResource,
    opts: Option<FlightPartialRecord1>,
) -> VideoTexture {
    return create_entity(Some(VideoTexture {
        __flight_identity: std::sync::Arc::new(()),
        color_space: (opts.as_ref().and_then(|value| (value.color_space).clone()))
            .unwrap_or("srgb".to_owned()),
        frame_id: (opts.as_ref().and_then(|value| value.frame_id)).unwrap_or((-1.0_f64)),
        sampler: if (opts.as_ref().and_then(|value| (value.sampler).clone())).is_some() {
            clone_sampler(opts.as_ref().unwrap().sampler.as_ref().unwrap())
        } else {
            create_sampler(None)
        },
        source: (opts.as_ref().and_then(|value| (value.source).clone()))
            .unwrap_or((*source).clone()),
        uv_offset: if (opts.as_ref().and_then(|value| (value.uv_offset).clone())).is_some() {
            clone_vector2(opts.as_ref().unwrap().uv_offset.as_ref().unwrap())
        } else {
            create_vector2(Some(0.0_f64), Some(0.0_f64))
        },
        uv_rotation: (opts.as_ref().and_then(|value| value.uv_rotation)).unwrap_or(0.0_f64),
        uv_scale: if (opts.as_ref().and_then(|value| (value.uv_scale).clone())).is_some() {
            clone_vector2(opts.as_ref().unwrap().uv_scale.as_ref().unwrap())
        } else {
            create_vector2(Some(1.0_f64), Some(1.0_f64))
        },
    }));
}

// Source: upstream/packages/texture/src/videoTexture.ts:67 (sha256:d7ca0d325924a689248b66466c21f8c497f33263db56a7ba20d5146992883330)
pub fn get_video_texture_height(video_texture: &VideoTextureLike) -> f64 {
    let element = (video_texture.source.element).clone();
    return if ((element).is_some()) && (crate::host_value::<f64>("host.videoHeight") > 0.0_f64) {
        crate::host_value::<f64>("host.videoHeight")
    } else {
        (-1.0_f64)
    };
}

// Source: upstream/packages/texture/src/videoTexture.ts:75 (sha256:c931d7b8c51f4869e96c30f0635eb7fbcda74b93bca2b6df42a2b5f1dde38bb8)
pub fn get_video_texture_inverse_uv_matrix(
    out: &mut Matrix3Like,
    video_texture: &VideoTextureLike,
) -> () {
    get_video_texture_uv_matrix(out, video_texture);
    {
        let __flight_argument_1 = (out).clone();
        inverse_matrix3(out, &__flight_argument_1)
    };
}

// Source: upstream/packages/texture/src/videoTexture.ts:83 (sha256:e1228ce7221c085fa4c70cae2c9cd1f36427c411adb848e80e7ca3234c532730)
pub fn get_video_texture_uv_matrix(out: &mut Matrix3Like, video_texture: &VideoTextureLike) -> () {
    let r = video_texture.uv_rotation;
    let sx = video_texture.uv_scale.x;
    let sy = video_texture.uv_scale.y;
    let tx = video_texture.uv_offset.x;
    let ty = video_texture.uv_offset.y;
    let cos_r = (r).cos();
    let sin_r = (r).sin();
    out.m[0.0_f64 as usize] = (sx * cos_r) as f32;
    out.m[1.0_f64 as usize] = (sx * sin_r) as f32;
    out.m[2.0_f64 as usize] = (0.0_f64) as f32;
    out.m[3.0_f64 as usize] = ((-sy) * sin_r) as f32;
    out.m[4.0_f64 as usize] = (sy * cos_r) as f32;
    out.m[5.0_f64 as usize] = (0.0_f64) as f32;
    out.m[6.0_f64 as usize] = (tx) as f32;
    out.m[7.0_f64 as usize] = (ty) as f32;
    out.m[8.0_f64 as usize] = (1.0_f64) as f32;
}

// Source: upstream/packages/texture/src/videoTexture.ts:105 (sha256:2c9bd5b7b7584330daa490cbcfa38b330e0b614638292370c6dc566ae2cb0b27)
pub fn get_video_texture_width(video_texture: &VideoTextureLike) -> f64 {
    let element = (video_texture.source.element).clone();
    return if ((element).is_some()) && (crate::host_value::<f64>("host.videoWidth") > 0.0_f64) {
        crate::host_value::<f64>("host.videoWidth")
    } else {
        (-1.0_f64)
    };
}

// Source: upstream/packages/texture/src/videoTexture.ts:113 (sha256:0333f750509401501bdfcd6aabc2ec720cc79bab1098229c56b0e702aa07adc5)
pub fn is_video_texture_frame_ready(video_texture: &VideoTextureLike) -> bool {
    let element = (video_texture.source.element).clone();
    return ((((element).is_some())
        && (crate::host_value::<f64>("host.readyState") >= HAVE_CURRENT_DATA))
        && (crate::host_value::<f64>("host.videoWidth") > 0.0_f64))
        && (crate::host_value::<f64>("host.videoHeight") > 0.0_f64);
}

// Source: upstream/packages/texture/src/videoTexture.ts:122 (sha256:d81bf10cf3b0357b2f7c323fc95ea9fb9ccbb3f353438e7ab16e99819ba6a9b3)
pub fn reset_video_texture_frame(video_texture: &mut VideoTextureLike) -> () {
    video_texture.frame_id = (-1.0_f64);
}

// Source: upstream/packages/texture/src/videoTexture.ts:128 (sha256:f00e3fc004aeac43ebf2436addecad5d5ce92664605aa5b2ff5bee419d774873)
pub fn set_video_texture_source(
    video_texture: &mut VideoTextureLike,
    source: &VideoResource,
) -> () {
    video_texture.source = (*source).clone();
    video_texture.frame_id = (-1.0_f64);
}

// Source: upstream/packages/texture/src/videoTexture.ts:134 (sha256:f19fb59a2dbc137e35204c2ecfc84498354efeacd5b47161e62109103105f6ba)
const HAVE_CURRENT_DATA: f64 = 2.0_f64;
