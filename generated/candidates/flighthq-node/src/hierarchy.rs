// @generated from upstream/packages/node/src/hierarchy.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ensure_node_world_matrix, get_node_runtime, get_node_world_matrix,
    invalidate_node_local_transform, invalidate_node_parent_reference,
};
use flighthq_geometry::{
    acquire_matrix, copy_matrix, inverse_matrix, multiply_matrix, release_matrix,
};
use flighthq_signals::emit_signal;
use flighthq_types::{Node, NodeOf, Transform2DNode};

// Source: upstream/packages/node/src/hierarchy.ts:14 (sha256:cb03ca6747bcfee242b049772b79d8e3c606ba88698154c57c75401604742cca)
pub fn add_node_child<Traits: Clone>(target: &Node, child: &Node) -> NodeOf<Traits> {
    return add_node_child_at(target, child, get_node_child_count(target));
}

// Source: upstream/packages/node/src/hierarchy.ts:24 (sha256:aee6deac3644abc695dbc4a0f458dc73386cfecd2614e0c65a4c6bebf228d740)
pub fn add_node_child_at<Traits: Clone>(target: &Node, child: &Node, index: f64) -> NodeOf<Traits> {
    let mut target_runtime = get_node_runtime(target);
    let mut children = (target_runtime.children).clone();
    if false {
        panic!("{}", "generated Flight function threw");
    } else {
        if (child == target) {
            panic!("{}", "generated Flight function threw");
        } else {
            if (((index < 0.0_f64)
                || ((children).is_some() && (index > (children.as_ref().unwrap().len() as f64))))
                || ((children).is_none() && (index > 0.0_f64)))
            {
                throw_out_of_bounds_error();
            }
        }
    }
    if (!((target_runtime.can_add_child).clone()).lock().unwrap()(
        (*target).clone(),
        (*child).clone(),
    )) {
        panic!("{}", "generated Flight function threw");
    }
    if (children).is_none() {
        children = Some({
            target_runtime.children = Some(vec![]);
            target_runtime.children
        });
    }
    let mut child_runtime = get_node_runtime(child);
    let parent = ((child_runtime.parent).clone()).unwrap();
    if (parent == target) {
        let i = {
            let __flight_value = (*child).clone();
            (children)
                .as_ref()
                .unwrap()
                .iter()
                .position(|item| item == &__flight_value)
                .map_or(-1.0_f64, |index| index as f64)
        };
        if (i != (-1.0_f64)) {
            if (i == index) {
                return child;
            }
            children
                .as_mut()
                .unwrap()
                .splice((i) as usize..((i) + (1.0_f64)) as usize, vec![]);
        }
    } else {
        if (parent).is_some() {
            remove_node_child(&parent, child);
        }
    }
    children.as_mut().unwrap().splice(
        (index) as usize..((index) + (0.0_f64)) as usize,
        vec![(*child).clone()],
    );
    let target_signals = (target_runtime.node_signals).clone();
    if (target_signals).is_some() {
        emit_signal(
            (target_signals.as_ref().unwrap().on_children_changed).clone(),
            (),
        );
    }
    if (parent != target) {
        child_runtime.parent = Some((*target).clone());
        if (target_signals).is_some() {
            emit_signal(
                (target_signals.as_ref().unwrap().on_child_added).clone(),
                (child,),
            );
        }
        let child_signals = (child_runtime.node_signals).clone();
        if (child_signals).is_some() {
            emit_signal(
                (child_signals.as_ref().unwrap().on_parent_changed).clone(),
                (),
            );
        }
        invalidate_node_parent_reference(child);
    }
    return child;
}

