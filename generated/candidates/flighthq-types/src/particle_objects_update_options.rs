// @generated from upstream/packages/types/src/ParticleObjectsUpdateOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ParticleObjectsUpdateOptions.ts:1 (sha256:1ed9b46eef521e88df1ffdb7977e1ad37acda2e09c6c5b198ec3b8aaf15e7fd0)
#[derive(Clone, Default)]
pub struct ParticleObjectsUpdateOptionsRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub on_death: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_spawn:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>>,
}
impl PartialEq for ParticleObjectsUpdateOptionsRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ParticleObjectsUpdateOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub callbacks: Option<ParticleObjectsUpdateOptionsRecord1>,
    pub emitter_x: Option<f64>,
    pub emitter_y: Option<f64>,
}
impl PartialEq for ParticleObjectsUpdateOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
