// @generated from upstream/packages/lighting/src/hemisphereLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{
    HEMISPHERE_LIGHT_KIND as hemisphere_light_kind_constant, HemisphereLight,
    HemisphereLightOptions,
};

// Source: upstream/packages/lighting/src/hemisphereLight.ts:6 (sha256:8449f979d1574913efde053aa6b1434b6374d9e859df8f2d7a6c10b9a2c1b56e)
pub fn clone_hemisphere_light(source: &HemisphereLight) -> HemisphereLight {
    return create_hemisphere_light(Some(HemisphereLightOptions {
        __flight_identity: std::sync::Arc::new(()),
        ground_color: Some(source.ground_color),
        intensity: Some(source.intensity),
        sky_color: Some(source.sky_color),
    }));
}

// Source: upstream/packages/lighting/src/hemisphereLight.ts:17 (sha256:6c6bb8b738c442d74c3fa2da831d801c2af00c9a943b1d501cb443dea500df3f)
pub fn create_hemisphere_light(options: Option<HemisphereLightOptions>) -> HemisphereLight {
    return create_entity(Some(HemisphereLight {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        ground_color: (options.as_ref().and_then(|value| value.ground_color))
            .clone()
            .unwrap_or(4294967295.0_f64),
        intensity: (options.as_ref().and_then(|value| value.intensity))
            .clone()
            .unwrap_or(1.0_f64),
        kind: (hemisphere_light_kind_constant).to_owned(),
        sky_color: (options.as_ref().and_then(|value| value.sky_color))
            .clone()
            .unwrap_or(4294967295.0_f64),
        ..Default::default()
    }));
}
