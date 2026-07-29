// @generated from upstream/packages/lighting/src/spotLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_geometry::{clone_vector3, create_vector3, set_vector3};
use flighthq_types::{SPOT_LIGHT_KIND as spot_light_kind_constant, SpotLight, Vector3Like};

// Source: upstream/packages/lighting/src/spotLight.ts:6 (sha256:568f8b40451fc552029c1d925406e49260d65cb59925a33e673446d908ae2c3a)
#[derive(Clone, Default)]
pub struct SpotLightConeAngles {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub inner_degrees: f64,
    pub outer_degrees: f64,
}
impl PartialEq for SpotLightConeAngles {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/lighting/src/spotLight.ts:11 (sha256:d0bf1578d5df68cbb0e862e25e3ac2b9d7a0141cb8461c4a9226d61baecee179)
#[derive(Clone, Default)]
pub struct SpotLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub casts_shadow: Option<bool>,
    pub color: Option<f64>,
    pub direction: Option<Vector3Like>,
    pub inner_cone_degrees: Option<f64>,
    pub intensity: Option<f64>,
    pub normal_bias: Option<f64>,
    pub outer_cone_degrees: Option<f64>,
    pub pcf_radius: Option<f64>,
    pub position: Option<Vector3Like>,
    pub range: Option<f64>,
    pub shadow_bias: Option<f64>,
}
impl PartialEq for SpotLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/lighting/src/spotLight.ts:28 (sha256:a98520291f8293be3b4fa9e54616e175ae0bb6288b13080c7b015546194e2a0c)
pub fn clone_spot_light(source: &SpotLight) -> SpotLight {
    return create_entity(Some(SpotLight {
        __flight_identity: std::sync::Arc::new(()),
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
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        }),
        inner_cone_cos: source.inner_cone_cos,
        intensity: source.intensity,
        kind: (spot_light_kind_constant).to_owned(),
        normal_bias: source.normal_bias,
        outer_cone_cos: source.outer_cone_cos,
        pcf_radius: source.pcf_radius,
        position: clone_vector3(&{
            let __flight_source = &(source.position);
            Vector3Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        }),
        range: source.range,
        shadow_bias: source.shadow_bias,
        ..Default::default()
    }));
}

// Source: upstream/packages/lighting/src/spotLight.ts:49 (sha256:1655c5b3871af75a1b788c591c2e85c99e3a6520ea38857d141c2a13442f2240)
pub fn create_spot_light(options: Option<SpotLightOptions>) -> SpotLight {
    let position = options.as_ref().and_then(|value| (value.position).clone());
    let direction = options.as_ref().and_then(|value| (value.direction).clone());
    let mut light: SpotLight = create_entity(Some(SpotLight {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        casts_shadow: (options.as_ref().and_then(|value| value.casts_shadow)).unwrap_or(false),
        color: (options.as_ref().and_then(|value| value.color)).unwrap_or(4294967295.0_f64),
        direction: if (direction).is_some() {
            clone_vector3(&direction.as_ref().unwrap())
        } else {
            create_vector3(Some(0.0_f64), Some((-1.0_f64)), Some(0.0_f64))
        },
        inner_cone_cos: 1.0_f64,
        intensity: (options.as_ref().and_then(|value| value.intensity)).unwrap_or(1.0_f64),
        kind: (spot_light_kind_constant).to_owned(),
        normal_bias: (options.as_ref().and_then(|value| value.normal_bias)).unwrap_or(0.0_f64),
        outer_cone_cos: 1.0_f64,
        pcf_radius: (options.as_ref().and_then(|value| value.pcf_radius)).unwrap_or(0.0_f64),
        position: if (position).is_some() {
            clone_vector3(&position.as_ref().unwrap())
        } else {
            create_vector3(Some(0.0_f64), Some(0.0_f64), Some(0.0_f64))
        },
        range: (options.as_ref().and_then(|value| value.range)).unwrap_or((-1.0_f64)),
        shadow_bias: (options.as_ref().and_then(|value| value.shadow_bias)).unwrap_or(0.0_f64),
        ..Default::default()
    }));
    set_spot_light_cone(
        &mut light,
        (options.as_ref().and_then(|value| value.inner_cone_degrees)).unwrap_or(0.0_f64),
        (options.as_ref().and_then(|value| value.outer_cone_degrees)).unwrap_or(45.0_f64),
    );
    return light;
}

// Source: upstream/packages/lighting/src/spotLight.ts:73 (sha256:8251cdf5a4d2a71d75f5e5bf39d147e89960443cb1686b27ff5ee7a0b7bb4cd9)
pub fn get_spot_light_cone_degrees(out: &mut SpotLightConeAngles, source: &SpotLight) -> () {
    out.inner_degrees = (((source.inner_cone_cos).acos() * 180.0_f64) / std::f64::consts::PI);
    out.outer_degrees = (((source.outer_cone_cos).acos() * 180.0_f64) / std::f64::consts::PI);
}

// Source: upstream/packages/lighting/src/spotLight.ts:81 (sha256:ebbee4f86889f9665ab8006d66401756585a69beb9ab7a6482ca055499321442)
pub fn set_spot_light_cone(out: &mut SpotLight, inner_degrees: f64, outer_degrees: f64) -> () {
    out.inner_cone_cos = ((inner_degrees * std::f64::consts::PI) / 180.0_f64).cos();
    out.outer_cone_cos = ((outer_degrees * std::f64::consts::PI) / 180.0_f64).cos();
}

// Source: upstream/packages/lighting/src/spotLight.ts:88 (sha256:7adfcf885cf7e71c2d712e2183136d84ca36938a65b8c22f61e56e7f0a1237ca)
pub fn set_spot_light_direction(out: &mut SpotLight, x: f64, y: f64, z: f64) -> () {
    let lx = x;
    let ly = y;
    let lz = z;
    let len = (((lx * lx) + (ly * ly)) + (lz * lz)).sqrt();
    if (len > 0.0_f64) {
        set_vector3(&mut out.direction, (lx / len), (ly / len), (lz / len));
    }
}

// Source: upstream/packages/lighting/src/spotLight.ts:101 (sha256:077f213b02aa1b86187ca1f1617c37390916d7574e4b56175c0f46b95e3171a8)
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
