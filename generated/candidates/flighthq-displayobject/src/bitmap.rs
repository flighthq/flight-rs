// @generated from upstream/packages/displayobject/src/bitmap.ts; do not edit.
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
    BITMAP_KIND as bitmap_kind_constant, Bitmap, BitmapData, BitmapRuntime, BoundsNodeAny, Node,
    Rectangle,
};

// Source: upstream/packages/displayobject/src/bitmap.ts:7 (sha256:47f1660cb2ac54773bfb30e2c6016d91fea1b15b7ae1b91d2196074cce9b849f)
pub fn compute_bitmap_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    let bitmap_data: BitmapData = ((source.data).clone()).unwrap();
    if ((bitmap_data.source_rectangle).clone()).is_some() {
        out.width = bitmap_data.source_rectangle.as_ref().unwrap().width;
        out.height = bitmap_data.source_rectangle.as_ref().unwrap().height;
    } else {
        if ((bitmap_data.image).clone()).is_some() {
            out.width = bitmap_data.image.as_ref().unwrap().width;
            out.height = bitmap_data.image.as_ref().unwrap().height;
        }
    }
}

// Source: upstream/packages/displayobject/src/bitmap.ts:18 (sha256:326d69409854f1a85f40bab4d79d2fbf7c0afdc7e848f95ef29b58a82dac68c3)
pub fn create_bitmap(obj: Option<Bitmap>) -> Bitmap {
    return create_display_object_generic(
        (bitmap_kind_constant).to_owned(),
        Some(((obj).clone().unwrap()).clone()),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<D>| -> D {
                create_bitmap_data(Some(((__flight_argument_0).clone().unwrap()).clone()))
            },
        )
            as Box<dyn FnMut(Option<D>) -> D + Send + 'static>))),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_bitmap_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
}

// Source: upstream/packages/displayobject/src/bitmap.ts:22 (sha256:0e72596a1f1e4872c179a5353601bf783204873cd0df8d2c3c98b48766e784d7)
pub fn create_bitmap_data(data: Option<BitmapData>) -> BitmapData {
    return BitmapData {
        __flight_identity: std::sync::Arc::new(()),
        image: data.as_ref().and_then(|value| (value.image).clone()),
        smoothing: (data.as_ref().map(|value| value.smoothing)).unwrap_or(true),
        source_rectangle: data
            .as_ref()
            .and_then(|value| (value.source_rectangle).clone()),
    };
}

// Source: upstream/packages/displayobject/src/bitmap.ts:30 (sha256:c3c1b1e36e721da1427be8ff79a7c8542b72975d70e0437a8f7196384c785796)
pub fn create_bitmap_runtime() -> BitmapRuntime {
    return create_display_object_runtime(Some(((*DEFAULT_METHODS).clone()).clone()));
}

// Source: upstream/packages/displayobject/src/bitmap.ts:34 (sha256:cd6bbb8537dab7d743e0491cd4b0782bc50733f76bd2baf2f755cf6c4934bbe0)
pub fn get_bitmap_runtime(source: &Bitmap) -> BitmapRuntime {
    return get_display_object_runtime(source);
}

// Source: upstream/packages/displayobject/src/bitmap.ts:38 (sha256:99b49a54bce78d0101967905ead64264e221774e4dd5a149cabdad288cdfd246)
pub fn set_bitmap_image(source: &mut Bitmap, value: crate::OpaqueHostValue) -> () {
    source.data.image = Some(value);
    invalidate_node_local_content(source);
    invalidate_node_local_bounds(source);
}

// Source: upstream/packages/displayobject/src/bitmap.ts:46 (sha256:9337f67a77b51dd32d3e0e2e53c959a0adf56a0d0ada1c29cafcdf220c6ed06b)
static DEFAULT_METHODS: std::sync::LazyLock<BitmapRuntime> =
    std::sync::LazyLock::new(|| BitmapRuntime {
        __flight_identity: std::sync::Arc::new(()),
        compute_local_bounds_rectangle: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: Rectangle, __flight_argument_1: BoundsNodeAny| -> () {
                compute_bitmap_local_bounds_rectangle(
                    &mut __flight_argument_0,
                    &__flight_argument_1,
                )
            },
        )
            as Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>)),
    });
