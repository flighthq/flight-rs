// @generated from upstream/packages/types/src/SwfFilterListGuard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SwfFilterListGuard.ts:3 (sha256:09323a2da04ea738be2b0297d146e752437ecfe2bc76f84604a548ab8fc66980)
pub type SwfFilterListGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>;
