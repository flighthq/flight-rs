// @generated from upstream/packages/mesh/src/morphMeshGeometry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_vertex_attribute_float_offset;
use flighthq_types::{MeshGeometry, MeshMorph, MeshMorphBindPose};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/mesh/src/morphMeshGeometry.ts:14 (sha256:37f3da3a0122ed63a282a37f52623ff835a590f1b885876d3696067d1278564d)
pub fn blend_mesh_geometry_morph(
    geometry: &mut MeshGeometry,
    morph: &MeshMorph,
    bind_pose: &mut MeshMorphBindPose,
) -> () {
    let mut blended_normals = (bind_pose.blended_normals).clone();
    let mut blended_tangents = (bind_pose.blended_tangents).clone();
    let normals = (bind_pose.normals).clone();
    let tangents = (bind_pose.tangents).clone();
    let vertex_count = (__flight_js_to_i32(((bind_pose.positions.len() as f64) / 3.0_f64))
        | __flight_js_to_i32(0.0_f64)) as f64;
    let floats = (vertex_count * 3.0_f64);
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<f32> =
            (bind_pose.positions[(0.0_f64) as usize..(floats) as usize].to_vec())
                .iter()
                .map(|value| (*value) as f32)
                .collect();
        bind_pose.blended_positions[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
    if ((blended_normals).is_some()) && ((normals).is_some()) {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = (normals[(0.0_f64) as usize..(floats) as usize]
                .to_vec())
            .iter()
            .map(|value| (*value) as f32)
            .collect();
            blended_normals.as_mut().unwrap()
                [__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    if ((blended_tangents).is_some()) && ((tangents).is_some()) {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = (tangents[(0.0_f64) as usize..(floats) as usize]
                .to_vec())
            .iter()
            .map(|value| (*value) as f32)
            .collect();
            blended_tangents.as_mut().unwrap()
                [__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    let target_count = (morph.targets.len() as f64).min((morph.weights.len() as f64));
    {
        let mut t = 0.0_f64;
        while (t < target_count) {
            let weight = (morph.weights[t as usize] as f64);
            if (weight == 0.0_f64) {
                {
                    t += 1.0;
                    t
                };
                continue;
            }
            let target = morph.targets[t as usize].clone();
            accumulate_deltas(
                &mut bind_pose.blended_positions,
                &target.position_deltas,
                (weight).clone(),
                floats,
            );
            if ((blended_normals).is_some()) && (((target.normal_deltas).clone()).is_some()) {
                accumulate_deltas(
                    blended_normals.as_mut().unwrap(),
                    target.normal_deltas.as_ref().unwrap(),
                    (weight).clone(),
                    floats,
                );
            }
            if ((blended_tangents).is_some()) && (((target.tangent_deltas).clone()).is_some()) {
                accumulate_deltas(
                    blended_tangents.as_mut().unwrap(),
                    target.tangent_deltas.as_ref().unwrap(),
                    (weight).clone(),
                    floats,
                );
            }
            {
                t += 1.0;
                t
            };
        }
    }
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let position_offset =
        get_vertex_attribute_float_offset(&geometry.layout, "position".to_owned());
    let normal_offset = get_vertex_attribute_float_offset(&geometry.layout, "normal".to_owned());
    let tangent_offset = get_vertex_attribute_float_offset(&geometry.layout, "tangent".to_owned());
    {
        let mut v = 0.0_f64;
        while (v < vertex_count) {
            let dst = (v * floats_per_vertex);
            let s = (v * 3.0_f64);
            if (position_offset >= 0.0_f64) {
                geometry.vertices[(dst + position_offset) as usize] =
                    (bind_pose.blended_positions[s as usize] as f64) as f32;
                geometry.vertices[((dst + position_offset) + 1.0_f64) as usize] =
                    (bind_pose.blended_positions[(s + 1.0_f64) as usize] as f64) as f32;
                geometry.vertices[((dst + position_offset) + 2.0_f64) as usize] =
                    (bind_pose.blended_positions[(s + 2.0_f64) as usize] as f64) as f32;
            }
            if ((blended_normals).is_some()) && (normal_offset >= 0.0_f64) {
                geometry.vertices[(dst + normal_offset) as usize] =
                    (blended_normals.as_mut().unwrap()[s as usize].clone()) as f32;
                geometry.vertices[((dst + normal_offset) + 1.0_f64) as usize] =
                    (blended_normals.as_mut().unwrap()[(s + 1.0_f64) as usize].clone()) as f32;
                geometry.vertices[((dst + normal_offset) + 2.0_f64) as usize] =
                    (blended_normals.as_mut().unwrap()[(s + 2.0_f64) as usize].clone()) as f32;
            }
            if ((blended_tangents).is_some()) && (tangent_offset >= 0.0_f64) {
                geometry.vertices[(dst + tangent_offset) as usize] =
                    (blended_tangents.as_mut().unwrap()[s as usize].clone()) as f32;
                geometry.vertices[((dst + tangent_offset) + 1.0_f64) as usize] =
                    (blended_tangents.as_mut().unwrap()[(s + 1.0_f64) as usize].clone()) as f32;
                geometry.vertices[((dst + tangent_offset) + 2.0_f64) as usize] =
                    (blended_tangents.as_mut().unwrap()[(s + 2.0_f64) as usize].clone()) as f32;
            }
            {
                v += 1.0;
                v
            };
        }
    }
    {
        geometry.version += 1.0;
        geometry.version
    };
}

// Source: upstream/packages/mesh/src/morphMeshGeometry.ts:80 (sha256:b437e2249edb809fc624c2a1217ade4b5c0845f57896ab6c1be49d04bed55a22)
pub fn capture_mesh_morph_bind_pose(geometry: &MeshGeometry) -> MeshMorphBindPose {
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        (__flight_js_to_i32(((geometry.vertices.len() as f64) / floats_per_vertex))
            | __flight_js_to_i32(0.0_f64)) as f64
    } else {
        0.0_f64
    };
    let position_offset =
        get_vertex_attribute_float_offset(&geometry.layout, "position".to_owned());
    let normal_offset = get_vertex_attribute_float_offset(&geometry.layout, "normal".to_owned());
    let tangent_offset = get_vertex_attribute_float_offset(&geometry.layout, "tangent".to_owned());
    let mut positions: Vec<f32> = vec![0.0_f32; (vertex_count * 3.0_f64) as usize];
    let mut normals = if (normal_offset >= 0.0_f64) {
        Some(vec![0.0_f32; (vertex_count * 3.0_f64) as usize])
    } else {
        None
    };
    let mut tangents = if (tangent_offset >= 0.0_f64) {
        Some(vec![0.0_f32; (vertex_count * 3.0_f64) as usize])
    } else {
        None
    };
    {
        let mut v = 0.0_f64;
        while (v < vertex_count) {
            let base = (v * floats_per_vertex);
            let p = (v * 3.0_f64);
            if (position_offset >= 0.0_f64) {
                positions[p as usize] =
                    (geometry.vertices[(base + position_offset) as usize] as f64) as f32;
                positions[(p + 1.0_f64) as usize] = (geometry.vertices
                    [((base + position_offset) + 1.0_f64) as usize]
                    as f64) as f32;
                positions[(p + 2.0_f64) as usize] = (geometry.vertices
                    [((base + position_offset) + 2.0_f64) as usize]
                    as f64) as f32;
            }
            if (normals).is_some() {
                normals.as_mut().unwrap()[p as usize] =
                    (geometry.vertices[(base + normal_offset) as usize] as f64) as f32;
                normals.as_mut().unwrap()[(p + 1.0_f64) as usize] =
                    (geometry.vertices[((base + normal_offset) + 1.0_f64) as usize] as f64) as f32;
                normals.as_mut().unwrap()[(p + 2.0_f64) as usize] =
                    (geometry.vertices[((base + normal_offset) + 2.0_f64) as usize] as f64) as f32;
            }
            if (tangents).is_some() {
                tangents.as_mut().unwrap()[p as usize] =
                    (geometry.vertices[(base + tangent_offset) as usize] as f64) as f32;
                tangents.as_mut().unwrap()[(p + 1.0_f64) as usize] =
                    (geometry.vertices[((base + tangent_offset) + 1.0_f64) as usize] as f64) as f32;
                tangents.as_mut().unwrap()[(p + 2.0_f64) as usize] =
                    (geometry.vertices[((base + tangent_offset) + 2.0_f64) as usize] as f64) as f32;
            }
            {
                v += 1.0;
                v
            };
        }
    }
    return MeshMorphBindPose {
        __flight_identity: std::sync::Arc::new(()),
        blended_normals: if (normals).is_some() {
            Some(vec![0.0_f32; (vertex_count * 3.0_f64) as usize])
        } else {
            None
        },
        blended_positions: vec![0.0_f32; (vertex_count * 3.0_f64) as usize],
        blended_tangents: if (tangents).is_some() {
            Some(vec![0.0_f32; (vertex_count * 3.0_f64) as usize])
        } else {
            None
        },
        normals: (normals).clone(),
        positions: (positions).clone(),
        tangents: (tangents).clone(),
    };
}

// Source: upstream/packages/mesh/src/morphMeshGeometry.ts:126 (sha256:d1e33998292769ea412e2e2cf0b6bf10fc39c8983fbe8399a53f4d9de63ffb6b)
fn accumulate_deltas(accumulator: &mut Vec<f32>, deltas: &Vec<f32>, weight: f64, count: f64) -> () {
    let n = (count).min((deltas.len() as f64));
    {
        let mut i = 0.0_f64;
        while (i < n) {
            accumulator[i as usize] += (weight * (deltas[i as usize] as f64)) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
}
