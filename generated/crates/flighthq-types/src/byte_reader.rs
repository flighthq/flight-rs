// @generated from upstream/packages/types/src/ByteReader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ByteReader.ts:5 (sha256:abd4b65266795311fec9a1be258c354fce859129899591606d3a2ff6f6a4a613)
#[derive(Clone, Default)]
pub struct ByteReader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub view: crate::OpaqueHostValue,
    pub offset: f64,
}
impl PartialEq for ByteReader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
