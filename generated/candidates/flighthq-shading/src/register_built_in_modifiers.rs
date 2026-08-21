// @generated from upstream/packages/shading/src/registerBuiltInModifiers.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_modifier;
use flighthq_types::{
    ANIMATED_NORMAL_MODIFIER_KIND as animated_normal_modifier_kind_constant,
    DISSOLVE_MODIFIER_KIND as dissolve_modifier_kind_constant,
    EMISSIVE_MODIFIER_FACING as emissive_modifier_facing_constant,
    EMISSIVE_MODIFIER_KIND as emissive_modifier_kind_constant,
    ENV_REFLECT_MODIFIER_KIND as env_reflect_modifier_kind_constant, EmissiveModifier,
    FOG_MODIFIER_KIND as fog_modifier_kind_constant,
    FOG_MODIFIER_MODE as fog_modifier_mode_constant, FogModifier,
    MODIFIER_SLOT as modifier_slot_constant, Modifier, ModifierDefinition, ModifierRegistry,
    RIM_MODIFIER_KIND as rim_modifier_kind_constant,
    TOON_MODIFIER_KIND as toon_modifier_kind_constant,
    VERTEX_DISPLACE_MODIFIER_KIND as vertex_displace_modifier_kind_constant,
    VERTEX_DISPLACE_MODIFIER_SOURCE as vertex_displace_modifier_source_constant,
    VertexDisplaceModifier,
};

// Source: upstream/packages/shading/src/registerBuiltInModifiers.ts:31 (sha256:826be1d5547ebffc292c0e882ddd02ad05bfdf42fba9c166221e8bdf83db3a8e)
pub static ANIMATED_NORMAL_MODIFIER_DEFINITION: std::sync::LazyLock<ModifierDefinition> =
    std::sync::LazyLock::new(|| ModifierDefinition {
        __flight_identity: std::sync::Arc::new(()),
        kind: (animated_normal_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.normal).clone(),
        get_define_signature: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier| -> String {
                let animated = modifier;
                if ((animated.map).clone()).is_none() {
                    return "0".to_owned();
                }
                return if ((animated.secondary_map).clone()).is_some() {
                    "2".to_owned()
                } else {
                    "1".to_owned()
                };
            },
        )
            as Box<dyn FnMut(Modifier) -> String + Send + 'static>))),
    });

// Source: upstream/packages/shading/src/registerBuiltInModifiers.ts:44 (sha256:075f269a39969c5bd65fe58c3f4f8ebc63e10b7d771c1d28c74ce0700c51b4a4)
pub static DISSOLVE_MODIFIER_DEFINITION: std::sync::LazyLock<ModifierDefinition> =
    std::sync::LazyLock::new(|| ModifierDefinition {
        __flight_identity: std::sync::Arc::new(()),
        kind: (dissolve_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.effect).clone(),
        get_define_signature: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier| -> String {
                return if (None::<crate::FlightValue>).is_some() {
                    "m".to_owned()
                } else {
                    "".to_owned()
                };
            },
        )
            as Box<dyn FnMut(Modifier) -> String + Send + 'static>))),
    });

// Source: upstream/packages/shading/src/registerBuiltInModifiers.ts:56 (sha256:5e4c5259d37319e4d48bea5f4e6120f12ec84634e7787fddbae427bf3b4d777c)
pub static EMISSIVE_MODIFIER_DEFINITION: std::sync::LazyLock<ModifierDefinition> =
    std::sync::LazyLock::new(|| ModifierDefinition {
        __flight_identity: std::sync::Arc::new(()),
        kind: (emissive_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.emissive).clone(),
        get_define_signature: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier| -> String {
                let emissive = {
                    let __flight_source = &((modifier).clone());
                    EmissiveModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        color: __flight_source.color,
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        ..Default::default()
                    }
                };
                let mut signature = "".to_owned();
                if ((emissive.mask).clone()).is_some() {
                    signature.push_str(&("m".to_owned()));
                }
                if (((emissive.facing).clone()).is_some())
                    && ((emissive.facing).clone() != emissive_modifier_facing_constant.ignore)
                {
                    signature.push_str(&("g".to_owned()));
                }
                return signature;
            },
        )
            as Box<dyn FnMut(Modifier) -> String + Send + 'static>))),
    });

