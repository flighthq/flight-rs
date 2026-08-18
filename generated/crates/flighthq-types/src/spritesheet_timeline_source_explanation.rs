// @generated from upstream/packages/types/src/SpritesheetTimelineSourceExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpritesheetAnimation;

// Source: upstream/packages/types/src/SpritesheetTimelineSourceExplanation.ts:5 (sha256:2268ec08dabefe50244ffd7e96f7b1286440f260b521bc3bca35e4a5e149d785)
pub type SpritesheetTimelineSourceUnsupportedField = String;

// Source: upstream/packages/types/src/SpritesheetTimelineSourceExplanation.ts:9 (sha256:d3136d6e205a4b882b82bc0f2cdc4137a9ba5c68586af551798651bda720fcd0)
#[derive(Clone, Default)]
pub struct SpritesheetTimelineSourceExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub direction_materialized: bool,
    pub unsupported_fields: Vec<SpritesheetTimelineSourceUnsupportedField>,
}
impl PartialEq for SpritesheetTimelineSourceExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpritesheetTimelineSourceExplanation.ts:16 (sha256:49bb2abda130c42753d7c28406b43a4cf7295175c66ccc0595c8e5aaa30cb4a5)
pub type SpritesheetTimelineSourceGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(SpritesheetAnimation, SpritesheetTimelineSourceExplanation) -> ()
                + Send
                + 'static,
        >,
    >,
>;
