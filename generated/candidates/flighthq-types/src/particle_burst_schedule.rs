// @generated from upstream/packages/types/src/ParticleBurstSchedule.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ParticleBurstSchedule.ts:2 (sha256:642ae888818bc21d880dc1557322e143548aa81a79806debeacc863c79ce20a9)
#[derive(Clone, Default)]
pub struct ParticleBurstEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub time: f64,
    pub count: f64,
    pub cycles: f64,
    pub interval: f64,
}
impl PartialEq for ParticleBurstEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ParticleBurstSchedule.ts:15 (sha256:b030f53cf15e3117b4f8e25c3edd2db63f7347c90cc258d78c97f0d63687a1ca)
pub type ParticleBurstSchedule = Vec<ParticleBurstEntry>;
