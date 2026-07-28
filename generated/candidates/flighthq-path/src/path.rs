// @generated from upstream/packages/path/src/path.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Path, PathCommand, PathWinding};

// Source: upstream/packages/path/src/path.ts:10 (sha256:ff28e3a46a871a2fa448591cf7470f1ca58dc13ece41853e0c28025b8ced2c1c)
pub fn append_path_arc(
    path: &mut Path,
    cx: f64,
    cy: f64,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    anticlockwise: Option<bool>,
    connect_to_current: Option<bool>,
) -> () {
    let anticlockwise = anticlockwise.unwrap_or(false);
    let connect_to_current = connect_to_current.unwrap_or(false);
    if (radius <= 0.0_f64) {
        return;
    }
    let mut sweep = (end_angle - start_angle);
    if anticlockwise {
        if (sweep > 0.0_f64) {
            sweep -= (std::f64::consts::PI * 2.0_f64);
        }
    } else {
        if (sweep < 0.0_f64) {
            sweep += (std::f64::consts::PI * 2.0_f64);
        }
    }
    let arc_start_x = (cx + ((start_angle).cos() * radius));
    let arc_start_y = (cy + ((start_angle).sin() * radius));
    if connect_to_current {
        append_path_line_to(path, arc_start_x, arc_start_y);
    } else {
        append_path_move_to(path, arc_start_x, arc_start_y);
    }
    append_arc_cubics(path, cx, cy, radius, radius, 0.0_f64, start_angle, sweep);
}

// Source: upstream/packages/path/src/path.ts:47 (sha256:cd5656b3a176a7ba68e98303d3d0011a62e0b76a9cd175843f9d567f6e88c344)
pub fn append_path_arc_to(
    path: &mut Path,
    radius_x: f64,
    radius_y: f64,
    x_axis_rotation: f64,
    large_arc: bool,
    sweep: bool,
    end_x: f64,
    end_y: f64,
) -> () {
    if (radius_x == 0.0_f64) || (radius_y == 0.0_f64) {
        append_path_line_to(path, end_x, end_y);
        return;
    }
    let mut x1 = 0.0_f64;
    let mut y1 = 0.0_f64;
    {
        let last = get_path_last_point(path);
        if (last).is_some() {
            x1 = last.as_ref().unwrap()[0.0_f64 as usize].clone();
            y1 = last.as_ref().unwrap()[1.0_f64 as usize].clone();
        }
    }
    let x2 = end_x;
    let y2 = end_y;
    if (x1 == x2) && (y1 == y2) {
        return;
    }
    let mut rx = (radius_x).abs();
    let mut ry = (radius_y).abs();
    let cosφ = (x_axis_rotation).cos();
    let sinφ = (x_axis_rotation).sin();
    let dx = ((x1 - x2) / 2.0_f64);
    let dy = ((y1 - y2) / 2.0_f64);
    let x1p = ((cosφ * dx) + (sinφ * dy));
    let y1p = (((-sinφ) * dx) + (cosφ * dy));
    let x1p_sq = (x1p * x1p);
    let y1p_sq = (y1p * y1p);
    let rx_sq = (rx * rx);
    let ry_sq = (ry * ry);
    let lambda = ((x1p_sq / rx_sq) + (y1p_sq / ry_sq));
    if (lambda > 1.0_f64) {
        let sqrt_lambda = (lambda).sqrt();
        rx *= sqrt_lambda;
        ry *= sqrt_lambda;
    }
    let rx_sq2 = (rx * rx);
    let ry_sq2 = (ry * ry);
    let num = (((rx_sq2 * ry_sq2) - (rx_sq2 * y1p_sq)) - (ry_sq2 * x1p_sq));
    let den = ((rx_sq2 * y1p_sq) + (ry_sq2 * x1p_sq));
    let sq = if (den <= 0.0_f64) {
        0.0_f64
    } else {
        ((0.0_f64).max((num / den))).sqrt()
    };
    let sign = if (large_arc == sweep) {
        (-1.0_f64)
    } else {
        1.0_f64
    };
    let cxp = (((sign * sq) * (rx * y1p)) / ry);
    let cyp = (((sign * sq) * ((-ry) * x1p)) / rx);
    let cx = (((cosφ * cxp) - (sinφ * cyp)) + ((x1 + x2) / 2.0_f64));
    let cy = (((sinφ * cxp) + (cosφ * cyp)) + ((y1 + y2) / 2.0_f64));
    let ux = ((x1p - cxp) / rx);
    let uy = ((y1p - cyp) / ry);
    let vx = (((-x1p) - cxp) / rx);
    let vy = (((-y1p) - cyp) / ry);
    let theta1 = vector_angle(1.0_f64, 0.0_f64, ux, uy);
    let mut dtheta = vector_angle(ux, uy, vx, vy);
    if (!sweep) && (dtheta > 0.0_f64) {
        dtheta -= (std::f64::consts::PI * 2.0_f64);
    }
    if (sweep) && (dtheta < 0.0_f64) {
        dtheta += (std::f64::consts::PI * 2.0_f64);
    }
    append_arc_cubics(path, cx, cy, rx, ry, x_axis_rotation, theta1, dtheta);
}

