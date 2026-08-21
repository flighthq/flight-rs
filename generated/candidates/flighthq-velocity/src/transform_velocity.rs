// @generated from upstream/packages/velocity/src/transformVelocity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ensure_velocity_sample;
use flighthq_geometry::{copy_matrix, create_matrix};
use flighthq_node::{
    ensure_node_world_matrix, get_node_child_at, get_node_child_count, get_node_world_matrix,
};
use flighthq_types::{MatrixLike, Node, Transform2DNode, VelocityField};

// Source: upstream/packages/velocity/src/transformVelocity.ts:16 (sha256:7679812e9ee7a1735a138952026c1e7490ec0c08dd6fd813799081e67c5d9124)
pub fn contribute_transform_velocity(field: &mut VelocityField, root: &Transform2DNode) -> () {
    visit_transform_velocity(field, root);
}

// Source: upstream/packages/velocity/src/transformVelocity.ts:23 (sha256:079b4cbd6c1a40ff799df76bb97073573674b0902db0ddb0dd44353e0e7fcec6)
fn visit_transform_velocity(field: &mut VelocityField, node: &Transform2DNode) -> () {
    let mutable_node = (*node).clone();
    ensure_node_world_matrix(&mutable_node);
    let world = get_node_world_matrix(&mutable_node);
    let mut sample = ensure_velocity_sample(field, (node).clone());
    if (sample.explicit_frame_id != field.frame_id) {
        if ((sample.previous_world_transform).clone()).is_some() {
            sample.velocity.x = (world.tx - sample.previous_world_transform.as_mut().unwrap().tx);
            sample.velocity.y = (world.ty - sample.previous_world_transform.as_mut().unwrap().ty);
        } else {
            sample.velocity.x = 0.0_f64;
            sample.velocity.y = 0.0_f64;
        }
        sample.last_frame_id = field.frame_id;
    }
    if ((sample.previous_world_transform).clone()).is_none() {
        sample.previous_world_transform = Some(create_matrix(None, None, None, None, None, None));
    }
    copy_matrix(sample.previous_world_transform.as_mut().unwrap(), &{
        let __flight_source = &(world);
        MatrixLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            a: __flight_source.a,
            b: __flight_source.b,
            c: __flight_source.c,
            d: __flight_source.d,
            tx: __flight_source.tx,
            ty: __flight_source.ty,
        }
    });
    let count = get_node_child_count(&{
        let __flight_source = &(mutable_node);
        Node {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            data: (__flight_source.data).clone(),
            enabled: __flight_source.enabled,
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
        }
    });
    {
        let mut i = 0.0_f64;
        while (i < count) {
            let child = get_node_child_at(
                &{
                    let __flight_source = &(mutable_node);
                    Node {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        data: (__flight_source.data).clone(),
                        enabled: __flight_source.enabled,
                        kind: (__flight_source.kind).clone(),
                        name: (__flight_source.name).clone(),
                    }
                },
                i,
            );
            if ((child).clone()).is_some() {
                visit_transform_velocity(field, &(child.as_ref().unwrap()).clone());
            }
            {
                i += 1.0;
                i
            };
        }
    }
}
