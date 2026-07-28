// @generated from upstream/packages/path/src/dashPath.ts; do not edit.
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

// Source: upstream/packages/path/src/dashPath.ts:11 (sha256:74873cb3e7b04da6eb88231fcddefcf0fbbc857fe7fe4a82c602400b4a7b1a29)
pub fn dash_path(
    source: &Path,
    dash: &Vec<f64>,
    dash_offset: f64,
    out: &mut Path,
    tolerance: Option<f64>,
) -> () {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    out.commands.clear();
    out.data.clear();
    out.winding = (source.winding).clone();
    let total_dash_length = dash_total(dash);
    if (total_dash_length <= 0.0_f64) {
        copy_commands(source, out);
        return;
    }
    let contours = flatten_path(source, Some(tolerance));
    for contour in (contours).iter().cloned() {
        apply_dash_to_contour(&contour, dash, dash_offset, total_dash_length, out);
    }
}

// Source: upstream/packages/path/src/dashPath.ts:34 (sha256:c99e1362184453cbc8a070ad65ee84eeab9a45623d9ad7e8c57424f17a129a6a)
fn apply_dash_to_contour(
    pts: &Vec<f64>,
    dash: &Vec<f64>,
    dash_offset: f64,
    total_dash_length: f64,
    out: &mut Path,
) -> () {
    let n = (__flight_js_to_i32((pts.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    if (n < 2.0_f64) {
        return;
    }
    let offset = (((dash_offset % total_dash_length) + total_dash_length) % total_dash_length);
    let mut dash_index = 0.0_f64;
    let mut remaining = 0.0_f64;
    let mut is_on = true;
    {
        let mut acc = 0.0_f64;
        {
            let mut i = 0.0_f64;
            while (i < (dash.len() as f64)) {
                if ((acc + dash[i as usize].clone()) > offset) {
                    dash_index = i;
                    remaining = (dash[i as usize].clone() - (offset - acc));
                    is_on = ((i % 2.0_f64) == 0.0_f64);
                    break;
                }
                acc += dash[i as usize].clone();
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    let mut seg_started = false;
    {
        let mut i = 0.0_f64;
        while (i < (n - 1.0_f64)) {
            let x0 = pts[(i * 2.0_f64) as usize].clone();
            let y0 = pts[((i * 2.0_f64) + 1.0_f64) as usize].clone();
            let x1 = pts[((i + 1.0_f64) * 2.0_f64) as usize].clone();
            let y1 = pts[(((i + 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone();
            let dx = (x1 - x0);
            let dy = (y1 - y0);
            let seg_len = ((dx * dx) + (dy * dy)).sqrt();
            if (is_on && (!seg_started)) {
                out.commands.push(PathCommand::MOVE_TO);
                out.data.extend(vec![x0, y0]);
                seg_started = true;
            }
            let mut consumed = 0.0_f64;
            while (consumed < seg_len) {
                let step = (remaining).min((seg_len - consumed));
                let t = if (seg_len > 0.0_f64) {
                    ((consumed + step) / seg_len)
                } else {
                    0.0_f64
                };
                let ix = (x0 + (t * dx));
                let iy = (y0 + (t * dy));
                if is_on {
                    if (!seg_started) {
                        let t_start = if (seg_len > 0.0_f64) {
                            (consumed / seg_len)
                        } else {
                            0.0_f64
                        };
                        out.commands.push(PathCommand::MOVE_TO);
                        out.data
                            .extend(vec![(x0 + (t_start * dx)), (y0 + (t_start * dy))]);
                        seg_started = true;
                    }
                    out.commands.push(PathCommand::LINE_TO);
                    out.data.extend(vec![ix, iy]);
                }
                consumed += step;
                remaining -= step;
                if (remaining <= 1e-10_f64) {
                    dash_index = ((dash_index + 1.0_f64) % (dash.len() as f64));
                    remaining = dash[dash_index as usize].clone();
                    let was_on = is_on;
                    is_on = ((dash_index % 2.0_f64) == 0.0_f64);
                    if (was_on && (!is_on)) {
                        seg_started = false;
                    }
                    if ((!was_on) && is_on) {
                        out.commands.push(PathCommand::MOVE_TO);
                        out.data.extend(vec![ix, iy]);
                        seg_started = true;
                    }
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/path/src/dashPath.ts:115 (sha256:29f693b10322c3b61467f938d0c479f5bb212c1ff55852114c2b3e99e1cad537)
fn copy_commands(source: &Path, out: &mut Path) -> () {
    {
        let mut i = 0.0_f64;
        while (i < (source.commands.len() as f64)) {
            out.commands.push(source.commands[i as usize].clone());
            {
                i += 1.0;
                i
            };
        }
    }
    {
        let mut i = 0.0_f64;
        while (i < (source.data.len() as f64)) {
            out.data.push(source.data[i as usize].clone());
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/path/src/dashPath.ts:124 (sha256:88a03d802f60baee87aee1955b6fdaf30f899d2ad57ad833a22ab76d4cd48548)
fn dash_total(dash: &Vec<f64>) -> f64 {
    let mut total = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (dash.len() as f64)) {
            total += dash[i as usize].clone();
            {
                i += 1.0;
                i
            };
        }
    }
    return total;
}
