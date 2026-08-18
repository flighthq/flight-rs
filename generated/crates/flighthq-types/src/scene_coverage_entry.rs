// @generated from upstream/packages/types/src/SceneCoverageEntry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, RenderRegistry};

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:9 (sha256:427e7f910d1faed28ad5f6b171c595fc70e03edfd73665d12f97986ce68a030e)
#[derive(Clone, Default)]
pub struct SceneCoverageValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fallback: String,
    pub missing: String,
    pub satisfied: String,
}
impl PartialEq for SceneCoverageValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SCENE_COVERAGE: std::sync::LazyLock<SceneCoverageValues> =
    std::sync::LazyLock::new(|| SceneCoverageValues {
        __flight_identity: std::sync::Arc::new(()),
        fallback: "Fallback".to_owned(),
        missing: "Missing".to_owned(),
        satisfied: "Satisfied".to_owned(),
    });

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:15 (sha256:f63e413d0aeddd22744b0d0b2e13e6851df3dd28e976f0c3df014fe6916d81d1)
pub type SceneCoverage = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:30 (sha256:f1a9028b808623292bafde20b0341859db0c91931b940ea4ca367fd7d026bd06)
#[derive(Clone)]
pub struct SceneCoverageEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub coverage: SceneCoverage,
    pub kind: Kind,
    pub registry: RenderRegistry,
}
impl PartialEq for SceneCoverageEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
