// @generated from upstream/packages/scene/src/sceneNodeTransform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::create_matrix4;
use flighthq_node::set_node_local_matrix4;
use flighthq_types::{
    Adjustment, ColorTransform, InteractionSignals, Matrix4, Node, NodeInteractionState,
    NodeSignals, NodeTraitsKey, SceneNode, Transform3DNode, Vector3Like,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub can_add_child: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    >,
    pub children: Option<Vec<Node>>,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub traits: Option<NodeTraitsKey>,
    pub interaction_signals: Option<InteractionSignals>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_using_local_bounds_id: Option<f64>,
    pub local_content_id: Option<f64>,
    pub local_transform_id: Option<f64>,
    pub local_transform_using_local_transform_id: Option<f64>,
    pub node_signals: Option<NodeSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub parent: Option<Node>,
    pub world_bounds_using_local_bounds_id: Option<f64>,
    pub world_bounds_using_world_transform_id: Option<f64>,
    pub world_transform_id: Option<f64>,
    pub world_transform_using_local_transform_id: Option<f64>,
    pub world_transform_using_parent_transform_id: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene/src/sceneNodeTransform.ts:18 (sha256:b3833befa0472d7879c60d04e8fdd04ff28a14c9a89c32ff8ef6b6b16bf9bcab)
pub fn set_scene_node_look_at(
    node: &SceneNode,
    eye: &Vector3Like,
    target: &Vector3Like,
    up: &Vector3Like,
) -> () {
    let eye_x = eye.x;
    let eye_y = eye.y;
    let eye_z = eye.z;
    let mut zx = (eye_x - target.x);
    let mut zy = (eye_y - target.y);
    let mut zz = (eye_z - target.z);
    let mut zl = (((zx * zx) + (zy * zy)) + (zz * zz)).sqrt();
    if (zl == 0.0_f64) {
        zz = 1.0_f64;
        zl = 1.0_f64;
    }
    zx /= zl;
    zy /= zl;
    zz /= zl;
    let mut xx = ((up.y * zz) - (up.z * zy));
    let mut xy = ((up.z * zx) - (up.x * zz));
    let mut xz = ((up.x * zy) - (up.y * zx));
    let xl = (((xx * xx) + (xy * xy)) + (xz * xz)).sqrt();
    if (xl != 0.0_f64) {
        xx /= xl;
        xy /= xl;
        xz /= xl;
    }
    let yx = ((zy * xz) - (zz * xy));
    let yy = ((zz * xx) - (zx * xz));
    let yz = ((zx * xy) - (zy * xx));
    (*_SCRATCH_MATRIX.lock().unwrap()).m[0.0_f64 as usize] = (xx) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[1.0_f64 as usize] = (xy) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[2.0_f64 as usize] = (xz) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[3.0_f64 as usize] = (0.0_f64) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[4.0_f64 as usize] = (yx) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[5.0_f64 as usize] = (yy) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[6.0_f64 as usize] = (yz) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[7.0_f64 as usize] = (0.0_f64) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[8.0_f64 as usize] = (zx) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[9.0_f64 as usize] = (zy) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[10.0_f64 as usize] = (zz) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[11.0_f64 as usize] = (0.0_f64) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[12.0_f64 as usize] = (eye_x) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[13.0_f64 as usize] = (eye_y) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[14.0_f64 as usize] = (eye_z) as f32;
    (*_SCRATCH_MATRIX.lock().unwrap()).m[15.0_f64 as usize] = (1.0_f64) as f32;
    set_node_local_matrix4(
        &Transform3DNode {
            __flight_identity: std::sync::Arc::clone(&(node).__flight_identity),
            data: ((node).data).clone(),
            enabled: (node).enabled,
            kind: ((node).kind).clone(),
            name: ((node).name).clone(),
            position: ((node).position).clone(),
            rotation: ((node).rotation).clone(),
            scale: ((node).scale).clone(),
        },
        &(*_SCRATCH_MATRIX.lock().unwrap()),
    );
}

// Source: upstream/packages/scene/src/sceneNodeTransform.ts:74 (sha256:5ae017fae638d9a4fe19f3ce9271ca480bfb44e1cd347f46ed47a1e376544a67)
static _SCRATCH_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });
