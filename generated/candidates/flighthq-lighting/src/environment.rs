// @generated from upstream/packages/lighting/src/environment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{CubeTexture, ENVIRONMENT_KIND as environment_kind_constant, Environment};

// Source: upstream/packages/lighting/src/environment.ts:5 (sha256:ce0e4dfc08f4a0dc24d5dd5edb21e699b40e425877de3d1721ef14d6ad242721)
#[derive(Clone)]
pub struct EnvironmentOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub environment: Option<CubeTexture>,
    pub intensity: Option<f64>,
}
impl PartialEq for EnvironmentOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/lighting/src/environment.ts:12 (sha256:3eec31dea4aa5748979de050401303e41a4df104642c8765eae3c53187b93939)
pub fn clone_environment(source: &Environment) -> Environment {
    return create_environment(Some(EnvironmentOptions {
        __flight_identity: std::sync::Arc::new(()),
        environment: (source.environment).clone(),
        intensity: Some(source.intensity),
    }));
}

// Source: upstream/packages/lighting/src/environment.ts:19 (sha256:1af65626199b1757459f91d0c5428b0c62a4671c6dfd5a2b24ea46c6824d7720)
pub fn create_environment(options: Option<EnvironmentOptions>) -> Environment {
    return create_entity(Some(Environment {
        __flight_identity: std::sync::Arc::new(()),
        environment: options
            .as_ref()
            .and_then(|value| (value.environment).clone()),
        intensity: (options.as_ref().and_then(|value| value.intensity)).unwrap_or(1.0_f64),
        kind: environment_kind_constant,
    }));
}
