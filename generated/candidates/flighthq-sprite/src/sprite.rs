// @generated from upstream/packages/sprite/src/sprite.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_displayobject::{
    create_display_object_generic, create_display_object_runtime, get_display_object_runtime,
};
use flighthq_signals::create_signal;
use flighthq_types::{
    BoundsNodeAny, Node, Rectangle, SPRITE_KIND as sprite_kind_constant, Sprite, SpriteData,
    SpriteRuntime, SpriteSignals, TextureAtlasRegion, Vector2,
};

// Source: upstream/packages/sprite/src/sprite.ts:25 (sha256:3f58ec086647d398ab85e5d57c2e418c436bba548164f45e2ca3922fcc227bcf)
pub fn clone_sprite(source: &Sprite) -> Sprite {
    return create_sprite(Some(Sprite {
        __flight_identity: std::sync::Arc::new(()),
        data: SpriteData {
            __flight_identity: std::sync::Arc::new(()),
            atlas: (source.data.atlas).clone(),
            id: source.data.id,
            rect: (source.data.rect).clone(),
        },
    }));
}

// Source: upstream/packages/sprite/src/sprite.ts:41 (sha256:f2bdfe616a8a0f3504831c6f0756dee428cee07e14b3a4c23747548424d7066a)
pub fn compute_sprite_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    if ((source.data.rect).clone()).is_some() {
        out.width = source.data.rect.as_ref().unwrap().width;
        out.height = source.data.rect.as_ref().unwrap().height;
        return;
    }
    if ((source.data.atlas).clone()).is_some() {
        let region = ((source.data.atlas.as_ref().unwrap().regions).clone())
            .iter()
            .find(|value| (|r: TextureAtlasRegion| -> bool { (r.id == data.id) })((*value).clone()))
            .cloned();
        if (region).is_some() {
            let pivot_x = (region.as_ref().unwrap().pivot_x).unwrap_or(0.0_f64);
            let pivot_y = (region.as_ref().unwrap().pivot_y).unwrap_or(0.0_f64);
            out.x = if (pivot_x == 0.0_f64) {
                0.0_f64
            } else {
                (-pivot_x)
            };
            out.y = if (pivot_y == 0.0_f64) {
                0.0_f64
            } else {
                (-pivot_y)
            };
            out.width = region.as_ref().unwrap().width;
            out.height = region.as_ref().unwrap().height;
        }
    }
}

// Source: upstream/packages/sprite/src/sprite.ts:61 (sha256:c1d157fd8e7c2568e241684ed7019f556f1557596ca4577b7647c3ab1225c182)
pub fn create_sprite(obj: Option<Sprite>) -> Sprite {
    return create_display_object_generic(
        (sprite_kind_constant).to_owned(),
        Some(((obj).clone().unwrap()).clone()),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<D>| -> D {
                create_sprite_data(Some(((__flight_argument_0).clone().unwrap()).clone()))
            },
        )
            as Box<dyn FnMut(Option<D>) -> D + Send + 'static>))),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_sprite_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
}

// Source: upstream/packages/sprite/src/sprite.ts:65 (sha256:5e38bedb946993c960f1d2585085eba848c680dd212b7c90e0c9a5c381841f8a)
pub fn create_sprite_data(data: Option<SpriteData>) -> SpriteData {
    return SpriteData {
        __flight_identity: std::sync::Arc::new(()),
        atlas: data.as_ref().and_then(|value| (value.atlas).clone()),
        id: (data.as_ref().map(|value| value.id)).unwrap_or(0.0_f64),
        rect: data.as_ref().and_then(|value| (value.rect).clone()),
    };
}

// Source: upstream/packages/sprite/src/sprite.ts:73 (sha256:36b7e82f17e5585f998102bfc7a033452c2bd261063abef58936b65976088853)
pub fn create_sprite_runtime() -> SpriteRuntime {
    return create_display_object_runtime(Some(((*DEFAULT_METHODS).clone()).clone()));
}

// Source: upstream/packages/sprite/src/sprite.ts:77 (sha256:42f76943fcbd528742e171c5ce3b6d001497d48940e7aca6ac9150c636961f51)
pub fn create_sprite_signals() -> SpriteSignals {
    return SpriteSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_frame_changed: create_signal(),
    };
}

