// @generated from upstream/packages/types/src/FontResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/FontResource.ts:1 (sha256:05e2cc1dd0df88cfa444c4916544b96e5e1d44ce68be13c3fae8e5842aa95f11)
#[derive(Clone)]
pub struct FontResource {
    pub family: String,
    pub face: Option<crate::OpaqueHostValue>,
}
