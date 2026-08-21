// @generated from upstream/packages/path/src/cleanPath.ts; do not edit.
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

// Source: upstream/packages/path/src/cleanPath.ts:14 (sha256:82d9f57c18148c1e05cf8ab9ec9ebe79c3cab242dac98494be65f101ccc0fbfa)
pub fn clean_path(
    source: &Path,
    tolerance: f64,
    out: &mut Path,
    flatten_tolerance: Option<f64>,
) -> () {
    let flatten_tolerance = flatten_tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(source, Some(flatten_tolerance));
    let winding = (source.winding).clone();
    let tolerance_sq = (tolerance * tolerance);
    out.commands.clear();
    out.data.clear();
    out.winding = (winding).clone();
    for contour in (contours).iter().cloned() {
        let n = (__flight_js_to_i32((contour.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31))
            as f64;
        if (n < 2.0_f64) {
            continue;
        }
        let closed = (n >= 3.0_f64)
            && (within_tolerance(
                contour[0.0_f64 as usize].clone(),
                contour[1.0_f64 as usize].clone(),
                contour[((n * 2.0_f64) - 2.0_f64) as usize].clone(),
                contour[((n * 2.0_f64) - 1.0_f64) as usize].clone(),
                tolerance_sq,
            ));
        let count = if closed { (n - 1.0_f64) } else { n };
        let mut kept: Vec<f64> = vec![];
        {
            let mut i = 0.0_f64;
            while (i < count) {
                push_clean_vertex(
                    &mut kept,
                    contour[(i * 2.0_f64) as usize].clone(),
                    contour[((i * 2.0_f64) + 1.0_f64) as usize].clone(),
                    tolerance_sq,
                );
                {
                    i += 1.0;
                    i
                };
            }
        }
        if closed {
            collapse_closed_seam(&mut kept, tolerance_sq);
        }
        let kept_count =
            (__flight_js_to_i32((kept.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
        if if closed {
            (kept_count < 3.0_f64)
        } else {
            (kept_count < 2.0_f64)
        } {
            continue;
        }
        out.commands.push(PathCommand::MOVE_TO);
        out.data.extend(vec![
            kept[0.0_f64 as usize].clone(),
            kept[1.0_f64 as usize].clone(),
        ]);
        {
            let mut i = 1.0_f64;
            while (i < kept_count) {
                out.commands.push(PathCommand::LINE_TO);
                out.data.extend(vec![
                    kept[(i * 2.0_f64) as usize].clone(),
                    kept[((i * 2.0_f64) + 1.0_f64) as usize].clone(),
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

// Source: upstream/packages/path/src/cleanPath.ts:52 (sha256:4f7b84ecabcbcfe710a3002a5c7ffd1e4fdccaddf28e909563003a498c215444)
fn collapse_closed_seam(kept: &mut Vec<f64>, tolerance_sq: f64) -> () {
    let mut changed = true;
    while (changed)
        && ((__flight_js_to_i32((kept.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64
            > 3.0_f64)
    {
        changed = false;
        let last =
            (__flight_js_to_i32((kept.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
        if is_middle_removable(
            kept,
            (last - 2.0_f64),
            (last - 1.0_f64),
            0.0_f64,
            tolerance_sq,
        ) {
            {
                let __flight_length = kept.len().saturating_sub((2.0_f64) as usize);
                kept.truncate(__flight_length);
            };
            changed = true;
            continue;
        }
        if is_middle_removable(kept, (last - 1.0_f64), 0.0_f64, 1.0_f64, tolerance_sq) {
            {
                let __flight_start = (2.0_f64) as usize;
                let __flight_end = (kept.len() as f64) as usize;
                let __flight_target = (0.0_f64) as usize;
                kept.copy_within(__flight_start..__flight_end, __flight_target);
                kept.clone()
            };
            {
                let __flight_length = kept.len().saturating_sub((2.0_f64) as usize);
                kept.truncate(__flight_length);
            };
            changed = true;
        }
    }
}

// Source: upstream/packages/path/src/cleanPath.ts:75 (sha256:7ee91a1a18ad2fe3467a6e38998b4940023e1302ce9a5dc66d4a2fd7e178c33a)
fn is_middle_removable(kept: &Vec<f64>, prev: f64, mid: f64, next: f64, tolerance_sq: f64) -> bool {
    let px = kept[(prev * 2.0_f64) as usize].clone();
    let py = kept[((prev * 2.0_f64) + 1.0_f64) as usize].clone();
    let mx = kept[(mid * 2.0_f64) as usize].clone();
    let my = kept[((mid * 2.0_f64) + 1.0_f64) as usize].clone();
    let sx = kept[(next * 2.0_f64) as usize].clone();
    let sy = kept[((next * 2.0_f64) + 1.0_f64) as usize].clone();
    return is_redundant_middle(px, py, mx, my, sx, sy, tolerance_sq);
}

// Source: upstream/packages/path/src/cleanPath.ts:94 (sha256:1e03a65809f98f20f2be463c52a05122b030cac5b106bf6a9f83cc029e02b2fc)
fn is_redundant_middle(
    px: f64,
    py: f64,
    mx: f64,
    my: f64,
    sx: f64,
    sy: f64,
    tolerance_sq: f64,
) -> bool {
    let dx = (sx - px);
    let dy = (sy - py);
    let length_sq = ((dx * dx) + (dy * dy));
    if (length_sq <= tolerance_sq) {
        return true;
    }
    let cross = ((dx * (py - my)) - (dy * (px - mx)));
    return (((cross * cross) / length_sq) <= tolerance_sq);
}

// Source: upstream/packages/path/src/cleanPath.ts:114 (sha256:9a2c26c4a1b81de27c8d0730a3aeee79f649dded6ffbdf040e445d3432131189)
fn push_clean_vertex(kept: &mut Vec<f64>, x: f64, y: f64, tolerance_sq: f64) -> () {
    let k = (kept.len() as f64);
    if (k >= 2.0_f64)
        && (within_tolerance(
            kept[(k - 2.0_f64) as usize].clone(),
            kept[(k - 1.0_f64) as usize].clone(),
            x,
            y,
            tolerance_sq,
        ))
    {
        return;
    }
    while ((kept.len() as f64) >= 4.0_f64) {
        let m = (kept.len() as f64);
        if (!is_redundant_middle(
            kept[(m - 4.0_f64) as usize].clone(),
            kept[(m - 3.0_f64) as usize].clone(),
            kept[(m - 2.0_f64) as usize].clone(),
            kept[(m - 1.0_f64) as usize].clone(),
            x,
            y,
            tolerance_sq,
        )) {
            break;
        }
        {
            let __flight_length = kept.len().saturating_sub((2.0_f64) as usize);
            kept.truncate(__flight_length);
        };
    }
    kept.extend(vec![x, y]);
}

// Source: upstream/packages/path/src/cleanPath.ts:126 (sha256:55a9367f0d4275fccdbb2875f58abe480bf90a3a6711393c9b62062a05bb3efc)
fn within_tolerance(ax: f64, ay: f64, bx: f64, by: f64, tolerance_sq: f64) -> bool {
    let dx = (ax - bx);
    let dy = (ay - by);
    return (((dx * dx) + (dy * dy)) <= tolerance_sq);
}
