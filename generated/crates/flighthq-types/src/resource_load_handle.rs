// @generated from upstream/packages/types/src/ResourceLoadHandle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ResourceLoadHandle.ts:1 (sha256:cbd995c7eceaf7e4628b1cf9dc58a1abd21d4989c57987f4a4b6151364af252b)
#[derive(Clone, Default)]
pub struct ResourceLoadHandle<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub key: String,
    pub promise: crate::Promise<T>,
}
impl<T> PartialEq for ResourceLoadHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
