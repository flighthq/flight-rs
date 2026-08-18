// @generated from upstream/packages/types/src/SpritesheetParseOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SpritesheetParseOptions.ts:1 (sha256:b55c47dff19fa11e65501c446b7ae147bffc09f88031ece9cddfa49bbdf05a54)
#[derive(Clone, Default)]
pub struct SpritesheetParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame_duration: Option<f64>,
    pub image_height: Option<f64>,
    pub image_width: Option<f64>,
}
impl PartialEq for SpritesheetParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
