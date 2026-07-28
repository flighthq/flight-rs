// @generated from upstream/packages/types/src/CircleCollider.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CircleCollider.ts:3 (sha256:dcfcc46aced0c60b01e65f51abfa2b424490c5877253164876e9db3fa19c7998)
#[derive(Clone)]
pub struct CircleCollider {
    pub restitution: Option<f64>,
    pub friction: Option<f64>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub mode: String,
}

// Source: upstream/packages/types/src/CircleCollider.ts:11 (sha256:8a8463ae886cdf3efe1f59036f5cada514205b21702dcb0ba19c9f14397c13c1)
pub const CIRCLE_COLLIDER_KIND: &'static str = "CircleCollider";
