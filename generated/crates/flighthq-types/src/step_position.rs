// @generated from upstream/packages/types/src/StepPosition.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/StepPosition.ts:4 (sha256:e7e3c6373e67cb6b473becd4f4a3f42ff6b3622b2527a3fbdf4e0f9dbd8d06dd)
pub type StepPosition = String;

// Source: upstream/packages/types/src/StepPosition.ts:9 (sha256:d644ab94df7d346994303a9ef9a5e3077b0bd40ae39e5a41745daf367274a8ef)
pub type EasingStepsGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, StepPosition) -> () + Send + 'static>>>;
