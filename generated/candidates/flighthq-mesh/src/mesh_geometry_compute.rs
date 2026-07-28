// @generated from upstream/packages/mesh/src/meshGeometryCompute.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{AabbLike, BoundingSphereLike, MeshGeometry};

// Source: upstream/packages/mesh/src/meshGeometryCompute.ts:15 (sha256:ae9de607f5186d216503ae38631567b31fec25e51f0648e9febb4b7fa2a572f4)
pub fn compute_mesh_geometry_bounding_sphere(
    out: &mut BoundingSphereLike,
    geometry: &MeshGeometry,
) -> () {
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    if (vertex_count == 0.0_f64) {
        out.center.x = 0.0_f64;
        out.center.y = 0.0_f64;
        out.center.z = 0.0_f64;
        out.radius = (-1.0_f64);
        return;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut max_z = f64::NEG_INFINITY;
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let base = ((i * floats_per_vertex) + POSITION_OFFSET);
            let px = (geometry.vertices[base as usize] as f64);
            let py = (geometry.vertices[(base + 1.0_f64) as usize] as f64);
            let pz = (geometry.vertices[(base + 2.0_f64) as usize] as f64);
            if (px < min_x) {
                min_x = ((px).clone()) as f32;
            }
            if (py < min_y) {
                min_y = ((py).clone()) as f32;
            }
            if (pz < min_z) {
                min_z = ((pz).clone()) as f32;
            }
            if (px > max_x) {
                max_x = ((px).clone()) as f32;
            }
            if (py > max_y) {
                max_y = ((py).clone()) as f32;
            }
            if (pz > max_z) {
                max_z = ((pz).clone()) as f32;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let cx = ((min_x + max_x) * 0.5_f64);
    let cy = ((min_y + max_y) * 0.5_f64);
    let cz = ((min_z + max_z) * 0.5_f64);
    let mut radius_sq = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let base = ((i * floats_per_vertex) + POSITION_OFFSET);
            let dx = ((geometry.vertices[base as usize] as f64) - cx);
            let dy = ((geometry.vertices[(base + 1.0_f64) as usize] as f64) - cy);
            let dz = ((geometry.vertices[(base + 2.0_f64) as usize] as f64) - cz);
            let d_sq = (((dx * dx) + (dy * dy)) + (dz * dz));
            if (d_sq > radius_sq) {
                radius_sq = d_sq;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    out.center.x = cx;
    out.center.y = cy;
    out.center.z = cz;
    out.radius = (radius_sq).sqrt();
}

// Source: upstream/packages/mesh/src/meshGeometryCompute.ts:72 (sha256:59020e49778fc676fed46ac5036950aea780e348b70a1dd5ff55760deed2fcb9)
pub fn compute_mesh_geometry_bounds(out: &mut AabbLike, geometry: &MeshGeometry) -> () {
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut max_z = f64::NEG_INFINITY;
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let base = ((i * floats_per_vertex) + POSITION_OFFSET);
            let px = (geometry.vertices[base as usize] as f64);
            let py = (geometry.vertices[(base + 1.0_f64) as usize] as f64);
            let pz = (geometry.vertices[(base + 2.0_f64) as usize] as f64);
            if (px < min_x) {
                min_x = ((px).clone()) as f32;
            }
            if (py < min_y) {
                min_y = ((py).clone()) as f32;
            }
            if (pz < min_z) {
                min_z = ((pz).clone()) as f32;
            }
            if (px > max_x) {
                max_x = ((px).clone()) as f32;
            }
            if (py > max_y) {
                max_y = ((py).clone()) as f32;
            }
            if (pz > max_z) {
                max_z = ((pz).clone()) as f32;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    out.min.x = min_x;
    out.min.y = min_y;
    out.min.z = min_z;
    out.max.x = max_x;
    out.max.y = max_y;
    out.max.z = max_z;
}

// Source: upstream/packages/mesh/src/meshGeometryCompute.ts:112 (sha256:5431551041d32dab31f0c31f2972c53b0ee24c47dbef1e266754dc538e46c866)
pub fn compute_mesh_geometry_flat_normals(out: &mut MeshGeometry, geometry: &MeshGeometry) -> () {
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let indices = (geometry.indices).clone();
    let index_count = if (indices).is_some() {
        (indices.as_ref().unwrap().len() as f64)
    } else {
        if (floats_per_vertex > 0.0_f64) {
            ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
        } else {
            0.0_f64
        }
    };
    {
        let mut t = 0.0_f64;
        while ((t + 2.0_f64) < index_count) {
            let i0 = if (indices).is_some() {
                (indices.as_ref().unwrap()[t as usize] as f64) as u32
            } else {
                (t) as u32
            };
            let i1 = if (indices).is_some() {
                (indices.as_ref().unwrap()[(t + 1.0_f64) as usize] as f64) as u32
            } else {
                (t + 1.0_f64) as u32
            };
            let i2 = if (indices).is_some() {
                (indices.as_ref().unwrap()[(t + 2.0_f64) as usize] as f64) as u32
            } else {
                (t + 2.0_f64) as u32
            };
            let p0 = ((i0 * floats_per_vertex) + POSITION_OFFSET);
            let p1 = ((i1 * floats_per_vertex) + POSITION_OFFSET);
            let p2 = ((i2 * floats_per_vertex) + POSITION_OFFSET);
            let x0 = (geometry.vertices[p0 as usize] as f64);
            let y0 = (geometry.vertices[(p0 + 1.0_f64) as usize] as f64);
            let z0 = (geometry.vertices[(p0 + 2.0_f64) as usize] as f64);
            let x1 = (geometry.vertices[p1 as usize] as f64);
            let y1 = (geometry.vertices[(p1 + 1.0_f64) as usize] as f64);
            let z1 = (geometry.vertices[(p1 + 2.0_f64) as usize] as f64);
            let x2 = (geometry.vertices[p2 as usize] as f64);
            let y2 = (geometry.vertices[(p2 + 1.0_f64) as usize] as f64);
            let z2 = (geometry.vertices[(p2 + 2.0_f64) as usize] as f64);
            let ex1 = (x1 - x0);
            let ey1 = (y1 - y0);
            let ez1 = (z1 - z0);
            let ex2 = (x2 - x0);
            let ey2 = (y2 - y0);
            let ez2 = (z2 - z0);
            let mut nx = ((ey1 * ez2) - (ez1 * ey2));
            let mut ny = ((ez1 * ex2) - (ex1 * ez2));
            let mut nz = ((ex1 * ey2) - (ey1 * ex2));
            let len = (((nx * nx) + (ny * ny)) + (nz * nz)).sqrt();
            if (len > 0.0_f64) {
                nx /= (len) as f32;
                ny /= (len) as f32;
                nz /= (len) as f32;
            }
            let n0 = ((i0 * floats_per_vertex) + NORMAL_OFFSET);
            let n1 = ((i1 * floats_per_vertex) + NORMAL_OFFSET);
            let n2 = ((i2 * floats_per_vertex) + NORMAL_OFFSET);
            out.vertices[n0 as usize] = ((nx).clone()) as f32;
            out.vertices[(n0 + 1.0_f64) as usize] = ((ny).clone()) as f32;
            out.vertices[(n0 + 2.0_f64) as usize] = ((nz).clone()) as f32;
            out.vertices[n1 as usize] = ((nx).clone()) as f32;
            out.vertices[(n1 + 1.0_f64) as usize] = ((ny).clone()) as f32;
            out.vertices[(n1 + 2.0_f64) as usize] = ((nz).clone()) as f32;
            out.vertices[n2 as usize] = ((nx).clone()) as f32;
            out.vertices[(n2 + 1.0_f64) as usize] = ((ny).clone()) as f32;
            out.vertices[(n2 + 2.0_f64) as usize] = ((nz).clone()) as f32;
            {
                t += 3.0_f64;
                t
            };
        }
    }
    {
        out.version += 1.0;
        out.version
    };
}

// Source: upstream/packages/mesh/src/meshGeometryCompute.ts:178 (sha256:35f7765161aefb8c401f740ca99303d2c67c96e63b5ce810aab3a63b9ba6acdc)
pub fn compute_mesh_geometry_normals(out: &mut MeshGeometry, geometry: &MeshGeometry) -> () {
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    let indices = (geometry.indices).clone();
    let index_count = if (indices).is_some() {
        (indices.as_ref().unwrap().len() as f64)
    } else {
        vertex_count
    };
    let mut accum = vec![0.0_f64; (vertex_count * 3.0_f64) as usize];
    {
        let mut t = 0.0_f64;
        while ((t + 2.0_f64) < index_count) {
            let i0 = if (indices).is_some() {
                (indices.as_ref().unwrap()[t as usize] as f64) as u32
            } else {
                (t) as u32
            };
            let i1 = if (indices).is_some() {
                (indices.as_ref().unwrap()[(t + 1.0_f64) as usize] as f64) as u32
            } else {
                (t + 1.0_f64) as u32
            };
            let i2 = if (indices).is_some() {
                (indices.as_ref().unwrap()[(t + 2.0_f64) as usize] as f64) as u32
            } else {
                (t + 2.0_f64) as u32
            };
            let b0 = ((i0 * floats_per_vertex) + POSITION_OFFSET);
            let b1 = ((i1 * floats_per_vertex) + POSITION_OFFSET);
            let b2 = ((i2 * floats_per_vertex) + POSITION_OFFSET);
            let e1x =
                ((geometry.vertices[b1 as usize] as f64) - (geometry.vertices[b0 as usize] as f64));
            let e1y = ((geometry.vertices[(b1 + 1.0_f64) as usize] as f64)
                - (geometry.vertices[(b0 + 1.0_f64) as usize] as f64));
            let e1z = ((geometry.vertices[(b1 + 2.0_f64) as usize] as f64)
                - (geometry.vertices[(b0 + 2.0_f64) as usize] as f64));
            let e2x =
                ((geometry.vertices[b2 as usize] as f64) - (geometry.vertices[b0 as usize] as f64));
            let e2y = ((geometry.vertices[(b2 + 1.0_f64) as usize] as f64)
                - (geometry.vertices[(b0 + 1.0_f64) as usize] as f64));
            let e2z = ((geometry.vertices[(b2 + 2.0_f64) as usize] as f64)
                - (geometry.vertices[(b0 + 2.0_f64) as usize] as f64));
            let mut nx = ((e1y * e2z) - (e1z * e2y));
            let mut ny = ((e1z * e2x) - (e1x * e2z));
            let mut nz = ((e1x * e2y) - (e1y * e2x));
            accum[(i0 * 3.0_f64) as usize] += nx;
            accum[((i0 * 3.0_f64) + 1.0_f64) as usize] += ny;
            accum[((i0 * 3.0_f64) + 2.0_f64) as usize] += nz;
            accum[(i1 * 3.0_f64) as usize] += nx;
            accum[((i1 * 3.0_f64) + 1.0_f64) as usize] += ny;
            accum[((i1 * 3.0_f64) + 2.0_f64) as usize] += nz;
            accum[(i2 * 3.0_f64) as usize] += nx;
            accum[((i2 * 3.0_f64) + 1.0_f64) as usize] += ny;
            accum[((i2 * 3.0_f64) + 2.0_f64) as usize] += nz;
            {
                t += 3.0_f64;
                t
            };
        }
    }
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let mut nx = (accum[(i * 3.0_f64) as usize] as f64);
            let mut ny = (accum[((i * 3.0_f64) + 1.0_f64) as usize] as f64);
            let mut nz = (accum[((i * 3.0_f64) + 2.0_f64) as usize] as f64);
            let len = (((nx * nx) + (ny * ny)) + (nz * nz)).sqrt();
            if (len > 0.0_f64) {
                nx /= len;
                ny /= len;
                nz /= len;
            }
            let base = ((i * floats_per_vertex) + NORMAL_OFFSET);
            out.vertices[base as usize] = (nx) as f32;
            out.vertices[(base + 1.0_f64) as usize] = (ny) as f32;
            out.vertices[(base + 2.0_f64) as usize] = (nz) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/mesh/src/meshGeometryCompute.ts:243 (sha256:509e9db11b3fc6d4f85fb6b2f24500a27f39a938cc19eaf9c06a8f1c64d22029)
pub fn compute_mesh_geometry_tangents(out: &mut MeshGeometry, geometry: &MeshGeometry) -> () {
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        ((geometry.vertices.len() as f64) / floats_per_vertex).floor()
    } else {
        0.0_f64
    };
    let indices = (geometry.indices).clone();
    let index_count = if (indices).is_some() {
        (indices.as_ref().unwrap().len() as f64)
    } else {
        vertex_count
    };
    let mut tan = vec![0.0_f64; (vertex_count * 3.0_f64) as usize];
    let mut bitan = vec![0.0_f64; (vertex_count * 3.0_f64) as usize];
    {
        let mut t = 0.0_f64;
        while ((t + 2.0_f64) < index_count) {
            let i0 = if (indices).is_some() {
                (indices.as_ref().unwrap()[t as usize] as f64) as u32
            } else {
                (t) as u32
            };
            let i1 = if (indices).is_some() {
                (indices.as_ref().unwrap()[(t + 1.0_f64) as usize] as f64) as u32
            } else {
                (t + 1.0_f64) as u32
            };
            let i2 = if (indices).is_some() {
                (indices.as_ref().unwrap()[(t + 2.0_f64) as usize] as f64) as u32
            } else {
                (t + 2.0_f64) as u32
            };
            let p0 = ((i0 * floats_per_vertex) + POSITION_OFFSET);
            let p1 = ((i1 * floats_per_vertex) + POSITION_OFFSET);
            let p2 = ((i2 * floats_per_vertex) + POSITION_OFFSET);
            let e1x =
                ((geometry.vertices[p1 as usize] as f64) - (geometry.vertices[p0 as usize] as f64));
            let e1y = ((geometry.vertices[(p1 + 1.0_f64) as usize] as f64)
                - (geometry.vertices[(p0 + 1.0_f64) as usize] as f64));
            let e1z = ((geometry.vertices[(p1 + 2.0_f64) as usize] as f64)
                - (geometry.vertices[(p0 + 2.0_f64) as usize] as f64));
            let e2x =
                ((geometry.vertices[p2 as usize] as f64) - (geometry.vertices[p0 as usize] as f64));
            let e2y = ((geometry.vertices[(p2 + 1.0_f64) as usize] as f64)
                - (geometry.vertices[(p0 + 1.0_f64) as usize] as f64));
            let e2z = ((geometry.vertices[(p2 + 2.0_f64) as usize] as f64)
                - (geometry.vertices[(p0 + 2.0_f64) as usize] as f64));
            let u0 = ((i0 * floats_per_vertex) + UV0_OFFSET);
            let u1 = ((i1 * floats_per_vertex) + UV0_OFFSET);
            let u2 = ((i2 * floats_per_vertex) + UV0_OFFSET);
            let du1 =
                ((geometry.vertices[u1 as usize] as f64) - (geometry.vertices[u0 as usize] as f64));
            let dv1 = ((geometry.vertices[(u1 + 1.0_f64) as usize] as f64)
                - (geometry.vertices[(u0 + 1.0_f64) as usize] as f64));
            let du2 =
                ((geometry.vertices[u2 as usize] as f64) - (geometry.vertices[u0 as usize] as f64));
            let dv2 = ((geometry.vertices[(u2 + 1.0_f64) as usize] as f64)
                - (geometry.vertices[(u0 + 1.0_f64) as usize] as f64));
            let det = ((du1 * dv2) - (du2 * dv1));
            let r = if (det != 0.0_f64) {
                (1.0_f64 / det)
            } else {
                0.0_f64
            };
            let mut tx = (((dv2 * e1x) - (dv1 * e2x)) * r);
            let mut ty = (((dv2 * e1y) - (dv1 * e2y)) * r);
            let mut tz = (((dv2 * e1z) - (dv1 * e2z)) * r);
            let bx = (((du1 * e2x) - (du2 * e1x)) * r);
            let by = (((du1 * e2y) - (du2 * e1y)) * r);
            let bz = (((du1 * e2z) - (du2 * e1z)) * r);
            tan[(i0 * 3.0_f64) as usize] += tx;
            tan[((i0 * 3.0_f64) + 1.0_f64) as usize] += ty;
            tan[((i0 * 3.0_f64) + 2.0_f64) as usize] += tz;
            tan[(i1 * 3.0_f64) as usize] += tx;
            tan[((i1 * 3.0_f64) + 1.0_f64) as usize] += ty;
            tan[((i1 * 3.0_f64) + 2.0_f64) as usize] += tz;
            tan[(i2 * 3.0_f64) as usize] += tx;
            tan[((i2 * 3.0_f64) + 1.0_f64) as usize] += ty;
            tan[((i2 * 3.0_f64) + 2.0_f64) as usize] += tz;
            bitan[(i0 * 3.0_f64) as usize] += bx;
            bitan[((i0 * 3.0_f64) + 1.0_f64) as usize] += by;
            bitan[((i0 * 3.0_f64) + 2.0_f64) as usize] += bz;
            bitan[(i1 * 3.0_f64) as usize] += bx;
            bitan[((i1 * 3.0_f64) + 1.0_f64) as usize] += by;
            bitan[((i1 * 3.0_f64) + 2.0_f64) as usize] += bz;
            bitan[(i2 * 3.0_f64) as usize] += bx;
            bitan[((i2 * 3.0_f64) + 1.0_f64) as usize] += by;
            bitan[((i2 * 3.0_f64) + 2.0_f64) as usize] += bz;
            {
                t += 3.0_f64;
                t
            };
        }
    }
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let n_base = ((i * floats_per_vertex) + NORMAL_OFFSET);
            let nx = (geometry.vertices[n_base as usize] as f64);
            let ny = (geometry.vertices[(n_base + 1.0_f64) as usize] as f64);
            let nz = (geometry.vertices[(n_base + 2.0_f64) as usize] as f64);
            let mut tx = (tan[(i * 3.0_f64) as usize] as f64);
            let mut ty = (tan[((i * 3.0_f64) + 1.0_f64) as usize] as f64);
            let mut tz = (tan[((i * 3.0_f64) + 2.0_f64) as usize] as f64);
            let ndt = (((nx * tx) + (ny * ty)) + (nz * tz));
            tx -= (nx * ndt);
            ty -= (ny * ndt);
            tz -= (nz * ndt);
            let len = (((tx * tx) + (ty * ty)) + (tz * tz)).sqrt();
            if (len > 0.0_f64) {
                tx /= len;
                ty /= len;
                tz /= len;
            } else {
                tx = 1.0_f64;
                ty = 0.0_f64;
                tz = 0.0_f64;
            }
            let cx = ((ny * tz) - (nz * ty));
            let cy = ((nz * tx) - (nx * tz));
            let cz = ((nx * ty) - (ny * tx));
            let w = if ((((cx * (bitan[(i * 3.0_f64) as usize] as f64))
                + (cy * (bitan[((i * 3.0_f64) + 1.0_f64) as usize] as f64)))
                + (cz * (bitan[((i * 3.0_f64) + 2.0_f64) as usize] as f64)))
                < 0.0_f64)
            {
                (-1.0_f64)
            } else {
                1.0_f64
            };
            let base = ((i * floats_per_vertex) + TANGENT_OFFSET);
            out.vertices[base as usize] = (tx) as f32;
            out.vertices[(base + 1.0_f64) as usize] = (ty) as f32;
            out.vertices[(base + 2.0_f64) as usize] = (tz) as f32;
            out.vertices[(base + 3.0_f64) as usize] = (w) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/mesh/src/meshGeometryCompute.ts:353 (sha256:c65c3674b9cfd896c8f0b4c257a4aa245cb36ef97d5cd287ded100da6074b5fe)
const NORMAL_OFFSET: f64 = 3.0_f64;

// Source: upstream/packages/mesh/src/meshGeometryCompute.ts:354 (sha256:fd507a202e2581bdf67780dc01b3bbbdcf0dc54382dd9d01b552c0a6150dcb8a)
const POSITION_OFFSET: f64 = 0.0_f64;

// Source: upstream/packages/mesh/src/meshGeometryCompute.ts:355 (sha256:157d8ed88f6434e30631cdc4bf8e4ff1965c2c7dd1b2108d6b536ef0d36d3735)
const TANGENT_OFFSET: f64 = 6.0_f64;

// Source: upstream/packages/mesh/src/meshGeometryCompute.ts:356 (sha256:f5a03d89c1d602c6741bffd8e83b18496e28fe78ff0d3e6f8abdc4a0fe8901b7)
const UV0_OFFSET: f64 = 10.0_f64;
