// @generated from upstream/packages/lighting/src/spotLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_geometry::{clone_vector3, create_vector3, set_vector3};
use flighthq_types::{
    ALL_LIGHT_LAYERS as all_light_layers_constant, EntityConstruction,
    SPOT_LIGHT_KIND as spot_light_kind_constant, SpotLight, SpotLightConeAngles, SpotLightOptions,
    UNITLESS_LIGHT_UNIT as unitless_light_unit_constant, Vector3Like,
};

// Source: upstream/packages/lighting/src/spotLight.ts:7 (sha256:572a56976e5a037659d289c5e2fe3adec38c546b88780b30ddb94313055694b6)
pub fn clone_spot_light(source: &SpotLight) -> SpotLight {
    let mut out = allocate_entity();
    crate::host_set("host.castsShadow", source.casts_shadow);
    crate::host_set("host.color", source.color);
    crate::host_set("host.decay", source.decay);
    crate::host_set(
        "host.direction",
        clone_vector3(&{
            let __flight_source = &(source.direction);
            Vector3Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        }),
    );
    crate::host_set("host.enabled", source.enabled);
    crate::host_set("host.innerConeCos", source.inner_cone_cos);
    crate::host_set("host.intensity", source.intensity);
    crate::host_set("host.intensityUnit", (source.intensity_unit).clone());
    crate::host_set("host.layerMask", source.layer_mask);
    crate::host_set("host.priority", source.priority);
    crate::host_set("host.kind", spot_light_kind_constant);
    crate::host_set("host.normalBias", source.normal_bias);
    crate::host_set("host.outerConeCos", source.outer_cone_cos);
    crate::host_set("host.pcfRadius", source.pcf_radius);
    crate::host_set(
        "host.position",
        clone_vector3(&{
            let __flight_source = &(source.position);
            Vector3Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        }),
    );
    crate::host_set("host.range", source.range);
    crate::host_set("host.shadowBias", source.shadow_bias);
    crate::host_set("host.shadowFar", source.shadow_far);
    crate::host_set("host.shadowMapSize", source.shadow_map_size);
    crate::host_set("host.shadowNear", source.shadow_near);
    crate::host_set("host.shadowStrength", source.shadow_strength);
    crate::host_set("host.spotBlend", source.spot_blend);
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/spotLight.ts:34 (sha256:88951105f1c9cbe21b57bf01c7bbbe0d58677e4ed6f0158af1983f812ffe2089)
pub fn create_spot_light(options: Option<SpotLightOptions>) -> SpotLight {
    let mut light = allocate_entity();
    initialize_spot_light((light).clone(), ((options).clone()).clone());
    return light;
}

// Source: upstream/packages/lighting/src/spotLight.ts:43 (sha256:8251cdf5a4d2a71d75f5e5bf39d147e89960443cb1686b27ff5ee7a0b7bb4cd9)
pub fn get_spot_light_cone_degrees(out: &mut SpotLightConeAngles, source: &SpotLight) -> () {
    out.inner_degrees = (((source.inner_cone_cos).acos() * 180.0_f64) / std::f64::consts::PI);
    out.outer_degrees = (((source.outer_cone_cos).acos() * 180.0_f64) / std::f64::consts::PI);
}

// Source: upstream/packages/lighting/src/spotLight.ts:52 (sha256:0b9de82cdd0739334ae14851f86b788c670a48316aae4aa604eed03c327a1ab4)
pub fn initialize_spot_light(
    light: EntityConstruction<SpotLight>,
    options: Option<SpotLightOptions>,
) -> () {
    let position = options.as_ref().and_then(|value| (value.position).clone());
    let direction = options.as_ref().and_then(|value| (value.direction).clone());
    crate::host_set(
        "host.castsShadow",
        (options.as_ref().and_then(|value| value.casts_shadow)).unwrap_or(false),
    );
    crate::host_set(
        "host.color",
        (options.as_ref().and_then(|value| value.color)).unwrap_or(4294967295.0_f64),
    );
    crate::host_set(
        "host.decay",
        (options.as_ref().and_then(|value| value.decay)).unwrap_or(2.0_f64),
    );
    crate::host_set(
        "host.direction",
        if (direction).is_some() {
            clone_vector3(&direction.as_ref().unwrap())
        } else {
            create_vector3(Some(0.0_f64), Some((-1.0_f64)), Some(0.0_f64))
        },
    );
    crate::host_set(
        "host.enabled",
        (options.as_ref().and_then(|value| value.enabled)).unwrap_or(true),
    );
    crate::host_set("host.innerConeCos", 1.0_f64);
    crate::host_set(
        "host.intensity",
        (options.as_ref().and_then(|value| value.intensity)).unwrap_or(1.0_f64),
    );
    crate::host_set(
        "host.intensityUnit",
        (options
            .as_ref()
            .and_then(|value| (value.intensity_unit).clone()))
        .unwrap_or((unitless_light_unit_constant).to_owned()),
    );
    crate::host_set(
        "host.layerMask",
        (options.as_ref().and_then(|value| value.layer_mask)).unwrap_or(all_light_layers_constant),
    );
    crate::host_set(
        "host.priority",
        (options.as_ref().and_then(|value| value.priority)).unwrap_or(0.0_f64),
    );
    crate::host_set("host.kind", spot_light_kind_constant);
    crate::host_set(
        "host.normalBias",
        (options.as_ref().and_then(|value| value.normal_bias)).unwrap_or(0.0_f64),
    );
    crate::host_set("host.outerConeCos", 1.0_f64);
    crate::host_set(
        "host.pcfRadius",
        (options.as_ref().and_then(|value| value.pcf_radius)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.position",
        if (position).is_some() {
            clone_vector3(&position.as_ref().unwrap())
        } else {
            create_vector3(Some(0.0_f64), Some(0.0_f64), Some(0.0_f64))
        },
    );
    crate::host_set(
        "host.range",
        (options.as_ref().and_then(|value| value.range)).unwrap_or((-1.0_f64)),
    );
    crate::host_set(
        "host.shadowBias",
        (options.as_ref().and_then(|value| value.shadow_bias)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.shadowFar",
        (options.as_ref().and_then(|value| value.shadow_far)).unwrap_or(500.0_f64),
    );
    crate::host_set(
        "host.shadowMapSize",
        (options.as_ref().and_then(|value| value.shadow_map_size)).unwrap_or(1024.0_f64),
    );
    crate::host_set(
        "host.shadowNear",
        (options.as_ref().and_then(|value| value.shadow_near)).unwrap_or(0.5_f64),
    );
    crate::host_set(
        "host.shadowStrength",
        (options.as_ref().and_then(|value| value.shadow_strength)).unwrap_or(1.0_f64),
    );
    crate::host_set("host.spotBlend", 0.0_f64);
    set_spot_light_cone(
        &mut light,
        (options.as_ref().and_then(|value| value.inner_cone_degrees)).unwrap_or(0.0_f64),
        (options.as_ref().and_then(|value| value.outer_cone_degrees)).unwrap_or(45.0_f64),
    );
    set_spot_light_blend(
        &mut light,
        (options.as_ref().and_then(|value| value.spot_blend)).unwrap_or(0.0_f64),
    );
}

// Source: upstream/packages/lighting/src/spotLight.ts:83 (sha256:37d21ff419dc1b20b97e1eabdb3f963cc5e1d006229629960272071d91e18a48)
pub fn set_spot_light_blend(out: &mut SpotLight, blend: f64) -> () {
    out.spot_blend = (0.0_f64).max((1.0_f64).min(blend));
}

// Source: upstream/packages/lighting/src/spotLight.ts:90 (sha256:ebbee4f86889f9665ab8006d66401756585a69beb9ab7a6482ca055499321442)
pub fn set_spot_light_cone(out: &mut SpotLight, inner_degrees: f64, outer_degrees: f64) -> () {
    out.inner_cone_cos = ((inner_degrees * std::f64::consts::PI) / 180.0_f64).cos();
    out.outer_cone_cos = ((outer_degrees * std::f64::consts::PI) / 180.0_f64).cos();
}

// Source: upstream/packages/lighting/src/spotLight.ts:97 (sha256:7adfcf885cf7e71c2d712e2183136d84ca36938a65b8c22f61e56e7f0a1237ca)
pub fn set_spot_light_direction(out: &mut SpotLight, x: f64, y: f64, z: f64) -> () {
    let lx = x;
    let ly = y;
    let lz = z;
    let len = (((lx * lx) + (ly * ly)) + (lz * lz)).sqrt();
    if (len > 0.0_f64) {
        set_vector3(&mut out.direction, (lx / len), (ly / len), (lz / len));
    }
}

// Source: upstream/packages/lighting/src/spotLight.ts:110 (sha256:077f213b02aa1b86187ca1f1617c37390916d7574e4b56175c0f46b95e3171a8)
pub fn set_spot_light_target(
    out: &mut SpotLight,
    target_x: f64,
    target_y: f64,
    target_z: f64,
) -> () {
    let px = out.position.x;
    let py = out.position.y;
    let pz = out.position.z;
    let dx = (target_x - px);
    let dy = (target_y - py);
    let dz = (target_z - pz);
    let len = (((dx * dx) + (dy * dy)) + (dz * dz)).sqrt();
    if (len > 0.0_f64) {
        set_vector3(&mut out.direction, (dx / len), (dy / len), (dz / len));
    }
}
