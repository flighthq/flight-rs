// @generated from upstream/packages/path/src/getPathLength.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::flatten_path;
use flighthq_types::Path;

// Source: upstream/packages/path/src/getPathLength.ts:8 (sha256:291630ce80f82a56af325741487a0646fe9cc556e493b3d41b2e2ffa1500583c)
pub fn get_path_length(path: &Path, tolerance: Option<f64>) -> f64 {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(path, Some(tolerance));
    let mut total = 0.0_f64;
    {
        let mut ci = 0.0_f64;
        while (ci < (contours.len() as f64)) {
            total += contour_length(&contours[ci as usize]);
            {
                ci += 1.0;
                ci
            };
        }
    }
    return total;
}

// Source: upstream/packages/path/src/getPathLength.ts:18 (sha256:f6a70b1f89e80b2bfda7ddb15d270defd713c353875ba915f408db536dc10fda)
fn contour_length(contour: &Vec<f64>) -> f64 {
    let mut len = 0.0_f64;
    {
        let mut i = 2.0_f64;
        while (i < (contour.len() as f64)) {
            let dx = (contour[i as usize].clone() - contour[(i - 2.0_f64) as usize].clone());
            let dy =
                (contour[(i + 1.0_f64) as usize].clone() - contour[(i - 1.0_f64) as usize].clone());
            len += ((dx * dx) + (dy * dy)).sqrt();
            {
                i += 2.0_f64;
                i
            };
        }
    }
    return len;
}
