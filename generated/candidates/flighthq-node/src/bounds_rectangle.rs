// @generated from upstream/packages/node/src/boundsRectangle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ensure_node_world_matrix, get_node_child_count, get_node_local_matrix, get_node_parent,
    get_node_runtime, get_node_world_matrix, invalidate_node_local_transform,
};
use flighthq_entity::get_entity_runtime;
use flighthq_geometry::{
    acquire_matrix, copy_rectangle, create_rectangle, inverse_matrix, matrix_transform_rectangle,
    merge_rectangle, release_matrix,
};
use flighthq_types::{BoundsNode, Rectangle, RectangleLike, Spatial2DNode};

// Source: upstream/packages/node/src/boundsRectangle.ts:30 (sha256:b16709cb6b4e81600c626260367e2c90af790b6cb6428d0076ebd70a41b6f2b8)
pub fn compute_node_bounds_rectangle(
    out: &mut RectangleLike,
    source: &Spatial2DNode,
    mut target_coordinate_space: Option<Spatial2DNode>,
) -> () {
    if (target_coordinate_space).is_none() {
        target_coordinate_space = Some((*source).clone());
    }
    let mut bounds: Option<Rectangle> = None;
    if (get_node_parent(&target_coordinate_space)).is_none() {
        bounds = Some(get_node_world_bounds_rectangle(source));
    } else {
        if (get_node_child_count(source) == 0.0_f64) {
            if (target_coordinate_space) == Some((*source).clone()) {
                bounds = Some(get_node_local_bounds_rectangle(source));
            } else {
                if (target_coordinate_space == get_node_parent(source)) {
                    bounds = Some(get_node_parent_bounds_rectangle(source));
                }
            }
        }
    }
    if (bounds).is_none() {
        let world_bounds = get_node_world_bounds_rectangle(source);
        let mut transform = acquire_matrix();
        inverse_matrix(
            &mut transform,
            &get_node_world_matrix(&target_coordinate_space),
        );
        matrix_transform_rectangle(out, &transform, &world_bounds);
        release_matrix(&transform);
    } else {
        copy_rectangle(out, &bounds);
    }
}

// Source: upstream/packages/node/src/boundsRectangle.ts:62 (sha256:bf97adb1af2344df175cdfeceea0487c3f1e90754602d0b6940f5c4052538dbd)
#[derive(Clone)]
struct EnsureNodeLocalBoundsRectangleRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for EnsureNodeLocalBoundsRectangleRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_node_local_bounds_rectangle(target: &BoundsNode) -> () {
    let mut runtime = get_entity_runtime(target);
    if (runtime.local_bounds_using_local_bounds_id != runtime.local_bounds_id) {
        recompute_local_bounds_rectangle(target, &mut runtime);
    }
}

// Source: upstream/packages/node/src/boundsRectangle.ts:69 (sha256:f16d49fecdac5d861f666955390e0e8a2c7f1cf76a4243a506fc0feca2be0216)
#[derive(Clone)]
struct EnsureNodeParentBoundsRectangleRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for EnsureNodeParentBoundsRectangleRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_node_parent_bounds_rectangle(target: &Spatial2DNode) -> () {
    let mut runtime = get_entity_runtime(target);
    if (runtime.bounds_using_local_bounds_id != runtime.local_bounds_id)
        || (runtime.bounds_using_local_transform_id != runtime.local_transform_id)
    {
        recompute_node_bounds_rectangle(target, &mut runtime);
    }
}

// Source: upstream/packages/node/src/boundsRectangle.ts:79 (sha256:220ca97e6825dc63f4da2605ebcb5dead3a55cc688b843d05eb7c201d9f32632)
#[derive(Clone)]
struct EnsureNodeWorldBoundsRectangleRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for EnsureNodeWorldBoundsRectangleRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_node_world_bounds_rectangle(target: &Spatial2DNode) -> () {
    let mut runtime = get_entity_runtime(target);
    let local_bounds_invalid =
        (runtime.world_bounds_using_local_bounds_id != runtime.local_bounds_id);
    let has_children = (get_node_child_count(target) != 0.0_f64);
    let mut force_recompute = false;
    if (!has_children) && (!local_bounds_invalid) {
        if try_fast_recompute_world_bounds_rectangle(target, &mut runtime) {
            return;
        }
        force_recompute = true;
    }
    ensure_node_world_matrix(target);
    if ((force_recompute) || (local_bounds_invalid))
        || (runtime.world_bounds_using_world_transform_id != runtime.world_transform_id)
    {
        recompute_world_bounds_rectangle(target, &mut runtime);
    }
}

