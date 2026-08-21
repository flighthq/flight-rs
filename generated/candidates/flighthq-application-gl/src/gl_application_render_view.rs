// @generated from upstream/packages/application-gl/src/glApplicationRenderView.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_application::{create_application_render_view, detach_application_render_view};
use flighthq_node::create_viewport;
use flighthq_render_gl::{
    create_gl_render_state, create_gl_render_target, destroy_gl_render_state,
    destroy_gl_render_target, invalidate_gl_render_state_cache, resize_gl_render_target,
};
use flighthq_types::{
    ApplicationWindow, GlApplicationRenderView, GlApplicationRenderViewOptions, GlRenderOptions,
    GlRenderState, GlRenderTarget, RenderTargetDescriptor, ViewportLike,
};

// Source: upstream/packages/application-gl/src/glApplicationRenderView.ts:23 (sha256:e09b38929b5cecc79e69d61be07a330497533585e04aa264aedb50371d99834f)
#[derive(Clone, Default)]
struct CreateGlApplicationRenderViewRecord12 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateGlApplicationRenderViewRecord12 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_gl_application_render_view(
    window: &ApplicationWindow,
    canvas: crate::OpaqueHostValue,
    options: Option<GlApplicationRenderViewOptions>,
) -> GlApplicationRenderView {
    let options = options.unwrap_or(GlApplicationRenderViewOptions {
        __flight_identity: std::sync::Arc::new(()),
        render: None,
        target: None,
    });
    let width = (0.0_f64).max((window.width * window.device_pixel_ratio).round());
    let height = (0.0_f64).max((window.height * window.device_pixel_ratio).round());
    synchronize_gl_canvas_backing_store((canvas).clone(), width, height);
    let render_state = create_gl_render_state(
        (canvas).clone(),
        Some({
            let __flight_spread_0 = ((options.render).clone()).unwrap_or_default();
            GlRenderOptions {
                __flight_identity: std::sync::Arc::new(()),
                allow_smoothing: __flight_spread_0.allow_smoothing,
                antialias: __flight_spread_0.antialias,
                background_color: __flight_spread_0.background_color,
                context_attributes: (__flight_spread_0.context_attributes).clone(),
                image_smoothing_enabled: __flight_spread_0.image_smoothing_enabled,
                pixel_ratio: Some(window.device_pixel_ratio),
                power_preference: (__flight_spread_0.power_preference).clone(),
                round_pixels: __flight_spread_0.round_pixels,
                scene_graph_sync_policy: (__flight_spread_0.scene_graph_sync_policy).clone(),
            }
        }),
    );
    let render_target = create_gl_render_target(
        &render_state,
        &{
            let __flight_spread_0 = ((options.target).clone()).unwrap_or_default();
            RenderTargetDescriptor {
                __flight_identity: std::sync::Arc::new(()),
                width: width,
                height: height,
                format: (__flight_spread_0.format).clone(),
                color_attachments: __flight_spread_0.color_attachments,
                color_formats: (__flight_spread_0.color_formats).clone(),
                sample_count: __flight_spread_0.sample_count,
                depth: (__flight_spread_0.depth).clone(),
                color_space: (__flight_spread_0.color_space).clone(),
                clear_colors: (__flight_spread_0.clear_colors).clone(),
                clear_depth: __flight_spread_0.clear_depth,
            }
        },
        None,
    );
    let viewport = create_viewport(Some(ViewportLike {
        __flight_identity: std::sync::Arc::new(()),
        device_pixel_ratio: Some(window.device_pixel_ratio),
        height: Some(height),
        width: Some(width),
        x: None,
        y: None,
    }));
    return {
        let __flight_source = &(create_application_render_view(
            (window).clone(),
            (render_state).clone(),
            ((render_target).clone().unwrap()).clone(),
            (viewport).clone(),
            std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                move |mut __flight_argument_0: GlRenderState,
                      mut __flight_argument_1: GlRenderTarget,
                      __flight_argument_2: f64,
                      __flight_argument_3: f64|
                      -> () {
                    resize_gl_application_render_view(
                        &mut __flight_argument_0,
                        &mut __flight_argument_1,
                        __flight_argument_2,
                        __flight_argument_3,
                    )
                },
            )
                as Box<
                    dyn FnMut(GlRenderState, GlRenderTarget, f64, f64) -> () + Send + 'static,
                >)),
        ));
        GlApplicationRenderView {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            render_state: (__flight_source.render_state).clone(),
            render_target: (__flight_source.render_target).clone(),
            viewport: (__flight_source.viewport).clone(),
            window: (__flight_source.window).clone(),
        }
    };
}

// Source: upstream/packages/application-gl/src/glApplicationRenderView.ts:51 (sha256:b6260e30c8ee9793dadea96ea612fb08ac6c69b87764d4250d91cce4ad757e90)
pub fn destroy_gl_application_render_view(view: &mut GlApplicationRenderView) -> () {
    detach_application_render_view(view);
    destroy_gl_render_target(&view.render_state, &view.render_target);
    destroy_gl_render_state(&view.render_state);
}

// Source: upstream/packages/application-gl/src/glApplicationRenderView.ts:57 (sha256:c5ca67069b042525fc9345f6326aebfc960bfe38facce970fba5aa40a1c33e5b)
fn resize_gl_application_render_view(
    render_state: &mut GlRenderState,
    render_target: &mut GlRenderTarget,
    width: f64,
    height: f64,
) -> () {
    if synchronize_gl_canvas_backing_store((render_state.canvas).clone(), width, height) {
        invalidate_gl_render_state_cache(render_state);
    }
    let storage_width = (1.0_f64).max((width).ceil());
    let storage_height = (1.0_f64).max((height).ceil());
    if (render_target.width != storage_width) || (render_target.height != storage_height) {
        resize_gl_render_target(render_state, render_target, width, height);
    }
}

// Source: upstream/packages/application-gl/src/glApplicationRenderView.ts:73 (sha256:82294a0c4110535ae3cb9d1a2fe2b30886bd94468fd6e1da29074a93556c8c1e)
fn synchronize_gl_canvas_backing_store(
    canvas: crate::OpaqueHostValue,
    width: f64,
    height: f64,
) -> bool {
    let mut changed = false;
    if (crate::host_value::<f64>("host.width") != width) {
        crate::host_set("host.width", width);
        changed = true;
    }
    if (crate::host_value::<f64>("host.height") != height) {
        crate::host_set("host.height", height);
        changed = true;
    }
    return changed;
}
