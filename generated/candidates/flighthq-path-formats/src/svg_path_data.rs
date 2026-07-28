// @generated from upstream/packages/path-formats/src/svgPathData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_path::{
    append_path_arc_to, append_path_close, append_path_cubic_curve_to, append_path_curve_to,
    append_path_line_to, append_path_move_to, create_path, for_each_path_segment,
};
use flighthq_types::{Path, PathSegment};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub precision: Option<f64>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path-formats/src/svgPathData.ts:26 (sha256:9167f8486ff8309d236e8a36954c943903721ac23ca3ed984357c332d0165e88)
pub fn append_svg_path_data(path: &mut Path, d: String) -> bool {
    let length = (d.encode_utf16().count() as f64);
    let pos: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let mut current_x = 0.0_f64;
    let mut current_y = 0.0_f64;
    let mut start_x = 0.0_f64;
    let mut start_y = 0.0_f64;
    let mut last_control2_x = 0.0_f64;
    let mut last_control2_y = 0.0_f64;
    let mut last_quad_control_x = 0.0_f64;
    let mut last_quad_control_y = 0.0_f64;
    let mut last_kind = "";
    let mut skip_separators: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut pos = pos.clone();
        move || -> () {
            while ((*pos.lock().unwrap()).clone() < length) {
                let c = (d.char_code_at)((*pos.lock().unwrap()).clone());
                if (((((c == 32.0_f64) || (c == 9.0_f64)) || (c == 10.0_f64)) || (c == 13.0_f64))
                    || (c == 12.0_f64))
                    || (c == 44.0_f64)
                {
                    {
                        (*pos.lock().unwrap()) += 1.0;
                        (*pos.lock().unwrap())
                    };
                } else {
                    break;
                }
            }
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
    let mut read_number: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> Option<f64> + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut pos = pos.clone();
        let skip_separators = skip_separators.clone();
        move || -> Option<f64> {
            {
                let __flight_callback = (skip_separators).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            };
            let start = (*pos.lock().unwrap()).clone();
            if ((*pos.lock().unwrap()).clone() < length)
                && ((d[(*pos.lock().unwrap()).clone() as usize].clone() == "+")
                    || (d[(*pos.lock().unwrap()).clone() as usize].clone() == "-"))
            {
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
            }
            let mut saw_digit = false;
            while (((*pos.lock().unwrap()).clone() < length)
                && (d[(*pos.lock().unwrap()).clone() as usize].clone() >= "0"))
                && (d[(*pos.lock().unwrap()).clone() as usize].clone() <= "9")
            {
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
                saw_digit = true;
            }
            if ((*pos.lock().unwrap()).clone() < length)
                && (d[(*pos.lock().unwrap()).clone() as usize].clone() == ".")
            {
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
                while (((*pos.lock().unwrap()).clone() < length)
                    && (d[(*pos.lock().unwrap()).clone() as usize].clone() >= "0"))
                    && (d[(*pos.lock().unwrap()).clone() as usize].clone() <= "9")
                {
                    {
                        (*pos.lock().unwrap()) += 1.0;
                        (*pos.lock().unwrap())
                    };
                    saw_digit = true;
                }
            }
            if (!saw_digit) {
                (*pos.lock().unwrap()) = start;
                return None;
            }
            if ((*pos.lock().unwrap()).clone() < length)
                && ((d[(*pos.lock().unwrap()).clone() as usize].clone() == "e")
                    || (d[(*pos.lock().unwrap()).clone() as usize].clone() == "E"))
            {
                let exp_start = (*pos.lock().unwrap()).clone();
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
                if ((*pos.lock().unwrap()).clone() < length)
                    && ((d[(*pos.lock().unwrap()).clone() as usize].clone() == "+")
                        || (d[(*pos.lock().unwrap()).clone() as usize].clone() == "-"))
                {
                    {
                        (*pos.lock().unwrap()) += 1.0;
                        (*pos.lock().unwrap())
                    };
                }
                let mut exp_digit = false;
                while (((*pos.lock().unwrap()).clone() < length)
                    && (d[(*pos.lock().unwrap()).clone() as usize].clone() >= "0"))
                    && (d[(*pos.lock().unwrap()).clone() as usize].clone() <= "9")
                {
                    {
                        (*pos.lock().unwrap()) += 1.0;
                        (*pos.lock().unwrap())
                    };
                    exp_digit = true;
                }
                if (!exp_digit) {
                    (*pos.lock().unwrap()) = exp_start;
                }
            }
            return Some((number.parse_float)((d.slice)(
                start,
                (*pos.lock().unwrap()).clone(),
            )));
        }
    })
        as Box<dyn FnMut() -> Option<f64> + Send + 'static>));
    let mut read_flag: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> Option<f64> + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut pos = pos.clone();
        let skip_separators = skip_separators.clone();
        move || -> Option<f64> {
            {
                let __flight_callback = (skip_separators).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            };
            if ((*pos.lock().unwrap()).clone() < length)
                && (d[(*pos.lock().unwrap()).clone() as usize].clone() == "0")
            {
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
                return Some(0.0_f64);
            }
            if ((*pos.lock().unwrap()).clone() < length)
                && (d[(*pos.lock().unwrap()).clone() as usize].clone() == "1")
            {
                {
                    (*pos.lock().unwrap()) += 1.0;
                    (*pos.lock().unwrap())
                };
                return Some(1.0_f64);
            }
            return None;
        }
    })
        as Box<dyn FnMut() -> Option<f64> + Send + 'static>));
    while true {
        {
            let __flight_callback = (skip_separators).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
        if ((*pos.lock().unwrap()).clone() >= length) {
            break;
        }
        let mut command_letter = d[(*pos.lock().unwrap()).clone() as usize].clone();
        if (!is_svg_command_letter(command_letter)) {
            return false;
        }
        {
            (*pos.lock().unwrap()) += 1.0;
            (*pos.lock().unwrap())
        };
        if ((last_kind == "") && (command_letter != "M")) && (command_letter != "m") {
            return false;
        }
        if (command_letter == "Z") || (command_letter == "z") {
            append_path_close(path);
            current_x = start_x;
            current_y = start_y;
            last_kind = "Z".to_owned();
            continue;
        }
        let mut active = command_letter;
        let mut first = true;
        while true {
            if (!first) {
                {
                    let __flight_callback = (skip_separators).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                if ((*pos.lock().unwrap()).clone() >= length) {
                    break;
                }
                if is_svg_command_letter(d[(*pos.lock().unwrap()).clone() as usize].clone()) {
                    break;
                }
            }
            let relative = (active >= "a");
            let upper = if relative {
                (active).to_uppercase()
            } else {
                (active).clone()
            };
            if (upper == "M") {
                let nx = {
                    let __flight_callback = (read_number).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                let ny = {
                    let __flight_callback = (read_number).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                if ((nx).is_none()) || ((ny).is_none()) {
                    return false;
                }
                current_x = if relative {
                    (current_x + nx)
                } else {
                    (nx).clone().unwrap()
                };
                current_y = if relative {
                    (current_y + ny)
                } else {
                    (ny).clone().unwrap()
                };
                start_x = current_x;
                start_y = current_y;
                append_path_move_to(path, current_x, current_y);
                last_kind = "M".to_owned();
            } else {
                if (upper == "L") {
                    let nx = {
                        let __flight_callback = (read_number).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    };
                    let ny = {
                        let __flight_callback = (read_number).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    };
                    if ((nx).is_none()) || ((ny).is_none()) {
                        return false;
                    }
                    current_x = if relative {
                        (current_x + nx)
                    } else {
                        (nx).clone().unwrap()
                    };
                    current_y = if relative {
                        (current_y + ny)
                    } else {
                        (ny).clone().unwrap()
                    };
                    append_path_line_to(path, current_x, current_y);
                    last_kind = "L".to_owned();
                } else {
                    if (upper == "H") {
                        let nx = {
                            let __flight_callback = (read_number).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        };
                        if (nx).is_none() {
                            return false;
                        }
                        current_x = if relative {
                            (current_x + nx)
                        } else {
                            (nx).clone().unwrap()
                        };
                        append_path_line_to(path, current_x, current_y);
                        last_kind = "L".to_owned();
                    } else {
                        if (upper == "V") {
                            let ny = {
                                let __flight_callback = (read_number).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            };
                            if (ny).is_none() {
                                return false;
                            }
                            current_y = if relative {
                                (current_y + ny)
                            } else {
                                (ny).clone().unwrap()
                            };
                            append_path_line_to(path, current_x, current_y);
                            last_kind = "L".to_owned();
                        } else {
                            if (upper == "C") {
                                let x1 = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                let y1 = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                let x2 = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                let y2 = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                let x = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                let y = {
                                    let __flight_callback = (read_number).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                if ((((((x1).is_none()) || ((y1).is_none())) || ((x2).is_none()))
                                    || ((y2).is_none()))
                                    || ((x).is_none()))
                                    || ((y).is_none())
                                {
                                    return false;
                                }
                                let c1x = if relative {
                                    (current_x + x1)
                                } else {
                                    (x1).clone().unwrap()
                                };
                                let c1y = if relative {
                                    (current_y + y1)
                                } else {
                                    (y1).clone().unwrap()
                                };
                                let c2x = if relative {
                                    (current_x + x2)
                                } else {
                                    (x2).clone().unwrap()
                                };
                                let c2y = if relative {
                                    (current_y + y2)
                                } else {
                                    (y2).clone().unwrap()
                                };
                                let ax = if relative {
                                    (current_x + x)
                                } else {
                                    (x).clone().unwrap()
                                };
                                let ay = if relative {
                                    (current_y + y)
                                } else {
                                    (y).clone().unwrap()
                                };
                                append_path_cubic_curve_to(path, c1x, c1y, c2x, c2y, ax, ay);
                                last_control2_x = c2x;
                                last_control2_y = c2y;
                                current_x = ax;
                                current_y = ay;
                                last_kind = "C".to_owned();
                            } else {
                                if (upper == "S") {
                                    let x2 = {
                                        let __flight_callback = (read_number).clone();
                                        let __flight_result = __flight_callback.lock().unwrap()();
                                        __flight_result
                                    };
                                    let y2 = {
                                        let __flight_callback = (read_number).clone();
                                        let __flight_result = __flight_callback.lock().unwrap()();
                                        __flight_result
                                    };
                                    let x = {
                                        let __flight_callback = (read_number).clone();
                                        let __flight_result = __flight_callback.lock().unwrap()();
                                        __flight_result
                                    };
                                    let y = {
                                        let __flight_callback = (read_number).clone();
                                        let __flight_result = __flight_callback.lock().unwrap()();
                                        __flight_result
                                    };
                                    if ((((x2).is_none()) || ((y2).is_none())) || ((x).is_none()))
                                        || ((y).is_none())
                                    {
                                        return false;
                                    }
                                    let reflect = (last_kind == "C") || (last_kind == "S");
                                    let c1x = if reflect {
                                        ((2.0_f64 * current_x) - last_control2_x)
                                    } else {
                                        current_x
                                    };
                                    let c1y = if reflect {
                                        ((2.0_f64 * current_y) - last_control2_y)
                                    } else {
                                        current_y
                                    };
                                    let c2x = if relative {
                                        (current_x + x2)
                                    } else {
                                        (x2).clone().unwrap()
                                    };
                                    let c2y = if relative {
                                        (current_y + y2)
                                    } else {
                                        (y2).clone().unwrap()
                                    };
                                    let ax = if relative {
                                        (current_x + x)
                                    } else {
                                        (x).clone().unwrap()
                                    };
                                    let ay = if relative {
                                        (current_y + y)
                                    } else {
                                        (y).clone().unwrap()
                                    };
                                    append_path_cubic_curve_to(path, c1x, c1y, c2x, c2y, ax, ay);
                                    last_control2_x = c2x;
                                    last_control2_y = c2y;
                                    current_x = ax;
                                    current_y = ay;
                                    last_kind = "S".to_owned();
                                } else {
                                    if (upper == "Q") {
                                        let x1 = {
                                            let __flight_callback = (read_number).clone();
                                            let __flight_result =
                                                __flight_callback.lock().unwrap()();
                                            __flight_result
                                        };
                                        let y1 = {
                                            let __flight_callback = (read_number).clone();
                                            let __flight_result =
                                                __flight_callback.lock().unwrap()();
                                            __flight_result
                                        };
                                        let x = {
                                            let __flight_callback = (read_number).clone();
                                            let __flight_result =
                                                __flight_callback.lock().unwrap()();
                                            __flight_result
                                        };
                                        let y = {
                                            let __flight_callback = (read_number).clone();
                                            let __flight_result =
                                                __flight_callback.lock().unwrap()();
                                            __flight_result
                                        };
                                        if ((((x1).is_none()) || ((y1).is_none()))
                                            || ((x).is_none()))
                                            || ((y).is_none())
                                        {
                                            return false;
                                        }
                                        let cx = if relative {
                                            (current_x + x1)
                                        } else {
                                            (x1).clone().unwrap()
                                        };
                                        let cy = if relative {
                                            (current_y + y1)
                                        } else {
                                            (y1).clone().unwrap()
                                        };
                                        let ax = if relative {
                                            (current_x + x)
                                        } else {
                                            (x).clone().unwrap()
                                        };
                                        let ay = if relative {
                                            (current_y + y)
                                        } else {
                                            (y).clone().unwrap()
                                        };
                                        append_path_curve_to(path, cx, cy, ax, ay);
                                        last_quad_control_x = cx;
                                        last_quad_control_y = cy;
                                        current_x = ax;
                                        current_y = ay;
                                        last_kind = "Q".to_owned();
                                    } else {
                                        if (upper == "T") {
                                            let x = {
                                                let __flight_callback = (read_number).clone();
                                                let __flight_result =
                                                    __flight_callback.lock().unwrap()();
                                                __flight_result
                                            };
                                            let y = {
                                                let __flight_callback = (read_number).clone();
                                                let __flight_result =
                                                    __flight_callback.lock().unwrap()();
                                                __flight_result
                                            };
                                            if ((x).is_none()) || ((y).is_none()) {
                                                return false;
                                            }
                                            let reflect = (last_kind == "Q") || (last_kind == "T");
                                            let cx = if reflect {
                                                ((2.0_f64 * current_x) - last_quad_control_x)
                                            } else {
                                                current_x
                                            };
                                            let cy = if reflect {
                                                ((2.0_f64 * current_y) - last_quad_control_y)
                                            } else {
                                                current_y
                                            };
                                            let ax = if relative {
                                                (current_x + x)
                                            } else {
                                                (x).clone().unwrap()
                                            };
                                            let ay = if relative {
                                                (current_y + y)
                                            } else {
                                                (y).clone().unwrap()
                                            };
                                            append_path_curve_to(path, cx, cy, ax, ay);
                                            last_quad_control_x = cx;
                                            last_quad_control_y = cy;
                                            current_x = ax;
                                            current_y = ay;
                                            last_kind = "T".to_owned();
                                        } else {
                                            if (upper == "A") {
                                                let rx = {
                                                    let __flight_callback = (read_number).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let ry = {
                                                    let __flight_callback = (read_number).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let rotation_degrees = {
                                                    let __flight_callback = (read_number).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let large_arc = {
                                                    let __flight_callback = (read_flag).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let sweep = {
                                                    let __flight_callback = (read_flag).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let x = {
                                                    let __flight_callback = (read_number).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                let y = {
                                                    let __flight_callback = (read_number).clone();
                                                    let __flight_result =
                                                        __flight_callback.lock().unwrap()();
                                                    __flight_result
                                                };
                                                if (((((((rx).is_none()) || ((ry).is_none()))
                                                    || ((rotation_degrees).is_none()))
                                                    || ((large_arc).is_none()))
                                                    || ((sweep).is_none()))
                                                    || ((x).is_none()))
                                                    || ((y).is_none())
                                                {
                                                    return false;
                                                }
                                                let ax = if relative {
                                                    (current_x + x)
                                                } else {
                                                    (x).clone().unwrap()
                                                };
                                                let ay = if relative {
                                                    (current_y + y)
                                                } else {
                                                    (y).clone().unwrap()
                                                };
                                                append_path_arc_to(
                                                    path,
                                                    (rx).clone().unwrap(),
                                                    (ry).clone().unwrap(),
                                                    ((rotation_degrees * std::f64::consts::PI)
                                                        / 180.0_f64),
                                                    (large_arc) == Some(1.0_f64),
                                                    (sweep) == Some(1.0_f64),
                                                    ax,
                                                    ay,
                                                );
                                                current_x = ax;
                                                current_y = ay;
                                                last_kind = "A".to_owned();
                                            } else {
                                                return false;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            first = false;
            if (active == "M") {
                active = "L".to_owned();
            } else {
                if (active == "m") {
                    active = "l".to_owned();
                }
            }
        }
    }
    return true;
}

// Source: upstream/packages/path-formats/src/svgPathData.ts:282 (sha256:55264b4768f768b2ea3800cf3ca542d138e2f3b9614d0a30a9a7c965bbd9e588)
pub fn format_svg_path_data(path: &Path, options: Option<SharedStructuralRecord1>) -> String {
    let precision = options.as_ref().and_then(|value| value.precision);
    let parts: std::sync::Arc<std::sync::Mutex<Vec<String>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    for_each_path_segment(path, &mut |segment: PathSegment| -> () {
        if ((segment.kind).clone() == "moveTo") {
            (*parts.lock().unwrap()).push(format!(
                "M{} {}",
                format_svg_number((segment.x).unwrap(), Some((precision).clone().unwrap())),
                format_svg_number((segment.y).unwrap(), Some((precision).clone().unwrap()))
            ));
        } else {
            if ((segment.kind).clone() == "lineTo") {
                (*parts.lock().unwrap()).push(format!(
                    "L{} {}",
                    format_svg_number((segment.x).unwrap(), Some((precision).clone().unwrap())),
                    format_svg_number((segment.y).unwrap(), Some((precision).clone().unwrap()))
                ));
            } else {
                if ((segment.kind).clone() == "curveTo") {
                    (*parts.lock().unwrap()).push(
                        (format!(
                            "Q{} {} ",
                            format_svg_number(
                                (segment.control_x).unwrap(),
                                Some((precision).clone().unwrap())
                            ),
                            format_svg_number(
                                (segment.control_y).unwrap(),
                                Some((precision).clone().unwrap())
                            )
                        ) + format!(
                            "{} {}",
                            format_svg_number(
                                (segment.x).unwrap(),
                                Some((precision).clone().unwrap())
                            ),
                            format_svg_number(
                                (segment.y).unwrap(),
                                Some((precision).clone().unwrap())
                            )
                        )),
                    );
                } else {
                    if ((segment.kind).clone() == "cubicCurveTo") {
                        (*parts.lock().unwrap()).push(
                            ((format!(
                                "C{} {} ",
                                format_svg_number(
                                    (segment.control1_x).unwrap(),
                                    Some((precision).clone().unwrap())
                                ),
                                format_svg_number(
                                    (segment.control1_y).unwrap(),
                                    Some((precision).clone().unwrap())
                                )
                            ) + format!(
                                "{} {} ",
                                format_svg_number(
                                    (segment.control2_x).unwrap(),
                                    Some((precision).clone().unwrap())
                                ),
                                format_svg_number(
                                    (segment.control2_y).unwrap(),
                                    Some((precision).clone().unwrap())
                                )
                            )) + format!(
                                "{} {}",
                                format_svg_number(
                                    (segment.x).unwrap(),
                                    Some((precision).clone().unwrap())
                                ),
                                format_svg_number(
                                    (segment.y).unwrap(),
                                    Some((precision).clone().unwrap())
                                )
                            )),
                        );
                    } else {
                        if ((segment.kind).clone() == "close") {
                            (*parts.lock().unwrap()).push("Z".to_owned());
                        }
                    }
                }
            }
        }
    });
    return ((*parts.lock().unwrap()).join)("");
}

// Source: upstream/packages/path-formats/src/svgPathData.ts:313 (sha256:53f9355169e9a9fa6c28fe78487246e5ed91738942fde2c3834d0f527bcad355)
pub fn parse_svg_path_data(d: String) -> Option<Path> {
    let mut path = create_path(None);
    if (!append_svg_path_data(&mut path, (d).clone())) {
        return None;
    }
    return Some((path).clone());
}

// Source: upstream/packages/path-formats/src/svgPathData.ts:319 (sha256:3d55b5b9648b702cfeeefab18e92d5fe18956a550ceca1e8cd07b8810e636fef)
fn format_svg_number(value: f64, precision: Option<f64>) -> String {
    if (precision).is_none() {
        return string(value);
    }
    let factor = (10.0_f64).powf(*(precision.as_ref().unwrap()));
    return string(((value * factor).round() / factor));
}

// Source: upstream/packages/path-formats/src/svgPathData.ts:326 (sha256:b96909a1054a73f2443e51c64f02be6673290494c302b7d1aa6b231cb00d229a)
fn is_svg_command_letter(c: String) -> bool {
    return (("MmLlHhVvCcSsQqTtAaZz".index_of)(c) != (-1.0_f64));
}
