// @generated from upstream/packages/types/src/ColladaParse.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ColladaElementDecoder, ImportDiagnostic, Scene3DDocument};

// Source: upstream/packages/types/src/ColladaParse.ts:4 (sha256:a6417668c157583b6d2d1ee0a587e23e31682f5d809885df7acd11489e46bf3e)
pub type ColladaUpAxis = String;

// Source: upstream/packages/types/src/ColladaParse.ts:5 (sha256:348fcf7d90cd3d789df7d5373b2f10f702a7faf378ea5b31f163e5bb9d5789f0)
#[derive(Clone, Default)]
pub struct ColladaImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub base_url: Option<String>,
    pub decoders: Option<Vec<ColladaElementDecoder>>,
}
impl PartialEq for ColladaImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ColladaParse.ts:16 (sha256:86360657fd99de9234a25f35d31e2f6e334d5b5757412de84e9f0369c613a2e3)
#[derive(Clone, Default)]
pub struct ColladaParseResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub document: Scene3DDocument,
    pub diagnostics: Vec<ImportDiagnostic>,
    pub up_axis: ColladaUpAxis,
    pub root_transform: Vec<f64>,
}
impl PartialEq for ColladaParseResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
