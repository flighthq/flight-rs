// @generated from upstream/packages/types/src/TiledProperty.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TiledProperty.ts:7 (sha256:1d67acffff75f97f93b3b6b855de6b151071820aafe9361bc0bde1ee4aa13dcc)
pub type TiledPropertyType = String;

// Source: upstream/packages/types/src/TiledProperty.ts:9 (sha256:e8f81c64bbdac1c2bfe70e245844a7449d62dfc1978d2a4d1340dd6f30e16109)
#[derive(Clone)]
pub struct TiledProperty {
    pub name: String,
    pub type_: TiledPropertyType,
    pub value: crate::OpaqueHostValue,
}
