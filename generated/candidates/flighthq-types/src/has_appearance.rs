// @generated from upstream/packages/types/src/HasAppearance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, NodeData};

// Source: upstream/packages/types/src/HasAppearance.ts:8 (sha256:13dfd355603960d1b4add1a1a1892729ae295119b141568c82c4d558e22338b5)
#[derive(Clone)]
pub struct HasAppearance {
    pub alpha: f64,
    pub visible: bool,
}

// Source: upstream/packages/types/src/HasAppearance.ts:18 (sha256:72db87ddbd47a6b7ed756512bc9062f1abb6f56c55f900740bfacdd32390a1f2)
#[derive(Clone)]
pub struct HasAppearanceRuntime {
    pub binding: Option<crate::OpaqueHostValue>,
    pub world_alpha: Option<f64>,
    pub world_alpha_using_appearance_id: f64,
    pub world_alpha_using_parent_appearance_id: f64,
    pub world_appearance_id: f64,
}

// Source: upstream/packages/types/src/HasAppearance.ts:25 (sha256:1206541436dbd2f98867827f4b01fecb86820ede63bc9237c0b21d9bcedb36fc)
#[derive(Clone)]
pub struct AppearanceNode {
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
}
