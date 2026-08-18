// @generated from upstream/packages/types/src/ShapeTessellationExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ShapeTessellationExplanation.ts:1 (sha256:419e8426103456f73c2d7f3854477fdd2eae646a8ba89aa67069589f90785262)
pub type ShapeTessellationStatus = String;

// Source: upstream/packages/types/src/ShapeTessellationExplanation.ts:3 (sha256:189a103bb58ea8872b8ffc331d080a29af676ce0891915b6a5cecf3bd9d7d260)
pub type ShapeTessellationBlocker = String;

// Source: upstream/packages/types/src/ShapeTessellationExplanation.ts:22 (sha256:6617c9632dcdc61b609824daa7355ead7e18eece2db30b444044d6030a04d311)
#[derive(Clone, Default)]
pub struct ShapeTessellationExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub blocked_by: ShapeTessellationBlocker,
    pub status: ShapeTessellationStatus,
}
impl PartialEq for ShapeTessellationExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
