// @generated from upstream/packages/lighting/src/directionalLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_geometry::{clone_vector3, create_vector3, set_vector3};
use flighthq_types::{
    DIRECTIONAL_LIGHT_KIND as directional_light_kind_constant, DirectionalLight,
    DirectionalLightOptions, EntityConstruction,
    UNITLESS_LIGHT_UNIT as unitless_light_unit_constant, Vector3Like,
};

// Source: upstream/packages/lighting/src/directionalLight.ts:7 (sha256:c623e2baac7d90555584318a7fc1cacfcac8b06545bc8be48e789c4be53bde6d)
pub fn clone_directional_light(source: &DirectionalLight) -> DirectionalLight {
    let mut out = allocate_entity();
    crate::host_set("host.cascadeCount", source.cascade_count);
    crate::host_set(
        "host.cascadeSplits",
        ((source.cascade_splits).clone()).clone(),
    );
    crate::host_set("host.castsShadow", source.casts_shadow);
    crate::host_set("host.color", source.color);
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
    crate::host_set("host.intensity", source.intensity);
    crate::host_set("host.intensityUnit", (source.intensity_unit).clone());
    crate::host_set("host.kind", directional_light_kind_constant);
    crate::host_set("host.normalBias", source.normal_bias);
    crate::host_set("host.pcfRadius", source.pcf_radius);
    crate::host_set("host.shadowBias", source.shadow_bias);
    crate::host_set("host.shadowFar", source.shadow_far);
    crate::host_set("host.shadowMapSize", source.shadow_map_size);
    crate::host_set("host.shadowNear", source.shadow_near);
    crate::host_set("host.shadowStrength", source.shadow_strength);
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/directionalLight.ts:28 (sha256:b451305f1a6f21bad0e505cb18194ef2e751a6db1c3106047223ac7c3c011347)
pub fn create_directional_light(options: Option<DirectionalLightOptions>) -> DirectionalLight {
    let mut light = allocate_entity();
    initialize_directional_light((light).clone(), ((options).clone()).clone());
    return light;
}

// Source: upstream/packages/lighting/src/directionalLight.ts:37 (sha256:919d06f2b5f64079e82b0706db625200fa84deea083ebb02a121ee1be82b0c7b)
pub fn initialize_directional_light(
    light: EntityConstruction<DirectionalLight>,
    options: Option<DirectionalLightOptions>,
) -> () {
    let direction = options.as_ref().and_then(|value| (value.direction).clone());
    crate::host_set(
        "host.cascadeCount",
        (options.as_ref().and_then(|value| value.cascade_count)).unwrap_or(1.0_f64),
    );
    crate::host_set(
        "host.cascadeSplits",
        (options.as_ref().unwrap().cascade_splits.as_ref().unwrap()).clone(),
    );
    crate::host_set(
        "host.castsShadow",
        (options.as_ref().and_then(|value| value.casts_shadow)).unwrap_or(false),
    );
    crate::host_set(
        "host.color",
        (options.as_ref().and_then(|value| value.color)).unwrap_or(4294967295.0_f64),
    );
    crate::host_set(
        "host.direction",
        create_vector3(Some(0.0_f64), Some((-1.0_f64)), Some(0.0_f64)),
    );
    crate::host_set(
        "host.enabled",
        (options.as_ref().and_then(|value| value.enabled)).unwrap_or(true),
    );
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
    crate::host_set("host.kind", directional_light_kind_constant);
    crate::host_set(
        "host.normalBias",
        (options.as_ref().and_then(|value| value.normal_bias)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.pcfRadius",
        (options.as_ref().and_then(|value| value.pcf_radius)).unwrap_or(0.0_f64),
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
    if (direction).is_some() {
        set_directional_light_direction(
            &mut light,
            direction.as_ref().unwrap().x,
            direction.as_ref().unwrap().y,
            direction.as_ref().unwrap().z,
        );
    }
}

// Source: upstream/packages/lighting/src/directionalLight.ts:63 (sha256:4ebb605251ad41a9c5ec890964ade685e83cafaec0b08b2e4cc6f6ba86091bbd)
pub fn set_directional_light_direction(out: &mut DirectionalLight, x: f64, y: f64, z: f64) -> () {
    let lx = x;
    let ly = y;
    let lz = z;
    let len = (((lx * lx) + (ly * ly)) + (lz * lz)).sqrt();
    if (len > 0.0_f64) {
        set_vector3(&mut out.direction, (lx / len), (ly / len), (lz / len));
    }
}

// Source: upstream/packages/lighting/src/directionalLight.ts:76 (sha256:6df49593877d65a0a303f937e4e1c00d2c17fb0b7b62c794cc303044d9483125)
pub fn set_directional_light_target(
    out: &mut DirectionalLight,
    from_x: f64,
    from_y: f64,
    from_z: f64,
    to_x: f64,
    to_y: f64,
    to_z: f64,
) -> () {
    let dx = (to_x - from_x);
    let dy = (to_y - from_y);
    let dz = (to_z - from_z);
    let len = (((dx * dx) + (dy * dy)) + (dz * dz)).sqrt();
    if (len > 0.0_f64) {
        set_vector3(&mut out.direction, (dx / len), (dy / len), (dz / len));
    }
}
