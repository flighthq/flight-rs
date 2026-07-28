// @generated from upstream/packages/node/src/revision.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_node_runtime;
use flighthq_types::{Node, NodeRuntime};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

// Source: upstream/packages/node/src/revision.ts:11 (sha256:c7f9f4d4ce86cf8e4acf1ccd9876925b18e7d618c826f5054fd84f26036c0a38)
pub fn compute_node_world_transform_revision<Traits: Clone>(
    runtime: &mut NodeRuntime<Traits>,
    parent_runtime: Option<NodeRuntime<Traits>>,
) -> () {
    runtime.world_transform_using_local_transform_id = runtime.local_transform_id;
    runtime.world_transform_using_parent_transform_id = if (parent_runtime).is_some() {
        parent_runtime.as_ref().unwrap().world_transform_id
    } else {
        0.0_f64
    };
    (*_WORLD_TRANSFORM_REVISION_COUNTER.lock().unwrap()) = (__flight_js_to_u32(
        ((*_WORLD_TRANSFORM_REVISION_COUNTER.lock().unwrap()).clone() + 1.0_f64),
    ) >> (__flight_js_to_u32(0.0_f64) & 31))
        as f64;
    if ((*_WORLD_TRANSFORM_REVISION_COUNTER.lock().unwrap()).clone() == 0.0_f64) {
        (*_WORLD_TRANSFORM_REVISION_COUNTER.lock().unwrap()) = 1.0_f64;
    }
    runtime.world_transform_id = (*_WORLD_TRANSFORM_REVISION_COUNTER.lock().unwrap()).clone();
}

// Source: upstream/packages/node/src/revision.ts:23 (sha256:8bc7305048b45bca2046d1d7a73e51ccfa6f98f07c492c9a756967d6e1bd62fa)
pub fn get_node_appearance_revision(source: &Node) -> f64 {
    return get_node_runtime(source).appearance_id;
}

// Source: upstream/packages/node/src/revision.ts:27 (sha256:3c2e9461d6b8e9a19bfa007c731e52f5dcfcbb1b8bb890f88571baa63082f677)
pub fn get_node_local_bounds_revision(source: &Node) -> f64 {
    return get_node_runtime(source).local_bounds_id;
}

// Source: upstream/packages/node/src/revision.ts:31 (sha256:390a82233b7b8c606ac104848f656d726be87a890d194d4344abd6d3e070426a)
pub fn get_node_local_content_revision(source: &Node) -> f64 {
    return get_node_runtime(source).local_content_id;
}

// Source: upstream/packages/node/src/revision.ts:35 (sha256:77cc73e0b018d4c99f5ac1d0aca18aae320ae203424885de44e0ca29368d170e)
pub fn get_node_local_transform_revision(source: &Node) -> f64 {
    return get_node_runtime(source).local_transform_id;
}

// Source: upstream/packages/node/src/revision.ts:39 (sha256:9fef63d4d0b3956e7c87d66dbc73107d493cdea450561a46e9ceb3ecab7860c3)
pub fn get_node_world_transform_revision(source: &Node) -> f64 {
    return get_node_runtime(source).world_transform_id;
}

// Source: upstream/packages/node/src/revision.ts:50 (sha256:330a720595a2a8fd99f57e9c36f7d59fe586a076a306042811800b361063e8ab)
pub fn invalidate_content(target: &Node) -> () {
    invalidate_node_local_content(target);
    invalidate_node_local_bounds(target);
}

// Source: upstream/packages/node/src/revision.ts:55 (sha256:ee2458f32c7f2451fe83afdc7037988799b08890a9e316285be9fd355a3257c4)
pub fn invalidate_node(target: &Node) -> () {
    invalidate_node_appearance(target);
    invalidate_node_local_bounds(target);
    invalidate_node_local_content(target);
    invalidate_node_local_transform(target);
    invalidate_node_parent_reference(target);
    invalidate_node_world_bounds(target);
}

// Source: upstream/packages/node/src/revision.ts:67 (sha256:96d82ab97efed603c1ebcf142f64e71eaddf3b65551fdf109fdabf1514f0003a)
pub fn invalidate_node_appearance(target: &Node) -> () {
    let mut runtime = get_node_runtime(target);
    runtime.appearance_id = (__flight_js_to_u32((runtime.appearance_id + 1.0_f64))
        >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/node/src/revision.ts:75 (sha256:ea40aac1e6167e493c9e3a7a547c637fa8bb7184e4660eb65c7a78fec94345b6)
pub fn invalidate_node_local_bounds(target: &Node) -> () {
    let mut runtime = get_node_runtime(target);
    runtime.local_bounds_id = (__flight_js_to_u32((runtime.local_bounds_id + 1.0_f64))
        >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/node/src/revision.ts:85 (sha256:ed95371eeb379ab1c8b4ad5de46de72fa7837065b236a6135403996962b7ae85)
pub fn invalidate_node_local_content(target: &Node) -> () {
    let mut runtime = get_node_runtime(target);
    runtime.local_content_id = (__flight_js_to_u32((runtime.local_content_id + 1.0_f64))
        >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/node/src/revision.ts:93 (sha256:9bd628a94bf067f463d79c50f9ab7359e45827386ff876976eb73f7466ff1091)
pub fn invalidate_node_local_transform(target: &Node) -> () {
    let mut runtime = get_node_runtime(target);
    runtime.local_transform_id = (__flight_js_to_u32((runtime.local_transform_id + 1.0_f64))
        >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/node/src/revision.ts:101 (sha256:f002d50ffeff6b245f9dae630c6c2e94095f69650eb1efdb211a1b4ec14c76af)
pub fn invalidate_node_parent_reference(target: &Node) -> () {
    let mut runtime = get_node_runtime(target);
    runtime.world_transform_using_parent_transform_id = (-1.0_f64);
}

// Source: upstream/packages/node/src/revision.ts:110 (sha256:0540c59fd76cda1cabde5e9746ff95b8ad826875fc6a6c8035204978c6dbd167)
pub fn invalidate_node_render(target: &Node) -> () {
    invalidate_node_appearance(target);
    invalidate_node_local_transform(target);
}

// Source: upstream/packages/node/src/revision.ts:118 (sha256:03cbc94eb38bc2778b17868a21cd8671fb8124149c43420cdc8d8077f90805f3)
pub fn invalidate_node_world_bounds(target: &Node) -> () {
    let mut runtime = get_node_runtime(target);
    runtime.world_bounds_using_world_transform_id = (-1.0_f64);
    runtime.world_bounds_using_local_bounds_id = (-1.0_f64);
}

// Source: upstream/packages/node/src/revision.ts:126 (sha256:e9fd64717920ccf433503b82217d47cc5600704d391934586763993e969f22a4)
static _WORLD_TRANSFORM_REVISION_COUNTER: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));
