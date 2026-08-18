// @generated from upstream/packages/types/src/ResourceLoadBytesReporter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ResourceLoadBytesReporter.ts:11 (sha256:b27478a70990895fa9c139e3ef3f30c4c7177a2207091ef9c5a7b6b7713cd899)
pub type ResourceLoadBytesReporter =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, Option<f64>) -> () + Send + 'static>>>;
