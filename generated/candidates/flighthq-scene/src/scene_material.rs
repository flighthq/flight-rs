// @generated from upstream/packages/scene/src/sceneMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_node::find_node;
use flighthq_types::{Material, Node, SceneNode};

// Source: upstream/packages/scene/src/sceneMaterial.ts:11 (sha256:dc30425b669d69d9e52afea629f1b3304a07e5e88c1b2b410be39451bff2c322)
pub fn find_scene_material_by_name(root: &SceneNode, name: String) -> Option<Material> {
    let root_match = get_named_node_material(root, (name).clone());
    if (root_match).is_some() {
        return Some((root_match.as_ref().unwrap()).clone());
    }
    let found: std::sync::Arc<std::sync::Mutex<Option<Material>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    find_node(root, &mut |node: Node| -> bool {
        let match_ = get_named_node_material(&node, (name).clone());
        if (match_).is_none() {
            return false;
        }
        (*found.lock().unwrap()) = (match_).clone();
        return true;
    });
    return (*found.lock().unwrap()).clone();
}

// Source: upstream/packages/scene/src/sceneMaterial.ts:26 (sha256:551ccf8e51b997ca62bb0e397eb0c657199dfc5ec63d835c9c931d5fea5eec5d)
fn get_named_node_material(node: &SceneNode, name: String) -> Option<Material> {
    if (node.materials).is_none() {
        return None;
    }
    {
        let mut i = 0.0_f64;
        while (i < (node.materials.len() as f64)) {
            let material = node.materials[i as usize].clone();
            if ((material).is_some())
                && (((material.as_ref().unwrap().name).clone()) == Some((name).clone()))
            {
                return (material).clone();
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return None;
}
