// @generated from upstream/packages/types/src/DomScene2DRectangle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/DomScene2DRectangle.ts:1 (sha256:3e0778c86763ff30fd5e4ca6383466d07adf2ea324b637d54dbb2ddc72c803be)
#[derive(Clone, Default)]
pub struct DomScene2DRectangle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
    pub top: f64,
}
impl PartialEq for DomScene2DRectangle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
