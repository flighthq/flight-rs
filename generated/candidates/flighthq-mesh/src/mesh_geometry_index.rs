// @generated from upstream/packages/mesh/src/meshGeometryIndex.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{MeshGeometryOptions, create_mesh_geometry};
use flighthq_types::MeshGeometry;

// Source: upstream/packages/mesh/src/meshGeometryIndex.ts:15 (sha256:2d4738ffee2251648f9a4c30d4a706e8c0c20064dbbf11c77f121cee927fb7f7)
pub fn compute_mesh_geometry_wireframe_indices(geometry: &MeshGeometry) -> Vec<u32> {
    let use_uint32 = if ((geometry.indices).clone()).is_some() {
        ((geometry.indices).clone()).is_some()
    } else {
        true
    };
    if ((geometry.topology).clone() != "triangle-list")
        && ((geometry.topology).clone() != "triangle-strip")
    {
        return if use_uint32 {
            vec![0_u32; (0.0_f64) as usize]
        } else {
            vec![0_u32; (0.0_f64) as usize]
        };
    }
    let indices = (geometry.indices).clone();
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    let index_count = if (indices).is_some() {
        (indices.as_ref().unwrap().len() as f64)
    } else {
        vertex_count
    };
    let mut lines: Vec<f64> = vec![];
    if ((geometry.topology).clone() == "triangle-list") {
        {
            let mut t = 0.0_f64;
            while ((t + 2.0_f64) < index_count) {
                let a = if (indices).is_some() {
                    (indices.as_ref().unwrap()[t as usize] as f64) as u32
                } else {
                    (t) as u32
                };
                let b = if (indices).is_some() {
                    (indices.as_ref().unwrap()[(t + 1.0_f64) as usize] as f64) as u32
                } else {
                    (t + 1.0_f64) as u32
                };
                let c = if (indices).is_some() {
                    (indices.as_ref().unwrap()[(t + 2.0_f64) as usize] as f64) as u32
                } else {
                    (t + 2.0_f64) as u32
                };
                lines.extend(vec![
                    (a).clone(),
                    (b).clone(),
                    (b).clone(),
                    (c).clone(),
                    (c).clone(),
                    (a).clone(),
                ]);
                {
                    t += 3.0_f64;
                    t
                };
            }
        }
    } else {
        {
            let mut t = 0.0_f64;
            while ((t + 2.0_f64) < index_count) {
                let a = if (indices).is_some() {
                    (indices.as_ref().unwrap()[t as usize] as f64) as u32
                } else {
                    (t) as u32
                };
                let b = if (indices).is_some() {
                    (indices.as_ref().unwrap()[(t + 1.0_f64) as usize] as f64) as u32
                } else {
                    (t + 1.0_f64) as u32
                };
                let c = if (indices).is_some() {
                    (indices.as_ref().unwrap()[(t + 2.0_f64) as usize] as f64) as u32
                } else {
                    (t + 2.0_f64) as u32
                };
                lines.extend(vec![
                    (a).clone(),
                    (b).clone(),
                    (b).clone(),
                    (c).clone(),
                    (c).clone(),
                    (a).clone(),
                ]);
                {
                    t += 1.0;
                    t
                };
            }
        }
    }
    if use_uint32 {
        let mut out = vec![0_u32; (lines.len() as f64) as usize];
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<u32> = (lines).iter().map(|value| (*value) as u32).collect();
            out[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
        return out;
    }
    let mut out = vec![0_u16; (lines.len() as f64) as usize];
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<u16> = (lines).iter().map(|value| (*value) as u16).collect();
        out[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
    return out;
}

// Source: upstream/packages/mesh/src/meshGeometryIndex.ts:60 (sha256:67e593ea3e77e7dbff288c01f226bfce46348f7d12f841dd28602045dc05e174)
pub fn expand_mesh_geometry_indices(geometry: &MeshGeometry) -> MeshGeometry {
    let indices = (geometry.indices).clone();
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    if (indices).is_none() {
        let mut vertices = vec![0.0_f32; (geometry.vertices.len() as f64) as usize];
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = (geometry.vertices)
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            vertices[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
        return create_mesh_geometry(&mut MeshGeometryOptions {
            __flight_identity: std::sync::Arc::new(()),
            indices: None,
            layout: (geometry.layout).clone(),
            topology: Some((geometry.topology).clone()),
            vertices: (vertices).clone(),
            subsets: None,
        });
    }
    let mut vertices =
        vec![0.0_f32; ((indices.as_ref().unwrap().len() as f64) * floats_per_vertex) as usize];
    {
        let mut i = 0.0_f64;
        while (i < (indices.as_ref().unwrap().len() as f64)) {
            let src = (indices.as_ref().unwrap()[i as usize].clone() * floats_per_vertex);
            let dst = (i * floats_per_vertex);
            {
                let mut f = 0.0_f64;
                while (f < floats_per_vertex) {
                    vertices[(dst + f) as usize] =
                        (geometry.vertices[(src + f) as usize] as f64) as f32;
                    {
                        f += 1.0;
                        f
                    };
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return create_mesh_geometry(&mut MeshGeometryOptions {
        __flight_identity: std::sync::Arc::new(()),
        indices: None,
        layout: (geometry.layout).clone(),
        topology: Some((geometry.topology).clone()),
        vertices: (vertices).clone(),
        subsets: None,
    });
}
