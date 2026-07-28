// @generated from upstream/packages/path/src/decimatePath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::flatten_path;
use flighthq_types::{Path, PathCommand};

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

// Source: upstream/packages/path/src/decimatePath.ts:14 (sha256:cc3c7554d3e941997a02028775e011b5c80d15cff87c47296907ce23ac63d756)
pub fn decimate_path(
    source: &Path,
    tolerance: f64,
    out: &mut Path,
    flatten_tolerance: Option<f64>,
) -> () {
    let flatten_tolerance = flatten_tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(source, Some(flatten_tolerance));
    out.commands.clear();
    out.data.clear();
    out.winding = (source.winding).clone();
    for contour in (contours).iter().cloned() {
        let n = (__flight_js_to_i32((contour.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31))
            as f64;
        if (n < 2.0_f64) {
            continue;
        }
        let closed = (((n >= 3.0_f64)
            && (contour[0.0_f64 as usize].clone()
                == contour[((contour.len() as f64) - 2.0_f64) as usize].clone()))
            && (contour[1.0_f64 as usize].clone()
                == contour[((contour.len() as f64) - 1.0_f64) as usize].clone()));
        let last = if closed { (n - 1.0_f64) } else { n };
        let mut keep = vec![0_u8; (last) as usize];
        keep[0.0_f64 as usize] = (1.0_f64) as u8;
        keep[(last - 1.0_f64) as usize] = (1.0_f64) as u8;
        douglas_peucker(
            &contour,
            0.0_f64,
            (last - 1.0_f64),
            (tolerance * tolerance),
            &mut keep,
        );
        let mut first = true;
        {
            let mut i = 0.0_f64;
            while (i < last) {
                if (!(keep[i as usize] as f64)) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                if first {
                    out.commands.push(PathCommand::MOVE_TO);
                    first = false;
                } else {
                    out.commands.push(PathCommand::LINE_TO);
                }
                out.data.extend(vec![
                    contour[(i * 2.0_f64) as usize].clone(),
                    contour[((i * 2.0_f64) + 1.0_f64) as usize].clone(),
                ]);
                {
                    i += 1.0;
                    i
                };
            }
        }
        if closed {
            out.commands.push(PathCommand::CLOSE);
        }
    }
}

// Source: upstream/packages/path/src/decimatePath.ts:48 (sha256:9c12364f6846a04e441fd3d3fa6b3da1abd5b9e33095dd4bfa1a0456736fd624)
fn douglas_peucker(
    pts: &Vec<f64>,
    first: f64,
    last: f64,
    tolerance_sq: f64,
    keep: &mut Vec<u8>,
) -> () {
    if ((last - first) < 2.0_f64) {
        return;
    }
    let x0 = pts[(first * 2.0_f64) as usize].clone();
    let y0 = pts[((first * 2.0_f64) + 1.0_f64) as usize].clone();
    let x1 = pts[(last * 2.0_f64) as usize].clone();
    let y1 = pts[((last * 2.0_f64) + 1.0_f64) as usize].clone();
    let dx = (x1 - x0);
    let dy = (y1 - y0);
    let len_sq = ((dx * dx) + (dy * dy));
    let mut max_dist_sq = 0.0_f64;
    let mut max_idx = first;
    {
        let mut i = (first + 1.0_f64);
        while (i < last) {
            let px = pts[(i * 2.0_f64) as usize].clone();
            let py = pts[((i * 2.0_f64) + 1.0_f64) as usize].clone();
            let mut dist_sq: f64;
            if (len_sq == 0.0_f64) {
                let ax = (px - x0);
                let ay = (py - y0);
                dist_sq = ((ax * ax) + (ay * ay));
            } else {
                let cross = ((dx * (y0 - py)) - (dy * (x0 - px)));
                dist_sq = ((cross * cross) / len_sq);
            }
            if (dist_sq > max_dist_sq) {
                max_dist_sq = dist_sq;
                max_idx = i;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    if (max_dist_sq > tolerance_sq) {
        keep[max_idx as usize] = (1.0_f64) as u8;
        douglas_peucker(pts, first, max_idx, tolerance_sq, keep);
        douglas_peucker(pts, max_idx, last, tolerance_sq, keep);
    }
}
