// @generated from upstream/packages/types/src/MarkupColorResolver.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MarkupColorResolver.ts:7 (sha256:df71d51106de2b2498432fe0fe49d4940a0420f650f47a0c97647b8c251a469c)
pub type MarkupColorResolver =
    std::sync::Arc<dyn Fn(String) -> Option<f64> + Send + Sync + 'static>;
