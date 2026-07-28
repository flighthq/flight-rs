// @generated from upstream/packages/path/src/getPathNearestPoint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::flatten_path;
use flighthq_types::{Path, Vector2Like};

// Source: upstream/packages/path/src/getPathNearestPoint.ts:8 (sha256:01ec99fd2926dd13e261c784edc056b21ff10949f103623e973b54834c126a96)
pub fn get_path_nearest_point(
    path: &Path,
    px: f64,
    py: f64,
    out: &mut Vector2Like,
    tolerance: Option<f64>,
) -> f64 {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(path, Some(tolerance));
    let mut best_dist_sq = f64::INFINITY;
    let mut best_x = 0.0_f64;
    let mut best_y = 0.0_f64;
    {
        let mut ci = 0.0_f64;
        while (ci < (contours.len() as f64)) {
            let contour = contours[ci as usize].clone();
            {
                let mut i = 2.0_f64;
                while (i < (contour.len() as f64)) {
                    let ax = contour[(i - 2.0_f64) as usize].clone();
                    let ay = contour[(i - 1.0_f64) as usize].clone();
                    let bx = contour[i as usize].clone();
                    let by = contour[(i + 1.0_f64) as usize].clone();
                    let dx = (bx - ax);
                    let dy = (by - ay);
                    let len_sq = ((dx * dx) + (dy * dy));
                    let mut t: f64;
                    if (len_sq == 0.0_f64) {
                        t = 0.0_f64;
                    } else {
                        t = ((((px - ax) * dx) + ((py - ay) * dy)) / len_sq);
                        if (t < 0.0_f64) {
                            t = 0.0_f64;
                        } else {
                            if (t > 1.0_f64) {
                                t = 1.0_f64;
                            }
                        }
                    }
                    let cx = (ax + (t * dx));
                    let cy = (ay + (t * dy));
                    let dist_sq = (((px - cx) * (px - cx)) + ((py - cy) * (py - cy)));
                    if (dist_sq < best_dist_sq) {
                        best_dist_sq = dist_sq;
                        best_x = cx;
                        best_y = cy;
                    }
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
    if (best_dist_sq == f64::INFINITY) {
        return (-1.0_f64);
    }
    out.x = best_x;
    out.y = best_y;
    return (best_dist_sq).sqrt();
}
