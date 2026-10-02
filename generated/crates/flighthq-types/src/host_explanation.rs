// @generated from upstream/packages/types/src/HostExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/HostExplanation.ts:4 (sha256:1d096e0dd5e9d5a32dee978982cbaf1ccc394eb5f3b9769042323d7faf411ad6)
#[derive(Clone, Default)]
pub struct HostExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub groups: Vec<HostCapabilityGroupExplanation>,
    pub capabilities: Vec<HostCapabilityCoverage>,
}
impl PartialEq for HostExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostExplanation.ts:14 (sha256:d9cf13aa4f68673cc9ea5cf5d812c0baa9e15a96c20f0bdac1f35421ba0eb5ac)
#[derive(Clone, Default)]
pub struct HostCapabilityGroupExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub group: String,
    pub slots: Vec<String>,
}
impl PartialEq for HostCapabilityGroupExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostExplanation.ts:20 (sha256:5dee178b1144f27df3e9355617f280ced1be9bcf11f1aaec2a0f4f59b0e01fc9)
#[derive(Clone, Default)]
pub struct HostCapabilityBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entry_point: String,
    pub package_name: String,
    pub platform: String,
}
impl PartialEq for HostCapabilityBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostExplanation.ts:28 (sha256:be72e899f7f8143d054d8b5f174b5bfe7a2946b92c41dc9dd742d3433836054c)
#[derive(Clone, Default)]
pub struct HostCapabilityExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub backends: Vec<HostCapabilityBackend>,
    pub capability: String,
    pub group: String,
    pub is_present: bool,
    pub message: String,
    pub slot: String,
}
impl PartialEq for HostCapabilityExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostExplanation.ts:39 (sha256:5be67b634d5beeca0208ae134326a84b9bdecb1ea0872f11823f1db583e868c6)
#[derive(Clone, Default)]
pub struct HostCapabilityCoverage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub capability: String,
    pub group: String,
    pub is_present: bool,
    pub slot: String,
}
impl PartialEq for HostCapabilityCoverage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
