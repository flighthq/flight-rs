// @generated from upstream/packages/types/src/HasClip.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ClipRegion, Kind, NodeData};

// Source: upstream/packages/types/src/HasClip.ts:5 (sha256:b39fa5ade7f4bc73aef3c9c9d0ae19562baf86ac8cb264b0b1c642fafde0e42f)
#[derive(Clone, Default)]
pub struct HasClip {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for HasClip {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HasClip.ts:14 (sha256:dc1acf36df8ea13ca9d52998f125def6512ad669abfd31590fa73865c2bbd85a)
#[derive(Clone, Default)]
pub struct ClipNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for ClipNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
