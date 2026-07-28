// @generated from upstream/packages/font/src/fontResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::FontResource;

// Source: upstream/packages/font/src/fontResource.ts:3 (sha256:6a671ddda6373401f84c06d9666f1651c49e30f0cbb88055b0aa3ee88b7352e5)
pub fn create_font_resource(family: String) -> FontResource {
    return FontResource {
        __flight_identity: std::sync::Arc::new(()),
        family: (family).clone(),
        face: None,
    };
}
