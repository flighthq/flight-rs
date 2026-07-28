// @generated from upstream/packages/types/src/ScalarRemap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ScalarRemap.ts:1 (sha256:ba5cedd99b1daf8983b0e83d91ee604bd782f5ba4e75f6481106b5036142e841)
pub type ScalarRemap = std::sync::Arc<dyn Fn(f64) -> f64 + Send + Sync + 'static>;
