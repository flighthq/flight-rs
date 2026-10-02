// @generated from upstream/packages/materials/src/material3d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_material;
use flighthq_types::{
    BLEND_MODE as blend_mode_constant, Kind, Material3D, Material3DOptions, MaterialAlphaMode,
};

// Source: upstream/packages/materials/src/material3d.ts:12 (sha256:94d7aeb40130b7dd852b7b3ae06e1faeeb7f5d8674c5c24cb4a84d39cd0af175)
pub fn create_material3_d(kind: Kind, opts: Option<Material3DOptions>) -> Material3D {
    let mut material = {
        let __flight_source = &(create_material((kind).clone()));
        Material3D {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
            alpha_cutoff: __flight_source.alpha_cutoff,
            alpha_mode: (__flight_source.alpha_mode).clone(),
            blend_mode: (__flight_source.blend_mode).clone(),
            double_sided: __flight_source.double_sided,
            color: __flight_source.color,
            thickness: __flight_source.thickness,
            tint: __flight_source.tint,
            base_color: __flight_source.base_color,
            base_color_map: (__flight_source.base_color_map).clone(),
            ramp: (__flight_source.ramp).clone(),
            steps: __flight_source.steps,
            alpha_map: (__flight_source.alpha_map).clone(),
            emissive: __flight_source.emissive,
            emissive_map: (__flight_source.emissive_map).clone(),
            emissive_strength: __flight_source.emissive_strength,
            metallic: __flight_source.metallic,
            metallic_roughness_map: (__flight_source.metallic_roughness_map).clone(),
            normal_map: (__flight_source.normal_map).clone(),
            normal_scale: __flight_source.normal_scale,
            occlusion_map: (__flight_source.occlusion_map).clone(),
            occlusion_strength: __flight_source.occlusion_strength,
            roughness: __flight_source.roughness,
            diffuse: __flight_source.diffuse,
            diffuse_map: (__flight_source.diffuse_map).clone(),
            glossiness: __flight_source.glossiness,
            specular: __flight_source.specular,
            specular_glossiness_map: (__flight_source.specular_glossiness_map).clone(),
            modifiers: (__flight_source.modifiers).clone(),
            shininess: __flight_source.shininess,
            specular_map: (__flight_source.specular_map).clone(),
            matcap: (__flight_source.matcap).clone(),
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            far: __flight_source.far,
            near: __flight_source.near,
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            ..Default::default()
        }
    };
    material.alpha_cutoff =
        (opts.as_ref().and_then(|value| value.alpha_cutoff)).unwrap_or(DEFAULT_ALPHA_CUTOFF);
    material.alpha_mode = (opts.as_ref().and_then(|value| (value.alpha_mode).clone()))
        .unwrap_or(((DEFAULT_ALPHA_MODE).clone()).to_owned());
    material.blend_mode = (opts.as_ref().and_then(|value| (value.blend_mode).clone()))
        .unwrap_or((blend_mode_constant.normal).clone());
    material.double_sided =
        (opts.as_ref().and_then(|value| value.double_sided)).unwrap_or(DEFAULT_DOUBLE_SIDED);
    return material;
}

// Source: upstream/packages/materials/src/material3d.ts:24 (sha256:10a2d2fa3ef80e352b8a05740f4ecf71c28b44d26fde7b8f9cbfaa00fb7440ee)
pub fn get_material3_d_alpha_mode(source: &Material3D) -> MaterialAlphaMode {
    return (source.alpha_mode).clone();
}

// Source: upstream/packages/materials/src/material3d.ts:30 (sha256:83d0de7065d2597cd9c1a499a17fd7a0966627bbaae257871fb56bd9921a500c)
pub fn is_material3_d_blended(source: &Material3D) -> bool {
    return ((source.alpha_mode).clone() == "blend");
}

// Source: upstream/packages/materials/src/material3d.ts:36 (sha256:2dc3965ec4af1c4344d445ca218bfd00baa37ec90bdce08f78f48da2718936e9)
pub fn is_material3_d_masked(source: &Material3D) -> bool {
    return ((source.alpha_mode).clone() == "mask");
}

// Source: upstream/packages/materials/src/material3d.ts:42 (sha256:8b8d0b34cb427964228276de4831a2a676ad0049af14b4f1e40e79332d8811f4)
pub fn is_material3_d_opaque(source: &Material3D) -> bool {
    return ((source.alpha_mode).clone() == "opaque");
}

// Source: upstream/packages/materials/src/material3d.ts:46 (sha256:34e90c18bc19a9bc5f2510d91751864751ac7980e8301885b96e60f58aef72a0)
const DEFAULT_ALPHA_CUTOFF: f64 = 0.5_f64;

// Source: upstream/packages/materials/src/material3d.ts:47 (sha256:401716d5b49444f6ce555a73a1459772cc9cab019086d5c70ea11ac13e42b06e)
const DEFAULT_ALPHA_MODE: &'static str = "opaque";

// Source: upstream/packages/materials/src/material3d.ts:48 (sha256:59abc25abd5a8cbe5fa6822845633843c57e374dba44d086d0760fb0d39cd176)
const DEFAULT_DOUBLE_SIDED: bool = false;