// Source: upstream/packages/path/src/path.ts:123 (sha256:e0f40287fb8e5ca82f7410dfc711f9470a6a0e526ca87c0ff91fe4298031360d)
pub fn append_path_circle(path: &mut Path, cx: f64, cy: f64, radius: f64) -> () {
    append_path_ellipse(path, cx, cy, radius, radius);
}

// Source: upstream/packages/path/src/path.ts:129 (sha256:651303efacb256cac9b586b891311ae380f094ea35ce5354cca6d5ba5952d0b0)
pub fn append_path_close(path: &mut Path) -> () {
    path.commands.push(PathCommand::CLOSE);
}

// Source: upstream/packages/path/src/path.ts:133 (sha256:54e9195c43ce09ea7bcbec2d14015b4adea221fc0fac8c65a1ff12ec408315a9)
pub fn append_path_cubic_curve_to(
    path: &mut Path,
    control1_x: f64,
    control1_y: f64,
    control2_x: f64,
    control2_y: f64,
    anchor_x: f64,
    anchor_y: f64,
) -> () {
    path.commands.push(PathCommand::CUBIC_CURVE_TO);
    path.data.extend(vec![
        control1_x, control1_y, control2_x, control2_y, anchor_x, anchor_y,
    ]);
}

// Source: upstream/packages/path/src/path.ts:146 (sha256:b5fbe6c38e5a655a0e865d31e3c53109c34027d040b39a00a55ea1e43188eed6)
pub fn append_path_curve_to(
    path: &mut Path,
    control_x: f64,
    control_y: f64,
    anchor_x: f64,
    anchor_y: f64,
) -> () {
    path.commands.push(PathCommand::CURVE_TO);
    path.data
        .extend(vec![control_x, control_y, anchor_x, anchor_y]);
}

// Source: upstream/packages/path/src/path.ts:160 (sha256:40d29acdcd91dd23e7cc85c849b2c66c6b830f9787ecf8a862b7a0ea641ed6e2)
pub fn append_path_ellipse(path: &mut Path, cx: f64, cy: f64, radius_x: f64, radius_y: f64) -> () {
    let kx = (radius_x * KAPPA);
    let ky = (radius_y * KAPPA);
    append_path_move_to(path, (cx + radius_x), cy);
    append_path_cubic_curve_to(
        path,
        (cx + radius_x),
        (cy - ky),
        (cx + kx),
        (cy - radius_y),
        cx,
        (cy - radius_y),
    );
    append_path_cubic_curve_to(
        path,
        (cx - kx),
        (cy - radius_y),
        (cx - radius_x),
        (cy - ky),
        (cx - radius_x),
        cy,
    );
    append_path_cubic_curve_to(
        path,
        (cx - radius_x),
        (cy + ky),
        (cx - kx),
        (cy + radius_y),
        cx,
        (cy + radius_y),
    );
    append_path_cubic_curve_to(
        path,
        (cx + kx),
        (cy + radius_y),
        (cx + radius_x),
        (cy + ky),
        (cx + radius_x),
        cy,
    );
    append_path_close(path);
}

