// @generated from upstream/packages/types/src/Group.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, NodeData, Quaternion, SceneNodeRuntime, Vector3};

// Source: upstream/packages/types/src/Group.ts:2 (sha256:915353b7c975ff8acc0de8188a6114cc3ba2c609c1f3f3f57944ca7a52dc0d30)
#[derive(Clone)]
pub struct Group {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Group.ts:3 (sha256:338f4e0bc89b950accbef3a311594b904db99e7adeecbc682d95fc6af3ba88bb)
pub type GroupRuntime = SceneNodeRuntime;

// Source: upstream/packages/types/src/Group.ts:4 (sha256:abeb23f5f4400780b2f81bdafad57f005072187637585f9ac6aa050ffd49b285)
pub const GROUP_KIND: &'static str = "Group";
