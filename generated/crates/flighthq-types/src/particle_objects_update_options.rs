// @generated from upstream/packages/types/src/ParticleObjectsUpdateOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ParticleEmitterCallbacks;

// Source: upstream/packages/types/src/ParticleObjectsUpdateOptions.ts:3 (sha256:578cde9f995317e1dbc5e32d7b42ff5ae83cd8ee41e21ec931f425741cc49633)
#[derive(Clone, Default)]
pub struct ParticleObjectsUpdateOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub callbacks: Option<ParticleEmitterCallbacks>,
    pub emitter_x: Option<f64>,
    pub emitter_y: Option<f64>,
}
impl PartialEq for ParticleObjectsUpdateOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
