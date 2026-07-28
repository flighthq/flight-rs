// @generated from upstream/packages/types/src/GridSliceOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GridSliceOptions.ts:1 (sha256:74eac29226f65baa0ccd8d51e1d6a30564af4f67ecd026148f3e2cf967127176)
#[derive(Clone, Default)]
pub struct GridSliceOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub columns: f64,
    pub frame_height: Option<f64>,
    pub frame_width: Option<f64>,
    pub image_file: String,
    pub image_height: f64,
    pub image_width: f64,
    pub margin_x: Option<f64>,
    pub margin_y: Option<f64>,
    pub name_prefix: Option<String>,
    pub rows: f64,
    pub spacing_x: Option<f64>,
    pub spacing_y: Option<f64>,
}
impl PartialEq for GridSliceOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
