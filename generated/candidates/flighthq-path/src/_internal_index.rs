// @generated from upstream/packages/path/src/index.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use crate::{
    PathMorph, PathMorphCreationExplanation, PathMorphCreationReason,
    StrokePathTessellationExplanation, StrokePathTessellationReason, StrokeStyle,
    acquire_path_mesh, acquire_path_mesh_typed, append_path_arc, append_path_arc_to,
    append_path_circle, append_path_close, append_path_cubic_curve_to, append_path_curve_to,
    append_path_ellipse, append_path_line_to, append_path_move_to, append_path_polygon,
    append_path_polyline, append_path_rectangle, append_path_round_rectangle, clean_path,
    clone_path, contains_path_point, copy_path, create_path, create_path_morph, dash_path,
    decimate_path, explain_path_morph_creation, explain_stroke_path_tessellation, fit_path_curves,
    flatten_path, for_each_path_segment, get_cubic_bezier_point, get_cubic_bezier_tangent,
    get_path_bounds, get_path_contour_lengths, get_path_contour_orientation, get_path_last_point,
    get_path_length, get_path_nearest_point, get_path_point_at_distance,
    get_path_position_at_distance, get_path_segment_point_at_parameter,
    get_path_segment_tangent_at_parameter, get_path_signed_area, get_path_tangent_at_distance,
    get_quadratic_bezier_point, get_quadratic_bezier_tangent, release_path_mesh,
    release_path_mesh_typed, reverse_path, sample_path_morph, stroke_path, tessellate_path,
    tessellate_path_typed, tessellate_stroke_path, transform_path, translate_path,
};
