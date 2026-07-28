// @generated from upstream/packages/path/src/getPathBounds.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Path, PathCommand, RectangleLike};

// Source: upstream/packages/path/src/getPathBounds.ts:10 (sha256:221af710454c801852fa4eaeb795765bbdaf804e3c46c03792eff7e8f4e44caf)
pub fn get_path_bounds(path: &Path, out: &mut RectangleLike) -> bool {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let max_x: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new((-f64::INFINITY)));
    let max_y: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new((-f64::INFINITY)));
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;
    let mut di = 0.0_f64;
    let mut expand: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut max_x = max_x.clone();
        let mut max_y = max_y.clone();
        move |px: f64, py: f64| -> () {
            if (px < (*min_x.lock().unwrap()).clone()) {
                (*min_x.lock().unwrap()) = px;
            }
            if (px > (*max_x.lock().unwrap()).clone()) {
                (*max_x.lock().unwrap()) = px;
            }
            if (py < (*min_y.lock().unwrap()).clone()) {
                (*min_y.lock().unwrap()) = py;
            }
            if (py > (*max_y.lock().unwrap()).clone()) {
                (*max_y.lock().unwrap()) = py;
            }
        }
    })
        as Box<dyn FnMut(f64, f64) -> () + Send + 'static>));
    {
        let mut ci = 0.0_f64;
        while (ci < (path.commands.len() as f64)) {
            let command = path.commands[ci as usize].clone();
            if (command == PathCommand::MOVE_TO) {
                x = path.data[di as usize].clone();
                y = path.data[(di + 1.0_f64) as usize].clone();
                di += 2.0_f64;
                ((expand).clone()).lock().unwrap()(x, y);
            } else {
                if (command == PathCommand::WIDE_MOVE_TO) {
                    x = path.data[(di + 2.0_f64) as usize].clone();
                    y = path.data[(di + 3.0_f64) as usize].clone();
                    di += 4.0_f64;
                    ((expand).clone()).lock().unwrap()(x, y);
                } else {
                    if (command == PathCommand::LINE_TO) {
                        let nx = path.data[di as usize].clone();
                        let ny = path.data[(di + 1.0_f64) as usize].clone();
                        di += 2.0_f64;
                        ((expand).clone()).lock().unwrap()(nx, ny);
                        x = nx;
                        y = ny;
                    } else {
                        if (command == PathCommand::WIDE_LINE_TO) {
                            let nx = path.data[(di + 2.0_f64) as usize].clone();
                            let ny = path.data[(di + 3.0_f64) as usize].clone();
                            di += 4.0_f64;
                            ((expand).clone()).lock().unwrap()(nx, ny);
                            x = nx;
                            y = ny;
                        } else {
                            if (command == PathCommand::CURVE_TO) {
                                let cx = path.data[di as usize].clone();
                                let cy = path.data[(di + 1.0_f64) as usize].clone();
                                let ax = path.data[(di + 2.0_f64) as usize].clone();
                                let ay = path.data[(di + 3.0_f64) as usize].clone();
                                di += 4.0_f64;
                                expand_quadratic_bounds(x, y, cx, cy, ax, ay, &mut expand);
                                x = ax;
                                y = ay;
                            } else {
                                if (command == PathCommand::CUBIC_CURVE_TO) {
                                    let c1x = path.data[di as usize].clone();
                                    let c1y = path.data[(di + 1.0_f64) as usize].clone();
                                    let c2x = path.data[(di + 2.0_f64) as usize].clone();
                                    let c2y = path.data[(di + 3.0_f64) as usize].clone();
                                    let ax = path.data[(di + 4.0_f64) as usize].clone();
                                    let ay = path.data[(di + 5.0_f64) as usize].clone();
                                    di += 6.0_f64;
                                    expand_cubic_bounds(
                                        x,
                                        y,
                                        c1x,
                                        c1y,
                                        c2x,
                                        c2y,
                                        ax,
                                        ay,
                                        &mut expand,
                                    );
                                    x = ax;
                                    y = ay;
                                }
                            }
                        }
                    }
                }
            }
            {
                ci += 1.0;
                ci
            };
        }
    }
    if ((*min_x.lock().unwrap()) == f64::INFINITY) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.width = 0.0_f64;
        out.height = 0.0_f64;
        return false;
    }
    out.x = (*min_x.lock().unwrap());
    out.y = (*min_y.lock().unwrap());
    out.width = ((*max_x.lock().unwrap()).clone() - (*min_x.lock().unwrap()));
    out.height = ((*max_y.lock().unwrap()).clone() - (*min_y.lock().unwrap()));
    return true;
}

