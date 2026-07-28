// @generated from upstream/packages/types/src/CompositeEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::CompositeOperator;

// Source: upstream/packages/types/src/CompositeEffect.ts:13 (sha256:bf05079710fd67f5aae2e4d6d461097859d32603e187134147b46dd2d6da9fcf)
#[derive(Clone)]
pub struct CompositeEffect {
    pub kind: String,
    pub operator: CompositeOperator,
    pub backdrop_key: Option<String>,
}
