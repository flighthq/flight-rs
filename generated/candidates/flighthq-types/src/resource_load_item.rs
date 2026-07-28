// @generated from upstream/packages/types/src/ResourceLoadItem.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ResourceLoadItem.ts:1 (sha256:c5e91fbd8057e9a97c972f45034056a989efc988c891cffa840569f13dc259a3)
#[derive(Clone)]
pub struct ResourceLoadItem {
    pub bytes_hint: Option<f64>,
    pub group: Option<String>,
    pub key: Option<String>,
    pub load: std::sync::Arc<
        dyn Fn(crate::OpaqueHostValue) -> crate::Promise<crate::OpaqueHostValue>
            + Send
            + Sync
            + 'static,
    >,
    pub on_bytes_progress: Option<std::sync::Arc<dyn Fn(f64, f64) -> () + Send + Sync + 'static>>,
    pub priority: Option<f64>,
    pub retries: Option<f64>,
    pub timeout_ms: Option<f64>,
    pub weight: Option<f64>,
}
