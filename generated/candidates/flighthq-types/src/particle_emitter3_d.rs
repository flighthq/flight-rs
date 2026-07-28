// @generated from upstream/packages/types/src/ParticleEmitter3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, ParticleBlendMode, ParticleEmitterData, Quaternion, SceneNodeRuntime, Vector3};

// Source: upstream/packages/types/src/ParticleEmitter3D.ts:5 (sha256:d66b6f092414db98431d68c37d08a251ff2e6ebfbec95b5e3e3d038794e94ae5)
#[derive(Clone)]
pub struct ParticleEmitter3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: ParticleEmitterData,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
    pub blend_mode: ParticleBlendMode,
}
impl PartialEq for ParticleEmitter3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ParticleEmitter3D.ts:13 (sha256:e20ec3b85ad5a39e40ce41eaa87bc07cc0a9c2c3c19c45f70ca492674f04cdc1)
pub type ParticleEmitter3DRuntime = SceneNodeRuntime;

// Source: upstream/packages/types/src/ParticleEmitter3D.ts:15 (sha256:353010cff3c09a3d101a72c968869edbda5e245192be162cece34fa3828f8ac1)
pub const PARTICLE_EMITTER3_D_KIND: &'static str = "ParticleEmitter3D";
