// @generated from upstream/packages/displayobject/src/video.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    create_display_object_generic, create_display_object_runtime, get_display_object_runtime,
};
use flighthq_node::{invalidate_node_local_bounds, invalidate_node_local_content};
use flighthq_types::{
    Node, Rectangle, VIDEO_KIND as video_kind_constant, Video, VideoData, VideoRuntime,
};

// Source: upstream/packages/displayobject/src/video.ts:7 (sha256:abc6de320572efec2ad862016241c33ac16a429cbac13617bb10bd7baf45c731)
pub fn compute_video_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    let element = ((source.data).clone())
        .unwrap()
        .source
        .as_ref()
        .and_then(|value| (value.element).clone());
    if ((element).is_some() && (element).is_some()) {
        out.width = (element.as_ref().unwrap().video_width).clone();
        out.height = (element.as_ref().unwrap().video_height).clone();
    }
}

// Source: upstream/packages/displayobject/src/video.ts:15 (sha256:5a8a3a7fb3283548e76ec7813886ee7317f280d4aafeb0dbb507b0e2fc6f3e25)
pub fn create_video(obj: Option<Video>) -> Video {
    return create_display_object_generic(
        video_kind_constant,
        Some(((obj).clone().unwrap()).clone()),
        Some(create_video_data),
        Some(create_video_runtime),
    );
}

// Source: upstream/packages/displayobject/src/video.ts:19 (sha256:d2b898feed674826c39526d7e5bda6948c24d3a358d4216a75cbade6d8b3d881)
pub fn create_video_data(data: Option<VideoData>) -> VideoData {
    return VideoData {
        __flight_identity: std::sync::Arc::new(()),
        smoothing: (data.as_ref().map(|value| value.smoothing)).unwrap_or(true),
        source: data.as_ref().and_then(|value| (value.source).clone()),
    };
}

// Source: upstream/packages/displayobject/src/video.ts:26 (sha256:2a93b0abf329687d7a60f139a5986f9b6bf3e6d34a099506601fd6286efa87cf)
pub fn create_video_runtime() -> VideoRuntime {
    return create_display_object_runtime(Some(((*DEFAULT_METHODS).clone()).clone()));
}

// Source: upstream/packages/displayobject/src/video.ts:30 (sha256:49df6675809e4b31dadf8b3be7dfd65827c1b32d573900dc5fba78a1c4a9a05f)
pub fn get_video_runtime(source: &Video) -> VideoRuntime {
    return get_display_object_runtime(source);
}

// Source: upstream/packages/displayobject/src/video.ts:34 (sha256:9bfb7dcbb230f067cb4bffbb1ec4a95fc7d9e29b9f0f3334b5b1f736de59ec61)
pub fn set_video_smoothing(source: &mut Video, value: bool) -> () {
    source.data.smoothing = value;
    invalidate_node_local_content(source);
}

// Source: upstream/packages/displayobject/src/video.ts:41 (sha256:ee96c2ea9aa16c5416b4ba23df4b87318ead677e19f4e7d3aa5082adcd655a5a)
pub fn set_video_source(source: &mut Video, value: crate::OpaqueHostValue) -> () {
    source.data.source = Some(value);
    invalidate_node_local_content(source);
    invalidate_node_local_bounds(source);
}

// Source: upstream/packages/displayobject/src/video.ts:48 (sha256:561eba068601365ac7aafcb99aecb19c2cb1ee8338ee3fd8e0d2ce185777acfe)
static DEFAULT_METHODS: std::sync::LazyLock<VideoRuntime> =
    std::sync::LazyLock::new(|| VideoRuntime {
        __flight_identity: std::sync::Arc::new(()),
        compute_local_bounds_rectangle: compute_video_local_bounds_rectangle,
    });
