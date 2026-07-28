// @generated from upstream/packages/path/src/fitPathCurves.ts; do not edit.
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

// Source: upstream/packages/path/src/fitPathCurves.ts:11 (sha256:2674c0681e8fd88cb3c3e069873df2e7cc2f03daaa392e0bb353e6cc8f7004de)
pub fn fit_path_curves(
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
    let tolerance_sq = (tolerance * tolerance);
    for contour in (contours).iter().cloned() {
        let n = (__flight_js_to_i32((contour.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31))
            as f64;
        if (n < 2.0_f64) {
            continue;
        }
        let closed = ((n >= 3.0_f64)
            && (contour[0.0_f64 as usize].clone()
                == contour[((contour.len() as f64) - 2.0_f64) as usize].clone()))
            && (contour[1.0_f64 as usize].clone()
                == contour[((contour.len() as f64) - 1.0_f64) as usize].clone());
        let pts = if closed {
            (contour)[(0.0_f64) as usize..((n - 1.0_f64) * 2.0_f64) as usize].to_vec()
        } else {
            (contour).clone()
        };
        let pn =
            (__flight_js_to_i32((pts.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
        if (pn < 2.0_f64) {
            continue;
        }
        if (pn == 2.0_f64) {
            out.commands.push(PathCommand::MOVE_TO);
            out.data.extend(vec![
                pts[0.0_f64 as usize].clone(),
                pts[1.0_f64 as usize].clone(),
            ]);
            out.commands.push(PathCommand::LINE_TO);
            out.data.extend(vec![
                pts[2.0_f64 as usize].clone(),
                pts[3.0_f64 as usize].clone(),
            ]);
            if closed {
                out.commands.push(PathCommand::CLOSE);
            }
            continue;
        }
        let corners = find_corners(&pts, pn);
        out.commands.push(PathCommand::MOVE_TO);
        out.data.extend(vec![
            pts[0.0_f64 as usize].clone(),
            pts[1.0_f64 as usize].clone(),
        ]);
        {
            let mut ci = 0.0_f64;
            while (ci < ((corners.len() as f64) - 1.0_f64)) {
                let first = corners[ci as usize].clone();
                let last = corners[(ci + 1.0_f64) as usize].clone();
                if ((last - first) < 2.0_f64) {
                    out.commands.push(PathCommand::LINE_TO);
                    out.data.extend(vec![
                        pts[(last * 2.0_f64) as usize].clone(),
                        pts[((last * 2.0_f64) + 1.0_f64) as usize].clone(),
                    ]);
                    {
                        ci += 1.0;
                        ci
                    };
                    continue;
                }
                let t_hat1 = compute_left_tangent(&pts, first);
                let t_hat2 = compute_right_tangent(&pts, last);
                fit_cubic(&pts, first, last, &t_hat1, &t_hat2, tolerance_sq, out);
                {
                    ci += 1.0;
                    ci
                };
            }
        }
        if closed {
            out.commands.push(PathCommand::CLOSE);
        }
    }
}

// Source: upstream/packages/path/src/fitPathCurves.ts:57 (sha256:85dbd14f2e8eab603182b6e250acd7d4758f76aacfc4efbeec51012d287ff342)
fn find_corners(pts: &Vec<f64>, n: f64) -> Vec<f64> {
    let mut corners = vec![0.0_f64];
    {
        let mut i = 1.0_f64;
        while (i < (n - 1.0_f64)) {
            let dx0 = (pts[(i * 2.0_f64) as usize].clone()
                - pts[((i - 1.0_f64) * 2.0_f64) as usize].clone());
            let dy0 = (pts[((i * 2.0_f64) + 1.0_f64) as usize].clone()
                - pts[(((i - 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone());
            let dx1 = (pts[((i + 1.0_f64) * 2.0_f64) as usize].clone()
                - pts[(i * 2.0_f64) as usize].clone());
            let dy1 = (pts[(((i + 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone()
                - pts[((i * 2.0_f64) + 1.0_f64) as usize].clone());
            let len0 = ((dx0 * dx0) + (dy0 * dy0)).sqrt();
            let len1 = ((dx1 * dx1) + (dy1 * dy1)).sqrt();
            if (len0 == 0.0_f64) || (len1 == 0.0_f64) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let dot = (((dx0 * dx1) + (dy0 * dy1)) / (len0 * len1));
            if (dot < 0.5_f64) {
                corners.push(i);
            }
            {
                i += 1.0;
                i
            };
        }
    }
    corners.push((n - 1.0_f64));
    return corners;
}

// Source: upstream/packages/path/src/fitPathCurves.ts:74 (sha256:2f32605ecbce7f49cea5166d924e769451082080f06727dc868cd5d32ea799ad)
fn compute_left_tangent(pts: &Vec<f64>, idx: f64) -> Vec<f64> {
    let dx =
        (pts[((idx + 1.0_f64) * 2.0_f64) as usize].clone() - pts[(idx * 2.0_f64) as usize].clone());
    let dy = (pts[(((idx + 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone()
        - pts[((idx * 2.0_f64) + 1.0_f64) as usize].clone());
    let len = ((dx * dx) + (dy * dy)).sqrt();
    return if (len > 0.0_f64) {
        vec![(dx / len), (dy / len)]
    } else {
        vec![1.0_f64, 0.0_f64]
    };
}

// Source: upstream/packages/path/src/fitPathCurves.ts:81 (sha256:4cf683dd866a0e58e2b57e62d1148d414d7dbd0fc6e898c7fb19ff5d476dff00)
fn compute_right_tangent(pts: &Vec<f64>, idx: f64) -> Vec<f64> {
    let dx =
        (pts[((idx - 1.0_f64) * 2.0_f64) as usize].clone() - pts[(idx * 2.0_f64) as usize].clone());
    let dy = (pts[(((idx - 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone()
        - pts[((idx * 2.0_f64) + 1.0_f64) as usize].clone());
    let len = ((dx * dx) + (dy * dy)).sqrt();
    return if (len > 0.0_f64) {
        vec![(dx / len), (dy / len)]
    } else {
        vec![(-1.0_f64), 0.0_f64]
    };
}

// Source: upstream/packages/path/src/fitPathCurves.ts:88 (sha256:0698dfdd19285016a94e7efe092c127ff1d70bd3d9e9ef1c69ef2bb8d18dd2f7)
fn chord_length_parameterize(pts: &Vec<f64>, first: f64, last: f64) -> Vec<f64> {
    let mut u: Vec<f64> = vec![0.0_f64];
    {
        let mut i = (first + 1.0_f64);
        while (i <= last) {
            let dx = (pts[(i * 2.0_f64) as usize].clone()
                - pts[((i - 1.0_f64) * 2.0_f64) as usize].clone());
            let dy = (pts[((i * 2.0_f64) + 1.0_f64) as usize].clone()
                - pts[(((i - 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone());
            u.push(
                (u[((u.len() as f64) - 1.0_f64) as usize].clone() + ((dx * dx) + (dy * dy)).sqrt()),
            );
            {
                i += 1.0;
                i
            };
        }
    }
    let total = u[((u.len() as f64) - 1.0_f64) as usize].clone();
    if (total > 0.0_f64) {
        {
            let mut i = 1.0_f64;
            while (i < (u.len() as f64)) {
                u[i as usize] /= total;
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    return u;
}

// Source: upstream/packages/path/src/fitPathCurves.ts:102 (sha256:6033cfbd8e30a64c3b96b43cf74e03f98736e68bdbff11d02ac26ccb1a1a1579)
fn fit_cubic(
    pts: &Vec<f64>,
    first: f64,
    last: f64,
    t_hat1: &Vec<f64>,
    t_hat2: &Vec<f64>,
    tolerance_sq: f64,
    out: &mut Path,
) -> () {
    let n_pts = ((last - first) + 1.0_f64);
    if (n_pts == 2.0_f64) {
        let dist = ((pts[(last * 2.0_f64) as usize].clone()
            - pts[(first * 2.0_f64) as usize].clone())
        .powf(2.0_f64)
            + (pts[((last * 2.0_f64) + 1.0_f64) as usize].clone()
                - pts[((first * 2.0_f64) + 1.0_f64) as usize].clone())
            .powf(2.0_f64))
        .sqrt();
        let d = (dist / 3.0_f64);
        out.commands.push(PathCommand::CUBIC_CURVE_TO);
        out.data.extend(vec![
            (pts[(first * 2.0_f64) as usize].clone() + (t_hat1[0.0_f64 as usize].clone() * d)),
            (pts[((first * 2.0_f64) + 1.0_f64) as usize].clone()
                + (t_hat1[1.0_f64 as usize].clone() * d)),
            (pts[(last * 2.0_f64) as usize].clone() + (t_hat2[0.0_f64 as usize].clone() * d)),
            (pts[((last * 2.0_f64) + 1.0_f64) as usize].clone()
                + (t_hat2[1.0_f64 as usize].clone() * d)),
            pts[(last * 2.0_f64) as usize].clone(),
            pts[((last * 2.0_f64) + 1.0_f64) as usize].clone(),
        ]);
        return;
    }
    let mut u = chord_length_parameterize(pts, first, last);
    let max_iterations = 4.0_f64;
    {
        let mut iter = 0.0_f64;
        while (iter <= max_iterations) {
            let bezier = generate_bezier(pts, first, last, &u, t_hat1, t_hat2);
            let __destructure0 = compute_max_error(pts, first, last, &bezier, &u);
            let max_err = __destructure0[0.0_f64 as usize].clone();
            let split_point = __destructure0[1.0_f64 as usize].clone();
            if (max_err < tolerance_sq) {
                out.commands.push(PathCommand::CUBIC_CURVE_TO);
                out.data.extend(vec![
                    bezier[2.0_f64 as usize].clone(),
                    bezier[3.0_f64 as usize].clone(),
                    bezier[4.0_f64 as usize].clone(),
                    bezier[5.0_f64 as usize].clone(),
                    bezier[6.0_f64 as usize].clone(),
                    bezier[7.0_f64 as usize].clone(),
                ]);
                return;
            }
            if (iter < max_iterations) {
                u = reparameterize(pts, first, last, &u, &bezier);
            } else {
                let t_hat_center = compute_center_tangent(pts, split_point);
                fit_cubic(
                    pts,
                    first,
                    split_point,
                    t_hat1,
                    &vec![
                        (-t_hat_center[0.0_f64 as usize].clone()),
                        (-t_hat_center[1.0_f64 as usize].clone()),
                    ],
                    tolerance_sq,
                    out,
                );
                fit_cubic(
                    pts,
                    split_point,
                    last,
                    &t_hat_center,
                    t_hat2,
                    tolerance_sq,
                    out,
                );
            }
            {
                iter += 1.0;
                iter
            };
        }
    }
}

// Source: upstream/packages/path/src/fitPathCurves.ts:150 (sha256:1111212cd2027e00e86570cb05a75cab0da5886c94acb0077123c078712b5f4a)
fn generate_bezier(
    pts: &Vec<f64>,
    first: f64,
    last: f64,
    u: &Vec<f64>,
    t_hat1: &Vec<f64>,
    t_hat2: &Vec<f64>,
) -> Vec<f64> {
    let n_pts = ((last - first) + 1.0_f64);
    let mut c00 = 0.0_f64;
    let mut c01 = 0.0_f64;
    let mut c11 = 0.0_f64;
    let mut x0 = 0.0_f64;
    let mut x1 = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < n_pts) {
            let t = u[i as usize].clone();
            let b1 = (((3.0_f64 * t) * (1.0_f64 - t)) * (1.0_f64 - t));
            let b2 = (((3.0_f64 * t) * t) * (1.0_f64 - t));
            let a1x = (t_hat1[0.0_f64 as usize].clone() * b1);
            let a1y = (t_hat1[1.0_f64 as usize].clone() * b1);
            let a2x = (t_hat2[0.0_f64 as usize].clone() * b2);
            let a2y = (t_hat2[1.0_f64 as usize].clone() * b2);
            c00 += ((a1x * a1x) + (a1y * a1y));
            c01 += ((a1x * a2x) + (a1y * a2y));
            c11 += ((a2x * a2x) + (a2y * a2y));
            let b0 = (((1.0_f64 - t) * (1.0_f64 - t)) * (1.0_f64 - t));
            let b3 = ((t * t) * t);
            let tmpx = (pts[((first + i) * 2.0_f64) as usize].clone()
                - ((((pts[(first * 2.0_f64) as usize].clone() * b0)
                    + (pts[(first * 2.0_f64) as usize].clone() * b1))
                    + (pts[(last * 2.0_f64) as usize].clone() * b2))
                    + (pts[(last * 2.0_f64) as usize].clone() * b3)));
            let tmpy = (pts[(((first + i) * 2.0_f64) + 1.0_f64) as usize].clone()
                - ((((pts[((first * 2.0_f64) + 1.0_f64) as usize].clone() * b0)
                    + (pts[((first * 2.0_f64) + 1.0_f64) as usize].clone() * b1))
                    + (pts[((last * 2.0_f64) + 1.0_f64) as usize].clone() * b2))
                    + (pts[((last * 2.0_f64) + 1.0_f64) as usize].clone() * b3)));
            x0 += ((a1x * tmpx) + (a1y * tmpy));
            x1 += ((a2x * tmpx) + (a2y * tmpy));
            {
                i += 1.0;
                i
            };
        }
    }
    let det = ((c00 * c11) - (c01 * c01));
    let mut alpha1: f64;
    let mut alpha2: f64;
    if ((det).abs() < 1e-12_f64) {
        let dist = ((pts[(last * 2.0_f64) as usize].clone()
            - pts[(first * 2.0_f64) as usize].clone())
        .powf(2.0_f64)
            + (pts[((last * 2.0_f64) + 1.0_f64) as usize].clone()
                - pts[((first * 2.0_f64) + 1.0_f64) as usize].clone())
            .powf(2.0_f64))
        .sqrt();
        alpha1 = {
            alpha2 = (dist / 3.0_f64);
            alpha2
        };
    } else {
        alpha1 = (((c11 * x0) - (c01 * x1)) / det);
        alpha2 = (((c00 * x1) - (c01 * x0)) / det);
    }
    let seg_length = ((pts[(last * 2.0_f64) as usize].clone()
        - pts[(first * 2.0_f64) as usize].clone())
    .powf(2.0_f64)
        + (pts[((last * 2.0_f64) + 1.0_f64) as usize].clone()
            - pts[((first * 2.0_f64) + 1.0_f64) as usize].clone())
        .powf(2.0_f64))
    .sqrt();
    let epsilon = (0.000001_f64 * seg_length);
    if (alpha1 < epsilon) || (alpha2 < epsilon) {
        alpha1 = {
            alpha2 = (seg_length / 3.0_f64);
            alpha2
        };
    }
    return vec![
        pts[(first * 2.0_f64) as usize].clone(),
        pts[((first * 2.0_f64) + 1.0_f64) as usize].clone(),
        (pts[(first * 2.0_f64) as usize].clone() + (t_hat1[0.0_f64 as usize].clone() * alpha1)),
        (pts[((first * 2.0_f64) + 1.0_f64) as usize].clone()
            + (t_hat1[1.0_f64 as usize].clone() * alpha1)),
        (pts[(last * 2.0_f64) as usize].clone() + (t_hat2[0.0_f64 as usize].clone() * alpha2)),
        (pts[((last * 2.0_f64) + 1.0_f64) as usize].clone()
            + (t_hat2[1.0_f64 as usize].clone() * alpha2)),
        pts[(last * 2.0_f64) as usize].clone(),
        pts[((last * 2.0_f64) + 1.0_f64) as usize].clone(),
    ];
}

// Source: upstream/packages/path/src/fitPathCurves.ts:220 (sha256:a3e1414ff952b6c63728c88d64dbf0b10ba7144b5eb8648894bffc1bd598fc58)
fn compute_max_error(
    pts: &Vec<f64>,
    first: f64,
    last: f64,
    bezier: &Vec<f64>,
    u: &Vec<f64>,
) -> Vec<f64> {
    let mut max_dist = 0.0_f64;
    let mut split_point = (__flight_js_to_i32(((last - first) + 1.0_f64))
        >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    {
        let mut i = 1.0_f64;
        while (i < (last - first)) {
            let t = u[i as usize].clone();
            let mt = (1.0_f64 - t);
            let bx = ((((((mt * mt) * mt) * bezier[0.0_f64 as usize].clone())
                + ((((3.0_f64 * mt) * mt) * t) * bezier[2.0_f64 as usize].clone()))
                + ((((3.0_f64 * mt) * t) * t) * bezier[4.0_f64 as usize].clone()))
                + (((t * t) * t) * bezier[6.0_f64 as usize].clone()));
            let by = ((((((mt * mt) * mt) * bezier[1.0_f64 as usize].clone())
                + ((((3.0_f64 * mt) * mt) * t) * bezier[3.0_f64 as usize].clone()))
                + ((((3.0_f64 * mt) * t) * t) * bezier[5.0_f64 as usize].clone()))
                + (((t * t) * t) * bezier[7.0_f64 as usize].clone()));
            let dx = (pts[((first + i) * 2.0_f64) as usize].clone() - bx);
            let dy = (pts[(((first + i) * 2.0_f64) + 1.0_f64) as usize].clone() - by);
            let dist_sq = ((dx * dx) + (dy * dy));
            if (dist_sq >= max_dist) {
                max_dist = dist_sq;
                split_point = (first + i);
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return vec![max_dist, split_point];
}

// Source: upstream/packages/path/src/fitPathCurves.ts:249 (sha256:d7df12b8a5d5b23c41c105d526da94014a2316140c5734e01767d529fa9271be)
fn reparameterize(
    pts: &Vec<f64>,
    first: f64,
    last: f64,
    u: &Vec<f64>,
    bezier: &Vec<f64>,
) -> Vec<f64> {
    let mut u_prime: Vec<f64> = vec![];
    {
        let mut i = 0.0_f64;
        while (i <= (last - first)) {
            u_prime.push(newton_raphson_root_find(
                bezier,
                pts[((first + i) * 2.0_f64) as usize].clone(),
                pts[(((first + i) * 2.0_f64) + 1.0_f64) as usize].clone(),
                u[i as usize].clone(),
            ));
            {
                i += 1.0;
                i
            };
        }
    }
    return u_prime;
}

// Source: upstream/packages/path/src/fitPathCurves.ts:263 (sha256:02df20cab541eebc2401f920a700ab941a7d5353d529365d27ff49d8e2de4b59)
fn newton_raphson_root_find(bezier: &Vec<f64>, px: f64, py: f64, u: f64) -> f64 {
    let mt = (1.0_f64 - u);
    let qx = ((((((mt * mt) * mt) * bezier[0.0_f64 as usize].clone())
        + ((((3.0_f64 * mt) * mt) * u) * bezier[2.0_f64 as usize].clone()))
        + ((((3.0_f64 * mt) * u) * u) * bezier[4.0_f64 as usize].clone()))
        + (((u * u) * u) * bezier[6.0_f64 as usize].clone()));
    let qy = ((((((mt * mt) * mt) * bezier[1.0_f64 as usize].clone())
        + ((((3.0_f64 * mt) * mt) * u) * bezier[3.0_f64 as usize].clone()))
        + ((((3.0_f64 * mt) * u) * u) * bezier[5.0_f64 as usize].clone()))
        + (((u * u) * u) * bezier[7.0_f64 as usize].clone()));
    let q1x = (((((3.0_f64 * mt) * mt)
        * (bezier[2.0_f64 as usize].clone() - bezier[0.0_f64 as usize].clone()))
        + (((6.0_f64 * mt) * u)
            * (bezier[4.0_f64 as usize].clone() - bezier[2.0_f64 as usize].clone())))
        + (((3.0_f64 * u) * u)
            * (bezier[6.0_f64 as usize].clone() - bezier[4.0_f64 as usize].clone())));
    let q1y = (((((3.0_f64 * mt) * mt)
        * (bezier[3.0_f64 as usize].clone() - bezier[1.0_f64 as usize].clone()))
        + (((6.0_f64 * mt) * u)
            * (bezier[5.0_f64 as usize].clone() - bezier[3.0_f64 as usize].clone())))
        + (((3.0_f64 * u) * u)
            * (bezier[7.0_f64 as usize].clone() - bezier[5.0_f64 as usize].clone())));
    let num = (((qx - px) * q1x) + ((qy - py) * q1y));
    let q2x = (((6.0_f64 * mt)
        * ((bezier[4.0_f64 as usize].clone() - (2.0_f64 * bezier[2.0_f64 as usize].clone()))
            + bezier[0.0_f64 as usize].clone()))
        + ((6.0_f64 * u)
            * ((bezier[6.0_f64 as usize].clone() - (2.0_f64 * bezier[4.0_f64 as usize].clone()))
                + bezier[2.0_f64 as usize].clone())));
    let q2y = (((6.0_f64 * mt)
        * ((bezier[5.0_f64 as usize].clone() - (2.0_f64 * bezier[3.0_f64 as usize].clone()))
            + bezier[1.0_f64 as usize].clone()))
        + ((6.0_f64 * u)
            * ((bezier[7.0_f64 as usize].clone() - (2.0_f64 * bezier[5.0_f64 as usize].clone()))
                + bezier[3.0_f64 as usize].clone())));
    let den = ((((q1x * q1x) + (q1y * q1y)) + ((qx - px) * q2x)) + ((qy - py) * q2y));
    if ((den).abs() < 1e-12_f64) {
        return u;
    }
    return (0.0_f64).max((1.0_f64).min((u - (num / den))));
}

// Source: upstream/packages/path/src/fitPathCurves.ts:284 (sha256:d347e9abc2605623a98ec0554be759fcf16977c47cb7c7c4ea941cb15935a9d2)
fn compute_center_tangent(pts: &Vec<f64>, idx: f64) -> Vec<f64> {
    let dx = (pts[((idx - 1.0_f64) * 2.0_f64) as usize].clone()
        - pts[((idx + 1.0_f64) * 2.0_f64) as usize].clone());
    let dy = (pts[(((idx - 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone()
        - pts[(((idx + 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone());
    let len = ((dx * dx) + (dy * dy)).sqrt();
    return if (len > 0.0_f64) {
        vec![(dx / len), (dy / len)]
    } else {
        vec![1.0_f64, 0.0_f64]
    };
}
