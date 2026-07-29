// @generated from upstream/packages/texture/src/sampler.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{Sampler, SamplerLike, TextureFilter, TextureWrap};

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

// Source: upstream/packages/texture/src/sampler.ts:6 (sha256:f149fc609e1f06e63c2578912a109efba616e60fbf38a24501dd2455c0d6d95e)
pub fn clone_sampler(source: &SamplerLike) -> Sampler {
    return create_entity(Some(Sampler {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        anisotropy: source.anisotropy,
        mag_filter: (source.mag_filter).clone(),
        min_filter: (source.min_filter).clone(),
        mipmaps: source.mipmaps,
        wrap_u: (source.wrap_u).clone(),
        wrap_v: (source.wrap_v).clone(),
    }));
}

// Source: upstream/packages/texture/src/sampler.ts:19 (sha256:f2cd2c4223b05365fdc06eb4909bfb5b0d4dea2fe9ac9561fe01d61903808a74)
pub fn copy_sampler(out: &mut SamplerLike, source: &SamplerLike) -> () {
    out.anisotropy = source.anisotropy;
    out.mag_filter = (source.mag_filter).clone();
    out.min_filter = (source.min_filter).clone();
    out.mipmaps = source.mipmaps;
    out.wrap_u = (source.wrap_u).clone();
    out.wrap_v = (source.wrap_v).clone();
}

// Source: upstream/packages/texture/src/sampler.ts:31 (sha256:e783b304cea5e57093ca83a984948a0dc02f55a2127552720395c8301d95fba0)
pub fn create_anisotropic_sampler(level: f64) -> Sampler {
    return create_sampler(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        anisotropy: Some(level),
        mag_filter: None,
        min_filter: None,
        mipmaps: None,
        wrap_u: None,
        wrap_v: None,
    }));
}

// Source: upstream/packages/texture/src/sampler.ts:37 (sha256:51a17dec115dc24516747ccafcc447a8520a3c5e9083f261a8a6ba73250a9878)
pub fn create_clamp_linear_sampler() -> Sampler {
    return create_sampler(None);
}

// Source: upstream/packages/texture/src/sampler.ts:43 (sha256:56db6bb97cd170e2169ac6cffa5d0993139e702bf6a9b71c666f7b14aba7dfa0)
#[derive(Clone, Default)]
struct CreatePixelArtSamplerRecord2 {
    __flight_identity: std::sync::Arc<()>,
    mag_filter: String,
    min_filter: String,
    mipmaps: bool,
}
impl PartialEq for CreatePixelArtSamplerRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_pixel_art_sampler() -> Sampler {
    return create_sampler(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        mag_filter: Some("nearest".to_owned()),
        min_filter: Some("nearest".to_owned()),
        mipmaps: Some(false),
        anisotropy: None,
        wrap_u: None,
        wrap_v: None,
    }));
}

// Source: upstream/packages/texture/src/sampler.ts:50 (sha256:991b5315db93b375f8d04a839a68c9def1574de94af82991d0fe820682e3a2cc)
pub fn create_sampler(opts: Option<FlightPartialRecord1>) -> Sampler {
    return create_entity(Some(Sampler {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        anisotropy: (opts.as_ref().and_then(|value| value.anisotropy)).unwrap_or(1.0_f64),
        mag_filter: (opts.as_ref().and_then(|value| (value.mag_filter).clone()))
            .unwrap_or("linear".to_owned()),
        min_filter: (opts.as_ref().and_then(|value| (value.min_filter).clone()))
            .unwrap_or("linear-mipmap-linear".to_owned()),
        mipmaps: (opts.as_ref().and_then(|value| value.mipmaps)).unwrap_or(true),
        wrap_u: (opts.as_ref().and_then(|value| (value.wrap_u).clone()))
            .unwrap_or("clamp-to-edge".to_owned()),
        wrap_v: (opts.as_ref().and_then(|value| (value.wrap_v).clone()))
            .unwrap_or("clamp-to-edge".to_owned()),
    }));
}

// Source: upstream/packages/texture/src/sampler.ts:63 (sha256:558556fee9353c533488e551645321a97e72a19a3c26bc48c08e73a1679d9a76)
#[derive(Clone, Default)]
struct CreateTilingSamplerRecord2 {
    __flight_identity: std::sync::Arc<()>,
    wrap_u: String,
    wrap_v: String,
}
impl PartialEq for CreateTilingSamplerRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_tiling_sampler() -> Sampler {
    return create_sampler(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        wrap_u: Some("repeat".to_owned()),
        wrap_v: Some("repeat".to_owned()),
        anisotropy: None,
        mag_filter: None,
        min_filter: None,
        mipmaps: None,
    }));
}

// Source: upstream/packages/texture/src/sampler.ts:69 (sha256:5d58898bb1447324a2c15bf58f07c5b07d1c262a8fc932f3a489be11e7220046)
pub fn equals_sampler(a: Option<SamplerLike>, b: Option<SamplerLike>) -> bool {
    if ((a).is_none()) || ((b).is_none()) {
        return false;
    }
    return (a == b)
        || ((((((a.as_ref().unwrap().anisotropy == b.as_ref().unwrap().anisotropy)
            && ((a.as_ref().unwrap().mag_filter).clone()
                == (b.as_ref().unwrap().mag_filter).clone()))
            && ((a.as_ref().unwrap().min_filter).clone()
                == (b.as_ref().unwrap().min_filter).clone()))
            && (a.as_ref().unwrap().mipmaps == b.as_ref().unwrap().mipmaps))
            && ((a.as_ref().unwrap().wrap_u).clone() == (b.as_ref().unwrap().wrap_u).clone()))
            && ((a.as_ref().unwrap().wrap_v).clone() == (b.as_ref().unwrap().wrap_v).clone()));
}
