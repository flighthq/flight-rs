// @generated from upstream/packages/types/src/StlImportOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/StlImportOptions.ts:17 (sha256:47d5fcdeaca9c2040e4d874510db10622ae2ab21301cc63fb15ec2cc103b0810)
pub type StlNormalPolicy = String;

// Source: upstream/packages/types/src/StlImportOptions.ts:28 (sha256:1daba04b4620c4b8b574bfd1b9af4d988e5b442894159dd405c39ce39f26c9d7)
#[derive(Clone, Default)]
pub struct StlImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub normals: Option<StlNormalPolicy>,
}
impl PartialEq for StlImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