// Source: upstream/packages/path/src/getPathBounds.ts:90 (sha256:a67da73501aed51ed2792b809539b22eafbcd3b0198defcdebb2091dea80e3d9)
fn cubic_extremum_roots(p0: f64, p1: f64, p2: f64, p3: f64, cb: &mut impl FnMut(f64) -> ()) -> () {
    let a = ((((-p0) + (3.0_f64 * p1)) - (3.0_f64 * p2)) + p3);
    let b = (2.0_f64 * ((p0 - (2.0_f64 * p1)) + p2));
    let c = (p1 - p0);
    if ((a).abs() < 1e-12_f64) {
        if ((b).abs() < 1e-12_f64) {
            return;
        }
        let t = ((-c) / b);
        if ((t > 0.0_f64) && (t < 1.0_f64)) {
            cb(t);
        }
        return;
    }
    let discriminant = ((b * b) - ((4.0_f64 * a) * c));
    if (discriminant < 0.0_f64) {
        return;
    }
    let sqrt_d = (discriminant).sqrt();
    let t1 = (((-b) + sqrt_d) / (2.0_f64 * a));
    let t2 = (((-b) - sqrt_d) / (2.0_f64 * a));
    if ((t1 > 0.0_f64) && (t1 < 1.0_f64)) {
        cb(t1);
    }
    if (((t2 > 0.0_f64) && (t2 < 1.0_f64)) && ((t2 - t1).abs() > 1e-12_f64)) {
        cb(t2);
    }
}

// Source: upstream/packages/path/src/getPathBounds.ts:111 (sha256:c2878649873ea146b79f1b2a1f5a5778634e2cc2469ae78e6d5f54d0b4e1af97)
fn eval_cubic(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    let mt = (1.0_f64 - t);
    return ((((((mt * mt) * mt) * p0) + ((((3.0_f64 * mt) * mt) * t) * p1))
        + ((((3.0_f64 * mt) * t) * t) * p2))
        + (((t * t) * t) * p3));
}

// Source: upstream/packages/path/src/getPathBounds.ts:116 (sha256:3dba30ada3a76a0edffe59b145001f46797db1b86792a3cb54a77b037241875a)
fn eval_quadratic(p0: f64, p1: f64, p2: f64, t: f64) -> f64 {
    let mt = (1.0_f64 - t);
    return ((((mt * mt) * p0) + (((2.0_f64 * mt) * t) * p1)) + ((t * t) * p2));
}

// Source: upstream/packages/path/src/getPathBounds.ts:123 (sha256:8eab1ae6f68fed78285a266130dc750bdbbb1c77893e8c44ddffbccc8684d6e6)
fn expand_cubic_bounds(
    x0: f64,
    y0: f64,
    c1x: f64,
    c1y: f64,
    c2x: f64,
    c2y: f64,
    x3: f64,
    y3: f64,
    expand: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>,
) -> () {
    ((expand).clone()).lock().unwrap()(x3, y3);
    cubic_extremum_roots(x0, c1x, c2x, x3, &mut |t: f64| -> () {
        ((expand).clone()).lock().unwrap()(
            eval_cubic(x0, c1x, c2x, x3, t),
            eval_cubic(y0, c1y, c2y, y3, t),
        );
    });
    cubic_extremum_roots(y0, c1y, c2y, y3, &mut |t: f64| -> () {
        ((expand).clone()).lock().unwrap()(
            eval_cubic(x0, c1x, c2x, x3, t),
            eval_cubic(y0, c1y, c2y, y3, t),
        );
    });
}

// Source: upstream/packages/path/src/getPathBounds.ts:149 (sha256:1228d9670c2f5223841a26329ebda5acd32452908fd6aaa78759b62323dadb90)
fn expand_quadratic_bounds(
    x0: f64,
    y0: f64,
    cx: f64,
    cy: f64,
    x2: f64,
    y2: f64,
    expand: &mut impl FnMut(f64, f64) -> (),
) -> () {
    expand(x2, y2);
    let tx = quadratic_extremum_t(x0, cx, x2);
    if (tx).is_some() {
        expand(
            eval_quadratic(x0, cx, x2, tx.as_ref().unwrap()),
            eval_quadratic(y0, cy, y2, tx.as_ref().unwrap()),
        );
    }
    let ty = quadratic_extremum_t(y0, cy, y2);
    if (ty).is_some() {
        expand(
            eval_quadratic(x0, cx, x2, ty.as_ref().unwrap()),
            eval_quadratic(y0, cy, y2, ty.as_ref().unwrap()),
        );
    }
}

// Source: upstream/packages/path/src/getPathBounds.ts:170 (sha256:b90290524ebe84170dc0e2761aa6496d011a2b18f73dfc3ecfeb57f06aab280b)
fn quadratic_extremum_t(p0: f64, p1: f64, p2: f64) -> Option<f64> {
    let denom = ((p0 - (2.0_f64 * p1)) + p2);
    if (denom == 0.0_f64) {
        return None;
    }
    let t = ((p0 - p1) / denom);
    return if ((t > 0.0_f64) && (t < 1.0_f64)) {
        Some(t)
    } else {
        None
    };
}
