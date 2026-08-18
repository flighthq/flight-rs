// @generated from upstream/packages/lighting/src/ambientLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{
    AMBIENT_LIGHT_KIND as ambient_light_kind_constant, AmbientLight, AmbientLightOptions,
};

// Source: upstream/packages/lighting/src/ambientLight.ts:6 (sha256:80d29666b2ca27ebaad8d8e549cf5a593a91a5579a1a1c7e0044f995f760aef8)
pub fn clone_ambient_light(source: &AmbientLight) -> AmbientLight {
    return create_ambient_light(Some(AmbientLightOptions {
        __flight_identity: std::sync::Arc::new(()),
        color: Some(source.color),
        intensity: Some(source.intensity),
    }));
}

// Source: upstream/packages/lighting/src/ambientLight.ts:12 (sha256:528ddba418833e184b363a5370c6e43a2c26770dfa6f91480c6bf3c29c50c5ad)
pub fn create_ambient_light(options: Option<AmbientLightOptions>) -> AmbientLight {
    return create_entity(Some(AmbientLight {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        color: (options.as_ref().and_then(|value| value.color)).unwrap_or(4294967295.0_f64),
        intensity: (options.as_ref().and_then(|value| value.intensity)).unwrap_or(1.0_f64),
        kind: (ambient_light_kind_constant).to_owned(),
        ..Default::default()
    }));
}
