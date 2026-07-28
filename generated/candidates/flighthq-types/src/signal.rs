// @generated from upstream/packages/types/src/Signal.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Signal.ts:3 (sha256:244c27c32b1b9d017992bedada49bbdd64b3fd34a9f53f0bbeb51d3600e5f0fb)
#[derive(Clone)]
pub struct Signal {
    pub data: Option<SignalData>,
    pub emit: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/Signal.ts:8 (sha256:e88b26e0aa626a28c90fb4b79d03451b809f71842206c804f5b9e3c00859f81a)
#[derive(Clone)]
pub struct SignalData {
    pub slots: Vec<crate::OpaqueHostValue>,
    pub priorities: Vec<f64>,
    pub repeat: Vec<bool>,
    pub cancelled: bool,
}
