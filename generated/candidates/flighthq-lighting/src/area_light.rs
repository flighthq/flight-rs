// @generated from upstream/packages/lighting/src/areaLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_geometry::{clone_vector3, create_vector3, normalize_vector3, set_vector3};
use flighthq_types::{
    AREA_LIGHT_KIND as area_light_kind_constant, AreaLight, AreaLightOptions, Vector3Like,
};

// Source: upstream/packages/lighting/src/areaLight.ts:7 (sha256:70d550d01e6432cb6441f4771d6b2ae4230de0070b9698929f3fe6b7bf345235)
pub fn clone_area_light(source: &AreaLight) -> AreaLight {
    return create_entity(Some(AreaLight {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        casts_shadow: source.casts_shadow,
        color: source.color,
        direction: clone_vector3(&{
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
        intensity: source.intensity,
        kind: (area_light_kind_constant).to_owned(),
        normal_bias: source.normal_bias,
        pcf_radius: source.pcf_radius,
        position: clone_vector3(&{
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
        range: source.range,
        right: clone_vector3(&{
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
        shadow_bias: source.shadow_bias,
        up: clone_vector3(&{
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
        ..Default::default()
    }));
}

// Source: upstream/packages/lighting/src/areaLight.ts:28 (sha256:619be895075278116bbc3aac51eb5362c6c167bcb2d0e246486bf17e72e7e842)
pub fn create_area_light(options: Option<AreaLightOptions>) -> AreaLight {
    let position = options.as_ref().and_then(|value| (value.position).clone());
    let direction = options.as_ref().and_then(|value| (value.direction).clone());
    let right = options.as_ref().and_then(|value| (value.right).clone());
    let up = options.as_ref().and_then(|value| (value.up).clone());
    return create_entity(Some(AreaLight {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        casts_shadow: (options.as_ref().and_then(|value| value.casts_shadow)).unwrap_or(false),
        color: (options.as_ref().and_then(|value| value.color)).unwrap_or(4294967295.0_f64),
        direction: if (direction).is_some() {
            clone_vector3(&direction.as_ref().unwrap())
        } else {
            create_vector3(Some(0.0_f64), Some((-1.0_f64)), Some(0.0_f64))
        },
        intensity: (options.as_ref().and_then(|value| value.intensity)).unwrap_or(1.0_f64),
        kind: (area_light_kind_constant).to_owned(),
        normal_bias: (options.as_ref().and_then(|value| value.normal_bias)).unwrap_or(0.0_f64),
        pcf_radius: (options.as_ref().and_then(|value| value.pcf_radius)).unwrap_or(0.0_f64),
        position: if (position).is_some() {
            clone_vector3(&position.as_ref().unwrap())
        } else {
            create_vector3(Some(0.0_f64), Some(0.0_f64), Some(0.0_f64))
        },
        range: (options.as_ref().and_then(|value| value.range)).unwrap_or((-1.0_f64)),
        right: if (right).is_some() {
            clone_vector3(&right.as_ref().unwrap())
        } else {
            create_vector3(Some(1.0_f64), Some(0.0_f64), Some(0.0_f64))
        },
        shadow_bias: (options.as_ref().and_then(|value| value.shadow_bias)).unwrap_or(0.0_f64),
        up: if (up).is_some() {
            clone_vector3(&up.as_ref().unwrap())
        } else {
            create_vector3(Some(0.0_f64), Some(0.0_f64), Some(1.0_f64))
        },
        ..Default::default()
    }));
}

// Source: upstream/packages/lighting/src/areaLight.ts:54 (sha256:afc815a570839e4d778a618b2859079fddb8ed1c12efc31de1fe5979f1ef686b)
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
