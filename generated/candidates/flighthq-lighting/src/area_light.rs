// @generated from upstream/packages/lighting/src/areaLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_geometry::{clone_vector3, create_vector3, normalize_vector3, set_vector3};
use flighthq_types::{
    AREA_LIGHT_KIND as area_light_kind_constant, AreaLight, AreaLightOptions, EntityConstruction,
    UNITLESS_LIGHT_UNIT as unitless_light_unit_constant, Vector3Like,
};

// Source: upstream/packages/lighting/src/areaLight.ts:7 (sha256:001b96cc0140b029d022e64d5363d1121136ec663bfad383df6cd134e0e58615)
pub fn clone_area_light(source: &AreaLight) -> AreaLight {
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
    crate::host_set("host.intensity", source.intensity);
    crate::host_set("host.intensityUnit", (source.intensity_unit).clone());
    crate::host_set("host.kind", area_light_kind_constant);
    crate::host_set("host.normalBias", source.normal_bias);
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
    crate::host_set(
        "host.right",
        clone_vector3(&{
            let __flight_source = &(source.right);
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
    crate::host_set("host.shadowBias", source.shadow_bias);
    crate::host_set("host.shadowFar", source.shadow_far);
    crate::host_set("host.shadowMapSize", source.shadow_map_size);
    crate::host_set("host.shadowNear", source.shadow_near);
    crate::host_set("host.shadowStrength", source.shadow_strength);
    crate::host_set(
        "host.up",
        clone_vector3(&{
            let __flight_source = &(source.up);
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
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/areaLight.ts:31 (sha256:37f0da765541927527fe302f7f2a3de088563b35508cec22757bd633c6f9aa0d)
pub fn create_area_light(options: Option<AreaLightOptions>) -> AreaLight {
    let mut out = allocate_entity();
    initialize_area_light((out).clone(), ((options).clone()).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/areaLight.ts:41 (sha256:4e1abffa912285e0467aee7a5b03730a6e190100952f0b024f27a8b802fc3990)
pub fn initialize_area_light(
    out: EntityConstruction<AreaLight>,
    options: Option<AreaLightOptions>,
) -> () {
    let position = options.as_ref().and_then(|value| (value.position).clone());
    let direction = options.as_ref().and_then(|value| (value.direction).clone());
    let right = options.as_ref().and_then(|value| (value.right).clone());
    let up = options.as_ref().and_then(|value| (value.up).clone());
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
    crate::host_set("host.kind", area_light_kind_constant);
    crate::host_set(
        "host.normalBias",
        (options.as_ref().and_then(|value| value.normal_bias)).unwrap_or(0.0_f64),
    );
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
        "host.right",
        if (right).is_some() {
            clone_vector3(&right.as_ref().unwrap())
        } else {
            create_vector3(Some(1.0_f64), Some(0.0_f64), Some(0.0_f64))
        },
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
    crate::host_set(
        "host.up",
        if (up).is_some() {
            clone_vector3(&up.as_ref().unwrap())
        } else {
            create_vector3(Some(0.0_f64), Some(0.0_f64), Some(1.0_f64))
        },
    );
}

// Source: upstream/packages/lighting/src/areaLight.ts:72 (sha256:afc815a570839e4d778a618b2859079fddb8ed1c12efc31de1fe5979f1ef686b)
pub fn set_area_light_orientation(
    out: &mut AreaLight,
    direction: &Vector3Like,
    right: &Vector3Like,
    up: &Vector3Like,
) -> () {
    let right_len = (((right.x * right.x) + (right.y * right.y)) + (right.z * right.z)).sqrt();
    let up_len = (((up.x * up.x) + (up.y * up.y)) + (up.z * up.z)).sqrt();
    let dir_len = (((direction.x * direction.x) + (direction.y * direction.y))
        + (direction.z * direction.z))
        .sqrt();
    let existing_right_len = (((out.right.x * out.right.x) + (out.right.y * out.right.y))
        + (out.right.z * out.right.z))
        .sqrt();
    let existing_up_len =
        (((out.up.x * out.up.x) + (out.up.y * out.up.y)) + (out.up.z * out.up.z)).sqrt();
    if (dir_len > 0.0_f64) {
        normalize_vector3(&mut out.direction, direction);
    }
    if (right_len > 0.0_f64) {
        set_vector3(
            &mut out.right,
            (right.x / right_len),
            (right.y / right_len),
            (right.z / right_len),
        );
        if (existing_right_len > 0.0_f64) {
            {
                let __flight_argument_1 = (out.right.x * existing_right_len);
                let __flight_argument_2 = (out.right.y * existing_right_len);
                let __flight_argument_3 = (out.right.z * existing_right_len);
                let __flight_result = set_vector3(
                    &mut out.right,
                    __flight_argument_1,
                    __flight_argument_2,
                    __flight_argument_3,
                );
                __flight_result
            };
        }
    }
    if (up_len > 0.0_f64) {
        set_vector3(
            &mut out.up,
            (up.x / up_len),
            (up.y / up_len),
            (up.z / up_len),
        );
        if (existing_up_len > 0.0_f64) {
            {
                let __flight_argument_1 = (out.up.x * existing_up_len);
                let __flight_argument_2 = (out.up.y * existing_up_len);
                let __flight_argument_3 = (out.up.z * existing_up_len);
                let __flight_result = set_vector3(
                    &mut out.up,
                    __flight_argument_1,
                    __flight_argument_2,
                    __flight_argument_3,
                );
                __flight_result
            };
        }
    }
}
