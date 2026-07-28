// @generated from upstream/packages/types/src/TurbulenceForce.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TurbulenceForce.ts:1 (sha256:c70dcdd281c70a6164e800a27f105327058f323e9ffcca75502bf78bbb0ebf78)
#[derive(Clone)]
pub struct TurbulenceForce {
    pub kind: String,
    pub strength: f64,
    pub scale: f64,
}

// Source: upstream/packages/types/src/TurbulenceForce.ts:7 (sha256:178cb097068d53978a569129ce3131cd8134c937fa649e9f0aae3e9cdd2ece84)
pub const TURBULENCE_FORCE_KIND: &'static str = "TurbulenceForce";
