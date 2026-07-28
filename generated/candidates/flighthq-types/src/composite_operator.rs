// @generated from upstream/packages/types/src/CompositeOperator.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CompositeOperator.ts:13 (sha256:1de09ebce0100e9f44efd2c4d31879c2262de4cb7062e216cbd7244d798af1cf)
#[derive(Clone)]
pub struct CompositeOperatorValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clear: String,
    pub copy: String,
    pub destination_atop: String,
    pub destination_in: String,
    pub destination_out: String,
    pub destination_over: String,
    pub source_atop: String,
    pub source_in: String,
    pub source_out: String,
    pub source_over: String,
    pub xor: String,
}
impl PartialEq for CompositeOperatorValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static COMPOSITE_OPERATOR: std::sync::LazyLock<CompositeOperatorValues> =
    std::sync::LazyLock::new(|| CompositeOperatorValues {
        __flight_identity: std::sync::Arc::new(()),
        clear: "Clear".to_owned(),
        copy: "Copy".to_owned(),
        destination_atop: "DestinationAtop".to_owned(),
        destination_in: "DestinationIn".to_owned(),
        destination_out: "DestinationOut".to_owned(),
        destination_over: "DestinationOver".to_owned(),
        source_atop: "SourceAtop".to_owned(),
        source_in: "SourceIn".to_owned(),
        source_out: "SourceOut".to_owned(),
        source_over: "SourceOver".to_owned(),
        xor: "Xor".to_owned(),
    });

// Source: upstream/packages/types/src/CompositeOperator.ts:27 (sha256:c7fd9aedb13c23501755aafeeaf0ab5d633ba9c045439947014d2c904a028e99)
pub type CompositeOperator = String;
