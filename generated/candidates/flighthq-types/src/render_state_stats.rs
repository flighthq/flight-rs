// @generated from upstream/packages/types/src/RenderStateStats.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RenderStateStats.ts:1 (sha256:e15542a000266526dd230ed0dfaf27250b30220b35c80cdfff0afa61c188a307)
#[derive(Clone)]
pub struct RenderStateStats {
    pub draw_call_count: f64,
    pub flush_count: f64,
    pub proxy_visited_count: f64,
}
