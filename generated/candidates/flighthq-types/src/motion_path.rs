// @generated from upstream/packages/types/src/MotionPath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Path;

// Source: upstream/packages/types/src/MotionPath.ts:13 (sha256:8b134fdf89edc14002bdfb7772da3967fb91db8069a65fbc6d84ed7295a1a8d1)
pub type MotionPathLoopMode = String;

// Source: upstream/packages/types/src/MotionPath.ts:23 (sha256:9ed90b466cc7a1b9394a8de4672a9f73333fe5c5379a26853f9d55533c9c0c5d)
#[derive(Clone)]
pub struct MotionPath {
    pub direction: f64,
    pub distance: f64,
    pub length: f64,
    pub loop_mode: MotionPathLoopMode,
    pub path: Path,
    pub speed: f64,
}
