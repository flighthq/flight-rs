// @generated from upstream/packages/types/src/MaterialConversionExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MaterialConversionExplanation.ts:13 (sha256:700a85a3f049e468d10655d6d066d5480582b5ebacc7f3f51d9e225e89c90b21)
#[derive(Clone, Default)]
pub struct MaterialConversionExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub dropped_maps: Vec<String>,
    pub reason: Option<MaterialConversionDropReason>,
}
impl PartialEq for MaterialConversionExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MaterialConversionExplanation.ts:21 (sha256:49b9cd95446eb8091b52a5deba7bb35fdbd3fe87bcff35ebecaeade0ff74829e)
pub type MaterialConversionDropReason = String;

// Source: upstream/packages/types/src/MaterialConversionExplanation.ts:26 (sha256:ab358dac87d3cfef21c3ebad178375cae4dbbcd1abb626ac47d03e54ab7104b5)
pub type MaterialConversionGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(MaterialConversionExplanation, String) -> () + Send + 'static>>,
>;
