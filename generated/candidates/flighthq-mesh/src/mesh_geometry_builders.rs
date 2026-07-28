// @generated from upstream/packages/mesh/src/meshGeometryBuilders.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    MeshGeometryOptions, compute_mesh_geometry_bounds, compute_mesh_geometry_tangents,
    create_mesh_geometry,
};
use flighthq_geometry::create_aabb;
use flighthq_types::{MeshGeometry, VertexAttribute, VertexAttributeLayout};

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:17 (sha256:7862234fdca538301c1161f506264cbdaee9e487fa0941e354fc8752d6ef21f3)
pub fn create_box_mesh_geometry(
    width: Option<f64>,
    height: Option<f64>,
    depth: Option<f64>,
) -> MeshGeometry {
    let width = width.unwrap_or(1.0_f64);
    let height = height.unwrap_or(1.0_f64);
    let depth = depth.unwrap_or(1.0_f64);
    let hx = (width * 0.5_f64);
    let hy = (height * 0.5_f64);
    let hz = (depth * 0.5_f64);
    let positions: std::sync::Arc<std::sync::Mutex<Vec<f64>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let normals: std::sync::Arc<std::sync::Mutex<Vec<f64>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let uvs: std::sync::Arc<std::sync::Mutex<Vec<f64>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let indices: std::sync::Arc<std::sync::Mutex<Vec<f64>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let mut add_face: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) -> ()
                    + Send
                    + 'static,
            >,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut indices = indices.clone();
        let mut normals = normals.clone();
        let mut positions = positions.clone();
        let mut uvs = uvs.clone();
        move |ox: f64,
              oy: f64,
              oz: f64,
              ux: f64,
              uy: f64,
              uz: f64,
              vx: f64,
              vy: f64,
              vz: f64,
              nx: f64,
              ny: f64,
              nz: f64|
              -> () {
            let start = (((*positions.lock().unwrap()).len() as f64) / 3.0_f64);
            {
                let mut iv = 0.0_f64;
                while (iv < 2.0_f64) {
                    {
                        let mut iu = 0.0_f64;
                        while (iu < 2.0_f64) {
                            (*positions.lock().unwrap()).extend(vec![
                                ((ox + (ux * iu)) + (vx * iv)),
                                ((oy + (uy * iu)) + (vy * iv)),
                                ((oz + (uz * iu)) + (vz * iv)),
                            ]);
                            (*normals.lock().unwrap()).extend(vec![nx, ny, nz]);
                            (*uvs.lock().unwrap()).extend(vec![iu, iv]);
                            {
                                iu += 1.0;
                                iu
                            };
                        }
                    }
                    {
                        iv += 1.0;
                        iv
                    };
                }
            }
            (*indices.lock().unwrap()).extend(vec![
                start,
                (start + 1.0_f64),
                (start + 3.0_f64),
                start,
                (start + 3.0_f64),
                (start + 2.0_f64),
            ]);
        }
    })
        as Box<
            dyn FnMut(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) -> ()
                + Send
                + 'static,
        >));
    {
        let __flight_callback = (add_face).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            hx,
            (-hy),
            hz,
            0.0_f64,
            0.0_f64,
            (-depth),
            0.0_f64,
            height,
            0.0_f64,
            1.0_f64,
            0.0_f64,
            0.0_f64,
        );
        __flight_result
    };
    {
        let __flight_callback = (add_face).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (-hx),
            (-hy),
            (-hz),
            0.0_f64,
            0.0_f64,
            depth,
            0.0_f64,
            height,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
            0.0_f64,
        );
        __flight_result
    };
    {
        let __flight_callback = (add_face).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (-hx),
            hy,
            hz,
            width,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            (-depth),
            0.0_f64,
            1.0_f64,
            0.0_f64,
        );
        __flight_result
    };
    {
        let __flight_callback = (add_face).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (-hx),
            (-hy),
            (-hz),
            width,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            depth,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
        );
        __flight_result
    };
    {
        let __flight_callback = (add_face).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (-hx),
            (-hy),
            hz,
            width,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            height,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            1.0_f64,
        );
        __flight_result
    };
    {
        let __flight_callback = (add_face).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            hx,
            (-hy),
            (-hz),
            (-width),
            0.0_f64,
            0.0_f64,
            0.0_f64,
            height,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
        );
        __flight_result
    };
    return build_canonical_mesh_geometry(
        &(*positions.lock().unwrap()),
        &(*normals.lock().unwrap()),
        &(*uvs.lock().unwrap()),
        &(*indices.lock().unwrap()),
    );
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:68 (sha256:2984d3b06a74bbb0aa2eb74c68fa5dce1384aef9379d8152269fea67b9516658)
pub fn create_capsule_mesh_geometry(
    radius: Option<f64>,
    height: Option<f64>,
    radial_segments: Option<f64>,
    cap_segments: Option<f64>,
) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let height = height.unwrap_or(1.0_f64);
    let radial_segments = radial_segments.unwrap_or(16.0_f64);
    let cap_segments = cap_segments.unwrap_or(8.0_f64);
    let r_seg = (3.0_f64).max(radial_segments);
    let c_seg = (1.0_f64).max(cap_segments);
    let half_h = (height * 0.5_f64);
    let positions: std::sync::Arc<std::sync::Mutex<Vec<f64>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let normals: std::sync::Arc<std::sync::Mutex<Vec<f64>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let uvs: std::sync::Arc<std::sync::Mutex<Vec<f64>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let mut indices: Vec<f64> = vec![];
    let mut add_ring: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut normals = normals.clone();
        let mut positions = positions.clone();
        let mut uvs = uvs.clone();
        move |phi: f64, y_offset: f64| -> () {
            let sin_phi = (phi).sin();
            let cos_phi = (phi).cos();
            {
                let mut i = 0.0_f64;
                while (i <= r_seg) {
                    let theta = (((i / r_seg) * std::f64::consts::PI) * 2.0_f64);
                    let cos_theta = (theta).cos();
                    let sin_theta = (theta).sin();
                    let nx = (sin_phi * cos_theta);
                    let ny = cos_phi;
                    let nz = (sin_phi * sin_theta);
                    (*positions.lock().unwrap()).extend(vec![
                        (radius * nx),
                        ((radius * ny) + y_offset),
                        (radius * nz),
                    ]);
                    (*normals.lock().unwrap()).extend(vec![nx, ny, nz]);
                    (*uvs.lock().unwrap()).extend(vec![(i / r_seg), 0.0_f64]);
                    {
                        i += 1.0;
                        i
                    };
                }
            }
        }
    })
        as Box<dyn FnMut(f64, f64) -> () + Send + 'static>));
    let ring_vertex_count = (r_seg + 1.0_f64);
    let v_divisor = ((2.0_f64 * c_seg) + 1.0_f64);
    {
        let mut j = 0.0_f64;
        while (j <= c_seg) {
            {
                let __flight_callback = (add_ring).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    ((j / c_seg) * (std::f64::consts::PI * 0.5_f64)),
                    half_h,
                );
                __flight_result
            };
            {
                j += 1.0;
                j
            };
        }
    }
    {
        let mut j = 1.0_f64;
        while (j <= c_seg) {
            {
                let __flight_callback = (add_ring).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    ((std::f64::consts::PI * 0.5_f64)
                        + ((j / c_seg) * (std::f64::consts::PI * 0.5_f64))),
                    (-half_h),
                );
                __flight_result
            };
            {
                j += 1.0;
                j
            };
        }
    }
    let ring_count = ((2.0_f64 * c_seg) + 1.0_f64);
    {
        let mut j = 0.0_f64;
        while (j < ring_count) {
            let v = (j / v_divisor);
            {
                let mut i = 0.0_f64;
                while (i <= r_seg) {
                    {
                        let __flight_index =
                            ((((j * ring_vertex_count) + i) * 2.0_f64) + 1.0_f64) as usize;
                        let __flight_value = v;
                        if __flight_index == (*uvs.lock().unwrap()).len() {
                            (*uvs.lock().unwrap()).push(__flight_value);
                        } else {
                            (*uvs.lock().unwrap())[__flight_index] = __flight_value;
                        }
                    };
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            {
                j += 1.0;
                j
            };
        }
    }
    let total_rings = ((2.0_f64 * c_seg) + 1.0_f64);
    {
        let mut j = 0.0_f64;
        while (j < (total_rings - 1.0_f64)) {
            {
                let mut i = 0.0_f64;
                while (i < r_seg) {
                    let a = ((j * ring_vertex_count) + i);
                    let b = (a + 1.0_f64);
                    let c = (a + ring_vertex_count);
                    let d = (c + 1.0_f64);
                    indices.extend(vec![a, c, b, b, c, d]);
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            {
                j += 1.0;
                j
            };
        }
    }
    return build_canonical_mesh_geometry(
        &(*positions.lock().unwrap()),
        &(*normals.lock().unwrap()),
        &(*uvs.lock().unwrap()),
        &indices,
    );
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:140 (sha256:c0f757025bc30247c8731dc1bd46530f0ef972c408666691146af254a796ed8c)
pub fn create_circle_mesh_geometry(radius: Option<f64>, segments: Option<f64>) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let segments = segments.unwrap_or(32.0_f64);
    let segs = (3.0_f64).max(segments);
    let mut positions: Vec<f64> = vec![];
    let mut normals: Vec<f64> = vec![];
    let mut uvs: Vec<f64> = vec![];
    let mut indices: Vec<f64> = vec![];
    add_disc(
        &mut positions,
        &mut normals,
        &mut uvs,
        &mut indices,
        segs,
        radius,
        0.0_f64,
        1.0_f64,
    );
    return build_canonical_mesh_geometry(&positions, &normals, &uvs, &indices);
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:153 (sha256:b71c879175678eb5f58a1a63dac741c204b593576265eb45930dbd57b3a4929d)
pub fn create_cone_mesh_geometry(
    radius: Option<f64>,
    height: Option<f64>,
    radial_segments: Option<f64>,
    capped: Option<bool>,
) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let height = height.unwrap_or(1.0_f64);
    let radial_segments = radial_segments.unwrap_or(32.0_f64);
    let capped = capped.unwrap_or(true);
    return create_cylinder_mesh_geometry(
        Some(0.0_f64),
        Some(radius),
        Some(height),
        Some(radial_segments),
        Some(capped),
    );
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:166 (sha256:873a76e1e42c57335b7e7077b549d59df8dd3530a6aaa57980363200e1babc56)
pub fn create_cylinder_mesh_geometry(
    top_radius: Option<f64>,
    bottom_radius: Option<f64>,
    height: Option<f64>,
    radial_segments: Option<f64>,
    capped: Option<bool>,
) -> MeshGeometry {
    let top_radius = top_radius.unwrap_or(0.5_f64);
    let bottom_radius = bottom_radius.unwrap_or(0.5_f64);
    let height = height.unwrap_or(1.0_f64);
    let radial_segments = radial_segments.unwrap_or(32.0_f64);
    let capped = capped.unwrap_or(true);
    let segments = (3.0_f64).max(radial_segments);
    let half_height = (height * 0.5_f64);
    let mut positions: Vec<f64> = vec![];
    let mut normals: Vec<f64> = vec![];
    let mut uvs: Vec<f64> = vec![];
    let mut indices: Vec<f64> = vec![];
    let mut slope = ((bottom_radius - top_radius) / height);
    let side_start = ((positions.len() as f64) / 3.0_f64);
    {
        let mut y = 0.0_f64;
        while (y <= 1.0_f64) {
            let radius = if (y == 0.0_f64) {
                bottom_radius
            } else {
                top_radius
            };
            let py = if (y == 0.0_f64) {
                (-half_height)
            } else {
                half_height
            };
            {
                let mut s = 0.0_f64;
                while (s <= segments) {
                    let theta = (((s / segments) * std::f64::consts::PI) * 2.0_f64);
                    let mut cos = (theta).cos();
                    let mut sin = (theta).sin();
                    positions.extend(vec![(radius * cos), py, (radius * sin)]);
                    let mut nx = cos;
                    let mut ny = slope;
                    let mut nz = sin;
                    let len = if ((((nx * nx) + (ny * ny)) + (nz * nz)).sqrt()) != 0.0_f64 {
                        (((nx * nx) + (ny * ny)) + (nz * nz)).sqrt()
                    } else {
                        1.0_f64
                    };
                    nx /= len;
                    ny /= len;
                    nz /= len;
                    normals.extend(vec![nx, ny, nz]);
                    uvs.extend(vec![(s / segments), y]);
                    {
                        s += 1.0;
                        s
                    };
                }
            }
            {
                y += 1.0;
                y
            };
        }
    }
    {
        let mut s = 0.0_f64;
        while (s < segments) {
            let a = (side_start + s);
            let b = ((side_start + s) + 1.0_f64);
            let c = ((side_start + (segments + 1.0_f64)) + s);
            let d = (((side_start + (segments + 1.0_f64)) + s) + 1.0_f64);
            indices.extend(vec![a, c, b, b, c, d]);
            {
                s += 1.0;
                s
            };
        }
    }
    if capped {
        if (bottom_radius > 0.0_f64) {
            add_disc(
                &mut positions,
                &mut normals,
                &mut uvs,
                &mut indices,
                segments,
                bottom_radius,
                (-half_height),
                (-1.0_f64),
            );
        }
        if (top_radius > 0.0_f64) {
            add_disc(
                &mut positions,
                &mut normals,
                &mut uvs,
                &mut indices,
                segments,
                top_radius,
                half_height,
                1.0_f64,
            );
        }
    }
    return build_canonical_mesh_geometry(&positions, &normals, &uvs, &indices);
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:224 (sha256:fca684bb0be8c2e6143f1958683b21bd0505f06ebc6a4f8d253776e89479e5ff)
pub fn create_dodecahedron_mesh_geometry(radius: Option<f64>, detail: Option<f64>) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let detail = detail.unwrap_or(0.0_f64);
    return create_polyhedron_mesh_geometry(
        &mut DODECAHEDRON_VERTS,
        &mut DODECAHEDRON_FACES,
        Some(radius),
        Some(detail),
    );
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:230 (sha256:48c9a5be6005e03be608f0a92b7a3c5fa28f8cd45de69ac85b2d7a190b15dbde)
pub fn create_icosahedron_mesh_geometry(radius: Option<f64>, detail: Option<f64>) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let detail = detail.unwrap_or(0.0_f64);
    return create_polyhedron_mesh_geometry(
        &mut ICOSAHEDRON_VERTS,
        &mut ICOSAHEDRON_FACES,
        Some(radius),
        Some(detail),
    );
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:239 (sha256:df66bc143f40e29273a6127f214816e92184bf35698dcddaf1e40e9fd2300031)
pub fn create_icosphere_mesh_geometry(
    radius: Option<f64>,
    subdivisions: Option<f64>,
) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let subdivisions = subdivisions.unwrap_or(2.0_f64);
    let subs = (0.0_f64).max((subdivisions).min(6.0_f64));
    let phi = ((1.0_f64 + (5.0_f64).sqrt()) * 0.5_f64);
    let scale = (1.0_f64 / (1.0_f64 + (phi * phi)).sqrt());
    let mut base_verts = (vec![
        vec![(-1.0_f64), phi, 0.0_f64],
        vec![1.0_f64, phi, 0.0_f64],
        vec![(-1.0_f64), (-phi), 0.0_f64],
        vec![1.0_f64, (-phi), 0.0_f64],
        vec![0.0_f64, (-1.0_f64), phi],
        vec![0.0_f64, 1.0_f64, phi],
        vec![0.0_f64, (-1.0_f64), (-phi)],
        vec![0.0_f64, 1.0_f64, (-phi)],
        vec![phi, 0.0_f64, (-1.0_f64)],
        vec![phi, 0.0_f64, 1.0_f64],
        vec![(-phi), 0.0_f64, (-1.0_f64)],
        vec![(-phi), 0.0_f64, 1.0_f64],
    ])
    .iter()
    .cloned()
    .map(|__parameter0: Vec<f64>| -> Vec<f64> {
        let x = __parameter0[0.0_f64 as usize].clone();
        let y = __parameter0[1.0_f64 as usize].clone();
        let z = __parameter0[2.0_f64 as usize].clone();
        return vec![(x * scale), (y * scale), (z * scale)];
    })
    .collect();
    let verts: std::sync::Arc<std::sync::Mutex<Vec<Vec<f64>>>> =
        std::sync::Arc::new(std::sync::Mutex::new(
            (base_verts)
                .iter()
                .cloned()
                .map(|v: Vec<f64>| -> Vec<f64> {
                    {
                        let mut __flight_array = Vec::new();
                        __flight_array.extend((v).iter().cloned());
                        __flight_array
                    }
                })
                .collect(),
        ));
    let mut faces: Vec<Vec<f64>> = vec![
        vec![0.0_f64, 11.0_f64, 5.0_f64],
        vec![0.0_f64, 5.0_f64, 1.0_f64],
        vec![0.0_f64, 1.0_f64, 7.0_f64],
        vec![0.0_f64, 7.0_f64, 10.0_f64],
        vec![0.0_f64, 10.0_f64, 11.0_f64],
        vec![1.0_f64, 5.0_f64, 9.0_f64],
        vec![5.0_f64, 11.0_f64, 4.0_f64],
        vec![11.0_f64, 10.0_f64, 2.0_f64],
        vec![10.0_f64, 7.0_f64, 6.0_f64],
        vec![7.0_f64, 1.0_f64, 8.0_f64],
        vec![3.0_f64, 9.0_f64, 4.0_f64],
        vec![3.0_f64, 4.0_f64, 2.0_f64],
        vec![3.0_f64, 2.0_f64, 6.0_f64],
        vec![3.0_f64, 6.0_f64, 8.0_f64],
        vec![3.0_f64, 8.0_f64, 9.0_f64],
        vec![4.0_f64, 9.0_f64, 5.0_f64],
        vec![2.0_f64, 4.0_f64, 11.0_f64],
        vec![6.0_f64, 2.0_f64, 10.0_f64],
        vec![8.0_f64, 6.0_f64, 7.0_f64],
        vec![9.0_f64, 8.0_f64, 1.0_f64],
    ];
    let midpoint_cache: std::sync::Arc<std::sync::Mutex<Vec<(String, f64)>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut get_midpoint: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, f64) -> f64 + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut midpoint_cache = midpoint_cache.clone();
        let mut verts = verts.clone();
        move |a: f64, b: f64| -> f64 {
            let key = if (a < b) {
                format!("{}_{}", a, b)
            } else {
                format!("{}_{}", b, a)
            };
            let cached = (*midpoint_cache.lock().unwrap())
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value.clone());
            if (cached).is_some() {
                return *(cached.as_ref().unwrap());
            }
            let va = (*verts.lock().unwrap())[a as usize].clone();
            let vb = (*verts.lock().unwrap())[b as usize].clone();
            let mut mx = ((va[0.0_f64 as usize].clone() + vb[0.0_f64 as usize].clone()) * 0.5_f64);
            let mut my = ((va[1.0_f64 as usize].clone() + vb[1.0_f64 as usize].clone()) * 0.5_f64);
            let mut mz = ((va[2.0_f64 as usize].clone() + vb[2.0_f64 as usize].clone()) * 0.5_f64);
            let len = (((mx * mx) + (my * my)) + (mz * mz)).sqrt();
            mx /= len;
            my /= len;
            mz /= len;
            let idx = ((*verts.lock().unwrap()).len() as f64);
            (*verts.lock().unwrap()).push(vec![mx, my, mz]);
            {
                let __flight_key = key;
                let __flight_value = idx;
                if let Some((_, value)) = (*midpoint_cache.lock().unwrap())
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    (*midpoint_cache.lock().unwrap()).push((__flight_key, __flight_value));
                }
            };
            return idx;
        }
    })
        as Box<dyn FnMut(f64, f64) -> f64 + Send + 'static>));
    {
        let mut s = 0.0_f64;
        while (s < subs) {
            let mut new_faces: Vec<Vec<f64>> = vec![];
            for __iteration1 in (faces).iter().cloned() {
                let a = __iteration1[0.0_f64 as usize].clone();
                let b = __iteration1[1.0_f64 as usize].clone();
                let c = __iteration1[2.0_f64 as usize].clone();
                let ab = {
                    let __flight_callback = (get_midpoint).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(a, b);
                    __flight_result
                };
                let bc = {
                    let __flight_callback = (get_midpoint).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(b, c);
                    __flight_result
                };
                let ca = {
                    let __flight_callback = (get_midpoint).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(c, a);
                    __flight_result
                };
                new_faces.extend(vec![
                    vec![a, ab, ca],
                    vec![b, bc, ab],
                    vec![c, ca, bc],
                    vec![ab, bc, ca],
                ]);
            }
            faces = (new_faces).clone();
            (*midpoint_cache.lock().unwrap()).clear();
            {
                s += 1.0;
                s
            };
        }
    }
    let mut positions: Vec<f64> = vec![];
    let mut normals: Vec<f64> = vec![];
    let mut uvs: Vec<f64> = vec![];
    let mut face_indices: Vec<f64> = vec![];
    for __iteration2 in (faces).iter().cloned() {
        let a = __iteration2[0.0_f64 as usize].clone();
        let b = __iteration2[1.0_f64 as usize].clone();
        let c = __iteration2[2.0_f64 as usize].clone();
        for vi in (vec![a, b, c]).iter().cloned() {
            let v = (*verts.lock().unwrap())[vi as usize].clone();
            let nx = v[0.0_f64 as usize].clone();
            let ny = v[1.0_f64 as usize].clone();
            let nz = v[2.0_f64 as usize].clone();
            positions.extend(vec![(radius * nx), (radius * ny), (radius * nz)]);
            normals.extend(vec![nx, ny, nz]);
            let u = (0.5_f64 + ((nz).atan2(nx) / (std::f64::consts::PI * 2.0_f64)));
            let sv =
                (0.5_f64 - (((-1.0_f64).max((1.0_f64).min(ny))).asin() / std::f64::consts::PI));
            uvs.extend(vec![u, sv]);
        }
    }
    {
        let mut i = 0.0_f64;
        while (i < ((positions.len() as f64) / 3.0_f64)) {
            face_indices.push(i);
            {
                i += 1.0;
                i
            };
        }
    }
    return build_canonical_mesh_geometry(&positions, &normals, &uvs, &face_indices);
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:349 (sha256:a3d58970bd77aa69384620bbac2a06501bc9c75f03b6c33306daeb3331d789ee)
pub fn create_octahedron_mesh_geometry(radius: Option<f64>, detail: Option<f64>) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let detail = detail.unwrap_or(0.0_f64);
    return create_polyhedron_mesh_geometry(
        &mut OCTAHEDRON_VERTS,
        &mut OCTAHEDRON_FACES,
        Some(radius),
        Some(detail),
    );
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:356 (sha256:47b362c1e4b296409cec60fc659fdb3a31ce652893d416237f6d246b232e9faf)
pub fn create_plane_mesh_geometry(
    width: Option<f64>,
    depth: Option<f64>,
    width_segments: Option<f64>,
    depth_segments: Option<f64>,
) -> MeshGeometry {
    let width = width.unwrap_or(1.0_f64);
    let depth = depth.unwrap_or(1.0_f64);
    let width_segments = width_segments.unwrap_or(1.0_f64);
    let depth_segments = depth_segments.unwrap_or(1.0_f64);
    let w_seg = (1.0_f64).max(width_segments);
    let d_seg = (1.0_f64).max(depth_segments);
    let hw = (width * 0.5_f64);
    let hd = (depth * 0.5_f64);
    let mut positions: Vec<f64> = vec![];
    let mut normals: Vec<f64> = vec![];
    let mut uvs: Vec<f64> = vec![];
    let mut indices: Vec<f64> = vec![];
    {
        let mut iz = 0.0_f64;
        while (iz <= d_seg) {
            let v = (iz / d_seg);
            let z = ((-hd) + (v * depth));
            {
                let mut ix = 0.0_f64;
                while (ix <= w_seg) {
                    let u = (ix / w_seg);
                    let x = ((-hw) + (u * width));
                    positions.extend(vec![x, 0.0_f64, z]);
                    normals.extend(vec![0.0_f64, 1.0_f64, 0.0_f64]);
                    uvs.extend(vec![u, v]);
                    {
                        ix += 1.0;
                        ix
                    };
                }
            }
            {
                iz += 1.0;
                iz
            };
        }
    }
    let row_stride = (w_seg + 1.0_f64);
    {
        let mut iz = 0.0_f64;
        while (iz < d_seg) {
            {
                let mut ix = 0.0_f64;
                while (ix < w_seg) {
                    let a = ((iz * row_stride) + ix);
                    let b = (a + 1.0_f64);
                    let c = (a + row_stride);
                    let d = (c + 1.0_f64);
                    indices.extend(vec![a, c, b, b, c, d]);
                    {
                        ix += 1.0;
                        ix
                    };
                }
            }
            {
                iz += 1.0;
                iz
            };
        }
    }
    return build_canonical_mesh_geometry(&positions, &normals, &uvs, &indices);
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:404 (sha256:4d5f8eae0c320a6c227c981b1dc5f3f86ed1b6c96de225a0f5891ab5fbef96ad)
pub fn create_polyhedron_mesh_geometry(
    vertex_positions: &mut Vec<Vec<f64>>,
    face_indices: &mut Vec<Vec<f64>>,
    radius: Option<f64>,
    detail: Option<f64>,
) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let detail = detail.unwrap_or(0.0_f64);
    let subs = (0.0_f64).max((detail).min(5.0_f64));
    let verts: std::sync::Arc<std::sync::Mutex<Vec<Vec<f64>>>> =
        std::sync::Arc::new(std::sync::Mutex::new(
            (vertex_positions)
                .iter()
                .cloned()
                .map(|__parameter3: Vec<f64>| -> crate::OpaqueHostValue {
                    let x = __parameter3[0.0_f64 as usize].clone();
                    let y = __parameter3[1.0_f64 as usize].clone();
                    let z = __parameter3[2.0_f64 as usize].clone();
                    let len = (((x * x) + (y * y)) + (z * z)).sqrt();
                    return vec![(x / len), (y / len), (z / len)];
                })
                .collect(),
        ));
    let mut faces: Vec<Vec<f64>> = (face_indices)
        .iter()
        .cloned()
        .map(|f: Vec<f64>| -> crate::OpaqueHostValue {
            vec![
                f[0.0_f64 as usize].clone(),
                f[1.0_f64 as usize].clone(),
                f[2.0_f64 as usize].clone(),
            ]
        })
        .collect();
    if (subs > 0.0_f64) {
        let mid_cache: std::sync::Arc<std::sync::Mutex<Vec<(String, f64)>>> =
            std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut get_mid: std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(f64, f64) -> f64 + Send + 'static>>,
        > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut mid_cache = mid_cache.clone();
            let mut verts = verts.clone();
            move |a: f64, b: f64| -> f64 {
                let key = if (a < b) {
                    format!("{}_{}", a, b)
                } else {
                    format!("{}_{}", b, a)
                };
                let hit = (*mid_cache.lock().unwrap())
                    .iter()
                    .find(|(key, _)| key == &key)
                    .map(|(_, value)| value.clone());
                if (hit).is_some() {
                    return *(hit.as_ref().unwrap());
                }
                let va = (*verts.lock().unwrap())[a as usize].clone();
                let vb = (*verts.lock().unwrap())[b as usize].clone();
                let mut mx =
                    ((va[0.0_f64 as usize].clone() + vb[0.0_f64 as usize].clone()) * 0.5_f64);
                let mut my =
                    ((va[1.0_f64 as usize].clone() + vb[1.0_f64 as usize].clone()) * 0.5_f64);
                let mut mz =
                    ((va[2.0_f64 as usize].clone() + vb[2.0_f64 as usize].clone()) * 0.5_f64);
                let mlen = (((mx * mx) + (my * my)) + (mz * mz)).sqrt();
                mx /= mlen;
                my /= mlen;
                mz /= mlen;
                let idx = ((*verts.lock().unwrap()).len() as f64);
                (*verts.lock().unwrap()).push(vec![mx, my, mz]);
                {
                    let __flight_key = key;
                    let __flight_value = idx;
                    if let Some((_, value)) = (*mid_cache.lock().unwrap())
                        .iter_mut()
                        .find(|(key, _)| key == &__flight_key)
                    {
                        *value = __flight_value;
                    } else {
                        (*mid_cache.lock().unwrap()).push((__flight_key, __flight_value));
                    }
                };
                return idx;
            }
        })
            as Box<dyn FnMut(f64, f64) -> f64 + Send + 'static>));
        {
            let mut s = 0.0_f64;
            while (s < subs) {
                let mut new_faces: Vec<Vec<f64>> = vec![];
                for __iteration4 in (faces).iter().cloned() {
                    let a = __iteration4[0.0_f64 as usize].clone();
                    let b = __iteration4[1.0_f64 as usize].clone();
                    let c = __iteration4[2.0_f64 as usize].clone();
                    let ab = {
                        let __flight_callback = (get_mid).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(a, b);
                        __flight_result
                    };
                    let bc = {
                        let __flight_callback = (get_mid).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(b, c);
                        __flight_result
                    };
                    let ca = {
                        let __flight_callback = (get_mid).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(c, a);
                        __flight_result
                    };
                    new_faces.extend(vec![
                        vec![a, ab, ca],
                        vec![b, bc, ab],
                        vec![c, ca, bc],
                        vec![ab, bc, ca],
                    ]);
                }
                faces = (new_faces).clone();
                (*mid_cache.lock().unwrap()).clear();
                {
                    s += 1.0;
                    s
                };
            }
        }
    }
    let mut positions: Vec<f64> = vec![];
    let mut normals: Vec<f64> = vec![];
    let mut uvs: Vec<f64> = vec![];
    let mut flat_indices: Vec<f64> = vec![];
    for __iteration5 in (faces).iter().cloned() {
        let a = __iteration5[0.0_f64 as usize].clone();
        let b = __iteration5[1.0_f64 as usize].clone();
        let c = __iteration5[2.0_f64 as usize].clone();
        for vi in (vec![a, b, c]).iter().cloned() {
            let v = (*verts.lock().unwrap())[vi as usize].clone();
            let nx = v[0.0_f64 as usize].clone();
            let ny = v[1.0_f64 as usize].clone();
            let nz = v[2.0_f64 as usize].clone();
            positions.extend(vec![(radius * nx), (radius * ny), (radius * nz)]);
            normals.extend(vec![nx, ny, nz]);
            let u = (0.5_f64 + ((nz).atan2(nx) / (std::f64::consts::PI * 2.0_f64)));
            let sv =
                (0.5_f64 - (((-1.0_f64).max((1.0_f64).min(ny))).asin() / std::f64::consts::PI));
            uvs.extend(vec![u, sv]);
        }
    }
    {
        let mut i = 0.0_f64;
        while (i < ((positions.len() as f64) / 3.0_f64)) {
            flat_indices.push(i);
            {
                i += 1.0;
                i
            };
        }
    }
    return build_canonical_mesh_geometry(&positions, &normals, &uvs, &flat_indices);
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:475 (sha256:a587e002a7b2a6b5fecd8a9af1024743ed558f7e27c5750d74631a0c7903f662)
pub fn create_quad_mesh_geometry(width: Option<f64>, height: Option<f64>) -> MeshGeometry {
    let width = width.unwrap_or(1.0_f64);
    let height = height.unwrap_or(1.0_f64);
    let hw = (width * 0.5_f64);
    let hh = (height * 0.5_f64);
    let positions = vec![
        (-hw),
        (-hh),
        0.0_f64,
        hw,
        (-hh),
        0.0_f64,
        (-hw),
        hh,
        0.0_f64,
        hw,
        hh,
        0.0_f64,
    ];
    let normals = vec![
        0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
        0.0_f64, 1.0_f64,
    ];
    let uvs = vec![
        0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 1.0_f64, 1.0_f64,
    ];
    let indices = vec![0.0_f64, 1.0_f64, 2.0_f64, 2.0_f64, 1.0_f64, 3.0_f64];
    return build_canonical_mesh_geometry(&positions, &normals, &uvs, &indices);
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:488 (sha256:b27cccbfa5cbb070242b89f3fe84f60849fab3e4e7537af88fe9eceb5b7c3424)
pub fn create_ring_mesh_geometry(
    inner_radius: Option<f64>,
    outer_radius: Option<f64>,
    segments: Option<f64>,
) -> MeshGeometry {
    let inner_radius = inner_radius.unwrap_or(0.25_f64);
    let outer_radius = outer_radius.unwrap_or(0.5_f64);
    let segments = segments.unwrap_or(32.0_f64);
    let segs = (3.0_f64).max(segments);
    let mut positions: Vec<f64> = vec![];
    let mut normals: Vec<f64> = vec![];
    let mut uvs: Vec<f64> = vec![];
    let mut indices: Vec<f64> = vec![];
    {
        let mut i = 0.0_f64;
        while (i <= segs) {
            let theta = (((i / segs) * std::f64::consts::PI) * 2.0_f64);
            let cos = (theta).cos();
            let sin = (theta).sin();
            positions.extend(vec![(inner_radius * cos), 0.0_f64, (inner_radius * sin)]);
            normals.extend(vec![0.0_f64, 1.0_f64, 0.0_f64]);
            uvs.extend(vec![0.0_f64, (i / segs)]);
            positions.extend(vec![(outer_radius * cos), 0.0_f64, (outer_radius * sin)]);
            normals.extend(vec![0.0_f64, 1.0_f64, 0.0_f64]);
            uvs.extend(vec![1.0_f64, (i / segs)]);
            {
                i += 1.0;
                i
            };
        }
    }
    {
        let mut i = 0.0_f64;
        while (i < segs) {
            let inner0 = (i * 2.0_f64);
            let outer0 = ((i * 2.0_f64) + 1.0_f64);
            let inner1 = ((i + 1.0_f64) * 2.0_f64);
            let outer1 = (((i + 1.0_f64) * 2.0_f64) + 1.0_f64);
            indices.extend(vec![inner0, inner1, outer0, outer0, inner1, outer1]);
            {
                i += 1.0;
                i
            };
        }
    }
    return build_canonical_mesh_geometry(&positions, &normals, &uvs, &indices);
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:528 (sha256:4375ae903da4d469843856f5aca65abd6d22afea6b67d0949f5c4675a77b2a91)
pub fn create_sphere_mesh_geometry(
    radius: Option<f64>,
    width_segments: Option<f64>,
    height_segments: Option<f64>,
) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let width_segments = width_segments.unwrap_or(32.0_f64);
    let height_segments = height_segments.unwrap_or(16.0_f64);
    let w_seg = (3.0_f64).max(width_segments);
    let h_seg = (2.0_f64).max(height_segments);
    let mut positions: Vec<f64> = vec![];
    let mut normals: Vec<f64> = vec![];
    let mut uvs: Vec<f64> = vec![];
    let mut indices: Vec<f64> = vec![];
    {
        let mut iy = 0.0_f64;
        while (iy <= h_seg) {
            let v = (iy / h_seg);
            let phi = (v * std::f64::consts::PI);
            let sin_phi = (phi).sin();
            let cos_phi = (phi).cos();
            {
                let mut ix = 0.0_f64;
                while (ix <= w_seg) {
                    let u = (ix / w_seg);
                    let theta = ((u * std::f64::consts::PI) * 2.0_f64);
                    let sin_theta = (theta).sin();
                    let cos_theta = (theta).cos();
                    let nx = ((-sin_phi) * cos_theta);
                    let ny = cos_phi;
                    let nz = (sin_phi * sin_theta);
                    positions.extend(vec![(radius * nx), (radius * ny), (radius * nz)]);
                    normals.extend(vec![nx, ny, nz]);
                    uvs.extend(vec![u, v]);
                    {
                        ix += 1.0;
                        ix
                    };
                }
            }
            {
                iy += 1.0;
                iy
            };
        }
    }
    let row_stride = (w_seg + 1.0_f64);
    {
        let mut iy = 0.0_f64;
        while (iy < h_seg) {
            {
                let mut ix = 0.0_f64;
                while (ix < w_seg) {
                    let a = ((iy * row_stride) + ix);
                    let b = (a + 1.0_f64);
                    let c = (a + row_stride);
                    let d = (c + 1.0_f64);
                    indices.extend(vec![a, c, b, b, c, d]);
                    {
                        ix += 1.0;
                        ix
                    };
                }
            }
            {
                iy += 1.0;
                iy
            };
        }
    }
    return build_canonical_mesh_geometry(&positions, &normals, &uvs, &indices);
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:575 (sha256:d988cbaad6fd53e10c62f12c543df83b09681478b8f7050b5bcdfe3ed40af84b)
pub fn create_tetrahedron_mesh_geometry(radius: Option<f64>, detail: Option<f64>) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let detail = detail.unwrap_or(0.0_f64);
    return create_polyhedron_mesh_geometry(
        &mut TETRAHEDRON_VERTS,
        &mut TETRAHEDRON_FACES,
        Some(radius),
        Some(detail),
    );
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:582 (sha256:36b0fdb596268ca932437e91a6a3dd4d5944a13ded4b5f0047a19b4309ff6300)
pub fn create_torus_knot_mesh_geometry(
    radius: Option<f64>,
    tube: Option<f64>,
    tubular_segments: Option<f64>,
    radial_segments: Option<f64>,
    p: Option<f64>,
    q: Option<f64>,
) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let tube = tube.unwrap_or(0.15_f64);
    let tubular_segments = tubular_segments.unwrap_or(64.0_f64);
    let radial_segments = radial_segments.unwrap_or(8.0_f64);
    let p = p.unwrap_or(2.0_f64);
    let q = q.unwrap_or(3.0_f64);
    let t_seg = (3.0_f64).max(tubular_segments);
    let r_seg = (3.0_f64).max(radial_segments);
    let mut positions: Vec<f64> = vec![];
    let mut normals: Vec<f64> = vec![];
    let mut uvs: Vec<f64> = vec![];
    let mut indices: Vec<f64> = vec![];
    let mut curve_point: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64) -> Vec<f64> + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |t: f64| -> Vec<f64> {
        let angle = ((t * std::f64::consts::PI) * 2.0_f64);
        let x = (((radius + tube) * (p * angle).cos()) * (q * angle).cos());
        let y = (((radius + tube) * (p * angle).cos()) * (q * angle).sin());
        let z = ((radius + tube) * (p * angle).sin());
        return vec![x, y, z];
    })
        as Box<dyn FnMut(f64) -> Vec<f64> + Send + 'static>));
    {
        let mut i = 0.0_f64;
        while (i <= t_seg) {
            let u = (i / t_seg);
            let __destructure6 = {
                let __flight_callback = (curve_point).clone();
                let __flight_result = __flight_callback.lock().unwrap()(u);
                __flight_result
            };
            let cx = __destructure6[0.0_f64 as usize].clone();
            let cy = __destructure6[1.0_f64 as usize].clone();
            let cz = __destructure6[2.0_f64 as usize].clone();
            let __destructure7 = {
                let __flight_callback = (curve_point).clone();
                let __flight_result = __flight_callback.lock().unwrap()((u + 0.001_f64));
                __flight_result
            };
            let tx1 = __destructure7[0.0_f64 as usize].clone();
            let ty1 = __destructure7[1.0_f64 as usize].clone();
            let tz1 = __destructure7[2.0_f64 as usize].clone();
            let __destructure8 = {
                let __flight_callback = (curve_point).clone();
                let __flight_result = __flight_callback.lock().unwrap()((u - 0.001_f64));
                __flight_result
            };
            let tx0 = __destructure8[0.0_f64 as usize].clone();
            let ty0 = __destructure8[1.0_f64 as usize].clone();
            let tz0 = __destructure8[2.0_f64 as usize].clone();
            let mut tgx = (tx1 - tx0);
            let mut tgy = (ty1 - ty0);
            let mut tgz = (tz1 - tz0);
            let tg_len = if ((((tgx * tgx) + (tgy * tgy)) + (tgz * tgz)).sqrt()) != 0.0_f64 {
                (((tgx * tgx) + (tgy * tgy)) + (tgz * tgz)).sqrt()
            } else {
                1.0_f64
            };
            tgx /= tg_len;
            tgy /= tg_len;
            tgz /= tg_len;
            let mut bx = (tgx + cx);
            let mut by = (tgy + cy);
            let mut bz = (tgz + cz);
            let b_len = if ((((bx * bx) + (by * by)) + (bz * bz)).sqrt()) != 0.0_f64 {
                (((bx * bx) + (by * by)) + (bz * bz)).sqrt()
            } else {
                1.0_f64
            };
            bx /= b_len;
            by /= b_len;
            bz /= b_len;
            let mut nnx = ((tgy * bz) - (tgz * by));
            let mut nny = ((tgz * bx) - (tgx * bz));
            let mut nnz = ((tgx * by) - (tgy * bx));
            let n_len = if ((((nnx * nnx) + (nny * nny)) + (nnz * nnz)).sqrt()) != 0.0_f64 {
                (((nnx * nnx) + (nny * nny)) + (nnz * nnz)).sqrt()
            } else {
                1.0_f64
            };
            nnx /= n_len;
            nny /= n_len;
            nnz /= n_len;
            let bnx = ((tgy * nnz) - (tgz * nny));
            let bny = ((tgz * nnx) - (tgx * nnz));
            let bnz = ((tgx * nny) - (tgy * nnx));
            {
                let mut j = 0.0_f64;
                while (j <= r_seg) {
                    let v = (j / r_seg);
                    let phi = ((v * std::f64::consts::PI) * 2.0_f64);
                    let cos_phi = (phi).cos();
                    let sin_phi = (phi).sin();
                    let px = (cx + (tube * ((cos_phi * nnx) + (sin_phi * bnx))));
                    let py = (cy + (tube * ((cos_phi * nny) + (sin_phi * bny))));
                    let pz = (cz + (tube * ((cos_phi * nnz) + (sin_phi * bnz))));
                    positions.extend(vec![px, py, pz]);
                    normals.extend(vec![
                        ((cos_phi * nnx) + (sin_phi * bnx)),
                        ((cos_phi * nny) + (sin_phi * bny)),
                        ((cos_phi * nnz) + (sin_phi * bnz)),
                    ]);
                    uvs.extend(vec![u, v]);
                    {
                        j += 1.0;
                        j
                    };
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let row_stride = (r_seg + 1.0_f64);
    {
        let mut i = 0.0_f64;
        while (i < t_seg) {
            {
                let mut j = 0.0_f64;
                while (j < r_seg) {
                    let a = ((i * row_stride) + j);
                    let b = (a + 1.0_f64);
                    let c = (a + row_stride);
                    let d = (c + 1.0_f64);
                    indices.extend(vec![a, c, b, b, c, d]);
                    {
                        j += 1.0;
                        j
                    };
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return build_canonical_mesh_geometry(&positions, &normals, &uvs, &indices);
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:670 (sha256:360ceecac8147962270f8a44771d6ddd1da3669e4c40e3c668597544beb48081)
pub fn create_torus_mesh_geometry(
    radius: Option<f64>,
    tube: Option<f64>,
    radial_segments: Option<f64>,
    tubular_segments: Option<f64>,
) -> MeshGeometry {
    let radius = radius.unwrap_or(0.5_f64);
    let tube = tube.unwrap_or(0.2_f64);
    let radial_segments = radial_segments.unwrap_or(24.0_f64);
    let tubular_segments = tubular_segments.unwrap_or(48.0_f64);
    let r_seg = (3.0_f64).max(radial_segments);
    let t_seg = (3.0_f64).max(tubular_segments);
    let mut positions: Vec<f64> = vec![];
    let mut normals: Vec<f64> = vec![];
    let mut uvs: Vec<f64> = vec![];
    let mut indices: Vec<f64> = vec![];
    {
        let mut j = 0.0_f64;
        while (j <= r_seg) {
            let v = (((j / r_seg) * std::f64::consts::PI) * 2.0_f64);
            let cos_v = (v).cos();
            let sin_v = (v).sin();
            {
                let mut i = 0.0_f64;
                while (i <= t_seg) {
                    let u = (((i / t_seg) * std::f64::consts::PI) * 2.0_f64);
                    let cos_u = (u).cos();
                    let sin_u = (u).sin();
                    let cx = (radius * cos_u);
                    let cy = (radius * sin_u);
                    let px = ((radius + (tube * cos_v)) * cos_u);
                    let py = ((radius + (tube * cos_v)) * sin_u);
                    let mut pz = (tube * sin_v);
                    positions.extend(vec![px, py, pz]);
                    let mut nx = (px - cx);
                    let mut ny = (py - cy);
                    let mut nz = pz;
                    let len = if ((((nx * nx) + (ny * ny)) + (nz * nz)).sqrt()) != 0.0_f64 {
                        (((nx * nx) + (ny * ny)) + (nz * nz)).sqrt()
                    } else {
                        1.0_f64
                    };
                    nx /= len;
                    ny /= len;
                    nz /= len;
                    normals.extend(vec![nx, ny, nz]);
                    uvs.extend(vec![(i / t_seg), (j / r_seg)]);
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            {
                j += 1.0;
                j
            };
        }
    }
    let row_stride = (t_seg + 1.0_f64);
    {
        let mut j = 0.0_f64;
        while (j < r_seg) {
            {
                let mut i = 0.0_f64;
                while (i < t_seg) {
                    let a = ((j * row_stride) + i);
                    let b = (a + 1.0_f64);
                    let c = (a + row_stride);
                    let d = (c + 1.0_f64);
                    indices.extend(vec![a, b, c, b, d, c]);
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            {
                j += 1.0;
                j
            };
        }
    }
    return build_canonical_mesh_geometry(&positions, &normals, &uvs, &indices);
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:727 (sha256:0aaaa4cc7d3b40deb922fa5b62cd1702fcd18ded7d41fd2dbcaca7052b879a7f)
fn add_disc(
    positions: &mut Vec<f64>,
    normals: &mut Vec<f64>,
    uvs: &mut Vec<f64>,
    indices: &mut Vec<f64>,
    segments: f64,
    radius: f64,
    y: f64,
    direction: f64,
) -> () {
    let center = ((positions.len() as f64) / 3.0_f64);
    positions.extend(vec![0.0_f64, y, 0.0_f64]);
    normals.extend(vec![0.0_f64, direction, 0.0_f64]);
    uvs.extend(vec![0.5_f64, 0.5_f64]);
    let ring_start = ((positions.len() as f64) / 3.0_f64);
    {
        let mut s = 0.0_f64;
        while (s <= segments) {
            let theta = (((s / segments) * std::f64::consts::PI) * 2.0_f64);
            let cos = (theta).cos();
            let sin = (theta).sin();
            positions.extend(vec![(radius * cos), y, (radius * sin)]);
            normals.extend(vec![0.0_f64, direction, 0.0_f64]);
            uvs.extend(vec![
                ((cos * 0.5_f64) + 0.5_f64),
                ((sin * 0.5_f64) + 0.5_f64),
            ]);
            {
                s += 1.0;
                s
            };
        }
    }
    {
        let mut s = 0.0_f64;
        while (s < segments) {
            let a = (ring_start + s);
            let b = ((ring_start + s) + 1.0_f64);
            if (direction > 0.0_f64) {
                indices.extend(vec![center, a, b]);
            } else {
                indices.extend(vec![center, b, a]);
            }
            {
                s += 1.0;
                s
            };
        }
    }
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:766 (sha256:caa27509afffb845ffdd0ff929f02ad43e26534a5a53d24ac12152add937c20d)
fn build_canonical_mesh_geometry(
    positions: &Vec<f64>,
    normals: &Vec<f64>,
    uvs: &Vec<f64>,
    indices: &Vec<f64>,
) -> MeshGeometry {
    let vertex_count = ((positions.len() as f64) / 3.0_f64);
    let mut vertices: Vec<f32> =
        vec![0.0_f32; (vertex_count * CANONICAL_FLOATS_PER_VERTEX) as usize];
    {
        let mut i = 0.0_f64;
        while (i < vertex_count) {
            let base = (i * CANONICAL_FLOATS_PER_VERTEX);
            vertices[base as usize] = (positions[(i * 3.0_f64) as usize].clone()) as f32;
            vertices[(base + 1.0_f64) as usize] =
                (positions[((i * 3.0_f64) + 1.0_f64) as usize].clone()) as f32;
            vertices[(base + 2.0_f64) as usize] =
                (positions[((i * 3.0_f64) + 2.0_f64) as usize].clone()) as f32;
            vertices[(base + 3.0_f64) as usize] = (normals[(i * 3.0_f64) as usize].clone()) as f32;
            vertices[(base + 4.0_f64) as usize] =
                (normals[((i * 3.0_f64) + 1.0_f64) as usize].clone()) as f32;
            vertices[(base + 5.0_f64) as usize] =
                (normals[((i * 3.0_f64) + 2.0_f64) as usize].clone()) as f32;
            vertices[(base + 10.0_f64) as usize] = (uvs[(i * 2.0_f64) as usize].clone()) as f32;
            vertices[(base + 11.0_f64) as usize] =
                (uvs[((i * 2.0_f64) + 1.0_f64) as usize].clone()) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    let mut index_array: Vec<u32> = vec![0_u32; (indices.len() as f64) as usize];
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<u32> = (indices).iter().map(|value| (*value) as u32).collect();
        index_array[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
    let mut geometry = create_mesh_geometry(&mut MeshGeometryOptions {
        __flight_identity: std::sync::Arc::new(()),
        indices: Some((index_array).clone()),
        layout: ((*CANONICAL_VERTEX_LAYOUT).clone()).clone(),
        vertices: (vertices).clone(),
        subsets: None,
        topology: None,
    });
    {
        let __flight_argument_1 = (geometry).clone();
        compute_mesh_geometry_tangents(&mut geometry, &__flight_argument_1)
    };
    let mut bounds = create_aabb(None, None, None, None, None, None);
    compute_mesh_geometry_bounds(&mut bounds, &geometry);
    geometry.bounds = Some((bounds).clone());
    return geometry;
}

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:806 (sha256:a7b0354de76d287339f53dd191039a613081dc1dd7ce7309de50dd4faab3d7e6)
const CANONICAL_FLOATS_PER_VERTEX: f64 = 12.0_f64;

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:810 (sha256:f3b4fb22e16750bee5b271ef244e7477a97aa693d338dd863148f4f99b652625)
static CANONICAL_VERTEX_LAYOUT: std::sync::LazyLock<VertexAttributeLayout> =
    std::sync::LazyLock::new(|| VertexAttributeLayout {
        __flight_identity: std::sync::Arc::new(()),
        attributes: vec![
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 0.0_f64,
                format: "float32x3".to_owned(),
                semantic: "position".to_owned(),
            },
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 12.0_f64,
                format: "float32x3".to_owned(),
                semantic: "normal".to_owned(),
            },
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 24.0_f64,
                format: "float32x4".to_owned(),
                semantic: "tangent".to_owned(),
            },
            VertexAttribute {
                __flight_identity: std::sync::Arc::new(()),
                byte_offset: 40.0_f64,
                format: "float32x2".to_owned(),
                semantic: "uv0".to_owned(),
            },
        ],
        stride: 48.0_f64,
    });

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:821 (sha256:b4462050674dcc9993d1595f2e6a95285eae03854ef0722b229ba2820d61de96)
static TETRAHEDRON_VERTS: std::sync::LazyLock<std::sync::Mutex<Vec<Vec<f64>>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![
            vec![1.0_f64, 1.0_f64, 1.0_f64],
            vec![(-1.0_f64), (-1.0_f64), 1.0_f64],
            vec![(-1.0_f64), 1.0_f64, (-1.0_f64)],
            vec![1.0_f64, (-1.0_f64), (-1.0_f64)],
        ])
    });

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:827 (sha256:4eb79ff230e175ca737adef41c542a3c1edb6e13c1337269f4f06524997f6caf)
static TETRAHEDRON_FACES: std::sync::LazyLock<std::sync::Mutex<Vec<Vec<f64>>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![
            vec![2.0_f64, 1.0_f64, 0.0_f64],
            vec![0.0_f64, 3.0_f64, 2.0_f64],
            vec![1.0_f64, 3.0_f64, 0.0_f64],
            vec![2.0_f64, 3.0_f64, 1.0_f64],
        ])
    });

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:835 (sha256:6eebb530e2af2af1e99aa49fa0ad81fd3643aeba594eec2f6259a134400a07be)
static OCTAHEDRON_VERTS: std::sync::LazyLock<std::sync::Mutex<Vec<Vec<f64>>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![
            vec![1.0_f64, 0.0_f64, 0.0_f64],
            vec![(-1.0_f64), 0.0_f64, 0.0_f64],
            vec![0.0_f64, 1.0_f64, 0.0_f64],
            vec![0.0_f64, (-1.0_f64), 0.0_f64],
            vec![0.0_f64, 0.0_f64, 1.0_f64],
            vec![0.0_f64, 0.0_f64, (-1.0_f64)],
        ])
    });

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:843 (sha256:b6a7790ba5d50996d7aa4680eb9996abf5d93028d8c6d61065e92db75cf40ed2)
static OCTAHEDRON_FACES: std::sync::LazyLock<std::sync::Mutex<Vec<Vec<f64>>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![
            vec![0.0_f64, 2.0_f64, 4.0_f64],
            vec![0.0_f64, 4.0_f64, 3.0_f64],
            vec![0.0_f64, 3.0_f64, 5.0_f64],
            vec![0.0_f64, 5.0_f64, 2.0_f64],
            vec![1.0_f64, 4.0_f64, 2.0_f64],
            vec![1.0_f64, 3.0_f64, 4.0_f64],
            vec![1.0_f64, 5.0_f64, 3.0_f64],
            vec![1.0_f64, 2.0_f64, 5.0_f64],
        ])
    });

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:855 (sha256:d91dd4ffb7d3f09014dd5c30a7797156ebe027bc49882dfe439d932c5b3c78c1)
const _PHI: f64 = 1.618033988749895_f64;

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:856 (sha256:df6838f304d9be1b2cccefcca9547e203ebdeeef1440110238cd70bf6317ad26)
static ICOSAHEDRON_VERTS: std::sync::LazyLock<std::sync::Mutex<Vec<Vec<f64>>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![
            vec![(-1.0_f64), _PHI, 0.0_f64],
            vec![1.0_f64, _PHI, 0.0_f64],
            vec![(-1.0_f64), (-_PHI), 0.0_f64],
            vec![1.0_f64, (-_PHI), 0.0_f64],
            vec![0.0_f64, (-1.0_f64), _PHI],
            vec![0.0_f64, 1.0_f64, _PHI],
            vec![0.0_f64, (-1.0_f64), (-_PHI)],
            vec![0.0_f64, 1.0_f64, (-_PHI)],
            vec![_PHI, 0.0_f64, (-1.0_f64)],
            vec![_PHI, 0.0_f64, 1.0_f64],
            vec![(-_PHI), 0.0_f64, (-1.0_f64)],
            vec![(-_PHI), 0.0_f64, 1.0_f64],
        ])
    });

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:870 (sha256:f824323b9da2ab44101ab17edf7e7c44f8508e5b1aaac8396ef9ae8f2a088f30)
static ICOSAHEDRON_FACES: std::sync::LazyLock<std::sync::Mutex<Vec<Vec<f64>>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![
            vec![0.0_f64, 11.0_f64, 5.0_f64],
            vec![0.0_f64, 5.0_f64, 1.0_f64],
            vec![0.0_f64, 1.0_f64, 7.0_f64],
            vec![0.0_f64, 7.0_f64, 10.0_f64],
            vec![0.0_f64, 10.0_f64, 11.0_f64],
            vec![1.0_f64, 5.0_f64, 9.0_f64],
            vec![5.0_f64, 11.0_f64, 4.0_f64],
            vec![11.0_f64, 10.0_f64, 2.0_f64],
            vec![10.0_f64, 7.0_f64, 6.0_f64],
            vec![7.0_f64, 1.0_f64, 8.0_f64],
            vec![3.0_f64, 9.0_f64, 4.0_f64],
            vec![3.0_f64, 4.0_f64, 2.0_f64],
            vec![3.0_f64, 2.0_f64, 6.0_f64],
            vec![3.0_f64, 6.0_f64, 8.0_f64],
            vec![3.0_f64, 8.0_f64, 9.0_f64],
            vec![4.0_f64, 9.0_f64, 5.0_f64],
            vec![2.0_f64, 4.0_f64, 11.0_f64],
            vec![6.0_f64, 2.0_f64, 10.0_f64],
            vec![8.0_f64, 6.0_f64, 7.0_f64],
            vec![9.0_f64, 8.0_f64, 1.0_f64],
        ])
    });

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:896 (sha256:6955c772dc7da0f91fecfcf6a9158c425e3d793f2002559de0e982c147016c9a)
const _D: f64 = 0.6180339887498948_f64;

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:897 (sha256:5d1601b4f2c13b631bad9d0ac15c59a8b98450f358b0c08f4642c3acf17bcc6b)
static DODECAHEDRON_VERTS: std::sync::LazyLock<std::sync::Mutex<Vec<Vec<f64>>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![
            vec![(-1.0_f64), (-1.0_f64), (-1.0_f64)],
            vec![(-1.0_f64), (-1.0_f64), 1.0_f64],
            vec![(-1.0_f64), 1.0_f64, (-1.0_f64)],
            vec![(-1.0_f64), 1.0_f64, 1.0_f64],
            vec![1.0_f64, (-1.0_f64), (-1.0_f64)],
            vec![1.0_f64, (-1.0_f64), 1.0_f64],
            vec![1.0_f64, 1.0_f64, (-1.0_f64)],
            vec![1.0_f64, 1.0_f64, 1.0_f64],
            vec![0.0_f64, (-_D), (-_PHI)],
            vec![0.0_f64, (-_D), _PHI],
            vec![0.0_f64, _D, (-_PHI)],
            vec![0.0_f64, _D, _PHI],
            vec![(-_D), (-_PHI), 0.0_f64],
            vec![(-_D), _PHI, 0.0_f64],
            vec![_D, (-_PHI), 0.0_f64],
            vec![_D, _PHI, 0.0_f64],
            vec![(-_PHI), 0.0_f64, (-_D)],
            vec![(-_PHI), 0.0_f64, _D],
            vec![_PHI, 0.0_f64, (-_D)],
            vec![_PHI, 0.0_f64, _D],
        ])
    });

