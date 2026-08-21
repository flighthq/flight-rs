// @generated from upstream/packages/path/src/strokePathGeometry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::flatten_path;
use flighthq_types::{Path, StrokeStyle};

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

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub issue: StrokePathTessellationIssue,
    pub piece: Option<StrokePathPieceGeometry>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:5 (sha256:46ba3323b393d7347a795ac01664de3d174417ccd7ea7c79063f41d02c6672b0)
pub const STROKE_PATH_TESSELLATION_ISSUE_NONE: f64 = 0.0_f64;

// Source: upstream/packages/path/src/strokePathGeometry.ts:6 (sha256:2be3a27d88cfe47a2b353d442662d97d686af1f94ff66a63e17af46bbb421976)
pub const STROKE_PATH_TESSELLATION_ISSUE_INVALID_STYLE: f64 = 1.0_f64;

// Source: upstream/packages/path/src/strokePathGeometry.ts:7 (sha256:11186c559b9cac93902cf989d40dde9a6fa88131d096220fa736dfa885d37bd9)
pub const STROKE_PATH_TESSELLATION_ISSUE_INVALID_PATH: f64 = 2.0_f64;

// Source: upstream/packages/path/src/strokePathGeometry.ts:8 (sha256:009e0b470da6b23b21acc76a2790090664dc3dfd0b920c38d4c02de853955789)
pub const STROKE_PATH_TESSELLATION_ISSUE_SELF_INTERSECTING_CENTERLINE: f64 = 3.0_f64;

// Source: upstream/packages/path/src/strokePathGeometry.ts:9 (sha256:d657d0dd520c2b976a0ca3390c95eb5db82d3860943669daed048ae814b33996)
pub const STROKE_PATH_TESSELLATION_ISSUE_REVERSING_JOIN: f64 = 4.0_f64;

// Source: upstream/packages/path/src/strokePathGeometry.ts:10 (sha256:550cda4f043d426fe7408287ba4f7ef988c51d953898480dcefad5fcbe649133)
pub const STROKE_PATH_TESSELLATION_ISSUE_SELF_INTERSECTING_OUTLINE: f64 = 5.0_f64;

// Source: upstream/packages/path/src/strokePathGeometry.ts:12 (sha256:7c4f07f3c5463bf254f430c818be5e933b1a520693889138e9b9e75da9cfa08d)
pub(crate) type StrokePathTessellationIssue = f64;

