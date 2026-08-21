// @generated from upstream/packages/path/src/tessellatePath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::flatten_path;
use flighthq_types::{Path, PathMesh};

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

// Source: upstream/packages/path/src/tessellatePath.ts:10 (sha256:8f89f218981961696e98494584b4f7f0cfedc01e1ce3a49ba5a12deafce80678)
pub fn tessellate_path(path: &Path, tolerance: Option<f64>) -> PathMesh {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(path, Some(tolerance));
    let mut vertices: Vec<f64> = vec![];
    let mut indices: Vec<f64> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (contours.len() as f64)) {
            tessellate_contour(&contours[i as usize], &mut vertices, &mut indices);
            {
                i += 1.0;
                i
            };
        }
    }
    return PathMesh {
        __flight_identity: std::sync::Arc::new(()),
        vertices: (vertices).clone(),
        indices: (indices).clone(),
    };
}

// Source: upstream/packages/path/src/tessellatePath.ts:22 (sha256:0a854f743d2bb1394bf0aca3195c5bd0d4d7e22bdd3e2a070d966e8cddb882db)
fn tessellate_contour(source: &Vec<f64>, vertices: &mut Vec<f64>, indices: &mut Vec<f64>) -> () {
    let mut pts: Vec<f64> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (source.len() as f64)) {
            let x = source[i as usize].clone();
            let y = source[(i + 1.0_f64) as usize].clone();
            if (((pts.len() as f64) >= 2.0_f64)
                && (pts[((pts.len() as f64) - 2.0_f64) as usize].clone() == x))
                && (pts[((pts.len() as f64) - 1.0_f64) as usize].clone() == y)
            {
                {
                    i += 2.0_f64;
                    i.clone()
                };
                continue;
            }
            pts.extend(vec![x, y]);
            {
                i += 2.0_f64;
                i.clone()
            };
        }
    }
    let mut count =
        (__flight_js_to_i32((pts.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    if ((count >= 2.0_f64)
        && (pts[0.0_f64 as usize].clone() == pts[((count - 1.0_f64) * 2.0_f64) as usize].clone()))
        && (pts[1.0_f64 as usize].clone()
            == pts[(((count - 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone())
    {
        count -= 1.0_f64;
    }
    if (count < 3.0_f64) {
        return;
    }
    let base =
        (__flight_js_to_i32((vertices.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    {
        let mut i = 0.0_f64;
        while (i < count) {
            vertices.extend(vec![
                pts[(i * 2.0_f64) as usize].clone(),
                pts[((i * 2.0_f64) + 1.0_f64) as usize].clone(),
            ]);
            {
                i += 1.0;
                i
            };
        }
    }
    let mut twice_area = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < count) {
            let j = ((i + 1.0_f64) % count);
            twice_area += ((pts[(i * 2.0_f64) as usize].clone()
                * pts[((j * 2.0_f64) + 1.0_f64) as usize].clone())
                - (pts[(j * 2.0_f64) as usize].clone()
                    * pts[((i * 2.0_f64) + 1.0_f64) as usize].clone()));
            {
                i += 1.0;
                i
            };
        }
    }
    let mut ring: Vec<f64> = vec![];
    if (twice_area < 0.0_f64) {
        {
            let mut i = (count - 1.0_f64);
            while (i >= 0.0_f64) {
                ring.push(i);
                {
                    i -= 1.0;
                    i
                };
            }
        }
    } else {
        {
            let mut i = 0.0_f64;
            while (i < count) {
                ring.push(i);
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    let mut guard = ((ring.len() as f64) * (ring.len() as f64));
    while ((ring.len() as f64) > 3.0_f64)
        && ({
            guard -= 1.0;
            guard
        } > 0.0_f64)
    {
        let mut clipped = false;
        {
            let mut i = 0.0_f64;
            while (i < (ring.len() as f64)) {
                let a = ring
                    [(((i + (ring.len() as f64)) - 1.0_f64) % (ring.len() as f64)) as usize]
                    .clone();
                let b = ring[i as usize].clone();
                let c = ring[((i + 1.0_f64) % (ring.len() as f64)) as usize].clone();
                if is_ear(&pts, &ring, a, b, c) {
                    indices.extend(vec![(base + a), (base + b), (base + c)]);
                    {
                        let __flight_start = (i);
                        let __flight_count = (1.0_f64);
                        ring.splice(
                            (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                            vec![],
                        )
                        .collect::<Vec<_>>()
                    };
                    clipped = true;
                    break;
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
        if (!clipped) {
            break;
        }
    }
    if ((ring.len() as f64) == 3.0_f64) {
        indices.extend(vec![
            (base + ring[0.0_f64 as usize].clone()),
            (base + ring[1.0_f64 as usize].clone()),
            (base + ring[2.0_f64 as usize].clone()),
        ]);
    }
}

// Source: upstream/packages/path/src/tessellatePath.ts:84 (sha256:7bfe67f7cd7890c6a981933e38ffd50bd7874fc5c62f1e09658634f4b4154faa)
fn is_ear(contour: &Vec<f64>, ring: &Vec<f64>, a: f64, b: f64, c: f64) -> bool {
    let ax = contour[(a * 2.0_f64) as usize].clone();
    let ay = contour[((a * 2.0_f64) + 1.0_f64) as usize].clone();
    let bx = contour[(b * 2.0_f64) as usize].clone();
    let by = contour[((b * 2.0_f64) + 1.0_f64) as usize].clone();
    let cx = contour[(c * 2.0_f64) as usize].clone();
    let cy = contour[((c * 2.0_f64) + 1.0_f64) as usize].clone();
    if ((((bx - ax) * (cy - by)) - ((by - ay) * (cx - bx))) <= 0.0_f64) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < (ring.len() as f64)) {
            let p = ring[i as usize].clone();
            if ((p == a) || (p == b)) || (p == c) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            if is_point_in_triangle(
                contour[(p * 2.0_f64) as usize].clone(),
                contour[((p * 2.0_f64) + 1.0_f64) as usize].clone(),
                ax,
                ay,
                bx,
                by,
                cx,
                cy,
            ) {
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

// Source: upstream/packages/path/src/tessellatePath.ts:101 (sha256:1aa230d3361ec12723b37da91ced275aeb76aac532b4e5c733db1f2e6864b17c)
fn is_point_in_triangle(
    px: f64,
    py: f64,
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    cx: f64,
    cy: f64,
) -> bool {
    let d1 = (((px - bx) * (ay - by)) - ((ax - bx) * (py - by)));
    let d2 = (((px - cx) * (by - cy)) - ((bx - cx) * (py - cy)));
    let d3 = (((px - ax) * (cy - ay)) - ((cx - ax) * (py - ay)));
    let has_negative = ((d1 < 0.0_f64) || (d2 < 0.0_f64)) || (d3 < 0.0_f64);
    let has_positive = ((d1 > 0.0_f64) || (d2 > 0.0_f64)) || (d3 > 0.0_f64);
    return (!(has_negative) && (has_positive));
}
