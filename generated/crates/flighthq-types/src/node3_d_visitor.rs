// @generated from upstream/packages/types/src/Node3DVisitor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Node3D;

// Source: upstream/packages/types/src/Node3DVisitor.ts:2 (sha256:d71d93104bfa764cb298f8048dd5a646bfcd58df4021daf8bf472bbb6bac4cb6)
pub type Node3DVisitor = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Node3D, f64) -> crate::FlightUnion2<bool, ()> + Send + 'static>>,
>;
