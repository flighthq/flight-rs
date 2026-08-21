// @generated from upstream/packages/types/src/Camera3DViewExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Camera3DViewExplanation.ts:4 (sha256:b9b7f7010d3b8a221b9f90eb5c46b3b59f82fd0e954194f124d608a6b7ee0223)
#[derive(Clone, Default)]
pub struct Camera3DViewExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub determinant: f64,
    pub is_orthonormal: bool,
    pub is_reflection: bool,
    pub scale_deviation: f64,
    pub shear_deviation: f64,
}
impl PartialEq for Camera3DViewExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
