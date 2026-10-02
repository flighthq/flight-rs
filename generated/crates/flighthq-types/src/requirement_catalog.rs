// @generated from upstream/packages/types/src/RequirementCatalog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, Requirement, RequirementFacet};

// Source: upstream/packages/types/src/RequirementCatalog.ts:8 (sha256:9179c1bea6c6e9574ae054daeb9f176d08f52feeb6d0577a1cb75cce41f273d9)
#[derive(Clone, Default)]
pub struct RequirementCatalogEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub backend: String,
    pub facet: RequirementFacet,
    pub implementation_import: String,
    pub implementation_symbol: String,
    pub kind: Kind,
    pub family_order: Option<f64>,
    pub parser_field: Option<String>,
    pub parser_export: Option<String>,
    pub content_parser_input_kind: Option<String>,
    pub registrar_import: Option<String>,
    pub registrar_symbol: Option<String>,
}
impl PartialEq for RequirementCatalogEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RequirementCatalog.ts:97 (sha256:711532ac223c6443da96924a06144600bf8ea6c85358ed340836ecb4c4a4a9cc)
#[derive(Clone, Default)]
pub struct RequirementTranslation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub from: Requirement,
    pub to: Vec<Requirement>,
}
impl PartialEq for RequirementTranslation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RequirementCatalog.ts:111 (sha256:d113c2156a0607d3de7858cbbdbbaf80be1fab3e06d925c8d669ced37f0e98c9)
#[derive(Clone, Default)]
pub struct RequirementBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub infrastructure_import: String,
    pub infrastructure_symbol: String,
    pub name: String,
}
impl PartialEq for RequirementBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RequirementCatalog.ts:135 (sha256:cfbb78d8437808ec0218dad39625c5f141f2e9636c744985b350590e0278c014)
#[derive(Clone, Default)]
pub struct RequirementDisposition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub backend: String,
    pub facet: RequirementFacet,
    pub kind: Kind,
    pub reason: String,
}
impl PartialEq for RequirementDisposition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RequirementCatalog.ts:143 (sha256:715ce2c1faaa11e85193c2ea6f2d842c69b1f97adf2ad013cd59454821e42dd4)
#[derive(Clone, Default)]
pub struct RequirementCatalog {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entries: Vec<RequirementCatalogEntry>,
    pub backends: Option<Vec<RequirementBackend>>,
    pub dispositions: Option<Vec<RequirementDisposition>>,
    pub translations: Option<Vec<RequirementTranslation>>,
}
impl PartialEq for RequirementCatalog {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
