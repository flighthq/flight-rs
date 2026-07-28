// @generated from upstream/packages/types/src/SignalScope.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SignalConnection;

// Source: upstream/packages/types/src/SignalScope.ts:11 (sha256:ab1284b002b4f39be560d8d1b63061737bf030c13630a89059462527e899c163)
#[derive(Clone)]
pub struct SignalScope {
    pub connections: Vec<SignalConnection>,
}
