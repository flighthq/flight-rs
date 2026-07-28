// @generated from upstream/packages/types/src/ScreenMode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ScreenMode.ts:4 (sha256:349708442009d9ca88cc966d0d145afd0cc7550f64a884cf0cc79436a412f93c)
#[derive(Clone)]
pub struct ScreenMode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub width: f64,
    pub height: f64,
    pub refresh_rate: f64,
    pub color_depth: f64,
    pub pixel_format: String,
}
impl PartialEq for ScreenMode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
