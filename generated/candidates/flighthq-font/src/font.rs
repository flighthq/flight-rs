// @generated from upstream/packages/font/src/font.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::Font;

// Source: upstream/packages/font/src/font.ts:4 (sha256:8a13ce2d6ed4637e6df2f36c78f03b78babab7d786fa0d9aa719527872cda81d)
pub fn create_font(name: String) -> Font {
    return create_entity(Some(Font {
        __flight_identity: std::sync::Arc::new(()),
        name: (name).clone(),
    }));
}
