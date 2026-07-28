// @generated from upstream/packages/materials/src/phongToPbr.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_standard_pbr_material;
use flighthq_color::get_color_luminance;
use flighthq_types::{PhongMaterial, StandardPbrMaterial, StandardPbrMaterialProperties};

// Source: upstream/packages/materials/src/phongToPbr.ts:17 (sha256:4f0ee09a628abe5b3f48687b2bbca15325445116f4b5e22318de1aa8cadd0c92)
pub fn convert_phong_to_standard_pbr_material(
    phong: &PhongMaterial,
    opts: Option<StandardPbrMaterialProperties>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(StandardPbrMaterial {
        base_color: phong.diffuse,
        base_color_map: (phong.diffuse_map).clone(),
        metallic: get_pbr_metallic_from_phong_specular(phong.specular, phong.diffuse),
        normal_map: (phong.normal_map).clone(),
        normal_scale: phong.normal_scale,
        roughness: get_pbr_roughness_from_phong_shininess(phong.shininess),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/phongToPbr.ts:36 (sha256:df6462ac13f10902000aa41bf7bb6d3963e14e4ae5c8aa50116192de3b47d264)
pub fn get_pbr_metallic_from_phong_specular(specular: f64, diffuse: f64) -> f64 {
    return if (get_color_luminance(specular) > 0.5_f64) && (get_color_luminance(diffuse) < 0.04_f64)
    {
        1.0_f64
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/materials/src/phongToPbr.ts:43 (sha256:522973c86a96c5b585de450437f9095e2be72bee298053f386222ab3a0b477e1)
pub fn get_pbr_roughness_from_phong_shininess(shininess: f64) -> f64 {
    return (1.0_f64).min((0.0_f64).max((2.0_f64 / ((0.0_f64).max(shininess) + 2.0_f64)).sqrt()));
}

// Source: upstream/packages/materials/src/phongToPbr.ts:59 (sha256:9d2d27b5f98df85910b1947d7df678b517e71163a596782fedeec122107c656e)
pub fn get_phong_to_pbr_light_exposure() -> f64 {
    return (std::f64::consts::PI).log2();
}