// Source: upstream/packages/shading/src/registerBuiltInModifiers.ts:71 (sha256:c9ab5f70181f2a7d2deaf66db382d5bfddb305f429b25dd1123a1f48f3aaf241)
pub static ENV_REFLECT_MODIFIER_DEFINITION: std::sync::LazyLock<ModifierDefinition> =
    std::sync::LazyLock::new(|| ModifierDefinition {
        __flight_identity: std::sync::Arc::new(()),
        kind: (env_reflect_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.effect).clone(),
        get_define_signature: None,
    });

// Source: upstream/packages/shading/src/registerBuiltInModifiers.ts:79 (sha256:4e2fa9f5de7dcfe8800a1d06cbae75a5d5be181f034c31e0b958fc7290a4883a)
pub static FOG_MODIFIER_DEFINITION: std::sync::LazyLock<ModifierDefinition> =
    std::sync::LazyLock::new(|| ModifierDefinition {
        __flight_identity: std::sync::Arc::new(()),
        kind: (fog_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.effect).clone(),
        get_define_signature: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier| -> String {
                let fog = {
                    let __flight_source = &((modifier).clone());
                    FogModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        color: __flight_source.color,
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        ..Default::default()
                    }
                };
                if ((fog.mode).clone() == fog_modifier_mode_constant.exponential) {
                    return "e".to_owned();
                }
                if ((fog.mode).clone() == fog_modifier_mode_constant.exponential2) {
                    return "x".to_owned();
                }
                return "l".to_owned();
            },
        )
            as Box<dyn FnMut(Modifier) -> String + Send + 'static>))),
    });

// Source: upstream/packages/shading/src/registerBuiltInModifiers.ts:94 (sha256:373a7297e64ca0f97d1752ce5fddc16cadbecc7f9bfe069bcf343af8b2cb3e08)
pub fn register_built_in_modifiers(registry: &mut ModifierRegistry) -> () {
    register_modifier(registry, &ANIMATED_NORMAL_MODIFIER_DEFINITION);
    register_modifier(registry, &DISSOLVE_MODIFIER_DEFINITION);
    register_modifier(registry, &EMISSIVE_MODIFIER_DEFINITION);
    register_modifier(registry, &ENV_REFLECT_MODIFIER_DEFINITION);
    register_modifier(registry, &FOG_MODIFIER_DEFINITION);
    register_modifier(registry, &RIM_MODIFIER_DEFINITION);
    register_modifier(registry, &TOON_MODIFIER_DEFINITION);
    register_modifier(registry, &VERTEX_DISPLACE_MODIFIER_DEFINITION);
}

// Source: upstream/packages/shading/src/registerBuiltInModifiers.ts:107 (sha256:e797898d305d5f43ffbd31f6b65d3d5fa9c9c90fd4c089d1b26f6fbcfdfc1336)
pub static RIM_MODIFIER_DEFINITION: std::sync::LazyLock<ModifierDefinition> =
    std::sync::LazyLock::new(|| ModifierDefinition {
        __flight_identity: std::sync::Arc::new(()),
        kind: (rim_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.effect).clone(),
        get_define_signature: None,
    });

// Source: upstream/packages/shading/src/registerBuiltInModifiers.ts:114 (sha256:2d5a3686f420076f2438cf1909da31553df32fca43ee846a476449121d104b08)
pub static TOON_MODIFIER_DEFINITION: std::sync::LazyLock<ModifierDefinition> =
    std::sync::LazyLock::new(|| ModifierDefinition {
        __flight_identity: std::sync::Arc::new(()),
        kind: (toon_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.effect).clone(),
        get_define_signature: None,
    });

// Source: upstream/packages/shading/src/registerBuiltInModifiers.ts:123 (sha256:0271274ae21cadffd5834b412074dcb6139e8c543fc7e14541b119d101b9c7a9)
pub static VERTEX_DISPLACE_MODIFIER_DEFINITION: std::sync::LazyLock<ModifierDefinition> =
    std::sync::LazyLock::new(|| ModifierDefinition {
        __flight_identity: std::sync::Arc::new(()),
        kind: (vertex_displace_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.vertex).clone(),
        get_define_signature: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier| -> String {
                let displace = {
                    let __flight_source = &((modifier).clone());
                    VertexDisplaceModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        color: __flight_source.color,
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        map: None,
                        ..Default::default()
                    }
                };
                let mut signature = if ((displace.source).clone()
                    == vertex_displace_modifier_source_constant.height_map)
                {
                    "h".to_owned()
                } else {
                    "s".to_owned()
                };
                if ((displace.axis).clone()).is_some() {
                    signature.push_str(&("a".to_owned()));
                }
                return signature;
            },
        )
            as Box<dyn FnMut(Modifier) -> String + Send + 'static>))),
    });
