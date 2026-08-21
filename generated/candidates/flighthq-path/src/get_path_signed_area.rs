// @generated from upstream/packages/path/src/getPathSignedArea.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::flatten_path;
use flighthq_types::Path;

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

// Source: upstream/packages/path/src/getPathSignedArea.ts:7 (sha256:3798ebddcdd2270f8c28759e05df4904617a103f7b8fa4976f0221a43f3a14b3)
pub fn get_path_contour_orientation(path: &Path, tolerance: Option<f64>) -> String {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(path, Some(tolerance));
    if ((contours.len() as f64) == 0.0_f64) {
        return "degenerate".to_owned();
    }
    let area = shoelace_area(&contours[0.0_f64 as usize]);
    if (area > 0.0_f64) {
        return "ccw".to_owned();
    }
    if (area < 0.0_f64) {
        return "cw".to_owned();
    }
    return "degenerate".to_owned();
}

// Source: upstream/packages/path/src/getPathSignedArea.ts:22 (sha256:90e05cb29a92c41d3d95a45a4b4caa17b8e4a8edb9e872b0a6f4c6a546fd5650)
pub fn get_path_signed_area(path: &Path, tolerance: Option<f64>) -> f64 {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(path, Some(tolerance));
    let mut total = 0.0_f64;
    {
        let mut ci = 0.0_f64;
        while (ci < (contours.len() as f64)) {
            total += shoelace_area(&contours[ci as usize]);
            {
                ci += 1.0;
                ci
            };
        }
    }
    return total;
}

// Source: upstream/packages/path/src/getPathSignedArea.ts:33 (sha256:534250229e9bc759555aacdfe4b1910db6d4fd2232cd5ad4f8cae608e58a9f9e)
fn shoelace_area(contour: &Vec<f64>) -> f64 {
    let n =
        (__flight_js_to_i32((contour.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    if (n < 3.0_f64) {
        return 0.0_f64;
    }
    let mut area = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < n) {
            let j = ((i + 1.0_f64) % n);
            area += (contour[(i * 2.0_f64) as usize].clone()
                * contour[((j * 2.0_f64) + 1.0_f64) as usize].clone());
            area -= (contour[(j * 2.0_f64) as usize].clone()
                * contour[((i * 2.0_f64) + 1.0_f64) as usize].clone());
            {
                i += 1.0;
                i
            };
        }
    }
    return (area / 2.0_f64);
}
