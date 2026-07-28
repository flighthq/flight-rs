// @generated from upstream/packages/skeleton3d/src/updateMeshSkin.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{capture_mesh_skin_bind_pose, compute_skeleton3_d_joint_matrices, skin_mesh_geometry};
use flighthq_mesh::{get_mesh_geometry_skin_bind_pose, set_mesh_geometry_skin_bind_pose};
use flighthq_types::{
    Adjustment, ColorTransform, InteractionSignals, Mesh, Node, NodeInteractionState, NodeSignals,
    NodeTraitsKey,
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

// Source: upstream/packages/skeleton3d/src/updateMeshSkin.ts:19 (sha256:aa2ccb7aeaccc9503d2bd0f2a747e5ee60be56dd097910b0546b007e7f6fac1e)
pub fn update_mesh_skin(mesh: &mut Mesh) -> () {
    let mut skin = (mesh.skin).clone();
    if (skin).is_none() {
        return;
    }
    compute_skeleton3_d_joint_matrices(&mut skin.as_mut().unwrap().skeleton);
    let mut bind_pose = get_mesh_geometry_skin_bind_pose(&mesh.geometry);
    if (bind_pose).is_none() {
        bind_pose = Some(capture_mesh_skin_bind_pose(&mesh.geometry));
        set_mesh_geometry_skin_bind_pose(&mut mesh.geometry, ((bind_pose).clone()).clone());
    }
    skin_mesh_geometry(
        &mut mesh.geometry,
        &skin.as_mut().unwrap().skeleton,
        bind_pose.as_mut().unwrap(),
    );
}
