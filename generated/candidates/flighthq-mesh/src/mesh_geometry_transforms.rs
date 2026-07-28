// @generated from upstream/packages/mesh/src/meshGeometryTransforms.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{compute_mesh_geometry_bounds, get_vertex_attribute_float_offset};
use flighthq_geometry::create_aabb;
use flighthq_types::{Matrix4Like, MeshGeometry};

// Source: upstream/packages/mesh/src/meshGeometryTransforms.ts:13 (sha256:783d719523158f5612c35ca7b5dcf61d876c3c285fe3245b85f8dc376e7bd091)
pub fn center_mesh_geometry(geometry: &mut MeshGeometry) -> () {
    if ((geometry.bounds).clone()).is_none() {
        let mut bounds = create_aabb(None, None, None, None, None, None);
        compute_mesh_geometry_bounds(&mut bounds, geometry);
        geometry.bounds = Some((bounds).clone());
    }
    let b = (geometry.bounds).clone();
    let cx = ((b.as_ref().unwrap().min.x + b.as_ref().unwrap().max.x) * 0.5_f64);
    let cy = ((b.as_ref().unwrap().min.y + b.as_ref().unwrap().max.y) * 0.5_f64);
    let cz = ((b.as_ref().unwrap().min.z + b.as_ref().unwrap().max.z) * 0.5_f64);
    if ((cx == 0.0_f64) && (cy == 0.0_f64)) && (cz == 0.0_f64) {
        return;
    }
    translate_mesh_geometry(geometry, (-cx), (-cy), (-cz));
}

// Source: upstream/packages/mesh/src/meshGeometryTransforms.ts:30 (sha256:575dd69b0a759eaca4cbb88d752ebc12015f60086e9e5f24c1cc4e6f4d5224f1)
pub fn scale_mesh_geometry(geometry: &mut MeshGeometry, sx: f64, sy: f64, sz: f64) -> () {
    {
        let __flight_argument_1 = (geometry).clone();
        transform_mesh_geometry_positions(
            geometry,
            &__flight_argument_1,
            sx,
            sy,
            sz,
            0.0_f64,
            0.0_f64,
            0.0_f64,
        )
    };
}

// Source: upstream/packages/mesh/src/meshGeometryTransforms.ts:39 (sha256:cc15e7dabce29f415d016fecd52d68598401d680fee8907dee3a4cc36996d8ea)
pub fn transform_mesh_geometry(geometry: &mut MeshGeometry, matrix: &Matrix4Like) -> bool {
    return {
        let __flight_argument_1 = (geometry).clone();
        transform_mesh_geometry_into(geometry, &__flight_argument_1, matrix)
    };
}

