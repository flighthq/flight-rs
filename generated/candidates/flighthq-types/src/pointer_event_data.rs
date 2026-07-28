// @generated from upstream/packages/types/src/PointerEventData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::NodeAny;

// Source: upstream/packages/types/src/PointerEventData.ts:3 (sha256:a21b27d68119da759ea2e963106f0280744090b06621aba95150c883bc80fb23)
#[derive(Clone, Default)]
pub struct PointerEventData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alt_key: bool,
    pub button: f64,
    pub buttons: f64,
    pub ctrl_key: bool,
    pub current_target: Option<NodeAny>,
    pub delta_x: f64,
    pub delta_y: f64,
    pub local_x: f64,
    pub local_y: f64,
    pub meta_key: bool,
    pub pointer_id: f64,
    pub pointer_type: PointerType,
    pub shift_key: bool,
    pub target: Option<NodeAny>,
    pub world_x: f64,
    pub world_y: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for PointerEventData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/PointerEventData.ts:24 (sha256:c584338f2125c37e6d11257c22c6b2a7672743f41eaba5b6ad7d6609d57f6ba8)
pub type PointerType = String;
