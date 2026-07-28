// @generated from upstream/packages/scene/src/sceneNode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_node::{
    create_node, create_node_runtime, enable_node_signals, get_node_runtime, get_node_signals,
    init_appearance_trait, init_transform3_d_runtime_trait, init_transform3_d_trait,
};
use flighthq_types::{Kind, NodeSignals, SCENE_NODE_TRAITS_KEY as scene_node_traits_key_constant};
pub use flighthq_types::{SCENE_NODE_KIND, SceneNode, SceneNodeRuntime, SceneNodeTraits};

// Source: upstream/packages/scene/src/sceneNode.ts:17 (sha256:63226bd7e603e1382a282e143d894ddef46a663b327e255b2bce15010dbbd5a6)
pub fn create_scene_node(kind: Option<Kind>, obj: Option<SceneNode>) -> SceneNode {
    let kind = kind.unwrap_or(SCENE_NODE_KIND);
    let mut node = create_node(
        (kind).clone(),
        Some(((obj).clone().unwrap()).clone()),
        Some(undefined),
        Some(create_scene_node_runtime),
    );
    init_appearance_trait(&mut node, Some(((obj).clone().unwrap()).clone()));
    init_transform3_d_trait(&mut node, None);
    return node;
}

// Source: upstream/packages/scene/src/sceneNode.ts:27 (sha256:05626d2482d96e38657ad413f0f100b1d8aa907d6ec95e29d08bb82df8971097)
pub fn create_scene_node_runtime() -> SceneNodeRuntime {
    let mut out = create_node_runtime(None);
    out.traits = Some(scene_node_traits_key_constant);
    out.world_alpha = None;
    out.world_alpha_using_appearance_id = (-1.0_f64);
    out.world_alpha_using_parent_appearance_id = (-1.0_f64);
    out.world_appearance_id = 0.0_f64;
    init_transform3_d_runtime_trait(&mut out);
    return (out).clone();
}

// Source: upstream/packages/scene/src/sceneNode.ts:38 (sha256:79451d392d0d20a823df24637068db86a986ceca476c01d14fded4205cda79a1)
pub fn enable_scene_node_signals(source: &SceneNode) -> NodeSignals {
    return enable_node_signals(source);
}

// Source: upstream/packages/scene/src/sceneNode.ts:42 (sha256:dba286523d03c3be126b782754ac4af20e855dc47ba27121f6aa81a335a2299c)
pub fn get_scene_node_runtime(source: &SceneNode) -> SceneNodeRuntime {
    return get_node_runtime(source);
}

// Source: upstream/packages/scene/src/sceneNode.ts:46 (sha256:34a017800aae991e1ed4494dedde801a41f52d1e588f074c87cac81b89daca82)
pub fn get_scene_node_signals(source: &SceneNode) -> Option<NodeSignals> {
    return get_node_signals(source);
}
