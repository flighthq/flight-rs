// @generated from upstream/packages/lighting/src/ambientLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{AMBIENT_LIGHT_KIND as ambient_light_kind_constant, AmbientLight};

// Source: upstream/packages/lighting/src/ambientLight.ts:5 (sha256:327e7902f53591c7b64ef38c05213c1a455523299f0a1843b732f97800f973b9)
#[derive(Clone)]
pub struct AmbientLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: Option<f64>,
    pub intensity: Option<f64>,
}
impl PartialEq for AmbientLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/lighting/src/ambientLight.ts:11 (sha256:80d29666b2ca27ebaad8d8e549cf5a593a91a5579a1a1c7e0044f995f760aef8)
pub fn clone_ambient_light(source: &AmbientLight) -> AmbientLight {
    return create_ambient_light(Some(AmbientLightOptions {
        __flight_identity: std::sync::Arc::new(()),
        color: Some(source.color),
        intensity: Some(source.intensity),
    }));
}

// Source: upstream/packages/lighting/src/ambientLight.ts:17 (sha256:528ddba418833e184b363a5370c6e43a2c26770dfa6f91480c6bf3c29c50c5ad)
pub fn create_ambient_light(options: Option<AmbientLightOptions>) -> AmbientLight {
    return create_entity(Some(AmbientLight {
        __flight_identity: std::sync::Arc::new(()),
        color: (options.as_ref().and_then(|value| value.color)).unwrap_or(4294967295.0_f64),
        intensity: (options.as_ref().and_then(|value| value.intensity)).unwrap_or(1.0_f64),
        kind: ambient_light_kind_constant,
    }));
}
