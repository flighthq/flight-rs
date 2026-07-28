// @generated from upstream/packages/render/src/renderMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{RenderProxy, RenderState};

// Source: upstream/packages/render/src/renderMaterial.ts:6 (sha256:c53b2ce5138a009239dccfe99751c94aa126b62efa577185d0bf72c80f4f382c)
pub fn update_render_proxy_material(
    state: &RenderState,
    data: &mut RenderProxy,
    _parent_data: Option<RenderProxy>,
) -> () {
    let source = (data.source).clone();
    data.material = (source.material).clone();
    data.material_data = (source.material_data).clone();
}