// Source: upstream/packages/path/src/strokePathGeometry.ts:14 (sha256:e36e5c24f09611d8ab2d69d53c5a52c86ef9f22a8df0252dc7ce975f8b9cc698)
#[derive(Clone, Default)]
pub(crate) struct StrokePathPieceGeometry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub closed: bool,
    pub end_cap: Vec<f64>,
    pub left: Vec<f64>,
    pub right: Vec<f64>,
    pub start_cap: Vec<f64>,
}
impl PartialEq for StrokePathPieceGeometry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:22 (sha256:07e920496984e5c987311c4071922f24f7dd348c941d5b1485162de82eb73dbf)
#[derive(Clone, Default)]
pub(crate) struct StrokePathGeometry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub issue: StrokePathTessellationIssue,
    pub issue_subpath: Option<f64>,
    pub pieces: Vec<StrokePathPieceGeometry>,
}
impl PartialEq for StrokePathGeometry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:28 (sha256:deca72ba596a78d4e9de007631430cb8c44860c8d59016479060d71ffdfc81a1)
#[derive(Clone, Default)]
pub(crate) struct StrokeSubpath {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub closed: bool,
    pub points: Vec<f64>,
    pub source_index: f64,
}
impl PartialEq for StrokeSubpath {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:34 (sha256:d14751825885d50da9a9875eaebe99974e8b9dbc7f4754307ded48d09f6c4801)
#[derive(Clone, Default)]
pub(crate) struct SegmentFrame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub nx: f64,
    pub ny: f64,
    pub tx: f64,
    pub ty: f64,
}
impl PartialEq for SegmentFrame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:45 (sha256:821971d43969e1f5a54f2cfbfc6f4bc6c69f9148849edba8ec9db7a61d9ffaec)
pub fn build_stroke_path_geometry(
    path: &Path,
    style: &StrokeStyle,
    tolerance: f64,
) -> StrokePathGeometry {
    let width = (style.width).clone().unwrap_or(1.0_f64);
    let dash_offset = (style.dash_offset).clone().unwrap_or(0.0_f64);
    let miter_limit = (style.miter_limit).clone().unwrap_or(4.0_f64);
    if ((((((!(width).is_finite()) || (width <= 0.0_f64)) || (!(tolerance).is_finite()))
        || (tolerance <= 0.0_f64))
        || (!(dash_offset).is_finite()))
        || (!(miter_limit).is_finite()))
        || (miter_limit < 0.0_f64)
    {
        return StrokePathGeometry {
            __flight_identity: std::sync::Arc::new(()),
            issue: STROKE_PATH_TESSELLATION_ISSUE_INVALID_STYLE,
            issue_subpath: None,
            pieces: vec![],
        };
    }
    let dash = ((style.dash).clone())
        .clone()
        .unwrap_or(((*EMPTY_DASH).clone()).clone());
    {
        let mut i = 0.0_f64;
        while (i < (dash.len() as f64)) {
            if (!(dash[i as usize].clone()).is_finite()) || (dash[i as usize].clone() < 0.0_f64) {
                return StrokePathGeometry {
                    __flight_identity: std::sync::Arc::new(()),
                    issue: STROKE_PATH_TESSELLATION_ISSUE_INVALID_STYLE,
                    issue_subpath: None,
                    pieces: vec![],
                };
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let source = create_stroke_subpaths(path, tolerance);
    {
        let mut i = 0.0_f64;
        while (i < (source.len() as f64)) {
            if (!are_finite_points(&source[i as usize].points)) {
                return StrokePathGeometry {
                    __flight_identity: std::sync::Arc::new(()),
                    issue: STROKE_PATH_TESSELLATION_ISSUE_INVALID_PATH,
                    issue_subpath: Some(source[i as usize].source_index),
                    pieces: vec![],
                };
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let centerline_intersection = find_centerline_intersection(&source);
    let mut issue: StrokePathTessellationIssue = if (centerline_intersection).is_none() {
        STROKE_PATH_TESSELLATION_ISSUE_NONE
    } else {
        STROKE_PATH_TESSELLATION_ISSUE_SELF_INTERSECTING_CENTERLINE
    };
    let mut issue_subpath = centerline_intersection;
    let mut pieces: Vec<StrokePathPieceGeometry> = vec![];
    let half_width = (width / 2.0_f64);
    let cap = ((style.cap).clone()).clone().unwrap_or("butt".to_owned());
    let join = ((style.join).clone()).clone().unwrap_or("miter".to_owned());
    {
        let mut i = 0.0_f64;
        while (i < (source.len() as f64)) {
            let subpath = source[i as usize].clone();
            let spans = if ((dash.len() as f64) > 0.0_f64) {
                apply_dash(&subpath, &dash, dash_offset)
            } else {
                vec![(subpath).clone()]
            };
            {
                let mut j = 0.0_f64;
                while (j < (spans.len() as f64)) {
                    let built = build_stroke_piece(
                        &spans[j as usize],
                        half_width,
                        (join).clone(),
                        (cap).clone(),
                        miter_limit,
                        tolerance,
                    );
                    if ((built.piece).clone()).is_some() {
                        pieces.push(((built.piece).clone()).unwrap());
                        if (issue == STROKE_PATH_TESSELLATION_ISSUE_NONE)
                            && (has_invalid_outline(built.piece.as_ref().unwrap()))
                        {
                            issue = STROKE_PATH_TESSELLATION_ISSUE_SELF_INTERSECTING_OUTLINE;
                            issue_subpath = Some(subpath.source_index);
                        }
                    }
                    if (issue == STROKE_PATH_TESSELLATION_ISSUE_NONE)
                        && (built.issue != STROKE_PATH_TESSELLATION_ISSUE_NONE)
                    {
                        issue = built.issue;
                        issue_subpath = Some(subpath.source_index);
                    }
                    {
                        j += 1.0;
                        j
                    };
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return StrokePathGeometry {
        __flight_identity: std::sync::Arc::new(()),
        issue: issue,
        issue_subpath: issue_subpath,
        pieces: (pieces).clone(),
    };
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:110 (sha256:f697164be508b304bf235a7791274b6367004c0d7331cd4b72b74339cb7cbfae)
fn create_stroke_subpaths(path: &Path, tolerance: f64) -> Vec<StrokeSubpath> {
    let contours = flatten_path(path, Some(tolerance));
    let mut result: Vec<StrokeSubpath> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (contours.len() as f64)) {
            let mut points = remove_consecutive_duplicates(&contours[i as usize]);
            if ((points.len() as f64) < 4.0_f64) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let last = ((points.len() as f64) - 2.0_f64);
            let closed = (((points.len() as f64) >= 8.0_f64)
                && (approximately_equal(
                    points[0.0_f64 as usize].clone(),
                    points[last as usize].clone(),
                )))
                && (approximately_equal(
                    points[1.0_f64 as usize].clone(),
                    points[(last + 1.0_f64) as usize].clone(),
                ));
            if closed {
                {
                    let __flight_index = (last) as usize;
                    let __flight_value = points[0.0_f64 as usize].clone();
                    if __flight_index == points.len() {
                        points.push(__flight_value);
                    } else {
                        points[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (last + 1.0_f64) as usize;
                    let __flight_value = points[1.0_f64 as usize].clone();
                    if __flight_index == points.len() {
                        points.push(__flight_value);
                    } else {
                        points[__flight_index] = __flight_value;
                    }
                };
            }
            result.push(StrokeSubpath {
                __flight_identity: std::sync::Arc::new(()),
                closed: closed,
                points: (points).clone(),
                source_index: i,
            });
            {
                i += 1.0;
                i
            };
        }
    }
    return result;
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:130 (sha256:3b1defe0a69768711625a7b94134b4e3dce3d67893b778a836ddb31af2f87ccf)
fn build_stroke_piece(
    subpath: &StrokeSubpath,
    half_width: f64,
    join: String,
    cap: String,
    miter_limit: f64,
    tolerance: f64,
) -> SharedStructuralRecord1 {
    let point_count = (__flight_js_to_i32((subpath.points.len() as f64))
        >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    let segment_count = (point_count - 1.0_f64);
    if (segment_count < 1.0_f64) {
        return SharedStructuralRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            issue: STROKE_PATH_TESSELLATION_ISSUE_NONE,
            piece: None,
        };
    }
    let mut frames: Vec<SegmentFrame> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < segment_count) {
            let x0 = subpath.points[(i * 2.0_f64) as usize].clone();
            let y0 = subpath.points[((i * 2.0_f64) + 1.0_f64) as usize].clone();
            let dx = (subpath.points[((i + 1.0_f64) * 2.0_f64) as usize].clone() - x0);
            let dy = (subpath.points[(((i + 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone() - y0);
            let length = ((dx * dx) + (dy * dy)).sqrt();
            if (length <= GEOMETRY_EPSILON) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let tx = (dx / length);
            let ty = (dy / length);
            frames.push(SegmentFrame {
                __flight_identity: std::sync::Arc::new(()),
                nx: (-ty),
                ny: tx,
                tx: tx,
                ty: ty,
            });
            {
                i += 1.0;
                i
            };
        }
    }
    if ((frames.len() as f64) != segment_count) {
        return SharedStructuralRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            issue: STROKE_PATH_TESSELLATION_ISSUE_INVALID_PATH,
            piece: None,
        };
    }
    let mut piece: StrokePathPieceGeometry = StrokePathPieceGeometry {
        __flight_identity: std::sync::Arc::new(()),
        closed: subpath.closed,
        end_cap: vec![],
        left: vec![],
        right: vec![],
        start_cap: vec![],
    };
    if (!subpath.closed) {
        append_endpoint_section(
            &mut piece,
            subpath.points[0.0_f64 as usize].clone(),
            subpath.points[1.0_f64 as usize].clone(),
            &frames[0.0_f64 as usize],
            half_width,
            (cap).clone(),
            true,
            tolerance,
        );
        {
            let mut i = 1.0_f64;
            while (i < (point_count - 1.0_f64)) {
                let issue = append_join_sections(
                    &mut piece,
                    subpath.points[(i * 2.0_f64) as usize].clone(),
                    subpath.points[((i * 2.0_f64) + 1.0_f64) as usize].clone(),
                    &frames[(i - 1.0_f64) as usize],
                    &frames[i as usize],
                    half_width,
                    (join).clone(),
                    miter_limit,
                    tolerance,
                );
                if (issue != STROKE_PATH_TESSELLATION_ISSUE_NONE) {
                    return SharedStructuralRecord1 {
                        __flight_identity: std::sync::Arc::new(()),
                        issue: issue,
                        piece: Some((piece).clone()),
                    };
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
        append_endpoint_section(
            &mut piece,
            subpath.points[((point_count - 1.0_f64) * 2.0_f64) as usize].clone(),
            subpath.points[(((point_count - 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone(),
            &frames[((frames.len() as f64) - 1.0_f64) as usize],
            half_width,
            (cap).clone(),
            false,
            tolerance,
        );
    } else {
        let unique_point_count = (point_count - 1.0_f64);
        {
            let mut i = 0.0_f64;
            while (i < unique_point_count) {
                let issue = append_join_sections(
                    &mut piece,
                    subpath.points[(i * 2.0_f64) as usize].clone(),
                    subpath.points[((i * 2.0_f64) + 1.0_f64) as usize].clone(),
                    &frames[(((i + (frames.len() as f64)) - 1.0_f64) % (frames.len() as f64))
                        as usize],
                    &frames[i as usize],
                    half_width,
                    (join).clone(),
                    miter_limit,
                    tolerance,
                );
                if (issue != STROKE_PATH_TESSELLATION_ISSUE_NONE) {
                    return SharedStructuralRecord1 {
                        __flight_identity: std::sync::Arc::new(()),
                        issue: issue,
                        piece: Some((piece).clone()),
                    };
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    return SharedStructuralRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        issue: STROKE_PATH_TESSELLATION_ISSUE_NONE,
        piece: Some((piece).clone()),
    };
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:204 (sha256:1195e0f21a4eaed173930f1ae0d804551a220fe0a2b6a5356074890759070b51)
fn append_endpoint_section(
    piece: &mut StrokePathPieceGeometry,
    px: f64,
    py: f64,
    frame: &SegmentFrame,
    half_width: f64,
    cap: String,
    start: bool,
    tolerance: f64,
) -> () {
    let extension = if (cap == "square") {
        if start { (-half_width) } else { half_width }
    } else {
        0.0_f64
    };
    let cx = (px + (frame.tx * extension));
    let cy = (py + (frame.ty * extension));
    append_section(
        piece,
        (cx + (frame.nx * half_width)),
        (cy + (frame.ny * half_width)),
        (cx - (frame.nx * half_width)),
        (cy - (frame.ny * half_width)),
    );
    if (cap != "round") {
        return;
    }
    if start {
        piece.start_cap = create_arc_interior_points(
            px,
            py,
            half_width,
            (-frame.ny).atan2((-frame.nx)),
            (-std::f64::consts::PI),
            tolerance,
        );
    } else {
        piece.end_cap = create_arc_interior_points(
            px,
            py,
            half_width,
            (frame.ny).atan2(frame.nx),
            (-std::f64::consts::PI),
            tolerance,
        );
    }
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:232 (sha256:35652300fd777a4411d89478bc5d9d005ac6fe949bf4b1229db20d950070795c)
fn append_join_sections(
    piece: &mut StrokePathPieceGeometry,
    px: f64,
    py: f64,
    previous: &SegmentFrame,
    next: &SegmentFrame,
    half_width: f64,
    join: String,
    miter_limit: f64,
    tolerance: f64,
) -> StrokePathTessellationIssue {
    let turn = cross(previous.tx, previous.ty, next.tx, next.ty);
    let direction = ((previous.tx * next.tx) + (previous.ty * next.ty));
    if ((turn).abs() <= GEOMETRY_EPSILON) {
        if (direction < 0.0_f64) {
            return STROKE_PATH_TESSELLATION_ISSUE_REVERSING_JOIN;
        }
        append_section(
            piece,
            (px + (next.nx * half_width)),
            (py + (next.ny * half_width)),
            (px - (next.nx * half_width)),
            (py - (next.ny * half_width)),
        );
        return STROKE_PATH_TESSELLATION_ISSUE_NONE;
    }
    let left0_x = (px + (previous.nx * half_width));
    let left0_y = (py + (previous.ny * half_width));
    let left1_x = (px + (next.nx * half_width));
    let left1_y = (py + (next.ny * half_width));
    let right0_x = (px - (previous.nx * half_width));
    let right0_y = (py - (previous.ny * half_width));
    let right1_x = (px - (next.nx * half_width));
    let right1_y = (py - (next.ny * half_width));
    let left_intersection = intersect_lines(
        left0_x,
        left0_y,
        previous.tx,
        previous.ty,
        left1_x,
        left1_y,
        next.tx,
        next.ty,
    );
    let right_intersection = intersect_lines(
        right0_x,
        right0_y,
        previous.tx,
        previous.ty,
        right1_x,
        right1_y,
        next.tx,
        next.ty,
    );
    if ((left_intersection).is_none()) || ((right_intersection).is_none()) {
        return STROKE_PATH_TESSELLATION_ISSUE_REVERSING_JOIN;
    }
    let outer_intersection = if (turn > 0.0_f64) {
        (right_intersection.as_ref().unwrap()).clone()
    } else {
        (left_intersection.as_ref().unwrap()).clone()
    };
    let outer_distance = ((outer_intersection[0.0_f64 as usize].clone() - px).powi(2)
        + (outer_intersection[1.0_f64 as usize].clone() - py).powi(2))
    .sqrt();
    if ((join == "miter") && ((miter_limit).is_finite()))
        && (outer_distance <= (half_width * (0.0_f64).max(miter_limit)))
    {
        append_section(
            piece,
            left_intersection.as_ref().unwrap()[0.0_f64 as usize].clone(),
            left_intersection.as_ref().unwrap()[1.0_f64 as usize].clone(),
            right_intersection.as_ref().unwrap()[0.0_f64 as usize].clone(),
            right_intersection.as_ref().unwrap()[1.0_f64 as usize].clone(),
        );
        return STROKE_PATH_TESSELLATION_ISSUE_NONE;
    }
    let inner = if (turn > 0.0_f64) {
        (left_intersection.as_ref().unwrap()).clone()
    } else {
        (right_intersection.as_ref().unwrap()).clone()
    };
    if (join == "round") {
        let outer0_x = if (turn > 0.0_f64) { right0_x } else { left0_x };
        let outer0_y = if (turn > 0.0_f64) { right0_y } else { left0_y };
        let outer_start_angle = (outer0_y - py).atan2((outer0_x - px));
        let sweep = signed_join_sweep(previous, next, turn);
        let outer = create_arc_points(px, py, half_width, outer_start_angle, sweep, tolerance);
        {
            let mut i = 0.0_f64;
            while (i < (outer.len() as f64)) {
                if (turn > 0.0_f64) {
                    append_section(
                        piece,
                        inner[0.0_f64 as usize].clone(),
                        inner[1.0_f64 as usize].clone(),
                        outer[i as usize].clone(),
                        outer[(i + 1.0_f64) as usize].clone(),
                    );
                } else {
                    append_section(
                        piece,
                        outer[i as usize].clone(),
                        outer[(i + 1.0_f64) as usize].clone(),
                        inner[0.0_f64 as usize].clone(),
                        inner[1.0_f64 as usize].clone(),
                    );
                }
                {
                    i += 2.0_f64;
                    i.clone()
                };
            }
        }
    } else {
        if (turn > 0.0_f64) {
            append_section(
                piece,
                inner[0.0_f64 as usize].clone(),
                inner[1.0_f64 as usize].clone(),
                right0_x,
                right0_y,
            );
            append_section(
                piece,
                inner[0.0_f64 as usize].clone(),
                inner[1.0_f64 as usize].clone(),
                right1_x,
                right1_y,
            );
        } else {
            append_section(
                piece,
                left0_x,
                left0_y,
                inner[0.0_f64 as usize].clone(),
                inner[1.0_f64 as usize].clone(),
            );
            append_section(
                piece,
                left1_x,
                left1_y,
                inner[0.0_f64 as usize].clone(),
                inner[1.0_f64 as usize].clone(),
            );
        }
    }
    return STROKE_PATH_TESSELLATION_ISSUE_NONE;
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:306 (sha256:030214eb7ae9bdfca535b27d5e59ae0f1b76bfb2248376ea8f7d2f79e774ea54)
fn signed_join_sweep(previous: &SegmentFrame, next: &SegmentFrame, turn: f64) -> f64 {
    let start = if (turn > 0.0_f64) {
        (-previous.ny).atan2((-previous.nx))
    } else {
        (previous.ny).atan2(previous.nx)
    };
    let end = if (turn > 0.0_f64) {
        (-next.ny).atan2((-next.nx))
    } else {
        (next.ny).atan2(next.nx)
    };
    let mut sweep = (end - start);
    if (turn > 0.0_f64) && (sweep < 0.0_f64) {
        sweep += (std::f64::consts::PI * 2.0_f64);
    }
    if (turn < 0.0_f64) && (sweep > 0.0_f64) {
        sweep -= (std::f64::consts::PI * 2.0_f64);
    }
    return sweep;
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:315 (sha256:4159532a68a64607b672ea4c5e378663e168688ca48977f6b751ee7b67e12354)
fn append_section(piece: &mut StrokePathPieceGeometry, lx: f64, ly: f64, rx: f64, ry: f64) -> () {
    piece.left.extend(vec![lx, ly]);
    piece.right.extend(vec![rx, ry]);
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:320 (sha256:6d5e4767cf94260910e2f38553fd3c28a92c3e948dc9537420c877f7a0b1d0f0)
fn create_arc_interior_points(
    cx: f64,
    cy: f64,
    radius: f64,
    start_angle: f64,
    sweep: f64,
    tolerance: f64,
) -> Vec<f64> {
    let points = create_arc_points(cx, cy, radius, start_angle, sweep, tolerance);
    return (points)[(2.0_f64) as usize..(-2.0_f64) as usize].to_vec();
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:332 (sha256:9c7d943b10a6f92e0b3653646a82f9e36321123e7e03295cf7dd7ef0546d9e0c)
fn create_arc_points(
    cx: f64,
    cy: f64,
    radius: f64,
    start_angle: f64,
    sweep: f64,
    tolerance: f64,
) -> Vec<f64> {
    let ratio = (-1.0_f64).max((1.0_f64).min((1.0_f64 - (tolerance / radius))));
    let max_step = (std::f64::consts::PI / 32.0_f64).max((2.0_f64 * (ratio).acos()));
    let steps = (1.0_f64).max(((sweep).abs() / max_step).ceil());
    let mut points: Vec<f64> = vec![];
    {
        let mut i = 0.0_f64;
        while (i <= steps) {
            let angle = (start_angle + ((sweep * i) / steps));
            points.extend(vec![
                (cx + ((angle).cos() * radius)),
                (cy + ((angle).sin() * radius)),
            ]);
            {
                i += 1.0;
                i
            };
        }
    }
    return points;
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:351 (sha256:6af8384c258f440f7f37f243a0577591c334d8926bcc9c1d7f11ebe463e0c7bc)
fn intersect_lines(
    ax: f64,
    ay: f64,
    adx: f64,
    ady: f64,
    bx: f64,
    by: f64,
    bdx: f64,
    bdy: f64,
) -> Option<Vec<f64>> {
    let denominator = cross(adx, ady, bdx, bdy);
    if ((denominator).abs() <= GEOMETRY_EPSILON) {
        return None;
    }
    let scale = (cross((bx - ax), (by - ay), bdx, bdy) / denominator);
    return Some(vec![(ax + (adx * scale)), (ay + (ady * scale))]);
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:367 (sha256:911a464092d2a56c48f9f81e3cb40895f5f7504a78cf293031decdf2a42f6e88)
fn apply_dash(subpath: &StrokeSubpath, dash: &Vec<f64>, dash_offset: f64) -> Vec<StrokeSubpath> {
    let pattern = if (((dash.len() as f64) % 2.0_f64) == 0.0_f64) {
        (*dash).clone()
    } else {
        {
            let mut __flight_values = (*dash).clone();
            let __flight_concat_0 = (*dash).clone();
            __flight_values.extend(__flight_concat_0.iter().cloned());
            __flight_values
        }
    };
    let total = (pattern)
        .iter()
        .cloned()
        .fold(0.0_f64, |sum: f64, value: f64| -> f64 { (sum + value) });
    if (total <= GEOMETRY_EPSILON) {
        return vec![(*subpath).clone()];
    }
    let offset = (((dash_offset % total) + total) % total);
    let mut pattern_index = 0.0_f64;
    let mut consumed_offset = offset;
    while (pattern[pattern_index as usize].clone() <= GEOMETRY_EPSILON)
        || (consumed_offset >= pattern[pattern_index as usize].clone())
    {
        if (pattern[pattern_index as usize].clone() > GEOMETRY_EPSILON) {
            consumed_offset -= pattern[pattern_index as usize].clone();
        }
        pattern_index = ((pattern_index + 1.0_f64) % (pattern.len() as f64));
    }
    let mut remaining = (pattern[pattern_index as usize].clone() - consumed_offset);
    let mut on = ((pattern_index % 2.0_f64) == 0.0_f64);
    let mut result: Vec<StrokeSubpath> = vec![];
    let mut current: Option<Vec<f64>> = None;
    {
        let mut segment = 0.0_f64;
        while (segment
            < ((__flight_js_to_i32((subpath.points.len() as f64))
                >> (__flight_js_to_u32(1.0_f64) & 31)) as f64
                - 1.0_f64))
        {
            let x0 = subpath.points[(segment * 2.0_f64) as usize].clone();
            let y0 = subpath.points[((segment * 2.0_f64) + 1.0_f64) as usize].clone();
            let x1 = subpath.points[((segment + 1.0_f64) * 2.0_f64) as usize].clone();
            let y1 = subpath.points[(((segment + 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone();
            let dx = (x1 - x0);
            let dy = (y1 - y0);
            let length = ((dx).powi(2) + (dy).powi(2)).sqrt();
            let mut distance = 0.0_f64;
            while (distance < (length - GEOMETRY_EPSILON)) {
                while (remaining <= GEOMETRY_EPSILON) {
                    pattern_index = ((pattern_index + 1.0_f64) % (pattern.len() as f64));
                    remaining = pattern[pattern_index as usize].clone();
                    on = ((pattern_index % 2.0_f64) == 0.0_f64);
                    if (!on) && (((current).clone()).is_some()) {
                        push_dash(
                            &mut result,
                            &current.as_mut().unwrap(),
                            subpath.source_index,
                        );
                        current = None;
                    }
                }
                let step = (remaining).min((length - distance));
                let start_x = (x0 + ((dx * distance) / length));
                let start_y = (y0 + ((dy * distance) / length));
                distance += step;
                let end_x = (x0 + ((dx * distance) / length));
                let end_y = (y0 + ((dy * distance) / length));
                if on {
                    if current.is_none() {
                        current = Some(vec![start_x, start_y]);
                    };
                    current.as_mut().unwrap().extend(vec![end_x, end_y]);
                } else {
                    if ((current).clone()).is_some() {
                        push_dash(
                            &mut result,
                            &current.as_mut().unwrap(),
                            subpath.source_index,
                        );
                        current = None;
                    }
                }
                remaining -= step;
            }
            {
                segment += 1.0;
                segment
            };
        }
    }
    if (current).is_some() {
        push_dash(
            &mut result,
            &current.as_mut().unwrap(),
            subpath.source_index,
        );
    }
    return result;
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:423 (sha256:7b0a1ef1396455d25db96d4ffa22b2a75bd8c032d57723284fa1247543367f4e)
fn push_dash(result: &mut Vec<StrokeSubpath>, points: &Vec<f64>, source_index: f64) -> () {
    if ((points.len() as f64) >= 4.0_f64) {
        result.push(StrokeSubpath {
            __flight_identity: std::sync::Arc::new(()),
            closed: false,
            points: (*points).clone(),
            source_index: source_index,
        });
    }
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:427 (sha256:0b2f27a706a899992ab2daf372b326c73d93187dd4947f6c3ff848b9bf1765e4)
fn find_centerline_intersection(subpaths: &Vec<StrokeSubpath>) -> Option<f64> {
    {
        let mut i = 0.0_f64;
        while (i < (subpaths.len() as f64)) {
            if has_polyline_self_intersection(
                &subpaths[i as usize].points,
                subpaths[i as usize].closed,
            ) {
                return Some(subpaths[i as usize].source_index);
            }
            {
                let mut j = 0.0_f64;
                while (j < i) {
                    if do_polylines_intersect(
                        &subpaths[i as usize].points,
                        subpaths[i as usize].closed,
                        &subpaths[j as usize].points,
                        subpaths[j as usize].closed,
                    ) {
                        return Some(subpaths[i as usize].source_index);
                    }
                    {
                        j += 1.0;
                        j
                    };
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return None;
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:439 (sha256:9117d5d49a70ec6226d653036158ea59eca39dfccb040c53966ec76bdf2811e0)
fn has_invalid_outline(piece: &StrokePathPieceGeometry) -> bool {
    let left = remove_consecutive_duplicates(&piece.left);
    let right = remove_consecutive_duplicates(&piece.right);
    if piece.closed {
        return ((has_polyline_self_intersection(&left, true))
            || (has_polyline_self_intersection(&right, true)))
            || (do_polylines_intersect(&left, true, &right, true));
    }
    let outline = {
        let mut __flight_values = (left).clone();
        let __flight_concat_0 = (piece.end_cap).clone();
        __flight_values.extend(__flight_concat_0.iter().cloned());
        let __flight_concat_1 = reverse_points(&right);
        __flight_values.extend(__flight_concat_1.iter().cloned());
        let __flight_concat_2 = (piece.start_cap).clone();
        __flight_values.extend(__flight_concat_2.iter().cloned());
        __flight_values
    };
    return has_polyline_self_intersection(&remove_consecutive_duplicates(&outline), true);
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:453 (sha256:2e503d4a4e095b87d8a1b4746f1581aadf62271061a6cee89365b9828f0ccb63)
fn has_polyline_self_intersection(points: &Vec<f64>, closed: bool) -> bool {
    let count = get_polyline_point_count(points, closed);
    let segment_count = if closed { count } else { (count - 1.0_f64) };
    {
        let mut i = 0.0_f64;
        while (i < segment_count) {
            let i_next = ((i + 1.0_f64) % count);
            if same_point(points, i, i_next) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            {
                let mut j = (i + 1.0_f64);
                while (j < segment_count) {
                    let j_next = ((j + 1.0_f64) % count);
                    if (same_point(points, j, j_next))
                        || (segments_are_adjacent(i, j, segment_count, closed))
                    {
                        {
                            j += 1.0;
                            j
                        };
                        continue;
                    }
                    if segments_intersect(points, i, i_next, points, j, j_next) {
                        return true;
                    }
                    {
                        j += 1.0;
                        j
                    };
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return false;
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:468 (sha256:fde591967d209991b31d41c165ab33a11ca252523664b53a2afcb107e25c292f)
fn do_polylines_intersect(a: &Vec<f64>, a_closed: bool, b: &Vec<f64>, b_closed: bool) -> bool {
    let a_count = get_polyline_point_count(a, a_closed);
    let b_count = get_polyline_point_count(b, b_closed);
    let a_segments = if a_closed {
        a_count
    } else {
        (a_count - 1.0_f64)
    };
    let b_segments = if b_closed {
        b_count
    } else {
        (b_count - 1.0_f64)
    };
    {
        let mut i = 0.0_f64;
        while (i < a_segments) {
            let i_next = ((i + 1.0_f64) % a_count);
            if same_point(a, i, i_next) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            {
                let mut j = 0.0_f64;
                while (j < b_segments) {
                    let j_next = ((j + 1.0_f64) % b_count);
                    if same_point(b, j, j_next) {
                        {
                            j += 1.0;
                            j
                        };
                        continue;
                    }
                    if segments_intersect(a, i, i_next, b, j, j_next) {
                        return true;
                    }
                    {
                        j += 1.0;
                        j
                    };
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return false;
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:485 (sha256:cc69991d0fb5916c3cb0aefb4c6fdb4d19ee0f779b45279b2d918b359c6b6bcb)
fn get_polyline_point_count(points: &Vec<f64>, closed: bool) -> f64 {
    let count =
        (__flight_js_to_i32((points.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    return if ((closed) && (count > 1.0_f64)) && (same_point(points, 0.0_f64, (count - 1.0_f64))) {
        (count - 1.0_f64)
    } else {
        count
    };
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:490 (sha256:5e732117f758de898741ba0d1efb674f10cba0215dee756d26bdb200a55a9ee4)
fn segments_are_adjacent(a: f64, b: f64, segment_count: f64, closed: bool) -> bool {
    if (b == (a + 1.0_f64)) {
        return true;
    }
    return ((closed) && (a == 0.0_f64)) && (b == (segment_count - 1.0_f64));
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:495 (sha256:aedfa60fa6394c26401aa252ec0a3b57dfecf56b731be1f012eb019876e70bdb)
fn segments_intersect(a: &Vec<f64>, ai: f64, aj: f64, b: &Vec<f64>, bi: f64, bj: f64) -> bool {
    let ax = a[(ai * 2.0_f64) as usize].clone();
    let ay = a[((ai * 2.0_f64) + 1.0_f64) as usize].clone();
    let bx = a[(aj * 2.0_f64) as usize].clone();
    let by = a[((aj * 2.0_f64) + 1.0_f64) as usize].clone();
    let cx = b[(bi * 2.0_f64) as usize].clone();
    let cy = b[((bi * 2.0_f64) + 1.0_f64) as usize].clone();
    let dx = b[(bj * 2.0_f64) as usize].clone();
    let dy = b[((bj * 2.0_f64) + 1.0_f64) as usize].clone();
    let ab_c = cross((bx - ax), (by - ay), (cx - ax), (cy - ay));
    let ab_d = cross((bx - ax), (by - ay), (dx - ax), (dy - ay));
    let cd_a = cross((dx - cx), (dy - cy), (ax - cx), (ay - cy));
    let cd_b = cross((dx - cx), (dy - cy), (bx - cx), (by - cy));
    if (((ab_c > GEOMETRY_EPSILON) && (ab_d < (-GEOMETRY_EPSILON)))
        || ((ab_c < (-GEOMETRY_EPSILON)) && (ab_d > GEOMETRY_EPSILON)))
        && (((cd_a > GEOMETRY_EPSILON) && (cd_b < (-GEOMETRY_EPSILON)))
            || ((cd_a < (-GEOMETRY_EPSILON)) && (cd_b > GEOMETRY_EPSILON)))
    {
        return true;
    }
    if ((ab_c).abs() <= GEOMETRY_EPSILON) && (point_on_segment(cx, cy, ax, ay, bx, by)) {
        return true;
    }
    if ((ab_d).abs() <= GEOMETRY_EPSILON) && (point_on_segment(dx, dy, ax, ay, bx, by)) {
        return true;
    }
    if ((cd_a).abs() <= GEOMETRY_EPSILON) && (point_on_segment(ax, ay, cx, cy, dx, dy)) {
        return true;
    }
    return ((cd_b).abs() <= GEOMETRY_EPSILON) && (point_on_segment(bx, by, cx, cy, dx, dy));
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:527 (sha256:61fd17f46adf7cf174db25bee26316773420b292ac569ef0964ef400cf737e63)
fn point_on_segment(px: f64, py: f64, ax: f64, ay: f64, bx: f64, by: f64) -> bool {
    return (((px >= ((ax).min(bx) - GEOMETRY_EPSILON))
        && (px <= ((ax).max(bx) + GEOMETRY_EPSILON)))
        && (py >= ((ay).min(by) - GEOMETRY_EPSILON)))
        && (py <= ((ay).max(by) + GEOMETRY_EPSILON));
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:536 (sha256:b40fc5aceac4368d2985a3925ae3e41e3c0654ebaa5f64466218378cc1bbb292)
fn remove_consecutive_duplicates(source: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (source.len() as f64)) {
            if (((result.len() as f64) > 0.0_f64)
                && (approximately_equal(
                    result[((result.len() as f64) - 2.0_f64) as usize].clone(),
                    source[i as usize].clone(),
                )))
                && (approximately_equal(
                    result[((result.len() as f64) - 1.0_f64) as usize].clone(),
                    source[(i + 1.0_f64) as usize].clone(),
                ))
            {
                {
                    i += 2.0_f64;
                    i.clone()
                };
                continue;
            }
            result.extend(vec![
                source[i as usize].clone(),
                source[(i + 1.0_f64) as usize].clone(),
            ]);
            {
                i += 2.0_f64;
                i.clone()
            };
        }
    }
    return result;
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:550 (sha256:0d5d922a6d7aa7aa257171a380a74d30168a1e90a0861908a6caece2b3f20162)
fn reverse_points(points: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![];
    {
        let mut i = ((points.len() as f64) - 2.0_f64);
        while (i >= 0.0_f64) {
            result.extend(vec![
                points[i as usize].clone(),
                points[(i + 1.0_f64) as usize].clone(),
            ]);
            {
                i -= 2.0_f64;
                i.clone()
            };
        }
    }
    return result;
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:556 (sha256:cf8ffcf1a4ea19d19468200a033e5b7cd9647c7f827c65a79946b9cae921ed34)
fn are_finite_points(points: &Vec<f64>) -> bool {
    {
        let mut i = 0.0_f64;
        while (i < (points.len() as f64)) {
            if (!(points[i as usize].clone()).is_finite()) {
                return false;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:561 (sha256:f16ff3285156abe72176e6e2eff258aa2d63bc140348aa28c147af7037e20fdd)
fn same_point(points: &Vec<f64>, a: f64, b: f64) -> bool {
    return (approximately_equal(
        points[(a * 2.0_f64) as usize].clone(),
        points[(b * 2.0_f64) as usize].clone(),
    )) && (approximately_equal(
        points[((a * 2.0_f64) + 1.0_f64) as usize].clone(),
        points[((b * 2.0_f64) + 1.0_f64) as usize].clone(),
    ));
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:565 (sha256:2b4e11bd8278e7c4fbd7cad79ba5734030478b1b68b9b11221504ab33044f260)
fn approximately_equal(a: f64, b: f64) -> bool {
    return ((a - b).abs() <= GEOMETRY_EPSILON);
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:569 (sha256:cd7c83468469bfa0ebd1b85bb0a0625319f2c1c477c2a998435fc9f416364576)
fn cross(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    return ((ax * by) - (ay * bx));
}

// Source: upstream/packages/path/src/strokePathGeometry.ts:573 (sha256:9d9aa80ecbf2564fc4f3143abd77dc1717f353000f3f3de8081f821586e609a0)
static EMPTY_DASH: std::sync::LazyLock<Vec<f64>> = std::sync::LazyLock::new(|| vec![]);

// Source: upstream/packages/path/src/strokePathGeometry.ts:574 (sha256:0897730fca8b20524e23a92b760a8a25a60513ac21e0c530df215630439ed9e1)
const GEOMETRY_EPSILON: f64 = 1e-8_f64;
