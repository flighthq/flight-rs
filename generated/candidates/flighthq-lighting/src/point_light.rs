// @generated from upstream/packages/lighting/src/pointLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_geometry::{clone_vector3, create_vector3};
use flighthq_types::{POINT_LIGHT_KIND as point_light_kind_constant, PointLight, Vector3Like};

// Source: upstream/packages/lighting/src/pointLight.ts:6 (sha256:35ec8d4b09d8b5e4ec4a3bb1b4cc4df6cae2b1df2ee5ff01c43296dc64029d2c)
#[derive(Clone)]
pub struct PointLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub casts_shadow: Option<bool>,
    pub color: Option<f64>,
    pub intensity: Option<f64>,
    pub normal_bias: Option<f64>,
    pub pcf_radius: Option<f64>,
    pub position: Option<Vector3Like>,
    pub range: Option<f64>,
    pub shadow_bias: Option<f64>,
}
impl PartialEq for PointLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/lighting/src/pointLight.ts:18 (sha256:42e4f42d55ab4f8e6cd5d01555e04d1620280f0beec43e37ab04ed936ae3b93e)
pub fn clone_point_light(source: &PointLight) -> PointLight {
    return create_entity(Some(PointLight {
        __flight_identity: std::sync::Arc::new(()),
        casts_shadow: source.casts_shadow,
        color: source.color,
        intensity: source.intensity,
        kind: (point_light_kind_constant).to_owned(),
        normal_bias: source.normal_bias,
        pcf_radius: source.pcf_radius,
        position: clone_vector3(&source.position),
        range: source.range,
        shadow_bias: source.shadow_bias,
    }));
}

// Source: upstream/packages/lighting/src/pointLight.ts:35 (sha256:2c773a8d4db5096a3f13fe6ebdac9a5c08c1180b93a5e71685daa69b24fb03a8)
pub fn create_point_light(options: Option<PointLightOptions>) -> PointLight {
    let position = options.as_ref().and_then(|value| (value.position).clone());
    return create_entity(Some(PointLight {
        __flight_identity: std::sync::Arc::new(()),
        casts_shadow: (options.as_ref().and_then(|value| value.casts_shadow)).unwrap_or(false),
        color: (options.as_ref().and_then(|value| value.color)).unwrap_or(4294967295.0_f64),
        intensity: (options.as_ref().and_then(|value| value.intensity)).unwrap_or(1.0_f64),
        kind: (point_light_kind_constant).to_owned(),
        normal_bias: (options.as_ref().and_then(|value| value.normal_bias)).unwrap_or(0.0_f64),
        pcf_radius: (options.as_ref().and_then(|value| value.pcf_radius)).unwrap_or(0.0_f64),
        position: if (position).is_some() {
            clone_vector3(&position.as_ref().unwrap())
        } else {
            create_vector3(Some(0.0_f64), Some(0.0_f64), Some(0.0_f64))
        },
        range: (options.as_ref().and_then(|value| value.range)).unwrap_or((-1.0_f64)),
        shadow_bias: (options.as_ref().and_then(|value| value.shadow_bias)).unwrap_or(0.0_f64),
    }));
}
