// @generated from upstream/packages/types/src/MarkupTagRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{MarkupClassResolver, MarkupColorResolver};

// Source: upstream/packages/types/src/MarkupTagRegistry.ts:10 (sha256:91817bed76d0f18f64a416ffaff3426e341ec0d81715b5a30d25d9cc7a434cc3)
#[derive(Clone)]
pub struct MarkupTagRegistry {
    pub class_resolver: Option<MarkupClassResolver>,
    pub color_resolver: Option<MarkupColorResolver>,
    pub handlers: crate::OpaqueHostValue,
}
