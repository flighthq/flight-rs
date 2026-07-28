// @generated from upstream/packages/scene/src/billboard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_scene_node, get_scene_node_runtime};
use flighthq_node::{enable_node_signals, get_node_signals};
pub use flighthq_types::{BILLBOARD_KIND, Billboard, BillboardMode, BillboardRuntime};
use flighthq_types::{Kind, Material, MeshGeometry, NodeSignals, SceneNode};

// Source: upstream/packages/scene/src/billboard.ts:32 (sha256:a91e0b104fe82e4b0509ebff9f96b5de5916a22ac4be7b71686a310a068f2cda)
pub fn create_billboard(
    geometry: &MeshGeometry,
    materials: &Vec<Option<Material>>,
    mode: Option<BillboardMode>,
    kind: Option<Kind>,
    obj: Option<Billboard>,
) -> Billboard {
    let mode = mode.unwrap_or("full".to_owned());
    let kind = kind.unwrap_or((BILLBOARD_KIND).to_owned());
    let mut billboard = create_scene_node(
        Some(((kind).clone()).clone()),
        Some(((obj).clone().unwrap()).clone()),
    );
    billboard.geometry = (*geometry).clone();
    billboard.materials = (*materials).clone();
    billboard.mode = (mode).clone();
    return billboard;
}

// Source: upstream/packages/scene/src/billboard.ts:46 (sha256:ad62048d6887b1083246a862661fe8bc0eb2a769c4851f08185965764da8607d)
pub fn enable_billboard_signals(source: &Billboard) -> NodeSignals {
    return enable_node_signals(source);
}

// Source: upstream/packages/scene/src/billboard.ts:50 (sha256:b5ec90820db4c13ab3fe94f7cc74a6606d363091821d191b85f5501481255efc)
pub fn get_billboard_runtime(source: &Billboard) -> BillboardRuntime {
    return get_scene_node_runtime(source);
}

// Source: upstream/packages/scene/src/billboard.ts:54 (sha256:7f4344bb9e2aee03de90f232d573ff202b13758de25c0363809cd6c3a07277c3)
pub fn get_billboard_signals(source: &Billboard) -> Option<NodeSignals> {
    return get_node_signals(source);
}

// Source: upstream/packages/scene/src/billboard.ts:61 (sha256:2df21ee89214648dc557fbf93bcf0025abe505634be62a03350b14933e694dd3)
pub fn is_billboard(source: &SceneNode) -> bool {
    let candidate = source;
    return (((candidate.geometry).clone()).is_some()) && (((candidate.mode).clone()).is_some());
}