// Source: upstream/packages/sprite/src/sprite.ts:88 (sha256:8becef86c591d3c7af387527e3de1bbcd8b8e161df3e378631cc8b8a8411a81e)
pub fn enable_sprite_signals(target: &mut Sprite) -> SpriteSignals {
    let mut s = target;
    return {
        s[*SPRITE_SIGNALS_SLOT as usize]?? = create_sprite_signals();
        s[*SPRITE_SIGNALS_SLOT as usize]
    };
}

// Source: upstream/packages/sprite/src/sprite.ts:99 (sha256:a67945fa536b7e32567c73192f3867d9c110513be0c223aca74b7fa8e6c1c8a7)
pub fn get_sprite_origin(out: &mut Vector2, source: &Sprite) -> () {
    let region = get_sprite_region(source);
    let pivot_x = if (region).is_some() {
        (region.as_ref().unwrap().pivot_x).unwrap_or(0.0_f64)
    } else {
        0.0_f64
    };
    let pivot_y = if (region).is_some() {
        (region.as_ref().unwrap().pivot_y).unwrap_or(0.0_f64)
    } else {
        0.0_f64
    };
    out.x = if (pivot_x == 0.0_f64) {
        0.0_f64
    } else {
        (-pivot_x)
    };
    out.y = if (pivot_y == 0.0_f64) {
        0.0_f64
    } else {
        (-pivot_y)
    };
}

// Source: upstream/packages/sprite/src/sprite.ts:111 (sha256:829eb14f69dd59aedd5db68832e9c60d6f5a14f83503a47ce3396a3115412f94)
pub fn get_sprite_region(source: &Sprite) -> Option<TextureAtlasRegion> {
    let atlas = (source.data.atlas).clone();
    let id = source.data.id;
    if (atlas).is_none() {
        return None;
    }
    return ((atlas.as_ref().unwrap().regions).clone())
        .iter()
        .find(|value| (|r: TextureAtlasRegion| -> bool { (r.id == id) })((*value).clone()))
        .cloned();
}

// Source: upstream/packages/sprite/src/sprite.ts:117 (sha256:d4bb9504cae5f853af6bdcd8122ded28608dcf4d32038d7486bfbc5bf50276c9)
pub fn get_sprite_runtime(source: &Sprite) -> SpriteRuntime {
    return get_display_object_runtime(source);
}

// Source: upstream/packages/sprite/src/sprite.ts:122 (sha256:fc8d3bc9e393112424dd1eff00f0f46ab020137361c3da9c2df667d3585b376f)
pub fn get_sprite_signals(source: &Sprite) -> Option<SpriteSignals> {
    return Some(source[*SPRITE_SIGNALS_SLOT as usize].clone());
}

// Source: upstream/packages/sprite/src/sprite.ts:127 (sha256:201a0d5dddd9f71a1af66755aef39b3ca858936739277dde14c060322994895d)
pub fn set_sprite_frame(target: &mut Sprite, id: f64) -> () {
    target.data.id = id;
    let signals = get_sprite_signals(target);
    if (signals).is_some() {
        ((signals.as_ref().unwrap().on_frame_changed.emit).clone())(id);
    }
}

// Source: upstream/packages/sprite/src/sprite.ts:134 (sha256:5325d94d67b7552a64165902a02e2bd64c64e12f536b4976e3e93b97abcffd39)
pub fn set_sprite_frame_rect(target: &mut Sprite, rect: Option<Rectangle>) -> () {
    target.data.rect = (rect).clone();
}

// Source: upstream/packages/sprite/src/sprite.ts:138 (sha256:55f269778a81845c6979988d701e877f6bde0e0999f7c513d6cd2e1d208f6f77)
static DEFAULT_METHODS: std::sync::LazyLock<SpriteRuntime> =
    std::sync::LazyLock::new(|| SpriteRuntime {
        __flight_identity: std::sync::Arc::new(()),
        compute_local_bounds_rectangle: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: Rectangle, __flight_argument_1: BoundsNodeAny| -> () {
                compute_sprite_local_bounds_rectangle(
                    &mut __flight_argument_0,
                    &__flight_argument_1,
                )
            },
        )
            as Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>)),
    });

// Source: upstream/packages/sprite/src/sprite.ts:142 (sha256:e8992e14d88aa22bd95c47d26e2973516b1c9f161d46cfba5e7f6e86b4282777)
static SPRITE_SIGNALS_SLOT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/sprite/src/sprite.ts:144 (sha256:23c1d3c9bc9e721214a37bacb876ec7dd1d1fb3134172a4e8bf6418b0d924783)
#[derive(Clone)]
struct SpriteWithSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for SpriteWithSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