// Source: upstream/packages/node/src/boundsRectangle.ts:94 (sha256:d6019ea68c13e9813d18a28133db486f6c2b6bd1b3455a2b29286ae5cd9ed08c)
pub fn get_node_height(source: &Spatial2DNode) -> f64 {
    compute_node_bounds_rectangle(
        &mut (*_TEMP_BOUNDS_RECTANGLE.lock().unwrap()),
        source,
        (get_node_parent(source)).clone(),
    );
    return (*_TEMP_BOUNDS_RECTANGLE.lock().unwrap()).height;
}

// Source: upstream/packages/node/src/boundsRectangle.ts:106 (sha256:63c194d72eae2f28b76ea5a693f0f660564407c5ac6b5402708545c110a9407b)
pub fn get_node_local_bounds_rectangle(target: &BoundsNode) -> Rectangle {
    ensure_node_local_bounds_rectangle(target);
    return ((get_entity_runtime(target).local_bounds_rectangle).clone()).unwrap();
}

// Source: upstream/packages/node/src/boundsRectangle.ts:114 (sha256:745948f81539c7186b249a43668bede60c3086794a4930b110bf3b1ae26b846e)
pub fn get_node_parent_bounds_rectangle(target: &Spatial2DNode) -> Rectangle {
    ensure_node_parent_bounds_rectangle(target);
    return ((get_entity_runtime(target).bounds_rectangle).clone()).unwrap();
}

// Source: upstream/packages/node/src/boundsRectangle.ts:121 (sha256:459a73cb920c75d999c291c9f040986177bfb60d81dddf26771664a22b61105d)
pub fn get_node_width(source: &Spatial2DNode) -> f64 {
    compute_node_bounds_rectangle(
        &mut (*_TEMP_BOUNDS_RECTANGLE.lock().unwrap()),
        source,
        (get_node_parent(source)).clone(),
    );
    return (*_TEMP_BOUNDS_RECTANGLE.lock().unwrap()).width;
}

// Source: upstream/packages/node/src/boundsRectangle.ts:133 (sha256:03f253128d326e0b9b064229161b5a65549af9164b443b657397ba75ec9563e6)
pub fn get_node_world_bounds_rectangle(target: &Spatial2DNode) -> Rectangle {
    ensure_node_world_bounds_rectangle(target);
    return ((get_entity_runtime(target).world_bounds_rectangle).clone()).unwrap();
}

// Source: upstream/packages/node/src/boundsRectangle.ts:138 (sha256:83b4f693d45ae87c5be095c10ab9274f480da3b9d50cc8fbf3419743436dfe18)
pub fn set_node_height(target: &mut Spatial2DNode, value: f64) -> () {
    if (target.scale_y == 0.0_f64) {
        return;
    }
    target.scale_y = ((value * target.scale_y) / get_node_height(target));
    invalidate_node_local_transform(target);
}

// Source: upstream/packages/node/src/boundsRectangle.ts:144 (sha256:57988ed1e899dd002ca593ca67976b6604253cea929f9b7da1f37c3dc906562c)
pub fn set_node_width(target: &mut Spatial2DNode, value: f64) -> () {
    if (target.scale_x == 0.0_f64) {
        return;
    }
    target.scale_x = ((value * target.scale_x) / get_node_width(target));
    invalidate_node_local_transform(target);
}

// Source: upstream/packages/node/src/boundsRectangle.ts:150 (sha256:c0cf3b7abc89f43669901ff02c948d7575f46af08c2ede32a084e50958a63f92)
#[derive(Clone)]
struct RecomputeNodeBoundsRectangleRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for RecomputeNodeBoundsRectangleRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recompute_node_bounds_rectangle(
    target: &Spatial2DNode,
    runtime: &mut RecomputeNodeBoundsRectangleRecord1,
) -> () {
    if ((runtime.bounds_rectangle).clone()).is_none() {
        runtime.bounds_rectangle = Some(create_rectangle(None, None, None, None));
    }
    matrix_transform_rectangle(
        &mut runtime.bounds_rectangle,
        &get_node_local_matrix(target),
        &get_node_local_bounds_rectangle(target),
    );
    runtime.bounds_using_local_bounds_id = runtime.local_bounds_id;
    runtime.bounds_using_local_transform_id = runtime.local_transform_id;
}

