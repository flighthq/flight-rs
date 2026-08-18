// @generated from upstream/packages/types/src/BitmapBoxBlurOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapBoxBlurOptions.ts:1 (sha256:11e94fea5246dd4af2023836bd04830e27c271d5334a3aadf238298597800f0e)
#[derive(Clone, Default)]
pub struct BitmapBoxBlurOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub radius_x: Option<f64>,
    pub radius_y: Option<f64>,
    pub passes: Option<f64>,
}
impl PartialEq for BitmapBoxBlurOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
