// @generated from upstream/packages/types/src/Scene2DKindUsage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, Kind};

// Source: upstream/packages/types/src/Scene2DKindUsage.ts:16 (sha256:496c05d2679fb7bd28f2ea80ce1ab909b2ad53eda73c19b1e1933cddacf8d877)
#[derive(Clone, Default)]
pub struct Scene2DKindUsage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_modes: Vec<BlendMode>,
    pub material_kinds: Vec<Kind>,
    pub node_kinds: Vec<Kind>,
    pub shape_command_keys: Vec<String>,
}
impl PartialEq for Scene2DKindUsage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
