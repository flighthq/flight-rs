// @generated from upstream/packages/types/src/XmlElement.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/XmlElement.ts:4 (sha256:8a358c6d8580079a28b5e8f9bea479f990c2415fa40a3e31c242e7fb8d75b294)
#[derive(Clone, Default)]
pub struct XmlElement {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<XmlElement>,
    pub content: Vec<crate::FlightUnion2<String, XmlElement>>,
    pub name: String,
    pub text: String,
}
impl PartialEq for XmlElement {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
