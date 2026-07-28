// @generated from upstream/packages/types/src/HasColorTransform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ColorTransform, Kind, NodeData};

// Source: upstream/packages/types/src/HasColorTransform.ts:11 (sha256:2741d9efd5f774a32dc979e21ec9e6a9e83d7965042b23eb2898e3d2b3dc5984)
#[derive(Clone)]
pub struct HasColorTransform {
    pub color_transform: Option<ColorTransform>,
}

// Source: upstream/packages/types/src/HasColorTransform.ts:15 (sha256:c71983ba71d03c397abfb2ca2c08fe27203037688f5d7c1588d0a3102144b6f4)
#[derive(Clone)]
pub struct ColorTransformNode {
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub color_transform: Option<ColorTransform>,
}