// Source: upstream/packages/node/src/boundsRectangle.ts:160 (sha256:966fbf3bc741dbbf04b3156fd969b9a289f0e6fd289f251c05cfbd6b68c46316)
#[derive(Clone)]
struct RecomputeLocalBoundsRectangleRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for RecomputeLocalBoundsRectangleRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recompute_local_bounds_rectangle(
    target: &BoundsNode,
    runtime: &mut RecomputeLocalBoundsRectangleRecord1,
) -> () {
    if ((runtime.local_bounds_rectangle).clone()).is_none() {
        runtime.local_bounds_rectangle = Some(create_rectangle(None, None, None, None));
    }
    {
        let __flight_callback = (runtime.compute_local_bounds_rectangle).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            ((runtime.local_bounds_rectangle).clone()).unwrap(),
            target,
        );
        __flight_result
    };
    runtime.local_bounds_using_local_bounds_id = runtime.local_bounds_id;
}

// Source: upstream/packages/node/src/boundsRectangle.ts:169 (sha256:b6f570ee89c07a88899e32fe595b3f544f5a58a4f2961eee5a9e586254a08d25)
#[derive(Clone)]
struct RecomputeWorldBoundsRectangleRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for RecomputeWorldBoundsRectangleRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn recompute_world_bounds_rectangle(
    target: &Spatial2DNode,
    runtime: &mut RecomputeWorldBoundsRectangleRecord1,
) -> () {
    if ((runtime.world_bounds_rectangle).clone()).is_none() {
        runtime.world_bounds_rectangle = Some(create_rectangle(None, None, None, None));
    }
    matrix_transform_rectangle(
        &mut runtime.world_bounds_rectangle,
        &get_node_world_matrix(target),
        &get_node_local_bounds_rectangle(target),
    );
    let children = (get_node_runtime(target).children).clone();
    if (children).is_some() {
        for child in (children.as_ref().unwrap()).iter().cloned() {
            if (!child.enabled) {
                continue;
            }
            let child_world_bounds = get_node_world_bounds_rectangle(&child);
            if (child_world_bounds.width != 0.0_f64) && (child_world_bounds.height != 0.0_f64) {
                {
                    let __flight_argument_1 = (runtime.world_bounds_rectangle).clone();
                    merge_rectangle(
                        &mut runtime.world_bounds_rectangle,
                        &__flight_argument_1,
                        &child_world_bounds,
                    )
                };
            }
        }
    }
    runtime.world_bounds_using_world_transform_id = runtime.world_transform_id;
    runtime.world_bounds_using_local_bounds_id = runtime.local_bounds_id;
}

// Source: upstream/packages/node/src/boundsRectangle.ts:193 (sha256:dea9d8d2fc53eb9622cb2929d9faf4be91cc0a2ee7f93e87d30c79d49ef2d7e9)
#[derive(Clone)]
struct TryFastRecomputeWorldBoundsRectangleRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for TryFastRecomputeWorldBoundsRectangleRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn try_fast_recompute_world_bounds_rectangle(
    target: &Spatial2DNode,
    runtime: &mut TryFastRecomputeWorldBoundsRectangleRecord1,
) -> bool {
    if (((runtime.world_bounds_rectangle).clone()).is_some())
        && (((runtime.world_matrix).clone()).is_some())
    {
        let __destructure0 = (runtime.world_matrix).clone();
        let _a = __destructure0.as_ref().unwrap().a;
        let _b = __destructure0.as_ref().unwrap().b;
        let _c = __destructure0.as_ref().unwrap().c;
        let _d = __destructure0.as_ref().unwrap().d;
        let _tx = __destructure0.as_ref().unwrap().tx;
        let _ty = __destructure0.as_ref().unwrap().ty;
        ensure_node_world_matrix(target);
        let __destructure1 = (runtime.world_matrix).clone();
        let a = __destructure1.as_ref().unwrap().a;
        let b = __destructure1.as_ref().unwrap().b;
        let c = __destructure1.as_ref().unwrap().c;
        let d = __destructure1.as_ref().unwrap().d;
        let tx = __destructure1.as_ref().unwrap().tx;
        let ty = __destructure1.as_ref().unwrap().ty;
        if (((a == _a) && (b == _b)) && (c == _c)) && (d == _d) {
            if (tx != _tx) || (ty != _ty) {
                runtime.world_bounds_rectangle.as_mut().unwrap().x += (tx - _tx);
                runtime.world_bounds_rectangle.as_mut().unwrap().y += (ty - _ty);
            }
            return true;
        }
    }
    return false;
}

// Source: upstream/packages/node/src/boundsRectangle.ts:214 (sha256:854dcc609c6f5b64c692db6da4731a77a8f62c741f35720e22455b6d28ff5057)
static _TEMP_BOUNDS_RECTANGLE: std::sync::LazyLock<std::sync::Mutex<Rectangle>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_rectangle(None, None, None, None)));
