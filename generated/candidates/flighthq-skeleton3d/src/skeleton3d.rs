// @generated from upstream/packages/skeleton3d/src/skeleton3d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{copy_matrix4, create_matrix4, inverse_matrix4, multiply_matrix4};
use flighthq_node::get_node_world_matrix4;
use flighthq_types::{
    Adjustment, ColorTransform, InteractionSignals, Matrix4, Matrix4Like, Node,
    NodeInteractionState, NodeSignals, NodeTraitsKey, SceneNode, Skeleton3D,
    Skeleton3DValidationDiagnostic, Transform3DNode,
};

#[derive(Clone, Default)]
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

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:5 (sha256:5046c509e06792a8abf01f7390c3af7e31e79e1074a5f9e9dd0a892abd3ec633)
pub fn clone_skeleton3_d(skeleton: &Skeleton3D) -> Skeleton3D {
    let mut clone: Skeleton3D = Skeleton3D {
        __flight_identity: std::sync::Arc::new(()),
        inverse_bind_matrices: ((skeleton.inverse_bind_matrices).clone())
            .iter()
            .map(|value| (*value) as f32)
            .collect(),
        joint_matrices: ((skeleton.joint_matrices).clone())
            .iter()
            .map(|value| (*value) as f32)
            .collect(),
        joints: ((skeleton.joints).clone()).clone(),
        names: None,
    };
    if ((skeleton.names).clone()).is_some() {
        clone.names = Some(((skeleton.names).clone()).as_ref().unwrap().clone());
    } else {
        if ((skeleton.names).clone()).is_none() {
            clone.names = None;
        }
    }
    return clone;
}

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:16 (sha256:9ae8c5a1a8f683deb8ae1d9e297ada2536e6e6af2f007fa0750ad02612972801)
pub fn compute_skeleton3_d_joint_matrices(skeleton: &mut Skeleton3D) -> () {
    {
        let mut j = 0.0_f64;
        while (j < (skeleton.joints.len() as f64)) {
            let base = (j * 16.0_f64);
            {
                let mut i = 0.0_f64;
                while (i < 16.0_f64) {
                    (*_INV_BIND.lock().unwrap()).m[i as usize] =
                        (skeleton.inverse_bind_matrices[(base + i) as usize] as f64) as f32;
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            multiply_matrix4(
                &mut (*_RESULT.lock().unwrap()),
                &get_node_world_matrix4(&{
                    let __flight_source = &(skeleton.joints[j as usize]);
                    Transform3DNode {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        data: (__flight_source.data).clone(),
                        enabled: __flight_source.enabled,
                        kind: (__flight_source.kind).clone(),
                        name: (__flight_source.name).clone(),
                        position: (__flight_source.position).clone(),
                        rotation: (__flight_source.rotation).clone(),
                        scale: (__flight_source.scale).clone(),
                    }
                }),
                &{
                    let __flight_source = &(*_INV_BIND.lock().unwrap());
                    Matrix4Like {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        m: (__flight_source.m).clone(),
                    }
                },
            );
            {
                let __flight_offset = (base) as usize;
                let __flight_values: Vec<f32> = (((*_RESULT.lock().unwrap()).m).clone())
                    .iter()
                    .map(|value| (*value) as f32)
                    .collect();
                skeleton.joint_matrices[__flight_offset..__flight_offset + __flight_values.len()]
                    .copy_from_slice(&__flight_values);
            };
            {
                j += 1.0;
                j
            };
        }
    }
}

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:26 (sha256:2bf2d7caf761d2830957e5f3bdc26fbd29146b091e74fa9d5b40fed1d276cc65)
pub fn create_skeleton3_d(
    joints: &Vec<SceneNode>,
    inverse_bind_matrices: Option<Vec<f32>>,
    names: Option<Vec<String>>,
) -> Skeleton3D {
    let count = (joints.len() as f64);
    let mut skeleton: Skeleton3D = Skeleton3D {
        __flight_identity: std::sync::Arc::new(()),
        inverse_bind_matrices: (inverse_bind_matrices)
            .unwrap_or(vec![0.0_f32; (count * 16.0_f64) as usize]),
        joint_matrices: vec![0.0_f32; (count * 16.0_f64) as usize],
        joints: (*joints).clone(),
        names: names,
    };
    if (inverse_bind_matrices).is_none() {
        set_skeleton3_d_bind_pose(&mut skeleton);
    }
    return skeleton;
}

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:42 (sha256:7573fc033e7c49de46c07984fbea9f06f97f30541e0707f583a08fcd02abbb10)
pub fn dispose_skeleton3_d(skeleton: &mut Skeleton3D) -> () {
    skeleton.joints.clear();
    skeleton.names = None;
}

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:47 (sha256:28de79c44c0f1f5a87e758a1cda37d31ac01a8e85bcd2680999108a28a6c5ce2)
pub fn equals_skeleton3_d(a: &Skeleton3D, b: &Skeleton3D) -> bool {
    if (a == b) {
        return true;
    }
    if ((a.joints.len() as f64) != (b.joints.len() as f64)) {
        return false;
    }
    if ((a.inverse_bind_matrices.len() as f64) != (b.inverse_bind_matrices.len() as f64)) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < (a.inverse_bind_matrices.len() as f64)) {
            if ((a.inverse_bind_matrices[i as usize] as f64)
                != (b.inverse_bind_matrices[i as usize] as f64))
            {
                return false;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let a_names = (a.names).clone();
    let b_names = (b.names).clone();
    if ((a_names).is_none()) || ((b_names).is_none()) {
        return (a_names == b_names);
    }
    if ((a_names.as_ref().unwrap().len() as f64) != (b_names.as_ref().unwrap().len() as f64)) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < (a_names.as_ref().unwrap().len() as f64)) {
            if (a_names.as_ref().unwrap()[i as usize].clone()
                != b_names.as_ref().unwrap()[i as usize].clone())
            {
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

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:64 (sha256:37055770ac4860533da3d7afe06fa4aca3710a2c1ae5069399226efb426f5ee5)
pub fn get_skeleton3_d_joint_index_by_name(skeleton: &Skeleton3D, name: String) -> f64 {
    let names = (skeleton.names).clone();
    if (names).is_none() {
        return (-1.0_f64);
    }
    return {
        let __flight_value = (name).clone();
        (names.as_ref().unwrap())
            .iter()
            .position(|item| item == &__flight_value)
            .map_or(-1.0_f64, |index| index as f64)
    };
}

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:70 (sha256:54f90d74868ff356cd437feeecdc56a8ad7ea74e442986d38a8cfef22fd15d6e)
pub fn get_skeleton3_d_joint_world_matrix(
    out: &mut Matrix4Like,
    skeleton: &Skeleton3D,
    joint_index: f64,
) -> bool {
    if (joint_index < 0.0_f64) || (joint_index >= (skeleton.joints.len() as f64)) {
        return false;
    }
    copy_matrix4(
        out,
        &get_node_world_matrix4(&{
            let __flight_source = &(skeleton.joints[joint_index as usize]);
            Transform3DNode {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                data: (__flight_source.data).clone(),
                enabled: __flight_source.enabled,
                kind: (__flight_source.kind).clone(),
                name: (__flight_source.name).clone(),
                position: (__flight_source.position).clone(),
                rotation: (__flight_source.rotation).clone(),
                scale: (__flight_source.scale).clone(),
            }
        }),
    );
    return true;
}

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:81 (sha256:0bdae0d3e59f5fc6a660e024b06c7875d52665a9f0daf0a3329e0d765df03523)
pub fn get_skeleton3_d_joint_world_matrix_by_name(
    out: &mut Matrix4Like,
    skeleton: &Skeleton3D,
    name: String,
) -> bool {
    return get_skeleton3_d_joint_world_matrix(
        out,
        skeleton,
        get_skeleton3_d_joint_index_by_name(skeleton, (name).clone()),
    );
}

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:89 (sha256:681ebd52d85b715f1ba764889e37d0843ab6910fc07d7ece448b2c56355373ba)
pub fn set_skeleton3_d_bind_pose(skeleton: &mut Skeleton3D) -> () {
    {
        let mut j = 0.0_f64;
        while (j < (skeleton.joints.len() as f64)) {
            inverse_matrix4(
                &mut (*_RESULT.lock().unwrap()),
                &get_node_world_matrix4(&{
                    let __flight_source = &(skeleton.joints[j as usize]);
                    Transform3DNode {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        data: (__flight_source.data).clone(),
                        enabled: __flight_source.enabled,
                        kind: (__flight_source.kind).clone(),
                        name: (__flight_source.name).clone(),
                        position: (__flight_source.position).clone(),
                        rotation: (__flight_source.rotation).clone(),
                        scale: (__flight_source.scale).clone(),
                    }
                }),
            );
            {
                let __flight_offset = (j * 16.0_f64) as usize;
                let __flight_values: Vec<f32> = (((*_RESULT.lock().unwrap()).m).clone())
                    .iter()
                    .map(|value| (*value) as f32)
                    .collect();
                skeleton.inverse_bind_matrices
                    [__flight_offset..__flight_offset + __flight_values.len()]
                    .copy_from_slice(&__flight_values);
            };
            {
                j += 1.0;
                j
            };
        }
    }
}

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:97 (sha256:da6319bfede47b9244a0c4ad3c8c2d0de42089668e337364f6696b1fea4ce743)
pub fn validate_skeleton3_d(skeleton: &Skeleton3D) -> Option<Skeleton3DValidationDiagnostic> {
    let joint_count = (skeleton.joints.len() as f64);
    let expected_inverse_bind_matrices_length = (joint_count * 16.0_f64);
    let inverse_bind_matrices_length = (skeleton.inverse_bind_matrices.len() as f64);
    if (inverse_bind_matrices_length == expected_inverse_bind_matrices_length) {
        return None;
    }
    return Some(Skeleton3DValidationDiagnostic {
        __flight_identity: std::sync::Arc::new(()),
        expected_inverse_bind_matrices_length: expected_inverse_bind_matrices_length,
        inverse_bind_matrices_length: inverse_bind_matrices_length,
        joint_count: joint_count,
        message: format!(
            "Skeleton3D inverseBindMatrices length {} does not match jointCount {} * 16 = {}.",
            inverse_bind_matrices_length, joint_count, expected_inverse_bind_matrices_length
        ),
    });
}

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:110 (sha256:fcabda72dea4cc0bfee7c6285fc17e8aa41819397dd0a18d3664994d557ce71e)
static _INV_BIND: std::sync::LazyLock<std::sync::Mutex<Matrix4>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(create_matrix4(
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None,
    ))
});

// Source: upstream/packages/skeleton3d/src/skeleton3d.ts:111 (sha256:fa90bd3828709c353651c00f0ee1992022c3c11cde62c44a4450e4976fc6b0a9)
static _RESULT: std::sync::LazyLock<std::sync::Mutex<Matrix4>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(create_matrix4(
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None,
    ))
});
