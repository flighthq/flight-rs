// @generated from upstream/packages/types/src/SceneCoverageEntry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, RenderRegistry, RequirementFacet};

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:8 (sha256:5c4dac21fc1d5e2bc900207560760ac7f645377fdc521885f615c9166c4fadae)
#[derive(Clone, Default)]
pub struct SceneCoverageValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fallback_remediable: String,
    pub fallback_unavailable: String,
    pub satisfied: String,
    pub unavailable: String,
    pub unregistered: String,
}
impl PartialEq for SceneCoverageValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SCENE_COVERAGE: std::sync::LazyLock<SceneCoverageValues> =
    std::sync::LazyLock::new(|| SceneCoverageValues {
        __flight_identity: std::sync::Arc::new(()),
        fallback_remediable: "FallbackRemediable".to_owned(),
        fallback_unavailable: "FallbackUnavailable".to_owned(),
        satisfied: "Satisfied".to_owned(),
        unavailable: "Unavailable".to_owned(),
        unregistered: "Unregistered".to_owned(),
    });

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:16 (sha256:f63e413d0aeddd22744b0d0b2e13e6851df3dd28e976f0c3df014fe6916d81d1)
pub type SceneCoverage = String;

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:31 (sha256:924b771a0cedb874d826c2c9763d173d2ca878030ed5fc5e97c6640ef3d4edaf)
#[derive(Clone)]
pub struct SceneCoverageEntryBase {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub facet: RequirementFacet,
    pub kind: Kind,
    pub registry: RenderRegistry,
}
impl PartialEq for SceneCoverageEntryBase {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:37 (sha256:9b70606f4c375a4c867163868c21b4bcb39aaca6641c92f830ac17f524d73ef4)
#[derive(Clone, Default)]
pub struct SceneCoverageRemedy {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub module: String,
    pub registrar: String,
}
impl PartialEq for SceneCoverageRemedy {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:42 (sha256:afa38ba6719d3469100d48a4c62df625be6ccdc3adfb1c068965daaa7a003592)
#[derive(Clone)]
pub struct SatisfiedSceneCoverageEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub facet: RequirementFacet,
    pub kind: Kind,
    pub registry: RenderRegistry,
    pub coverage: String,
}
impl PartialEq for SatisfiedSceneCoverageEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:46 (sha256:c16eae94dc44606ba7d122855875ddc9233a50f566d29424430f4ffa1e648d22)
#[derive(Clone)]
pub struct UnregisteredSceneCoverageEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub facet: RequirementFacet,
    pub kind: Kind,
    pub registry: RenderRegistry,
    pub module: String,
    pub registrar: String,
    pub coverage: String,
}
impl PartialEq for UnregisteredSceneCoverageEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:50 (sha256:2c4692af3bb7cc7c175b78e564c6033525ee19726fdb1d5891dee8b1f15de91c)
#[derive(Clone)]
pub struct UnavailableSceneCoverageEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub facet: RequirementFacet,
    pub kind: Kind,
    pub registry: RenderRegistry,
    pub coverage: String,
}
impl PartialEq for UnavailableSceneCoverageEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:54 (sha256:941754b5d2fec1a49393ef55e60f00bc369a289bfb5ac981108d13979a8a54c9)
#[derive(Clone)]
pub struct FallbackRemediableSceneCoverageEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub facet: RequirementFacet,
    pub kind: Kind,
    pub registry: RenderRegistry,
    pub module: String,
    pub registrar: String,
    pub coverage: String,
}
impl PartialEq for FallbackRemediableSceneCoverageEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:58 (sha256:b76f02b5e8fecd2c00348eaffb6eb1c5c8e710d774cfbc844d3952512aa55f5e)
#[derive(Clone)]
pub struct FallbackUnavailableSceneCoverageEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub facet: RequirementFacet,
    pub kind: Kind,
    pub registry: RenderRegistry,
    pub coverage: String,
}
impl PartialEq for FallbackUnavailableSceneCoverageEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneCoverageEntry.ts:64 (sha256:e000a8d40751f7b3cabed71ae89cc114c27712c6255f46c2475a30a202d8540b)
pub type SceneCoverageEntry = crate::FlightUnion2<
    FallbackRemediableSceneCoverageEntry,
    crate::FlightUnion2<
        FallbackUnavailableSceneCoverageEntry,
        crate::FlightUnion2<
            SatisfiedSceneCoverageEntry,
            crate::FlightUnion2<UnavailableSceneCoverageEntry, UnregisteredSceneCoverageEntry>,
        >,
    >,
>;
