// @generated from upstream/packages/types/src/HasMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, Material, MaterialData, NodeData};

// Source: upstream/packages/types/src/HasMaterial.ts:7 (sha256:50eacbe15271d1df42963302c6bed2ca8c6bd60772e9e4a3f3a0cbdc6896cdc3)
#[derive(Clone)]
pub struct HasMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for HasMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HasMaterial.ts:12 (sha256:e30a568fec2eeea97adb852cb0aa803330e195caa60677fa862123007e17fad4)
#[derive(Clone)]
pub struct MaterialNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for MaterialNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
