// @generated from upstream/packages/shading/src/createVertexDisplaceModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    MODIFIER_SLOT as modifier_slot_constant, Texture,
    VERTEX_DISPLACE_MODIFIER_KIND as vertex_displace_modifier_kind_constant, Vector3Like,
    VertexDisplaceModifier, VertexDisplaceModifierSource,
};

// Source: upstream/packages/shading/src/createVertexDisplaceModifier.ts:9 (sha256:4831daeafb37b213acd119f1b235c36ab8b0c9539d23d2958379792fa2a48f98)
#[derive(Clone, Default)]
pub struct VertexDisplaceModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub source: VertexDisplaceModifierSource,
    pub amplitude: f64,
    pub axis: Option<Vector3Like>,
    pub map: Option<Texture>,
    pub frequency: Option<f64>,
    pub speed: Option<f64>,
    pub direction: Option<Vector3Like>,
}
impl PartialEq for VertexDisplaceModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/shading/src/createVertexDisplaceModifier.ts:24 (sha256:4e369c05b17a554e3cc578939f977605462fb938dad8157bf9f74abb882c0a56)
pub fn create_vertex_displace_modifier(
    options: &VertexDisplaceModifierOptions,
) -> VertexDisplaceModifier {
    let mut modifier: VertexDisplaceModifier = VertexDisplaceModifier {
        __flight_identity: std::sync::Arc::new(()),
        kind: (vertex_displace_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.vertex).clone(),
        source: (options.source).clone(),
        amplitude: options.amplitude,
        frequency: Some((options.frequency).unwrap_or(1.0_f64)),
        speed: Some((options.speed).unwrap_or(1.0_f64)),
        direction: Some(
            ((options.direction).clone()).unwrap_or(((*DEFAULT_DIRECTION).clone()).clone()),
        ),
        axis: None,
        map: None,
        ..Default::default()
    };
    if ((options.axis).clone()).is_some() {
        modifier.axis = (options.axis).clone();
    }
    if ((options.map).clone()).is_some() {
        modifier.map = (options.map).clone();
    }
    return modifier;
}

// Source: upstream/packages/shading/src/createVertexDisplaceModifier.ts:39 (sha256:571c782554379f84a68b469d69bf8ed9b27d52f4c7e8b71dbce06c20bc3d424a)
static DEFAULT_DIRECTION: std::sync::LazyLock<Vector3Like> =
    std::sync::LazyLock::new(|| Vector3Like {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        x: 1.0_f64,
        y: 0.0_f64,
        z: 0.0_f64,
    });
