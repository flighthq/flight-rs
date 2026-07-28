// @generated from upstream/packages/texture/src/texture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{clone_sampler, copy_sampler, create_sampler, equals_sampler};
use flighthq_entity::create_entity;
use flighthq_geometry::{clone_vector2, copy_vector2, create_vector2, inverse_matrix3};
use flighthq_types::{
    ImageResource, Matrix3Like, Sampler, SceneResourceRef, Texture, TextureColorSpace,
    TextureFilter, TextureLike, TextureUvTransform, TextureWrap, Vector2, Vector2Like,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub uv_offset: Option<Vector2>,
    pub uv_rotation: Option<f64>,
    pub uv_scale: Option<Vector2>,
    pub color_space: Option<TextureColorSpace>,
    pub image: Option<ImageResource>,
    pub resource: Option<SceneResourceRef>,
    pub sampler: Option<Sampler>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
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

// Source: upstream/packages/texture/src/texture.ts:17 (sha256:81864e252af442c7c384ff6550aa2430ca65dfe56ef85af3ee54f9d33320d069)
pub fn clone_texture(source: &TextureLike) -> Texture {
    return create_entity(Some(Texture {
        __flight_identity: std::sync::Arc::new(()),
        color_space: (source.color_space).clone(),
        image: (source.image).clone(),
        resource: (source.resource).clone(),
        sampler: clone_sampler(&source.sampler),
        uv_offset: clone_vector2(&source.uv_offset),
        uv_rotation: source.uv_rotation,
        uv_scale: clone_vector2(&source.uv_scale),
    }));
}

// Source: upstream/packages/texture/src/texture.ts:32 (sha256:cb7837d6e484eef129962dce54a7d45d19a792340a65ef84bd88adf7f892b135)
pub fn copy_texture(out: &mut TextureLike, source: &TextureLike) -> () {
    let color_space = (source.color_space).clone();
    let image = (source.image).clone();
    let resource = (source.resource).clone();
    let uv_rotation = source.uv_rotation;
    copy_sampler(&mut out.sampler, &source.sampler);
    copy_vector2(&mut out.uv_offset, &source.uv_offset);
    copy_vector2(&mut out.uv_scale, &source.uv_scale);
    out.color_space = (color_space).clone();
    out.image = (image).clone();
    out.resource = (resource).clone();
    out.uv_rotation = uv_rotation;
}

// Source: upstream/packages/texture/src/texture.ts:49 (sha256:15d079f849240a755e4198490188b48b408fa933f4cfac6635f33a541f0a3fd8)
pub fn create_texture(opts: Option<FlightPartialRecord1>) -> Texture {
    return create_entity(Some(Texture {
        __flight_identity: std::sync::Arc::new(()),
        color_space: (opts.as_ref().and_then(|value| (value.color_space).clone()))
            .unwrap_or("srgb".to_owned()),
        image: opts.as_ref().and_then(|value| (value.image).clone()),
        resource: opts.as_ref().and_then(|value| (value.resource).clone()),
        sampler: if (opts.as_ref().and_then(|value| (value.sampler).clone())).is_some() {
            clone_sampler(opts.as_ref().unwrap().sampler.as_ref().unwrap())
        } else {
            create_sampler(None)
        },
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

// Source: upstream/packages/texture/src/texture.ts:63 (sha256:1622da1aafd580b221b7a60466cc1a295f3a237ea35488cdc096f938f59979fc)
pub fn equals_texture(a: Option<TextureLike>, b: Option<TextureLike>) -> bool {
    if ((a).is_none()) || ((b).is_none()) {
        return false;
    }
    if (a == b) {
        return true;
    }
    return ((((((((a.as_ref().unwrap().color_space).clone()
        == (b.as_ref().unwrap().color_space).clone())
        && ((a.as_ref().unwrap().image).clone() == (b.as_ref().unwrap().image).clone()))
        && (a.as_ref().unwrap().uv_rotation == b.as_ref().unwrap().uv_rotation))
        && (a.as_ref().unwrap().uv_offset.x == b.as_ref().unwrap().uv_offset.x))
        && (a.as_ref().unwrap().uv_offset.y == b.as_ref().unwrap().uv_offset.y))
        && (a.as_ref().unwrap().uv_scale.x == b.as_ref().unwrap().uv_scale.x))
        && (a.as_ref().unwrap().uv_scale.y == b.as_ref().unwrap().uv_scale.y))
        && (equals_sampler(
            Some(((a.as_ref().unwrap().sampler).clone()).clone()),
            Some(((b.as_ref().unwrap().sampler).clone()).clone()),
        ));
}

// Source: upstream/packages/texture/src/texture.ts:82 (sha256:6362f9fb9ad5b2565afbb9cf440997ab46e0c2cbe15be74c6f87dd72cd520b37)
pub fn get_texture_height(texture: &TextureLike) -> f64 {
    return if ((texture.image).clone()).is_some() {
        texture.image.as_ref().unwrap().height
    } else {
        (-1.0_f64)
    };
}

// Source: upstream/packages/texture/src/texture.ts:92 (sha256:605da1f5f7df9eba571d5488dc744827b12cf9f2b69a04b30a186df53cbabe16)
pub fn get_texture_inverse_uv_matrix(out: &mut Matrix3Like, texture: &TextureLike) -> () {
    get_texture_uv_matrix(
        out,
        &TextureUvTransform {
            __flight_identity: std::sync::Arc::clone(&(texture).__flight_identity),
            uv_offset: ((texture).uv_offset).clone(),
            uv_rotation: (texture).uv_rotation,
            uv_scale: ((texture).uv_scale).clone(),
        },
    );
    {
        let __flight_argument_1 = (out).clone();
        inverse_matrix3(out, &__flight_argument_1)
    };
}

// Source: upstream/packages/texture/src/texture.ts:104 (sha256:87bb3bc51b738fc9954b09e0d0f6ffe45d13aa970f64797b16cf737b395d2b47)
pub fn get_texture_uv_matrix(out: &mut Matrix3Like, texture: &TextureUvTransform) -> () {
    let r = texture.uv_rotation;
    let sx = texture.uv_scale.x;
    let sy = texture.uv_scale.y;
    let tx = texture.uv_offset.x;
    let ty = texture.uv_offset.y;
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

// Source: upstream/packages/texture/src/texture.ts:125 (sha256:23b570e5e39e7cfdda657f99a7f049fc637d2822610009547ae7add92b0a221e)
pub fn get_texture_width(texture: &TextureLike) -> f64 {
    return if ((texture.image).clone()).is_some() {
        texture.image.as_ref().unwrap().width
    } else {
        (-1.0_f64)
    };
}

// Source: upstream/packages/texture/src/texture.ts:133 (sha256:217bc68b232ecf21918d7785082802a973436fd090c6bb6e2fd3486a0835e33e)
pub fn has_texture_uv_transform(texture: &TextureUvTransform) -> bool {
    return ((((texture.uv_scale.x != 1.0_f64) || (texture.uv_scale.y != 1.0_f64))
        || (texture.uv_offset.x != 0.0_f64))
        || (texture.uv_offset.y != 0.0_f64))
        || (texture.uv_rotation != 0.0_f64);
}

// Source: upstream/packages/texture/src/texture.ts:145 (sha256:a5f8ec6862a851c1a1310d72bd5ac7dca1de671dd0000386519c24e9f869c541)
pub fn is_texture_ready(texture: &TextureLike) -> bool {
    return ((texture.image).clone()).is_some();
}

// Source: upstream/packages/texture/src/texture.ts:151 (sha256:c1507a4732fea3dcc441597c1f62abe663d96d7baaec4f9afecfdd13d39b8667)
pub fn reset_texture_uv_transform(texture: &mut TextureLike) -> () {
    texture.uv_offset.x = 0.0_f64;
    texture.uv_offset.y = 0.0_f64;
    texture.uv_rotation = 0.0_f64;
    texture.uv_scale.x = 1.0_f64;
    texture.uv_scale.y = 1.0_f64;
}

// Source: upstream/packages/texture/src/texture.ts:161 (sha256:660cab56217f4ee1180dfc9a6624fd685b3515e2cc965c2e5d8d8d48f2ab76f5)
pub fn set_texture_image(texture: &mut TextureLike, image: Option<ImageResource>) -> () {
    texture.image = (image).clone();
}

// Source: upstream/packages/texture/src/texture.ts:167 (sha256:8016bf54c626f63ccd598236211fe2b0abf2ef48e7e68d2361a399b66c6d1797)
pub fn set_texture_uv_offset(texture: &mut TextureLike, x: f64, y: f64) -> () {
    texture.uv_offset.x = x;
    texture.uv_offset.y = y;
}

// Source: upstream/packages/texture/src/texture.ts:173 (sha256:8637ba6ad85ce43b386e07f9282281d7c1d7a35129082f591aad5e2006e466e6)
pub fn set_texture_uv_rotation(texture: &mut TextureLike, radians: f64) -> () {
    texture.uv_rotation = radians;
}

// Source: upstream/packages/texture/src/texture.ts:178 (sha256:26a3a8886ff4bb5f191a1ee5ba7798f7d97665c559ed6d6ebfc37f6663ac49d9)
pub fn set_texture_uv_scale(texture: &mut TextureLike, x: f64, y: f64) -> () {
    texture.uv_scale.x = x;
    texture.uv_scale.y = y;
}

// Source: upstream/packages/texture/src/texture.ts:187 (sha256:13278118940759035d8b55e490336588a0b39d5f74b75abc78b5dfc12f50e28a)
pub fn transform_texture_uv(out: &mut Vector2Like, texture: &TextureLike, u: f64, v: f64) -> () {
    let r = texture.uv_rotation;
    let sx = texture.uv_scale.x;
    let sy = texture.uv_scale.y;
    let tx = texture.uv_offset.x;
    let ty = texture.uv_offset.y;
    let cos_r = (r).cos();
    let sin_r = (r).sin();
    out.x = ((((sx * cos_r) * u) - ((sy * sin_r) * v)) + tx);
    out.y = ((((sx * sin_r) * u) + ((sy * cos_r) * v)) + ty);
}
