// @generated from upstream/packages/displayobject/src/htmlView.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    create_display_object_generic, create_display_object_runtime, get_display_object_runtime,
};
use flighthq_node::invalidate_node_local_bounds;
use flighthq_types::{
    HTML_VIEW_KIND as html_view_kind_constant, HtmlView, HtmlViewData, HtmlViewRuntime, Node,
    Rectangle,
};

// Source: upstream/packages/displayobject/src/htmlView.ts:7 (sha256:7dc87ec911bb21de9c8b0e30c8a51123e6625110f4e23005a6075be68296dfb8)
pub fn compute_html_view_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    out.width = source.data.width;
    out.height = source.data.height;
}

// Source: upstream/packages/displayobject/src/htmlView.ts:13 (sha256:456ee90b8bb4067523a77d4b57581c0171a30d3705481e1795fb4592f2b2220a)
pub fn create_html_view(obj: Option<HtmlView>) -> HtmlView {
    return create_display_object_generic(
        html_view_kind_constant,
        Some(((obj).clone().unwrap()).clone()),
        Some(create_html_view_data),
        Some(create_html_view_runtime),
    );
}

// Source: upstream/packages/displayobject/src/htmlView.ts:17 (sha256:1d146ce6ff8398ea85cd40df01f0316124a94f51fa891b4470a251d5688efe84)
pub fn create_html_view_data(data: Option<HtmlViewData>) -> HtmlViewData {
    return HtmlViewData {
        __flight_identity: std::sync::Arc::new(()),
        element: data.as_ref().and_then(|value| (value.element).clone()),
        height: (data.as_ref().map(|value| value.height)).unwrap_or(100.0_f64),
        width: (data.as_ref().map(|value| value.width)).unwrap_or(100.0_f64),
    };
}

// Source: upstream/packages/displayobject/src/htmlView.ts:25 (sha256:6201bbd0c3f0f4b89fe375124dbbeb6242f1c1bda1770f99677370b09c845cde)
pub fn create_html_view_runtime() -> HtmlViewRuntime {
    return create_display_object_runtime(Some(((*DEFAULT_METHODS).clone()).clone()));
}

// Source: upstream/packages/displayobject/src/htmlView.ts:29 (sha256:c6c4563b2f6992502ec83f2ae5bc3cd74dae972b4171b3d254f854d399a066ed)
pub fn get_html_view_runtime(source: &HtmlView) -> HtmlViewRuntime {
    return get_display_object_runtime(source);
}

// Source: upstream/packages/displayobject/src/htmlView.ts:33 (sha256:f2543255c3a7982453ff94464b0646cd9ef4d1b1bbf0094a79ff36bbf1315944)
pub fn set_html_view_size(source: &mut HtmlView, width: f64, height: f64) -> () {
    if ((source.data.width == width) && (source.data.height == height)) {
        return;
    }
    source.data.width = width;
    source.data.height = height;
    invalidate_node_local_bounds(source);
}

// Source: upstream/packages/displayobject/src/htmlView.ts:40 (sha256:598e2a451d040f8337cc8e0834e84ad6db057fe0830b3497d7280393e70b22f3)
static DEFAULT_METHODS: std::sync::LazyLock<HtmlViewRuntime> =
    std::sync::LazyLock::new(|| HtmlViewRuntime {
        __flight_identity: std::sync::Arc::new(()),
        compute_local_bounds_rectangle: compute_html_view_local_bounds_rectangle,
    });