// Source: upstream/packages/node/src/hierarchy.ts:82 (sha256:c6d3c0c211f12eb98cb3da0a1ada90eabcb0a8591e30a7d247c7c514d78950f3)
pub fn add_node_children(target: &Node, children: Vec<Node>) -> () {
    {
        let mut i = 0.0_f64;
        while (i < (children.len() as f64)) {
            add_node_child(target, &children[i as usize]);
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/node/src/hierarchy.ts:92 (sha256:8fec8cab407e76878884eed2e26593b4885c530d8a566a9cb5ca8a78d5ec7b92)
pub fn contains_node_child(source: &Node, child: &mut Node) -> bool {
    let mut current: Option<Node> = child;
    while (!((current) == Some((*source).clone())) && (current).is_some()) {
        current = get_node_parent(current.as_ref().unwrap());
    }
    return (current) == Some((*source).clone());
}

// Source: upstream/packages/node/src/hierarchy.ts:107 (sha256:21332fd5e77515d489e2b53d126c8bd7e772933284986f2e887155d4c5110a78)
pub fn for_each_node_child(
    source: &Node,
    callback: &mut impl FnMut(Node, f64) -> crate::FlightUnion2<bool, ()>,
) -> () {
    let children = (get_node_runtime(source).children).clone();
    if (children).is_none() {
        return;
    }
    {
        let mut i = 0.0_f64;
        while (i < (children.as_ref().unwrap().len() as f64)) {
            if (callback(children.as_ref().unwrap()[i as usize].clone(), i) == false) {
                return;
            }
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/node/src/hierarchy.ts:122 (sha256:44a464b18a8663cef0a76748408b3c2e35becb5e6633f32d00388a8c58cee1ea)
pub fn get_node_ancestors<Traits: Clone>(source: &Node) -> Vec<NodeOf<Traits>> {
    let mut result: Vec<NodeOf<Traits>> = vec![];
    let mut current = get_node_parent(&(*source).clone());
    while (current).is_some() {
        result.push(((current).clone().unwrap()).clone());
        current = get_node_parent(&current);
    }
    return (result).clone();
}

// Source: upstream/packages/node/src/hierarchy.ts:136 (sha256:80d8d16e177bec8a86266e3a1d1d203f60a1122de5ae0987c305a829048482d0)
pub fn get_node_child_at<Traits: Clone>(source: &Node, index: f64) -> Option<NodeOf<Traits>> {
    let children = (get_node_runtime(source).children).clone();
    if (((children).is_some() && (index >= 0.0_f64))
        && (index < (children.as_ref().unwrap().len() as f64)))
    {
        return Some(children.as_ref().unwrap()[index as usize].clone());
    }
    return None;
}

// Source: upstream/packages/node/src/hierarchy.ts:152 (sha256:51e332b26f6cd521c84f11bab8ee555c6c5b3985d81a6d8aa4e2bc072fd350d8)
pub fn get_node_child_by_name<Traits: Clone>(
    source: &Node,
    name: String,
) -> Option<NodeOf<Traits>> {
    let children = (get_node_runtime(source).children).clone();
    if (children).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < (children.as_ref().unwrap().len() as f64)) {
                if ((children.as_ref().unwrap()[i as usize].name).clone()) == Some((name).clone()) {
                    return Some(children.as_ref().unwrap()[i as usize].clone());
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    return None;
}

// Source: upstream/packages/node/src/hierarchy.ts:165 (sha256:5276beeedaba34cabee421c20839465df9cb3f0684e779bb5421f804cd387ebd)
pub fn get_node_child_count(source: &Node) -> f64 {
    let children = (get_node_runtime(source).children).clone();
    return if (children).is_some() {
        (children.as_ref().unwrap().len() as f64)
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/node/src/hierarchy.ts:173 (sha256:eedc984ff823fa04942e606852ad5749debd29f5f7a5bc76886b7a917882c814)
pub fn get_node_child_index(source: &Node, child: &Node) -> f64 {
    let children = (get_node_runtime(source).children).clone();
    if (children).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < (children.as_ref().unwrap().len() as f64)) {
                if (children.as_ref().unwrap()[i as usize].clone() == child) {
                    return i;
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    return (-1.0_f64);
}

// Source: upstream/packages/node/src/hierarchy.ts:190 (sha256:e869f4e99ff04be76b4f368be8040f3cbd6ee722efb96f2a931ffeba45f5b1b9)
pub fn get_node_common_ancestor<Traits: Clone>(a: &Node, b: &mut Node) -> Option<NodeOf<Traits>> {
    let mut a_ancestors = Vec::new();
    {
        let __flight_value = (*a).clone();
        if !a_ancestors.contains(&__flight_value) {
            a_ancestors.push(__flight_value);
        }
    };
    let mut cur = get_node_parent(&(*a).clone());
    while (cur).is_some() {
        {
            let __flight_value = (cur).clone().unwrap();
            if !a_ancestors.contains(&__flight_value) {
                a_ancestors.push(__flight_value);
            }
        };
        cur = get_node_parent(&cur);
    }
    let mut b_cur: Option<Node> = (*b).clone();
    while (b_cur).is_some() {
        if a_ancestors
            .iter()
            .any(|item| item == &(b_cur).clone().unwrap())
        {
            return Some((b_cur).clone().unwrap());
        }
        b_cur = get_node_parent(b_cur.as_ref().unwrap());
    }
    return None;
}

// Source: upstream/packages/node/src/hierarchy.ts:210 (sha256:92f4436fc4bac4c61918b0e125ba3fdae48daca6c59d08ddd139e16c361e017e)
pub fn get_node_parent<Traits: Clone>(source: &Node) -> Option<NodeOf<Traits>> {
    return Some(((get_node_runtime(source).parent).clone()).unwrap());
}

// Source: upstream/packages/node/src/hierarchy.ts:218 (sha256:55991388fe1d241abcf88b2fcebf696d1baf41e046ef7ada8aecd177c89ff5db)
pub fn get_node_root<Traits: Clone>(source: &mut Node) -> NodeOf<Traits> {
    let mut current: NodeOf<Traits> = source;
    let mut parent = get_node_parent(&current);
    while (parent).is_some() {
        current = (parent).clone().unwrap();
        parent = get_node_parent(&current);
    }
    return (current).clone();
}

// Source: upstream/packages/node/src/hierarchy.ts:232 (sha256:490ca838aa4952803080336d9668f96069626cefd8c0def45a89a588b3c2ee99)
pub fn is_node_ancestor_of(ancestor: &Node, descendant: &mut Node) -> bool {
    let mut current: Option<Node> = (*descendant).clone();
    while (current).is_some() {
        if (current) == Some((*ancestor).clone()) {
            return true;
        }
        current = get_node_parent(current.as_ref().unwrap());
    }
    return false;
}

// Source: upstream/packages/node/src/hierarchy.ts:252 (sha256:4eb47901b8d63670c59c930467b0b360f42d9172f31b88c1d1357c945ba7b2bb)
pub fn remove_node_child<Traits: Clone>(target: &Node, child: &Node) -> NodeOf<Traits> {
    if false {
        return child.clone();
    }
    let mut target_runtime = get_node_runtime(target);
    let mut child_runtime = get_node_runtime(child);
    let mut children = (target_runtime.children).clone();
    if ((children).is_some() && ((child_runtime.parent).clone()) == Some((*target).clone())) {
        child_runtime.parent = None;
        let child_signals = (child_runtime.node_signals).clone();
        if (child_signals).is_some() {
            emit_signal(
                (child_signals.as_ref().unwrap().on_parent_changed).clone(),
                (),
            );
        }
        invalidate_node_parent_reference(child);
        let i = {
            let __flight_value = (*child).clone();
            (children)
                .as_ref()
                .unwrap()
                .iter()
                .position(|item| item == &__flight_value)
                .map_or(-1.0_f64, |index| index as f64)
        };
        if (i != (-1.0_f64)) {
            children
                .as_mut()
                .unwrap()
                .splice((i) as usize..((i) + (1.0_f64)) as usize, vec![]);
        }
        let target_signals = (target_runtime.node_signals).clone();
        if (target_signals).is_some() {
            emit_signal(
                (target_signals.as_ref().unwrap().on_child_removed).clone(),
                (child,),
            );
            emit_signal(
                (target_signals.as_ref().unwrap().on_children_changed).clone(),
                (),
            );
        }
    }
    return child;
}

// Source: upstream/packages/node/src/hierarchy.ts:283 (sha256:a6cb1995e3abc76605ce1ef8cc8581a5f1f10c6768a57c8bcd8e7c9db10e48c5)
pub fn remove_node_child_at<Traits: Clone>(target: &Node, index: f64) -> Option<NodeOf<Traits>> {
    let children = (get_node_runtime(target).children).clone();
    if (((children).is_some() && (index >= 0.0_f64))
        && (index < (children.as_ref().unwrap().len() as f64)))
    {
        return Some(remove_node_child(
            target,
            &children.as_ref().unwrap()[index as usize].clone(),
        ));
    }
    return None;
}

// Source: upstream/packages/node/src/hierarchy.ts:296 (sha256:1c83338385b159da1f137f165cdf3c63beb4e0a0bf85bc06bd92215d15df0672)
pub fn remove_node_children(
    target: &Node,
    begin_index: Option<f64>,
    mut end_index: Option<f64>,
) -> () {
    let begin_index = begin_index.unwrap_or(0.0_f64);
    let children = (get_node_runtime(target).children).clone();
    if (children).is_none() {
        return;
    }
    if (begin_index > ((children.as_ref().unwrap().len() as f64) - 1.0_f64)) {
        return;
    }
    if (end_index).is_none() {
        end_index = Some(((children.as_ref().unwrap().len() as f64) - 1.0_f64));
    }
    if (((end_index < begin_index) || (begin_index < 0.0_f64))
        || (end_index > (children.as_ref().unwrap().len() as f64)))
    {
        throw_out_of_bounds_error();
    }
    let mut num_removals = (end_index - begin_index);
    while (num_removals >= 0.0_f64) {
        remove_node_child_at(target, begin_index);
        {
            num_removals -= 1.0;
            num_removals
        };
    }
}

// Source: upstream/packages/node/src/hierarchy.ts:329 (sha256:55a3386329fcdf02ac3b8019eb1630cf0284329479246bf164ec6acc066a2c17)
pub fn reparent_node<Traits: Clone>(
    child: &mut Transform2DNode,
    new_parent: &Transform2DNode,
) -> NodeOf<Traits> {
    ensure_node_world_matrix(child);
    let mut old_world = acquire_matrix();
    let mut local_m = acquire_matrix();
    {
        copy_matrix(&mut old_world, &get_node_world_matrix(child));
        add_node_child(new_parent, child);
        inverse_matrix(&mut local_m, &get_node_world_matrix(new_parent));
        {
            let __flight_argument_1 = (local_m).clone();
            multiply_matrix(&mut local_m, &__flight_argument_1, &old_world)
        };
        let a = local_m.a;
        let b = local_m.b;
        let c = local_m.c;
        let d = local_m.d;
        child.scale_x = ((a * a) + (b * b)).sqrt();
        child.scale_y = ((c * c) + (d * d)).sqrt();
        if (((a * d) - (b * c)) < 0.0_f64) {
            child.scale_y = (-child.scale_y);
        }
        let skew_y_rad = (child.skew_y * DEG_TO_RAD);
        child.rotation = (((b).atan2(a) - skew_y_rad) * RAD_TO_DEG);
        child.x = (local_m.tx + ((a * child.pivot_x) + (c * child.pivot_y)));
        child.y = (local_m.ty + ((b * child.pivot_x) + (d * child.pivot_y)));
        invalidate_node_local_transform(child);
    }
    {
        release_matrix(&old_world);
        release_matrix(&local_m);
    }
    return child;
}

// Source: upstream/packages/node/src/hierarchy.ts:373 (sha256:28d73aeb3d841821044c411c0500aee0a0581d87b23521d6fc6cac96ff598f4d)
pub fn replace_node_child(target: &Node, old_child: &Node, new_child: &Node) -> () {
    let index = get_node_child_index(target, old_child);
    if (index == (-1.0_f64)) {
        return;
    }
    remove_node_child(target, old_child);
    add_node_child_at(target, new_child, index);
}

// Source: upstream/packages/node/src/hierarchy.ts:388 (sha256:ce6b7746bf55fb3fe5ef0a9765c402c72e1dcbc9ea780c0f9fe7ef8764c642f5)
pub fn set_node_child_index(target: &Node, child: &Node, index: f64) -> () {
    let mut target_runtime = get_node_runtime(target);
    let mut children = (target_runtime.children).clone();
    if (children).is_none() {
        return;
    }
    if (((index >= 0.0_f64) && (index <= (children.as_ref().unwrap().len() as f64)))
        && (get_node_parent(child) == target))
    {
        let i = {
            let __flight_value = (*child).clone();
            (children)
                .as_ref()
                .unwrap()
                .iter()
                .position(|item| item == &__flight_value)
                .map_or(-1.0_f64, |index| index as f64)
        };
        if ((i != (-1.0_f64)) && (i != index)) {
            children
                .as_mut()
                .unwrap()
                .splice((i) as usize..((i) + (1.0_f64)) as usize, vec![]);
            children.as_mut().unwrap().splice(
                (index) as usize..((index) + (0.0_f64)) as usize,
                vec![(*child).clone()],
            );
            let target_signals = (target_runtime.node_signals).clone();
            if (target_signals).is_some() {
                emit_signal(
                    (target_signals.as_ref().unwrap().on_children_order_changed).clone(),
                    (),
                );
            }
        }
    }
}

// Source: upstream/packages/node/src/hierarchy.ts:417 (sha256:b8623f2e5472b1d21c3fb16ea5c54e2c547786d19810031b132d363d2bbc64d5)
pub fn swap_node_children(target: &Node, child1: &Node, child2: &Node) -> () {
    let mut target_runtime = get_node_runtime(target);
    let mut children = (target_runtime.children).clone();
    if (((children).is_some() && (get_node_parent(child1) == target))
        && (get_node_parent(child2) == target))
    {
        let index1 = {
            let __flight_value = (*child1).clone();
            (children)
                .as_ref()
                .unwrap()
                .iter()
                .position(|item| item == &__flight_value)
                .map_or(-1.0_f64, |index| index as f64)
        };
        let index2 = {
            let __flight_value = (*child2).clone();
            (children)
                .as_ref()
                .unwrap()
                .iter()
                .position(|item| item == &__flight_value)
                .map_or(-1.0_f64, |index| index as f64)
        };
        children.as_mut().unwrap()[index1 as usize] = (*child2).clone();
        children.as_mut().unwrap()[index2 as usize] = (*child1).clone();
        let target_signals = (get_node_runtime(target).node_signals).clone();
        if (target_signals).is_some() {
            emit_signal(
                (target_signals.as_ref().unwrap().on_children_order_changed).clone(),
                (),
            );
        }
    }
}

// Source: upstream/packages/node/src/hierarchy.ts:439 (sha256:1b8fc9381de77754a5b94e860737b55caaac26adf80a0b68852be7e3cb65435d)
pub fn swap_node_children_at(target: &Node, index1: f64, index2: f64) -> () {
    let mut target_runtime = get_node_runtime(target);
    let mut children = (target_runtime.children).clone();
    if ((children).is_none() || (index1 == index2)) {
        return;
    }
    let len = (children.as_ref().unwrap().len() as f64);
    if ((((index1 < 0.0_f64) || (index2 < 0.0_f64)) || (index1 >= len)) || (index2 >= len)) {
        throw_out_of_bounds_error();
    }
    let swap = children.as_mut().unwrap()[index1 as usize].clone();
    children.as_mut().unwrap()[index1 as usize] =
        children.as_mut().unwrap()[index2 as usize].clone();
    children.as_mut().unwrap()[index2 as usize] = (swap).clone();
    let target_signals = (target_runtime.node_signals).clone();
    if (target_signals).is_some() {
        emit_signal(
            (target_signals.as_ref().unwrap().on_children_order_changed).clone(),
            (),
        );
    }
}

// Source: upstream/packages/node/src/hierarchy.ts:454 (sha256:4520623caa7a562fea40c469edd0e147665e1cfcd3b4855309bac642c2249754)
fn throw_out_of_bounds_error() -> () {
    panic!("{}", "generated Flight function threw");
}

// Source: upstream/packages/node/src/hierarchy.ts:458 (sha256:20f0ca1e133840394a2a40394cc19c0be291922e87a2b68980feb609e87508f4)
const DEG_TO_RAD: f64 = 0.017453292519943295_f64;

// Source: upstream/packages/node/src/hierarchy.ts:459 (sha256:718676123c3e6c774ce683da988c83efb4917c00586357144ece34b691299ca8)
const RAD_TO_DEG: f64 = 57.29577951308232_f64;
