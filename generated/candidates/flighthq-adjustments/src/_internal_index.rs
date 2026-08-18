// @generated from upstream/packages/adjustments/src/index.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use crate::{
    COLOR_ADJUSTMENT_AFFINE, COLOR_ADJUSTMENT_CHANNEL_MIXING, COLOR_ADJUSTMENT_NONE,
    apply_color_matrix_to_color, bake_color_lut, bake_color_lut_for_run, concat_color_matrix,
    create_brightness_color_matrix, create_brightness_contrast_adjustment,
    create_channel_mixer_adjustment, create_color_blind_simulation_adjustment,
    create_color_grade_adjustment, create_color_lut_cache, create_color_matrix_adjustment,
    create_color_matrix_from_tint, create_color_scale_bias_adjustment,
    create_contrast_color_matrix, create_exposure_adjustment, create_grayscale_adjustment,
    create_hue_rotate_color_matrix, create_hue_saturation_adjustment, create_identity_color_matrix,
    create_invert_adjustment, create_lift_gamma_gain_adjustment,
    create_lookup_table_grade_adjustment, create_saturation_color_matrix, create_sepia_adjustment,
    create_tint_adjustment, fuse_color_matrices, get_adjustment_color_matrix,
    get_adjustment_color_transform, is_affine_color_matrix, is_color_lut_adjustment,
    is_color_matrix_adjustment, multiply_color_matrix, resolve_color_adjustments_color_matrix,
    resolve_color_adjustments_color_scale_bias, sample_color_lut,
};
