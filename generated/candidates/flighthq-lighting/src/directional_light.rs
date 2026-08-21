// @generated from upstream/packages/lighting/src/directionalLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_geometry::{clone_vector3, create_vector3, set_vector3};
use flighthq_types::{
    DIRECTIONAL_LIGHT_KIND as directional_light_kind_constant, DirectionalLight,
    DirectionalLightOptions, Vector3Like,
};

// Source: upstream/packages/lighting/src/directionalLight.ts:7 (sha256:5f0392990c8eaa6a846b2d300a4eac0bef98355d17752d3dd0795ba567e2f83f)
pub fn clone_directional_light(source: &DirectionalLight) -> DirectionalLight {
    return create_entity(Some(DirectionalLight {
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
        kind: (directional_light_kind_constant).to_owned(),
        normal_bias: source.normal_bias,
        pcf_radius: source.pcf_radius,
        shadow_bias: source.shadow_bias,
        ..Default::default()
    }));
}

// Source: upstream/packages/lighting/src/directionalLight.ts:23 (sha256:5dad8dd877a42bb83777f9c716aa5247da09b6d46c5505d9be25e756299b8d0a)
pub fn create_directional_light(options: Option<DirectionalLightOptions>) -> DirectionalLight {
    let direction = options.as_ref().and_then(|value| (value.direction).clone());
    return create_entity(Some(DirectionalLight {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        casts_shadow: (options.as_ref().and_then(|value| value.casts_shadow))
            .clone()
            .unwrap_or(false),
        color: (options.as_ref().and_then(|value| value.color))
            .clone()
            .unwrap_or(4294967295.0_f64),
        direction: if (direction).is_some() {
            clone_vector3(&direction.as_ref().unwrap())
        } else {
            create_vector3(Some(0.0_f64), Some((-1.0_f64)), Some(0.0_f64))
        },
        intensity: (options.as_ref().and_then(|value| value.intensity))
            .clone()
            .unwrap_or(1.0_f64),
        kind: (directional_light_kind_constant).to_owned(),
        normal_bias: (options.as_ref().and_then(|value| value.normal_bias))
            .clone()
            .unwrap_or(0.0_f64),
        pcf_radius: (options.as_ref().and_then(|value| value.pcf_radius))
            .clone()
            .unwrap_or(0.0_f64),
        shadow_bias: (options.as_ref().and_then(|value| value.shadow_bias))
            .clone()
            .unwrap_or(0.0_f64),
        ..Default::default()
    }));
}

// Source: upstream/packages/lighting/src/directionalLight.ts:39 (sha256:4ebb605251ad41a9c5ec890964ade685e83cafaec0b08b2e4cc6f6ba86091bbd)
pub fn set_directional_light_direction(out: &mut DirectionalLight, x: f64, y: f64, z: f64) -> () {
    let lx = x;
    let ly = y;
    let lz = z;
    let len = (((lx * lx) + (ly * ly)) + (lz * lz)).sqrt();
    if (len > 0.0_f64) {
        set_vector3(&mut out.direction, (lx / len), (ly / len), (lz / len));
    }
}

// Source: upstream/packages/lighting/src/directionalLight.ts:52 (sha256:6df49593877d65a0a303f937e4e1c00d2c17fb0b7b62c794cc303044d9483125)
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
