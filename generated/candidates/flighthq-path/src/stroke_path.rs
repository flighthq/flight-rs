// @generated from upstream/packages/path/src/strokePath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    append_path_close, append_path_line_to, append_path_move_to, build_stroke_path_geometry,
    create_path,
};
use flighthq_types::Path;
pub use flighthq_types::StrokeStyle;

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub closed: bool,
    pub end_cap: Vec<f64>,
    pub left: Vec<f64>,
    pub right: Vec<f64>,
    pub start_cap: Vec<f64>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/strokePath.ts:12 (sha256:fff1ff67d36c0c2d1c0432b66309932f424c734e4a4fc7a2c8ec19ede15b68a6)
pub fn stroke_path(path: &Path, style: &StrokeStyle, tolerance: Option<f64>) -> Path {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let mut result = create_path(Some(("nonZero".to_owned()).clone()));
    let geometry = build_stroke_path_geometry(path, style, tolerance);
    {
        let mut i = 0.0_f64;
        while (i < (geometry.pieces.len() as f64)) {
            append_piece_outline(&mut result, &{
                let __flight_source = &(geometry.pieces[i as usize]);
                SharedStructuralRecord1 {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    closed: __flight_source.closed,
                    end_cap: (__flight_source.end_cap).clone(),
                    left: (__flight_source.left).clone(),
                    right: (__flight_source.right).clone(),
                    start_cap: (__flight_source.start_cap).clone(),
                }
            });
            {
                i += 1.0;
                i
            };
        }
    }
    return result;
}

// Source: upstream/packages/path/src/strokePath.ts:19 (sha256:507bb2df01a2a8992d8c3a441663d7201d43ca08a6d94a18544f77e749c51520)
fn append_piece_outline(path: &mut Path, piece: &SharedStructuralRecord1) -> () {
    if ((piece.left.len() as f64) < 4.0_f64) || ((piece.right.len() as f64) < 4.0_f64) {
        return;
    }
    append_contour(
        path,
        &piece.left,
        false,
        &if piece.closed {
            ((*EMPTY_POINTS).clone()).clone()
        } else {
            (piece.end_cap).clone()
        },
        &if piece.closed {
            ((*EMPTY_POINTS).clone()).clone()
        } else {
            (piece.start_cap).clone()
        },
        &piece.right,
    );
    if piece.closed {
        append_contour(
            path,
            &piece.right,
            true,
            &EMPTY_POINTS,
            &EMPTY_POINTS,
            &EMPTY_POINTS,
        );
    }
}

// Source: upstream/packages/path/src/strokePath.ts:41 (sha256:d54decc2268a48abcc150af5065b8e56886cc40d66994ace366101c805bcfbdd)
fn append_contour(
    path: &mut Path,
    primary: &Vec<f64>,
    reverse_primary: bool,
    after_primary: &Vec<f64>,
    after_secondary: &Vec<f64>,
    secondary: &Vec<f64>,
) -> () {
    if reverse_primary {
        append_path_move_to(
            path,
            primary[((primary.len() as f64) - 2.0_f64) as usize].clone(),
            primary[((primary.len() as f64) - 1.0_f64) as usize].clone(),
        );
        {
            let mut i = ((primary.len() as f64) - 4.0_f64);
            while (i >= 0.0_f64) {
                append_path_line_to(
                    path,
                    primary[i as usize].clone(),
                    primary[(i + 1.0_f64) as usize].clone(),
                );
                {
                    i -= 2.0_f64;
                    i.clone()
                };
            }
        }
    } else {
        append_path_move_to(
            path,
            primary[0.0_f64 as usize].clone(),
            primary[1.0_f64 as usize].clone(),
        );
        append_points(path, primary, 2.0_f64, 2.0_f64);
    }
    append_points(path, after_primary, 0.0_f64, 2.0_f64);
    append_points(
        path,
        secondary,
        ((secondary.len() as f64) - 2.0_f64),
        (-2.0_f64),
    );
    append_points(path, after_secondary, 0.0_f64, 2.0_f64);
    append_path_close(path);
}

// Source: upstream/packages/path/src/strokePath.ts:62 (sha256:c3b222690da327ed3380e51266b915de2941957433802c708fcd5935952afdcb)
fn append_points(path: &mut Path, points: &Vec<f64>, start: f64, step: f64) -> () {
    {
        let mut i = start;
        while (i >= 0.0_f64) && (i < (points.len() as f64)) {
            append_path_line_to(
                path,
                points[i as usize].clone(),
                points[(i + 1.0_f64) as usize].clone(),
            );
            {
                i += step;
                i.clone()
            };
        }
    }
}

// Source: upstream/packages/path/src/strokePath.ts:66 (sha256:b6e7f50bd002a12f5b3290ce77c435e7e7536fdcc845363e50a51bf54a0463b8)
static EMPTY_POINTS: std::sync::LazyLock<Vec<f64>> = std::sync::LazyLock::new(|| vec![]);
