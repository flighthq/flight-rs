// @generated from upstream/packages/texture/src/cubeTexture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{clone_sampler, copy_sampler, create_sampler, equals_sampler};
use flighthq_entity::create_entity;
use flighthq_types::{
    CubeTexture, CubeTextureLike, ImageResource, Sampler, TextureColorSpace, TextureFilter,
    TextureWrap,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_space: Option<TextureColorSpace>,
    pub faces: Option<Vec<Option<ImageResource>>>,
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

// Source: upstream/packages/texture/src/cubeTexture.ts:9 (sha256:4595ed2a26f9507489ceb9c3434313aecdebce5e09199144599180a3feb17e76)
pub fn clone_cube_texture(source: &CubeTextureLike) -> CubeTexture {
    return create_entity(Some(CubeTexture {
        __flight_identity: std::sync::Arc::new(()),
        color_space: (source.color_space).clone(),
        faces: ((source.faces).clone()).clone(),
        sampler: clone_sampler(&source.sampler),
    }));
}

// Source: upstream/packages/texture/src/cubeTexture.ts:20 (sha256:d0770c6d212bbbf4027904ec11f036398f196cc72d5de03cbb65ec7abc7d7943)
pub fn copy_cube_texture(out: &mut CubeTextureLike, source: &CubeTextureLike) -> () {
    let color_space = (source.color_space).clone();
    let f0 = source.faces[0.0_f64 as usize].clone();
    let f1 = source.faces[1.0_f64 as usize].clone();
    let f2 = source.faces[2.0_f64 as usize].clone();
    let f3 = source.faces[3.0_f64 as usize].clone();
    let f4 = source.faces[4.0_f64 as usize].clone();
    let f5 = source.faces[5.0_f64 as usize].clone();
    copy_sampler(&mut out.sampler, &source.sampler);
    out.color_space = (color_space).clone();
    let mut faces = (out.faces).clone();
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (f0).clone();
        if __flight_index == faces.len() {
            faces.push(__flight_value);
        } else {
            faces[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (f1).clone();
        if __flight_index == faces.len() {
            faces.push(__flight_value);
        } else {
            faces[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = (f2).clone();
        if __flight_index == faces.len() {
            faces.push(__flight_value);
        } else {
            faces[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = (f3).clone();
        if __flight_index == faces.len() {
            faces.push(__flight_value);
        } else {
            faces[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (4.0_f64) as usize;
        let __flight_value = (f4).clone();
        if __flight_index == faces.len() {
            faces.push(__flight_value);
        } else {
            faces[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (5.0_f64) as usize;
        let __flight_value = (f5).clone();
        if __flight_index == faces.len() {
            faces.push(__flight_value);
        } else {
            faces[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/texture/src/cubeTexture.ts:42 (sha256:10f7bb085bdb359911da1cbfbc17fa8377025fe76e30b6260da3f1c4861f2f90)
pub fn create_cube_texture(opts: Option<FlightPartialRecord1>) -> CubeTexture {
    return create_entity(Some(CubeTexture {
        __flight_identity: std::sync::Arc::new(()),
        color_space: (opts.as_ref().and_then(|value| (value.color_space).clone()))
            .unwrap_or("srgb".to_owned()),
        faces: if (opts.as_ref().and_then(|value| (value.faces).clone())).is_some() {
            ((opts.as_ref().unwrap().faces).clone())
                .as_ref()
                .unwrap()
                .clone()
        } else {
            vec![None, None, None, None, None, None]
        },
        sampler: if (opts.as_ref().and_then(|value| (value.sampler).clone())).is_some() {
            clone_sampler(opts.as_ref().unwrap().sampler.as_ref().unwrap())
        } else {
            create_sampler(None)
        },
    }));
}

// Source: upstream/packages/texture/src/cubeTexture.ts:52 (sha256:9ce4534069aeece7104a1bb76af4a8d460e925ebbedc12e9cb25573edf7d6f66)
pub fn equals_cube_texture(a: Option<CubeTextureLike>, b: Option<CubeTextureLike>) -> bool {
    if ((a).is_none()) || ((b).is_none()) {
        return false;
    }
    if (a == b) {
        return true;
    }
    if ((a.as_ref().unwrap().color_space).clone() != (b.as_ref().unwrap().color_space).clone()) {
        return false;
    }
    if (!equals_sampler(
        Some(((a.as_ref().unwrap().sampler).clone()).clone()),
        Some(((b.as_ref().unwrap().sampler).clone()).clone()),
    )) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < 6.0_f64) {
            if (a.as_ref().unwrap().faces[i as usize].clone()
                != b.as_ref().unwrap().faces[i as usize].clone())
            {
                return false;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/texture/src/cubeTexture.ts:68 (sha256:caeddf4c2cee8ca2aaecd35d903263e3ec53a47cdcd7779ec33488aa57d960e1)
pub fn get_cube_texture_face_size(cube: &CubeTextureLike) -> f64 {
    {
        let mut i = 0.0_f64;
        while (i < 6.0_f64) {
            let face = cube.faces[i as usize].clone();
            if (face).is_some() {
                return face.as_ref().unwrap().width;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return (-1.0_f64);
}

// Source: upstream/packages/texture/src/cubeTexture.ts:78 (sha256:9496ea2802f642930ff88cb065aa24d713f8f55f4e9bad6e13f01f89633b6364)
pub fn is_cube_texture_complete(cube: &CubeTextureLike) -> bool {
    return ((cube.faces).clone())
        .iter()
        .cloned()
        .all(|face: Option<ImageResource>| -> bool { (face).is_some() });
}

// Source: upstream/packages/texture/src/cubeTexture.ts:86 (sha256:56d19643acaa2425bb88ab4f26b3d8fd59b41bec0c40ddf55b6bdc1aef5ebaac)
pub fn set_cube_texture_face(
    cube: &mut CubeTextureLike,
    face_index: f64,
    image: Option<ImageResource>,
) -> () {
    {
        let __flight_index = (face_index) as usize;
        let __flight_value = (image).clone();
        if __flight_index == (cube.faces).clone().len() {
            (cube.faces).clone().push(__flight_value);
        } else {
            (cube.faces).clone()[__flight_index] = __flight_value;
        }
    };
}