// Source: upstream/packages/mesh/src/meshGeometryBuilders.ts:919 (sha256:a360a55e1d816e93e7044a06c1ac7b328504e0a5f942948f9c7b94946c413e7a)
static DODECAHEDRON_FACES: std::sync::LazyLock<std::sync::Mutex<Vec<Vec<f64>>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![
            vec![3.0_f64, 11.0_f64, 7.0_f64],
            vec![3.0_f64, 7.0_f64, 15.0_f64],
            vec![3.0_f64, 15.0_f64, 13.0_f64],
            vec![7.0_f64, 19.0_f64, 11.0_f64],
            vec![7.0_f64, 11.0_f64, 9.0_f64],
            vec![7.0_f64, 9.0_f64, 19.0_f64],
            vec![15.0_f64, 7.0_f64, 19.0_f64],
            vec![15.0_f64, 19.0_f64, 18.0_f64],
            vec![15.0_f64, 18.0_f64, 6.0_f64],
            vec![13.0_f64, 15.0_f64, 6.0_f64],
            vec![13.0_f64, 6.0_f64, 2.0_f64],
            vec![13.0_f64, 2.0_f64, 16.0_f64],
            vec![3.0_f64, 13.0_f64, 16.0_f64],
            vec![3.0_f64, 16.0_f64, 17.0_f64],
            vec![3.0_f64, 17.0_f64, 11.0_f64],
            vec![11.0_f64, 17.0_f64, 1.0_f64],
            vec![11.0_f64, 1.0_f64, 9.0_f64],
            vec![9.0_f64, 1.0_f64, 5.0_f64],
            vec![5.0_f64, 1.0_f64, 17.0_f64],
            vec![5.0_f64, 17.0_f64, 4.0_f64],
            vec![5.0_f64, 4.0_f64, 14.0_f64],
            vec![9.0_f64, 5.0_f64, 14.0_f64],
            vec![9.0_f64, 14.0_f64, 12.0_f64],
            vec![9.0_f64, 12.0_f64, 0.0_f64],
            vec![1.0_f64, 0.0_f64, 12.0_f64],
            vec![1.0_f64, 12.0_f64, 4.0_f64],
            vec![1.0_f64, 4.0_f64, 17.0_f64],
            vec![6.0_f64, 18.0_f64, 8.0_f64],
            vec![6.0_f64, 8.0_f64, 10.0_f64],
            vec![6.0_f64, 10.0_f64, 2.0_f64],
            vec![18.0_f64, 19.0_f64, 8.0_f64],
            vec![19.0_f64, 7.0_f64, 8.0_f64],
            vec![7.0_f64, 15.0_f64, 8.0_f64],
            vec![2.0_f64, 10.0_f64, 16.0_f64],
            vec![10.0_f64, 8.0_f64, 0.0_f64],
            vec![0.0_f64, 8.0_f64, 12.0_f64],
        ])
    });
