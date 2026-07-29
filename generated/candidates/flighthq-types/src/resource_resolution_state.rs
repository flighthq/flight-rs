// @generated from upstream/packages/types/src/ResourceResolutionState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ResourceResolutionState.ts:13 (sha256:e6f358cfd733d99644950b435f4f6d03df783abd35c42700ab41de86e63960ba)
#[derive(Clone, Default)]
pub struct ResourceResolutionStateValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub failed: String,
    pub loading: String,
    pub resolved: String,
    pub unresolved: String,
}
impl PartialEq for ResourceResolutionStateValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static RESOURCE_RESOLUTION_STATE: std::sync::LazyLock<ResourceResolutionStateValues> =
    std::sync::LazyLock::new(|| ResourceResolutionStateValues {
        __flight_identity: std::sync::Arc::new(()),
        failed: "Failed".to_owned(),
        loading: "Loading".to_owned(),
        resolved: "Resolved".to_owned(),
        unresolved: "Unresolved".to_owned(),
    });

// Source: upstream/packages/types/src/ResourceResolutionState.ts:20 (sha256:bd9345ec114ed8b323d413d80d2a5359289f48257620fe7937b4e9d4f5fcc1ed)
pub type ResourceResolutionState = crate::OpaqueHostValue;
