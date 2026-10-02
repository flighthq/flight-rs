// @generated from upstream/packages/types/src/ObjMaterialHandler.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, MaterialLike, ObjMaterial, Scene3DDocument};

// Source: upstream/packages/types/src/ObjMaterialHandler.ts:18 (sha256:7d9e4054fd71e5cb95b69c7d60249473a3d21e8b38b25328b4fc3460e9ca2925)
#[derive(Clone)]
pub struct ObjMaterialHandler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub feature: String,
    pub matches:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ObjMaterial) -> bool + Send + 'static>>>,
    pub resolve: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        ObjMaterial,
                        Scene3DDocument,
                        Option<Vec<ImportDiagnostic>>,
                    ) -> MaterialLike
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ObjMaterialHandler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ObjMaterialHandler.ts:48 (sha256:769bc0f7ed8a01c050e0eb77b10e814993b1cac5087f6662ce93dcdce83de409)
#[derive(Clone, Default)]
pub struct ObjImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub material_handlers: Option<Vec<ObjMaterialHandler>>,
}
impl PartialEq for ObjImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ObjMaterialHandler.ts:53 (sha256:ee0860e83a12e0729fc5922c8c345259359123abf68b0c09a5b6f1dc86d3ea9b)
pub const OBJ_MATERIAL_BLINN_PHONG_FEATURE: &'static str = "MaterialBlinnPhong";

// Source: upstream/packages/types/src/ObjMaterialHandler.ts:56 (sha256:e805ef635c1bbfcb6baba7b252688de6451d6238fb5529f262cad5c1c244bb2f)
pub const OBJ_MATERIAL_STANDARD_PBR_FEATURE: &'static str = "MaterialStandardPbr";
