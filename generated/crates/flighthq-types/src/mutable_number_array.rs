// @generated from upstream/packages/types/src/MutableNumberArray.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MutableNumberArray.ts:9 (sha256:a14f7df9e0a18d40af4ed167c5a1eac027df43b8db8ffda101bdf06a629c9a7c)
pub type MutableNumberArray =
    crate::FlightUnion2<Vec<f32>, crate::FlightUnion2<Vec<f64>, Vec<f64>>>;
