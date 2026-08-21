// @generated from upstream/packages/path/src/tessellateStrokePath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    STROKE_PATH_TESSELLATION_ISSUE_NONE as stroke_path_tessellation_issue_none_constant,
    build_stroke_path_geometry,
};
use flighthq_types::{Path, PathMesh, StrokeStyle};

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

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub closed: bool,
    pub end_cap: Vec<f64>,
    pub left: Vec<f64>,
    pub right: Vec<f64>,
    pub start_cap: Vec<f64>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/tessellateStrokePath.ts:9 (sha256:0be7746224b133b6fd7cbf2db8901c28dafce5933a4a68f489f2191667ccfe42)
pub fn tessellate_stroke_path(
    path: &Path,
    style: &StrokeStyle,
    tolerance: Option<f64>,
) -> Option<PathMesh> {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let geometry = build_stroke_path_geometry(path, style, tolerance);
    if (geometry.issue != stroke_path_tessellation_issue_none_constant) {
        return None;
    }
    let mut mesh: PathMesh = PathMesh {
        __flight_identity: std::sync::Arc::new(()),
        indices: vec![],
        vertices: vec![],
    };
    {
        let mut i = 0.0_f64;
        while (i < (geometry.pieces.len() as f64)) {
            append_stroke_piece_mesh(&mut mesh, &{
                let __flight_source = &(geometry.pieces[i as usize]);
                SharedStructuralRecord1 {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    closed: __flight_source.closed,
                    end_cap: (__flight_source.end_cap).clone(),
                    left: (__flight_source.left).clone(),
                    right: (__flight_source.right).clone(),
                    start_cap: (__flight_source.start_cap).clone(),
                }
            });
            {
                i += 1.0;
                i
            };
        }
    }
    return Some((mesh).clone());
}

// Source: upstream/packages/path/src/tessellateStrokePath.ts:21 (sha256:aba39c25764a121f4bb1ba09614575e14b561649c41d4551d8e163e31e2736a3)
fn append_stroke_piece_mesh(mesh: &mut PathMesh, piece: &SharedStructuralRecord1) -> () {
    let section_count = (__flight_js_to_i32((piece.left.len() as f64))
        >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    if (section_count < 2.0_f64) {
        return;
    }
    let base = (__flight_js_to_i32((mesh.vertices.len() as f64))
        >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    {
        let mut i = 0.0_f64;
        while (i < section_count) {
            mesh.vertices.extend(vec![
                piece.left[(i * 2.0_f64) as usize].clone(),
                piece.left[((i * 2.0_f64) + 1.0_f64) as usize].clone(),
                piece.right[(i * 2.0_f64) as usize].clone(),
                piece.right[((i * 2.0_f64) + 1.0_f64) as usize].clone(),
            ]);
            {
                i += 1.0;
                i
            };
        }
    }
    let connection_count = if piece.closed {
        section_count
    } else {
        (section_count - 1.0_f64)
    };
    {
        let mut i = 0.0_f64;
        while (i < connection_count) {
            let next = ((i + 1.0_f64) % section_count);
            let left = (base + (i * 2.0_f64));
            let right = (left + 1.0_f64);
            let next_left = (base + (next * 2.0_f64));
            let next_right = (next_left + 1.0_f64);
            append_triangle(mesh, left, right, next_left);
            append_triangle(mesh, next_left, right, next_right);
            {
                i += 1.0;
                i
            };
        }
    }
    if (!piece.closed) && ((piece.start_cap.len() as f64) > 0.0_f64) {
        let right = vec![
            piece.right[0.0_f64 as usize].clone(),
            piece.right[1.0_f64 as usize].clone(),
        ];
        let left = vec![
            piece.left[0.0_f64 as usize].clone(),
            piece.left[1.0_f64 as usize].clone(),
        ];
        append_round_cap(mesh, &right, &piece.start_cap, &left);
    }
    if (!piece.closed) && ((piece.end_cap.len() as f64) > 0.0_f64) {
        let end = ((piece.left.len() as f64) - 2.0_f64);
        let left = vec![
            piece.left[end as usize].clone(),
            piece.left[(end + 1.0_f64) as usize].clone(),
        ];
        let right = vec![
            piece.right[end as usize].clone(),
            piece.right[(end + 1.0_f64) as usize].clone(),
        ];
        append_round_cap(mesh, &left, &piece.end_cap, &right);
    }
}

// Source: upstream/packages/path/src/tessellateStrokePath.ts:60 (sha256:fcc5eecb6383afc7f3bc4d447f20637304fed3bddef885c14e3ece79133627e7)
fn append_round_cap(
    mesh: &mut PathMesh,
    start: &Vec<f64>,
    interior: &Vec<f64>,
    end: &Vec<f64>,
) -> () {
    let center = (__flight_js_to_i32((mesh.vertices.len() as f64))
        >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    mesh.vertices.extend(vec![
        ((start[0.0_f64 as usize].clone() + end[0.0_f64 as usize].clone()) / 2.0_f64),
        ((start[1.0_f64 as usize].clone() + end[1.0_f64 as usize].clone()) / 2.0_f64),
    ]);
    let arc_base = (__flight_js_to_i32((mesh.vertices.len() as f64))
        >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    {
        mesh.vertices.push(start[0.0_f64 as usize].clone());
        mesh.vertices.push(start[1.0_f64 as usize].clone());
        mesh.vertices.extend((interior).iter().cloned());
        mesh.vertices.push(end[0.0_f64 as usize].clone());
        mesh.vertices.push(end[1.0_f64 as usize].clone());
        mesh.vertices.len() as f64
    };
    let arc_count = ((__flight_js_to_i32((interior.len() as f64))
        >> (__flight_js_to_u32(1.0_f64) & 31)) as f64
        + 2.0_f64);
    {
        let mut i = 0.0_f64;
        while (i < (arc_count - 1.0_f64)) {
            append_triangle(mesh, center, (arc_base + i), ((arc_base + i) + 1.0_f64));
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/path/src/tessellateStrokePath.ts:74 (sha256:c7c06ce636ace1b5368e6b0c879dcc2362c2f268b1d943ffa27b38a8ce87207e)
fn append_triangle(mesh: &mut PathMesh, a: f64, b: f64, c: f64) -> () {
    let ax = mesh.vertices[(a * 2.0_f64) as usize].clone();
    let ay = mesh.vertices[((a * 2.0_f64) + 1.0_f64) as usize].clone();
    let bx = mesh.vertices[(b * 2.0_f64) as usize].clone();
    let by = mesh.vertices[((b * 2.0_f64) + 1.0_f64) as usize].clone();
    let cx = mesh.vertices[(c * 2.0_f64) as usize].clone();
    let cy = mesh.vertices[((c * 2.0_f64) + 1.0_f64) as usize].clone();
    if ((((bx - ax) * (cy - ay)) - ((by - ay) * (cx - ax))).abs() <= TRIANGLE_EPSILON) {
        return;
    }
    mesh.indices.extend(vec![a, b, c]);
}

// Source: upstream/packages/path/src/tessellateStrokePath.ts:85 (sha256:a54f4ff25884fd6283291624dea9bae55e80fccc14beb7f3fc178a78c153cf61)
const TRIANGLE_EPSILON: f64 = 1e-10_f64;