// Source: upstream/packages/path/src/path.ts:172 (sha256:3b00e0304231e85ea67ce3c2975d139593ea9adc1c386d008fee8a5ff17fc94a)
pub fn append_path_line_to(path: &mut Path, x: f64, y: f64) -> () {
    path.commands.push(PathCommand::LINE_TO);
    path.data.extend(vec![x, y]);
}

// Source: upstream/packages/path/src/path.ts:177 (sha256:ed1282fd76e7d1289c7520f568bb526d4f646827d099fd9c3cff9939cdd093c1)
pub fn append_path_move_to(path: &mut Path, x: f64, y: f64) -> () {
    path.commands.push(PathCommand::MOVE_TO);
    path.data.extend(vec![x, y]);
}

// Source: upstream/packages/path/src/path.ts:185 (sha256:3c18a0ec6c8dfa70477cbab3c6bb85409849df2dfa2041ae945a06bd34eb3f64)
pub fn append_path_polygon(path: &mut Path, points: &Vec<f64>) -> () {
    if ((points.len() as f64) < 6.0_f64) {
        return;
    }
    append_path_move_to(
        path,
        points[0.0_f64 as usize].clone(),
        points[1.0_f64 as usize].clone(),
    );
    {
        let mut i = 2.0_f64;
        while (i < (points.len() as f64)) {
            append_path_line_to(
                path,
                points[i as usize].clone(),
                points[(i + 1.0_f64) as usize].clone(),
            );
            {
                i += 2.0_f64;
                i
            };
        }
    }
    append_path_close(path);
}

// Source: upstream/packages/path/src/path.ts:197 (sha256:8befdddbc558987d4a0ad185b6e00bf4849726fb37b66720eabb46ffa12b6b79)
pub fn append_path_polyline(path: &mut Path, points: &Vec<f64>) -> () {
    if ((points.len() as f64) < 4.0_f64) {
        return;
    }
    append_path_move_to(
        path,
        points[0.0_f64 as usize].clone(),
        points[1.0_f64 as usize].clone(),
    );
    {
        let mut i = 2.0_f64;
        while (i < (points.len() as f64)) {
            append_path_line_to(
                path,
                points[i as usize].clone(),
                points[(i + 1.0_f64) as usize].clone(),
            );
            {
                i += 2.0_f64;
                i
            };
        }
    }
}

// Source: upstream/packages/path/src/path.ts:207 (sha256:b7383cadaa111dfd0a6ead55668dc3fcbbcf788c43c18194e98dfd44e21ce1db)
pub fn append_path_rectangle(path: &mut Path, x: f64, y: f64, width: f64, height: f64) -> () {
    append_path_move_to(path, x, y);
    append_path_line_to(path, (x + width), y);
    append_path_line_to(path, (x + width), (y + height));
    append_path_line_to(path, x, (y + height));
    append_path_close(path);
}

// Source: upstream/packages/path/src/path.ts:219 (sha256:0941c30ea8e8d7925157a17b2069681ab51a7be12a2682009e19e18aa3441802)
pub fn append_path_round_rectangle(
    path: &mut Path,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    radius: &crate::FlightUnion2<f64, Vec<f64>>,
) -> () {
    let __destructure0 = normalize_corner_radii(&((*radius).clone()), width, height);
    let rtl = __destructure0[0.0_f64 as usize].clone();
    let rtr = __destructure0[1.0_f64 as usize].clone();
    let rbr = __destructure0[2.0_f64 as usize].clone();
    let rbl = __destructure0[3.0_f64 as usize].clone();
    append_path_move_to(path, (x + rtl), y);
    append_path_line_to(path, ((x + width) - rtr), y);
    append_corner_arc(
        path,
        ((x + width) - rtr),
        (y + rtr),
        rtr,
        ((-std::f64::consts::PI) / 2.0_f64),
        0.0_f64,
    );
    append_path_line_to(path, (x + width), ((y + height) - rbr));
    append_corner_arc(
        path,
        ((x + width) - rbr),
        ((y + height) - rbr),
        rbr,
        0.0_f64,
        (std::f64::consts::PI / 2.0_f64),
    );
    append_path_line_to(path, (x + rbl), (y + height));
    append_corner_arc(
        path,
        (x + rbl),
        ((y + height) - rbl),
        rbl,
        (std::f64::consts::PI / 2.0_f64),
        std::f64::consts::PI,
    );
    append_path_line_to(path, x, (y + rtl));
    append_corner_arc(
        path,
        (x + rtl),
        (y + rtl),
        rtl,
        std::f64::consts::PI,
        ((std::f64::consts::PI * 3.0_f64) / 2.0_f64),
    );
    append_path_close(path);
}

