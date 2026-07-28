// @generated from upstream/packages/node/src/viewport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_node_runtime;
use flighthq_entity::create_entity;
use flighthq_geometry::create_rectangle;
use flighthq_types::{MatrixLike, Rectangle, Viewport, ViewportAlign};

// Source: upstream/packages/node/src/viewport.ts:14 (sha256:dc28e0ed6cffa9ecb19009e5e5d2f713e25b3850dc3a23608caa3e5420632241)
pub fn compute_viewport_align_x(
    scaled_content_width: f64,
    view_width: f64,
    align: ViewportAlign,
) -> f64 {
    if (align.includes)("left") {
        return 0.0_f64;
    }
    if (align.includes)("right") {
        return (view_width - scaled_content_width);
    }
    return ((view_width - scaled_content_width) / 2.0_f64);
}

// Source: upstream/packages/node/src/viewport.ts:20 (sha256:c7e6397ce2f8e9d96e9cc5bfe5c1372a9ab645860ae1fed5a3a12e81c098b6f7)
pub fn compute_viewport_align_y(
    scaled_content_height: f64,
    view_height: f64,
    align: ViewportAlign,
) -> f64 {
    if (align.includes)("top") {
        return 0.0_f64;
    }
    if (align.includes)("bottom") {
        return (view_height - scaled_content_height);
    }
    return ((view_height - scaled_content_height) / 2.0_f64);
}

// Source: upstream/packages/node/src/viewport.ts:26 (sha256:ef29745147c677f1d8a171441aeecdc5ca937995c90c8da5b92c9d22099d4f61)
pub fn compute_viewport_fill_scale(
    content_width: f64,
    content_height: f64,
    view_width: f64,
    view_height: f64,
) -> f64 {
    return (view_width / content_width).max((view_height / content_height));
}

// Source: upstream/packages/node/src/viewport.ts:35 (sha256:1cba447a54b470447641b7be236e3273a5024c703f7b9aa1e4bc431928ef638d)
pub fn compute_viewport_fit_scale(
    content_width: f64,
    content_height: f64,
    view_width: f64,
    view_height: f64,
) -> f64 {
    return (view_width / content_width).min((view_height / content_height));
}

// Source: upstream/packages/node/src/viewport.ts:44 (sha256:e8d3fd82e6db93610f893ae7e98b9d175d933f62dcb48b52939a9f7cf4596101)
pub fn compute_viewport_render_transform<Traits: Clone>(
    out: &mut MatrixLike,
    scene: &Viewport<Traits>,
    view_width: f64,
    view_height: f64,
) -> () {
    let mut content_width = 0.0_f64;
    let mut content_height = 0.0_f64;
    if ((scene.root).clone()).is_some() {
        let runtime = get_node_runtime(&((scene.root).clone()).unwrap());
        if (runtime
            .as_ref()
            .map(|value| (value.compute_local_bounds_rectangle).clone()))
        .is_some()
        {
            (*_TEMP_RECTANGLE.lock().unwrap()).width = 0.0_f64;
            (*_TEMP_RECTANGLE.lock().unwrap()).height = 0.0_f64;
            {
                let __flight_callback =
                    (runtime.as_ref().unwrap().compute_local_bounds_rectangle).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    (*_TEMP_RECTANGLE.lock().unwrap()).clone(),
                    ((scene.root).clone()).unwrap(),
                );
                __flight_result
            };
            content_width = (*_TEMP_RECTANGLE.lock().unwrap()).width;
            content_height = (*_TEMP_RECTANGLE.lock().unwrap()).height;
        }
    }
    if (content_width == 0.0_f64) || (content_height == 0.0_f64) {
        out.a = 1.0_f64;
        out.b = 0.0_f64;
        out.c = 0.0_f64;
        out.d = 1.0_f64;
        out.tx = 0.0_f64;
        out.ty = 0.0_f64;
        return;
    }
    let mut sx: f64;
    let mut sy: f64;
    if ((scene.scale_mode).clone() == "noscale") {
        sx = 1.0_f64;
        sy = 1.0_f64;
    } else {
        if ((scene.scale_mode).clone() == "exactfit") {
            sx = (view_width / content_width);
            sy = (view_height / content_height);
        } else {
            if ((scene.scale_mode).clone() == "showall") {
                sx = {
                    sy = compute_viewport_fit_scale(
                        content_width,
                        content_height,
                        view_width,
                        view_height,
                    );
                    sy
                };
            } else {
                sx = {
                    sy = compute_viewport_fill_scale(
                        content_width,
                        content_height,
                        view_width,
                        view_height,
                    );
                    sy
                };
            }
        }
    }
    out.a = sx;
    out.b = 0.0_f64;
    out.c = 0.0_f64;
    out.d = sy;
    out.tx = compute_viewport_align_x((content_width * sx), view_width, (scene.align).clone());
    out.ty = compute_viewport_align_y((content_height * sy), view_height, (scene.align).clone());
}

// Source: upstream/packages/node/src/viewport.ts:98 (sha256:5e7ea835eb8bebbfe5e1f01d1bc0ac32a95387ca805bc37567a204f9376b3cf7)
pub fn create_viewport<Traits: Clone>(obj: Option<Viewport<Traits>>) -> Viewport<Traits> {
    return create_entity(Some(Viewport::<Traits> {
        __flight_identity: std::sync::Arc::new(()),
        align: (obj.as_ref().map(|value| (value.align).clone())).unwrap_or("topleft".to_owned()),
        root: obj.as_ref().and_then(|value| (value.root).clone()),
        scale_mode: (obj.as_ref().map(|value| (value.scale_mode).clone()))
            .unwrap_or("noscale".to_owned()),
    }));
}

// Source: upstream/packages/node/src/viewport.ts:108 (sha256:5384a4981cbd7d4aac682173a45f0c39c9eb3eca35a7d4da729926e13d6acf77)
static _TEMP_RECTANGLE: std::sync::LazyLock<std::sync::Mutex<Rectangle>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_rectangle(None, None, None, None)));
