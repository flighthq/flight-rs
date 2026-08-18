// @generated from upstream/packages/types/src/BitmapHistogram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapHistogram.ts:1 (sha256:4d6fee878e832a5920b6e74128697cf0366625a630903b6b34753cdaf46c647b)
#[derive(Clone, Default)]
pub struct BitmapHistogram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Vec<f64>,
    pub blue: Vec<f64>,
    pub green: Vec<f64>,
    pub red: Vec<f64>,
}
impl PartialEq for BitmapHistogram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
