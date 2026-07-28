// @generated from upstream/packages/types/src/NodeSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/NodeSignals.ts:4 (sha256:3debcb6bbfc793c4ea95337f8afd26ae83f81485392ba271ba5300c0a5eb35fa)
#[derive(Clone)]
pub struct NodeSignals {
    pub on_child_added: Signal,
    pub on_child_removed: Signal,
    pub on_children_changed: Signal,
    pub on_children_order_changed: Signal,
    pub on_parent_changed: Signal,
}
