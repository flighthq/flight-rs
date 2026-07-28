// @generated from upstream/packages/node/src/traversal.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_node_parent, get_node_runtime};
use flighthq_types::{Node, NodeOf};

// Source: upstream/packages/node/src/traversal.ts:10 (sha256:e1cbd11b5c9b93af3c4bf94d0c651a7616faf538a9445cdceec4f8ae7b3e1ee6)
pub fn find_node<Traits: Clone>(
    source: &Node,
    predicate: &mut impl FnMut(Node) -> bool,
) -> Option<NodeOf<Traits>> {
    let children = (get_node_runtime(source).children).clone();
    if (children).is_none() {
        return None;
    }
    {
        let mut i = 0.0_f64;
        while (i < (children.as_ref().unwrap().len() as f64)) {
            let child = children.as_ref().unwrap()[i as usize].clone();
            if predicate((child).clone()) {
                return Some(child);
            }
            let found = find_node(&child, predicate);
            if (found).is_some() {
                return Some((found.as_ref().unwrap()).clone());
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return None;
}

// Source: upstream/packages/node/src/traversal.ts:30 (sha256:d04f93f42cbbafe063d8d2413d5832eb1938b4322c01613a586cba34a6fbebf7)
pub fn find_node_by_name<Traits: Clone>(source: &Node, name: String) -> Option<NodeOf<Traits>> {
    return find_node(source, &mut |node: Node| -> bool {
        ((node.name).clone()) == Some((name).clone())
    });
}

// Source: upstream/packages/node/src/traversal.ts:41 (sha256:8d4c089cfee6bf6920ad4f9cdbe797ce410b227fa306db20ff2dd6fba434c3a1)
pub fn for_each_node_ancestor(source: &Node, callback: &mut impl FnMut(Node) -> bool) -> () {
    let mut current = get_node_parent(&(*source).clone());
    while (current).is_some() {
        if (!callback((current).clone().unwrap())) {
            return;
        }
        current = get_node_parent(&current);
    }
}

// Source: upstream/packages/node/src/traversal.ts:57 (sha256:2164fd5be272eaeb0139d03d13c499411545e8b437b2310f42806e2313e909be)
pub fn for_each_node_descendant(source: &Node, callback: &mut impl FnMut(Node) -> ()) -> () {
    let children = (get_node_runtime(source).children).clone();
    if (children).is_none() {
        return;
    }
    {
        let mut i = 0.0_f64;
        while (i < (children.as_ref().unwrap().len() as f64)) {
            callback(children.as_ref().unwrap()[i as usize].clone());
            for_each_node_descendant(&children.as_ref().unwrap()[i as usize].clone(), callback);
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/node/src/traversal.ts:73 (sha256:fd4ff8b0b93a8d15c6a005cf1f32afaaf2dafb684b414dfae3d6523627ac3bad)
pub fn get_node_children<Traits: Clone>(source: &Node) -> Vec<NodeOf<Traits>> {
    let children = (get_node_runtime(source).children).clone();
    if (children).is_none() {
        return (*_EMPTY_CHILDREN).clone();
    }
    return (children).as_ref().unwrap().clone();
}

// Source: upstream/packages/node/src/traversal.ts:85 (sha256:3d269f9e201cb75d76fa45fb6990cf0a084e4feed3b219b99a790bd71ab37f6f)
pub fn get_node_depth(source: &Node) -> f64 {
    let mut depth = 0.0_f64;
    let mut current = get_node_parent(&(*source).clone());
    while (current).is_some() {
        {
            depth += 1.0;
            depth
        };
        current = get_node_parent(&current);
    }
    return depth;
}

// Source: upstream/packages/node/src/traversal.ts:99 (sha256:baa0ccb47d5b09ad9608be1b276e7504b4e3a2b52833647e4b3bb6ef3f82dde8)
pub fn get_node_next_sibling<Traits: Clone>(source: &Node) -> Option<NodeOf<Traits>> {
    let parent = get_node_parent(&(*source).clone());
    if (parent).is_none() {
        return None;
    }
    let siblings = (get_node_runtime(&parent).children).clone();
    if (siblings).is_none() {
        return None;
    }
    let idx = {
        let __flight_value = (*source).clone();
        (siblings)
            .as_ref()
            .unwrap()
            .iter()
            .position(|item| item == &__flight_value)
            .map_or(-1.0_f64, |index| index as f64)
    };
    if (idx == (-1.0_f64)) || (idx == ((siblings.as_ref().unwrap().len() as f64) - 1.0_f64)) {
        return None;
    }
    return Some(siblings.as_ref().unwrap()[(idx + 1.0_f64) as usize].clone());
}

// Source: upstream/packages/node/src/traversal.ts:115 (sha256:ea66e3f05f4facea3b702ea8df77f5082042c12690a4ae9bacd88f2f0c9043b4)
pub fn get_node_previous_sibling<Traits: Clone>(source: &Node) -> Option<NodeOf<Traits>> {
    let parent = get_node_parent(&(*source).clone());
    if (parent).is_none() {
        return None;
    }
    let siblings = (get_node_runtime(&parent).children).clone();
    if (siblings).is_none() {
        return None;
    }
    let idx = {
        let __flight_value = (*source).clone();
        (siblings)
            .as_ref()
            .unwrap()
            .iter()
            .position(|item| item == &__flight_value)
            .map_or(-1.0_f64, |index| index as f64)
    };
    if (idx <= 0.0_f64) {
        return None;
    }
    return Some(siblings.as_ref().unwrap()[(idx - 1.0_f64) as usize].clone());
}

// Source: upstream/packages/node/src/traversal.ts:133 (sha256:2146fc3e716d6abe6681bf9b9a6d5229630b9f945cbb72c8b6b94ee7094ec9cf)
pub fn walk_node_descendants(source: &Node, visit: &mut impl FnMut(Node) -> bool) -> bool {
    let children = (get_node_runtime(source).children).clone();
    if (children).is_none() {
        return true;
    }
    {
        let mut i = 0.0_f64;
        while (i < (children.as_ref().unwrap().len() as f64)) {
            let child = children.as_ref().unwrap()[i as usize].clone();
            if (!visit((child).clone())) {
                return false;
            }
            if (!walk_node_descendants(&child, visit)) {
                return false;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/node/src/traversal.ts:147 (sha256:2126b7bfe2bae88a88f993eb85be79992676920085797f392f790c1088fa0277)
static _EMPTY_CHILDREN: std::sync::LazyLock<Vec<crate::OpaqueHostValue>> =
    std::sync::LazyLock::new(|| vec![]);
