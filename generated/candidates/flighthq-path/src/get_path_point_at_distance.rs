// @generated from upstream/packages/path/src/getPathPointAtDistance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::flatten_path;
use flighthq_types::{Path, Vector2Like};

// Source: upstream/packages/path/src/getPathPointAtDistance.ts:13 (sha256:8c7546c2349ce464710c75c083dcc06164047cae536972cdc2c6185a4c170a9f)
pub fn get_path_point_at_distance(
    path: &Path,
    distance: f64,
    out: &mut Vector2Like,
    tolerance: Option<f64>,
) -> bool {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(path, Some(tolerance));
    return sample_path_point(&contours, distance, out);
}

// Source: upstream/packages/path/src/getPathPointAtDistance.ts:25 (sha256:1f1e283e24d8384961486615f91b441303d8cdd80f4247927b9c08992eb7ff9d)
pub fn get_path_position_at_distance(
    path: &Path,
    distance: f64,
    point_out: &mut Vector2Like,
    tangent_out: &mut Vector2Like,
    tolerance: Option<f64>,
) -> bool {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(path, Some(tolerance));
    let has_point = sample_path_point(&contours, distance, point_out);
    sample_path_tangent(&contours, distance, tangent_out);
    return has_point;
}

// Source: upstream/packages/path/src/getPathPointAtDistance.ts:42 (sha256:4a73b6fbfa5e73a614f27342d4674baca0ae183215884f266dd3bad7402a75b6)
pub fn get_path_tangent_at_distance(
    path: &Path,
    distance: f64,
    out: &mut Vector2Like,
    tolerance: Option<f64>,
) -> bool {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(path, Some(tolerance));
    return sample_path_tangent(&contours, distance, out);
}

// Source: upstream/packages/path/src/getPathPointAtDistance.ts:53 (sha256:49565304f65d0d7b54f3e740046d53e24fac3a07923f58d47ac83492bc950e5f)
fn sample_path_point(contours: &Vec<Vec<f64>>, distance: f64, out: &mut Vector2Like) -> bool {
    if ((contours.len() as f64) == 0.0_f64) {
        return false;
    }
    let mut remaining = distance;
    {
        let mut ci = 0.0_f64;
        while (ci < (contours.len() as f64)) {
            let contour = contours[ci as usize].clone();
            if ((contour.len() as f64) < 2.0_f64) {
                {
                    ci += 1.0;
                    ci
                };
                continue;
            }
            if (remaining <= 0.0_f64) {
                out.x = contour[0.0_f64 as usize].clone();
                out.y = contour[1.0_f64 as usize].clone();
                return true;
            }
            {
                let mut i = 2.0_f64;
                while (i < (contour.len() as f64)) {
                    let dx =
                        (contour[i as usize].clone() - contour[(i - 2.0_f64) as usize].clone());
                    let dy = (contour[(i + 1.0_f64) as usize].clone()
                        - contour[(i - 1.0_f64) as usize].clone());
                    let seg_len = ((dx * dx) + (dy * dy)).sqrt();
                    if (remaining <= seg_len) {
                        let t = if (seg_len > 0.0_f64) {
                            (remaining / seg_len)
                        } else {
                            0.0_f64
                        };
                        out.x = (contour[(i - 2.0_f64) as usize].clone() + (t * dx));
                        out.y = (contour[(i - 1.0_f64) as usize].clone() + (t * dy));
                        return true;
                    }
                    remaining -= seg_len;
                    {
                        i += 2.0_f64;
                        i
                    };
                }
            }
            {
                ci += 1.0;
                ci
            };
        }
    }
    let last = contours[((contours.len() as f64) - 1.0_f64) as usize].clone();
    out.x = last[((last.len() as f64) - 2.0_f64) as usize].clone();
    out.y = last[((last.len() as f64) - 1.0_f64) as usize].clone();
    return true;
}

// Source: upstream/packages/path/src/getPathPointAtDistance.ts:87 (sha256:b5605ccac6e252c2d0ac6e1af63313a9bbee322c78aba2d8f50a57dee92e6f78)
fn sample_path_tangent(contours: &Vec<Vec<f64>>, distance: f64, out: &mut Vector2Like) -> bool {
    if ((contours.len() as f64) == 0.0_f64) {
        out.x = 1.0_f64;
        out.y = 0.0_f64;
        return false;
    }
    let mut remaining = distance;
    let mut last_tx = 1.0_f64;
    let mut last_ty = 0.0_f64;
    {
        let mut ci = 0.0_f64;
        while (ci < (contours.len() as f64)) {
            let contour = contours[ci as usize].clone();
            if ((contour.len() as f64) < 4.0_f64) {
                {
                    ci += 1.0;
                    ci
                };
                continue;
            }
            {
                let mut i = 2.0_f64;
                while (i < (contour.len() as f64)) {
                    let dx =
                        (contour[i as usize].clone() - contour[(i - 2.0_f64) as usize].clone());
                    let dy = (contour[(i + 1.0_f64) as usize].clone()
                        - contour[(i - 1.0_f64) as usize].clone());
                    let seg_len = ((dx * dx) + (dy * dy)).sqrt();
                    if (seg_len > 0.0_f64) {
                        let inv_len = (1.0_f64 / seg_len);
                        last_tx = (dx * inv_len);
                        last_ty = (dy * inv_len);
                    }
                    if (remaining <= seg_len) {
                        out.x = last_tx;
                        out.y = last_ty;
                        return true;
                    }
                    remaining -= seg_len;
                    {
                        i += 2.0_f64;
                        i
                    };
                }
            }
            {
                ci += 1.0;
                ci
            };
        }
    }
    out.x = last_tx;
    out.y = last_ty;
    return true;
}