// Source: upstream/packages/mesh/src/meshGeometryTransforms.ts:48 (sha256:c9dcaa3a733a0d748777f2012afbd650559090a5febfe772494678d2079ec7fd)
pub fn transform_mesh_geometry_into(
    out: &mut MeshGeometry,
    source: &MeshGeometry,
    matrix: &Matrix4Like,
) -> bool {
    let inv_t = compute_matrix3x3_inverse_transpose(matrix);
    if (inv_t).is_none() {
        return false;
    }
    let pos_float_offset = get_vertex_attribute_float_offset(&source.layout, "position".to_owned());
    let norm_float_offset = get_vertex_attribute_float_offset(&source.layout, "normal".to_owned());
    let tan_float_offset = get_vertex_attribute_float_offset(&source.layout, "tangent".to_owned());
    let floats_per_vertex = (source.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((source.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    let mut dst_verts = if (out == source) {
        (source.vertices).clone()
    } else {
        (out.vertices).clone()
    };
    if (out != source) {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = (source.vertices)
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            dst_verts[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let vert_base = (i * floats_per_vertex);
            if (pos_float_offset >= 0.0_f64) {
                let pb = (vert_base + pos_float_offset);
                let px = (source.vertices[pb as usize] as f64);
                let py = (source.vertices[(pb + 1.0_f64) as usize] as f64);
                let pz = (source.vertices[(pb + 2.0_f64) as usize] as f64);
                dst_verts[pb as usize] = (((((matrix.m[0.0_f64 as usize] as f64) * px)
                    + ((matrix.m[4.0_f64 as usize] as f64) * py))
                    + ((matrix.m[8.0_f64 as usize] as f64) * pz))
                    + (matrix.m[12.0_f64 as usize] as f64))
                    as f32;
                dst_verts[(pb + 1.0_f64) as usize] =
                    (((((matrix.m[1.0_f64 as usize] as f64) * px)
                        + ((matrix.m[5.0_f64 as usize] as f64) * py))
                        + ((matrix.m[9.0_f64 as usize] as f64) * pz))
                        + (matrix.m[13.0_f64 as usize] as f64)) as f32;
                dst_verts[(pb + 2.0_f64) as usize] =
                    (((((matrix.m[2.0_f64 as usize] as f64) * px)
                        + ((matrix.m[6.0_f64 as usize] as f64) * py))
                        + ((matrix.m[10.0_f64 as usize] as f64) * pz))
                        + (matrix.m[14.0_f64 as usize] as f64)) as f32;
            }
            if (norm_float_offset >= 0.0_f64) {
                let nb = (vert_base + norm_float_offset);
                let nx = (source.vertices[nb as usize] as f64);
                let ny = (source.vertices[(nb + 1.0_f64) as usize] as f64);
                let nz = (source.vertices[(nb + 2.0_f64) as usize] as f64);
                let mut tnx = (((inv_t.as_ref().unwrap()[0.0_f64 as usize].clone() * nx)
                    + (inv_t.as_ref().unwrap()[3.0_f64 as usize].clone() * ny))
                    + (inv_t.as_ref().unwrap()[6.0_f64 as usize].clone() * nz));
                let mut tny = (((inv_t.as_ref().unwrap()[1.0_f64 as usize].clone() * nx)
                    + (inv_t.as_ref().unwrap()[4.0_f64 as usize].clone() * ny))
                    + (inv_t.as_ref().unwrap()[7.0_f64 as usize].clone() * nz));
                let mut tnz = (((inv_t.as_ref().unwrap()[2.0_f64 as usize].clone() * nx)
                    + (inv_t.as_ref().unwrap()[5.0_f64 as usize].clone() * ny))
                    + (inv_t.as_ref().unwrap()[8.0_f64 as usize].clone() * nz));
                let len = (((tnx * tnx) + (tny * tny)) + (tnz * tnz)).sqrt();
                if (len > 0.0_f64) {
                    tnx /= len;
                    tny /= len;
                    tnz /= len;
                }
                dst_verts[nb as usize] = (tnx) as f32;
                dst_verts[(nb + 1.0_f64) as usize] = (tny) as f32;
                dst_verts[(nb + 2.0_f64) as usize] = (tnz) as f32;
            }
            if (tan_float_offset >= 0.0_f64) {
                let tb = (vert_base + tan_float_offset);
                let tx = (source.vertices[tb as usize] as f64);
                let ty = (source.vertices[(tb + 1.0_f64) as usize] as f64);
                let tz = (source.vertices[(tb + 2.0_f64) as usize] as f64);
                let tw = (source.vertices[(tb + 3.0_f64) as usize] as f64);
                let mut ttx = (((inv_t.as_ref().unwrap()[0.0_f64 as usize].clone() * tx)
                    + (inv_t.as_ref().unwrap()[3.0_f64 as usize].clone() * ty))
                    + (inv_t.as_ref().unwrap()[6.0_f64 as usize].clone() * tz));
                let mut tty = (((inv_t.as_ref().unwrap()[1.0_f64 as usize].clone() * tx)
                    + (inv_t.as_ref().unwrap()[4.0_f64 as usize].clone() * ty))
                    + (inv_t.as_ref().unwrap()[7.0_f64 as usize].clone() * tz));
                let mut ttz = (((inv_t.as_ref().unwrap()[2.0_f64 as usize].clone() * tx)
                    + (inv_t.as_ref().unwrap()[5.0_f64 as usize].clone() * ty))
                    + (inv_t.as_ref().unwrap()[8.0_f64 as usize].clone() * tz));
                let len = (((ttx * ttx) + (tty * tty)) + (ttz * ttz)).sqrt();
                if (len > 0.0_f64) {
                    ttx /= len;
                    tty /= len;
                    ttz /= len;
                }
                dst_verts[tb as usize] = (ttx) as f32;
                dst_verts[(tb + 1.0_f64) as usize] = (tty) as f32;
                dst_verts[(tb + 2.0_f64) as usize] = (ttz) as f32;
                dst_verts[(tb + 3.0_f64) as usize] = ((tw).clone()) as f32;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    {
        out.version += 1.0;
        out.version
    };
    if ((out.bounds).clone()).is_some() {
        {
            let __flight_argument_1 = (out).clone();
            compute_mesh_geometry_bounds(&mut out.bounds, &__flight_argument_1)
        };
    }
    return true;
}

// Source: upstream/packages/mesh/src/meshGeometryTransforms.ts:129 (sha256:96f0bc4ec8ad9b3ba48fb7521d9b7ac0fe9322131cf11e83164e248808a3e654)
pub fn translate_mesh_geometry(geometry: &mut MeshGeometry, x: f64, y: f64, z: f64) -> () {
    let pos_float_offset =
        get_vertex_attribute_float_offset(&geometry.layout, "position".to_owned());
    if (pos_float_offset < 0.0_f64) {
        return;
    }
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let base = ((i * floats_per_vertex) + pos_float_offset);
            geometry.vertices[base as usize] += (x) as f32;
            geometry.vertices[(base + 1.0_f64) as usize] += (y) as f32;
            geometry.vertices[(base + 2.0_f64) as usize] += (z) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    {
        geometry.version += 1.0;
        geometry.version
    };
    if ((geometry.bounds).clone()).is_some() {
        geometry.bounds.as_mut().unwrap().min.x += x;
        geometry.bounds.as_mut().unwrap().min.y += y;
        geometry.bounds.as_mut().unwrap().min.z += z;
        geometry.bounds.as_mut().unwrap().max.x += x;
        geometry.bounds.as_mut().unwrap().max.y += y;
        geometry.bounds.as_mut().unwrap().max.z += z;
    }
}

// Source: upstream/packages/mesh/src/meshGeometryTransforms.ts:155 (sha256:ec9ab5ab82860c7777a7175a74eac584dfdda5b4ea792f0fc15231b619238482)
fn compute_matrix3x3_inverse_transpose(matrix: &Matrix4Like) -> Option<Vec<f32>> {
    let a00 = (matrix.m[0.0_f64 as usize] as f64);
    let a01 = (matrix.m[1.0_f64 as usize] as f64);
    let a02 = (matrix.m[2.0_f64 as usize] as f64);
    let a10 = (matrix.m[4.0_f64 as usize] as f64);
    let a11 = (matrix.m[5.0_f64 as usize] as f64);
    let a12 = (matrix.m[6.0_f64 as usize] as f64);
    let a20 = (matrix.m[8.0_f64 as usize] as f64);
    let a21 = (matrix.m[9.0_f64 as usize] as f64);
    let a22 = (matrix.m[10.0_f64 as usize] as f64);
    let c00 = ((a11 * a22) - (a12 * a21));
    let c01 = (-((a10 * a22) - (a12 * a20)));
    let c02 = ((a10 * a21) - (a11 * a20));
    let c10 = (-((a01 * a22) - (a02 * a21)));
    let c11 = ((a00 * a22) - (a02 * a20));
    let c12 = (-((a00 * a21) - (a01 * a20)));
    let c20 = ((a01 * a12) - (a02 * a11));
    let c21 = (-((a00 * a12) - (a02 * a10)));
    let c22 = ((a00 * a11) - (a01 * a10));
    let det = (((a00 * c00) + (a01 * c01)) + (a02 * c02));
    if ((det).abs() < 1e-10_f64) {
        return None;
    }
    let inv_det = (1.0_f64 / det);
    let mut out: Vec<f32> = vec![0.0_f32; (9.0_f64) as usize];
    out[0.0_f64 as usize] = (c00 * inv_det) as f32;
    out[1.0_f64 as usize] = (c10 * inv_det) as f32;
    out[2.0_f64 as usize] = (c20 * inv_det) as f32;
    out[3.0_f64 as usize] = (c01 * inv_det) as f32;
    out[4.0_f64 as usize] = (c11 * inv_det) as f32;
    out[5.0_f64 as usize] = (c21 * inv_det) as f32;
    out[6.0_f64 as usize] = (c02 * inv_det) as f32;
    out[7.0_f64 as usize] = (c12 * inv_det) as f32;
    out[8.0_f64 as usize] = (c22 * inv_det) as f32;
    return Some((out).clone());
}

// Source: upstream/packages/mesh/src/meshGeometryTransforms.ts:197 (sha256:45e531c366c71dc7783470b2ff62d865d7caa1c2d2d165724c83b923b31aa430)
fn transform_mesh_geometry_positions(
    out: &mut MeshGeometry,
    source: &MeshGeometry,
    sx: f64,
    sy: f64,
    sz: f64,
    tx: f64,
    ty: f64,
    tz: f64,
) -> () {
    let pos_float_offset = get_vertex_attribute_float_offset(&source.layout, "position".to_owned());
    let norm_float_offset = get_vertex_attribute_float_offset(&source.layout, "normal".to_owned());
    let tan_float_offset = get_vertex_attribute_float_offset(&source.layout, "tangent".to_owned());
    let floats_per_vertex = (source.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((source.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    if (out != source) {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = (source.vertices)
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            out.vertices[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    let inv_sx = if (sx != 0.0_f64) {
        (1.0_f64 / sx)
    } else {
        0.0_f64
    };
    let inv_sy = if (sy != 0.0_f64) {
        (1.0_f64 / sy)
    } else {
        0.0_f64
    };
    let inv_sz = if (sz != 0.0_f64) {
        (1.0_f64 / sz)
    } else {
        0.0_f64
    };
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let vert_base = (i * floats_per_vertex);
            if (pos_float_offset >= 0.0_f64) {
                let pb = (vert_base + pos_float_offset);
                let px = (source.vertices[pb as usize] as f64);
                let py = (source.vertices[(pb + 1.0_f64) as usize] as f64);
                let pz = (source.vertices[(pb + 2.0_f64) as usize] as f64);
                out.vertices[pb as usize] = ((px * sx) + tx) as f32;
                out.vertices[(pb + 1.0_f64) as usize] = ((py * sy) + ty) as f32;
                out.vertices[(pb + 2.0_f64) as usize] = ((pz * sz) + tz) as f32;
            }
            if (norm_float_offset >= 0.0_f64) {
                let nb = (vert_base + norm_float_offset);
                let nx = (source.vertices[nb as usize] as f64);
                let ny = (source.vertices[(nb + 1.0_f64) as usize] as f64);
                let nz = (source.vertices[(nb + 2.0_f64) as usize] as f64);
                let mut nnx = (nx * inv_sx);
                let mut nny = (ny * inv_sy);
                let mut nnz = (nz * inv_sz);
                let len = (((nnx * nnx) + (nny * nny)) + (nnz * nnz)).sqrt();
                if (len > 0.0_f64) {
                    nnx /= len;
                    nny /= len;
                    nnz /= len;
                }
                out.vertices[nb as usize] = (nnx) as f32;
                out.vertices[(nb + 1.0_f64) as usize] = (nny) as f32;
                out.vertices[(nb + 2.0_f64) as usize] = (nnz) as f32;
            }
            if (tan_float_offset >= 0.0_f64) {
                let tb = (vert_base + tan_float_offset);
                let ttx = (source.vertices[tb as usize] as f64);
                let tty = (source.vertices[(tb + 1.0_f64) as usize] as f64);
                let ttz = (source.vertices[(tb + 2.0_f64) as usize] as f64);
                let tw = (source.vertices[(tb + 3.0_f64) as usize] as f64);
                let mut ntx = (ttx * inv_sx);
                let mut nty = (tty * inv_sy);
                let mut ntz = (ttz * inv_sz);
                let len = (((ntx * ntx) + (nty * nty)) + (ntz * ntz)).sqrt();
                if (len > 0.0_f64) {
                    ntx /= len;
                    nty /= len;
                    ntz /= len;
                }
                out.vertices[tb as usize] = (ntx) as f32;
                out.vertices[(tb + 1.0_f64) as usize] = (nty) as f32;
                out.vertices[(tb + 2.0_f64) as usize] = (ntz) as f32;
                out.vertices[(tb + 3.0_f64) as usize] = ((tw).clone()) as f32;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    {
        out.version += 1.0;
        out.version
    };
    if ((out.bounds).clone()).is_some() {
        {
            let __flight_argument_1 = (out).clone();
            compute_mesh_geometry_bounds(&mut out.bounds, &__flight_argument_1)
        };
    }
}
