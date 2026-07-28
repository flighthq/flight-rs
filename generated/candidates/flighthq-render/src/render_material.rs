// @generated from upstream/packages/render/src/renderMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Material, MaterialData, RenderProxy, RenderState};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

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
