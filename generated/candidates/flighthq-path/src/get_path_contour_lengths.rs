// @generated from upstream/packages/path/src/getPathContourLengths.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::flatten_path;
use flighthq_types::Path;

// Source: upstream/packages/path/src/getPathContourLengths.ts:7 (sha256:bab63d1489acd348e16f3e963e2d505c597f6fe738c4140bf86817d6bdd149e5)
pub fn get_path_contour_lengths(path: &Path, tolerance: Option<f64>) -> Vec<f64> {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(path, Some(tolerance));
    let mut lengths: Vec<f64> = vec![];
    {
        let mut ci = 0.0_f64;
        while (ci < (contours.len() as f64)) {
            lengths.push(contour_length(&contours[ci as usize]));
            {
                ci += 1.0;
                ci
            };
        }
    }
    return (lengths).clone();
}

// Source: upstream/packages/path/src/getPathContourLengths.ts:16 (sha256:f6a70b1f89e80b2bfda7ddb15d270defd713c353875ba915f408db536dc10fda)
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