// Source: upstream/packages/path/src/path.ts:247 (sha256:59f67d24a0a03c3dfe74992817f29786a538b25b73c46377269fdf88ce82985a)
pub fn create_path(winding: Option<PathWinding>) -> Path {
    let winding = winding.unwrap_or("nonZero".to_owned());
    return Path {
        __flight_identity: std::sync::Arc::new(()),
        commands: vec![],
        data: vec![],
        winding: (winding).clone(),
    };
}

// Source: upstream/packages/path/src/path.ts:253 (sha256:6421ee3775b7222a550eeeb2f0d5e01dbc9e51496dda957a992f0ad77acb1d6a)
fn append_corner_arc(
    path: &mut Path,
    cx: f64,
    cy: f64,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> () {
    if (radius <= 0.0_f64) {
        return;
    }
    let k = (radius * KAPPA);
    let cos_start = (start_angle).cos();
    let sin_start = (start_angle).sin();
    let cos_end = (end_angle).cos();
    let sin_end = (end_angle).sin();
    append_path_cubic_curve_to(
        path,
        ((cx + (cos_start * radius)) - (sin_start * k)),
        ((cy + (sin_start * radius)) + (cos_start * k)),
        ((cx + (cos_end * radius)) + (sin_end * k)),
        ((cy + (sin_end * radius)) - (cos_end * k)),
        (cx + (cos_end * radius)),
        (cy + (sin_end * radius)),
    );
}

// Source: upstream/packages/path/src/path.ts:282 (sha256:2d29fd19bfec4af2aad8e617a0a05fb3d174e6079915f5275f4c1c55e3cc23be)
fn append_arc_cubics(
    path: &mut Path,
    cx: f64,
    cy: f64,
    rx: f64,
    ry: f64,
    x_axis_rotation: f64,
    theta1: f64,
    dtheta: f64,
) -> () {
    if (dtheta == 0.0_f64) {
        return;
    }
    let n_segs = (1.0_f64).max(((dtheta).abs() / (std::f64::consts::PI / 2.0_f64)).ceil());
    let dt = (dtheta / n_segs);
    let cosφ = (x_axis_rotation).cos();
    let sinφ = (x_axis_rotation).sin();
    {
        let mut i = 0.0_f64;
        while (i < n_segs) {
            let t1 = (theta1 + (i * dt));
            let t2 = (t1 + dt);
            let cos1 = (t1).cos();
            let sin1 = (t1).sin();
            let cos2 = (t2).cos();
            let sin2 = (t2).sin();
            let alpha = ((4.0_f64 / 3.0_f64) * (dt / 4.0_f64).tan());
            let dx1 = (((-rx) * sin1) * alpha);
            let dy1 = ((ry * cos1) * alpha);
            let dx2 = ((rx * sin2) * alpha);
            let dy2 = (((-ry) * cos2) * alpha);
            let p1x = ((cx + ((cosφ * rx) * cos1)) - ((sinφ * ry) * sin1));
            let p1y = ((cy + ((sinφ * rx) * cos1)) + ((cosφ * ry) * sin1));
            let p2x = ((cx + ((cosφ * rx) * cos2)) - ((sinφ * ry) * sin2));
            let p2y = ((cy + ((sinφ * rx) * cos2)) + ((cosφ * ry) * sin2));
            let c1x = ((p1x + (cosφ * dx1)) - (sinφ * dy1));
            let c1y = ((p1y + (sinφ * dx1)) + (cosφ * dy1));
            let c2x = ((p2x + (cosφ * dx2)) - (sinφ * dy2));
            let c2y = ((p2y + (sinφ * dx2)) + (cosφ * dy2));
            append_path_cubic_curve_to(path, c1x, c1y, c2x, c2y, p2x, p2y);
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/path/src/path.ts:328 (sha256:7735b381cbc3b8bbdd01b99d2ee8302644975a350970e8135a68a0db79f7e576)
pub fn get_path_last_point(path: &Path) -> Option<Vec<f64>> {
    if ((path.data.len() as f64) < 2.0_f64) {
        return None;
    }
    return Some(vec![
        path.data[((path.data.len() as f64) - 2.0_f64) as usize].clone(),
        path.data[((path.data.len() as f64) - 1.0_f64) as usize].clone(),
    ]);
}

// Source: upstream/packages/path/src/path.ts:336 (sha256:8564d3348ff3eca124e9759607edda42c8b0a48038c302223fa45bfc8ab023dd)
fn normalize_corner_radii(
    radius: &crate::FlightUnion2<f64, Vec<f64>>,
    width: f64,
    height: f64,
) -> Vec<f64> {
    let __destructure1 = if (match &(radius) {
        crate::FlightUnion2::A(_) => "number",
        crate::FlightUnion2::B(value) => "object",
    } == "number")
    {
        vec![
            match (*radius).clone() {
                crate::FlightUnion2::A(value) => value,
                crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
            },
            match (*radius).clone() {
                crate::FlightUnion2::A(value) => value,
                crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
            },
            match (*radius).clone() {
                crate::FlightUnion2::A(value) => value,
                crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
            },
            match (*radius).clone() {
                crate::FlightUnion2::A(value) => value,
                crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
            },
        ]
    } else {
        match (*radius).clone() {
            crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
            crate::FlightUnion2::B(value) => value,
        }
    };
    let rtl = __destructure1[0.0_f64 as usize].clone();
    let rtr = __destructure1[1.0_f64 as usize].clone();
    let rbr = __destructure1[2.0_f64 as usize].clone();
    let rbl = __destructure1[3.0_f64 as usize].clone();
    let half_w = ((width).abs() / 2.0_f64);
    let half_h = ((height).abs() / 2.0_f64);
    let clamp_tl = (0.0_f64).max(((rtl).min(half_w)).min(half_h));
    let clamp_tr = (0.0_f64).max(((rtr).min(half_w)).min(half_h));
    let clamp_br = (0.0_f64).max(((rbr).min(half_w)).min(half_h));
    let clamp_bl = (0.0_f64).max(((rbl).min(half_w)).min(half_h));
    return vec![clamp_tl, clamp_tr, clamp_br, clamp_bl];
}

// Source: upstream/packages/path/src/path.ts:354 (sha256:9dff43690813f6f2baba3988c33baa0fcdb41287e4020d2a8ee79e9008e5ff06)
fn vector_angle(ux: f64, uy: f64, vx: f64, vy: f64) -> f64 {
    let dot = ((ux * vx) + (uy * vy));
    let len_u = ((ux * ux) + (uy * uy)).sqrt();
    let len_v = ((vx * vx) + (vy * vy)).sqrt();
    if (len_u == 0.0_f64) || (len_v == 0.0_f64) {
        return 0.0_f64;
    }
    let cos_angle = (-1.0_f64).max((1.0_f64).min((dot / (len_u * len_v))));
    let angle = (cos_angle).acos();
    return if (((ux * vy) - (uy * vx)) < 0.0_f64) {
        (-angle)
    } else {
        angle
    };
}

// Source: upstream/packages/path/src/path.ts:366 (sha256:adaa49232521685098fa67200888dc125dc6c3eb4f712cbd7116cbad442a3837)
const KAPPA: f64 = 0.5522847498308936_f64;
