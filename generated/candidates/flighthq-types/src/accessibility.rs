// @generated from upstream/packages/types/src/Accessibility.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Rectangle;

// Source: upstream/packages/types/src/Accessibility.ts:14 (sha256:f7f46e1ad1154345bbd9252e407c8735cc7131a7208dbc4abe34711155d025a2)
pub type AccessibilityRole = String;

// Source: upstream/packages/types/src/Accessibility.ts:38 (sha256:1b94d96cd414b45711b780ffeb18475bad999008289fe31bdc528c97db6c2d7c)
pub type AccessibilityLiveness = String;

// Source: upstream/packages/types/src/Accessibility.ts:44 (sha256:1f209c4f7d90191f56d8beee8987f5e88fd79dde04fdb55d2259a7ed5061c8e7)
#[derive(Clone)]
pub struct AccessibilityState {
    pub disabled: Option<bool>,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
    pub selected: Option<bool>,
    pub pressed: Option<bool>,
    pub busy: Option<bool>,
    pub hidden: Option<bool>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub level: Option<f64>,
    pub value_min: Option<f64>,
    pub value_max: Option<f64>,
    pub value_now: Option<f64>,
}

// Source: upstream/packages/types/src/Accessibility.ts:65 (sha256:0d54531616bd2ab0cae1a50a1978b2e6307e45e6937724a91c4f5dee64f19703)
#[derive(Clone)]
pub struct AccessibilityNode {
    pub id: String,
    pub role: AccessibilityRole,
    pub label: Option<String>,
    pub description: Option<String>,
    pub value: Option<String>,
    pub parent_id: Option<String>,
    pub bounds: Option<Rectangle>,
    pub states: Option<AccessibilityState>,
}

// Source: upstream/packages/types/src/Accessibility.ts:80 (sha256:a51d3b613a3fd006371860c5d5bd1886bb7d54be4977bbb89c8d051cbd90c9e5)
#[derive(Clone)]
pub struct AccessibilityBackend {
    pub set_node: crate::OpaqueHostValue,
    pub remove_node: crate::OpaqueHostValue,
    pub clear: crate::OpaqueHostValue,
    pub set_focus: crate::OpaqueHostValue,
    pub announce: crate::OpaqueHostValue,
}
