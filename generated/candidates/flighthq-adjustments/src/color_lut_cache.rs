// @generated from upstream/packages/adjustments/src/colorLutCache.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    COLOR_LUT_DEFAULT_SIZE as color_lut_default_size_constant, bake_color_lut,
    get_adjustment_color_transform,
};
use flighthq_types::{ColorLut, ColorLutCache, ColorTransformFunction};

// Source: upstream/packages/adjustments/src/colorLutCache.ts:20 (sha256:43b7051bda2b12fe9ccb47c39eaebd4ec760dff96ad22db73055c349ecefe0c7)
#[derive(Clone)]
struct BakeColorLutForRunRecord1 {
    __flight_identity: std::sync::Arc<()>,
    kind: String,
}
impl PartialEq for BakeColorLutForRunRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn bake_color_lut_for_run(
    cache: &mut ColorLutCache,
    run: &Vec<BakeColorLutForRunRecord1>,
    size: Option<f64>,
) -> ColorLut {
    let size = size.unwrap_or(color_lut_default_size_constant);
    let signature = color_lut_run_signature(run, size);
    if (((cache.signature).clone()) == Some((signature).clone()))
        && (((cache.lut).clone()).is_some())
    {
        return ((cache.lut).clone()).unwrap();
    }
    let mut transforms: Vec<ColorTransformFunction> = vec![];
    for operation in (run).iter().cloned() {
        let transform = get_adjustment_color_transform(&operation);
        if (transform).is_some() {
            transforms.push(((transform.as_ref().unwrap()).clone()).clone());
        }
    }
    let lut = bake_color_lut(&transforms, Some(size));
    cache.signature = Some((signature).clone());
    cache.lut = Some((lut).clone());
    return lut;
}

// Source: upstream/packages/adjustments/src/colorLutCache.ts:40 (sha256:57f403fdf8d5bd7afff86bdc44e1c23863c0e2d90257ec4d19180469fd165e93)
pub fn create_color_lut_cache() -> ColorLutCache {
    return ColorLutCache {
        __flight_identity: std::sync::Arc::new(()),
        signature: None,
        lut: None,
    };
}

// Source: upstream/packages/adjustments/src/colorLutCache.ts:48 (sha256:15cbc958713f0e9b3b31ba476d3c5cb73ef8c6673ab1b45d8837bed603377581)
#[derive(Clone)]
struct ColorLutRunSignatureRecord1 {
    __flight_identity: std::sync::Arc<()>,
    kind: String,
}
impl PartialEq for ColorLutRunSignatureRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn color_lut_run_signature(run: &Vec<ColorLutRunSignatureRecord1>, size: f64) -> String {
    return format!("{}\n{}", size, (json.stringify)(run));
}
