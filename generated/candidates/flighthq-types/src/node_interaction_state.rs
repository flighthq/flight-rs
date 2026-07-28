// @generated from upstream/packages/types/src/NodeInteractionState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Cursor, HitArea};

// Source: upstream/packages/types/src/NodeInteractionState.ts:18 (sha256:1a80443ef92b7e9bc7ac2dd87e10ced28db13469f8d2df5f858aa35a5a986944)
#[derive(Clone)]
pub struct NodeInteractionState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hit_test_enabled: bool,
    pub hit_area: Option<HitArea>,
    pub cursor: Option<Cursor>,
    pub focusable: bool,
    pub tab_index: f64,
}
impl PartialEq for NodeInteractionState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
