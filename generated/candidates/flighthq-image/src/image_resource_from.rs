// @generated from upstream/packages/image/src/imageResourceFrom.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::ImageResource;

// Source: upstream/packages/image/src/imageResourceFrom.ts:11 (sha256:5911ca37f5c8577dfc39c2238b209b4bbc48a04fccaec1876f787f391982418c)
pub fn create_canvas_from_image_resource(image: &ImageResource) -> Option<crate::OpaqueHostValue> {
    if ((image.data).clone()).is_none() {
        return None;
    }
    let mut canvas = crate::host_value::<()>("host.createElement");
    canvas.width = image.width;
    canvas.height = image.height;
    let mut image_data = crate::host_image_data(crate::FlightImageDataRequest::Dimensions {
        width: image.width,
        height: image.height,
    });
    crate::host_value::<()>("host.set");
    crate::host_value::<()>("host.call");
    return Some(canvas);
}

// Source: upstream/packages/image/src/imageResourceFrom.ts:22 (sha256:6e9ede06a14d274f9c67a34fbd030c01f7081d42e31321c632d7668d066f800c)
pub fn create_image_resource_from_canvas(canvas: crate::OpaqueHostValue) -> ImageResource {
    return create_entity(Some(ImageResource {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        alpha_type: "straight".to_owned(),
        compressed: None,
        data: None,
        format: "rgba8unorm".to_owned(),
        height: crate::host_value::<f64>("host.height"),
        source: canvas,
        version: 0.0_f64,
        width: crate::host_value::<f64>("host.width"),
    }));
}

// Source: upstream/packages/image/src/imageResourceFrom.ts:35 (sha256:bf14d47a156cce1b9d347f249487895d1e9a9f12ec5abcd0594ee5548d5e0c2e)
pub fn create_image_resource_from_image_bitmap(bitmap: crate::OpaqueHostValue) -> ImageResource {
    return create_entity(Some(ImageResource {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        alpha_type: "straight".to_owned(),
        compressed: None,
        data: None,
        format: "rgba8unorm".to_owned(),
        height: crate::host_value::<f64>("host.height"),
        source: bitmap,
        version: 0.0_f64,
        width: crate::host_value::<f64>("host.width"),
    }));
}

// Source: upstream/packages/image/src/imageResourceFrom.ts:48 (sha256:4494121a7ee410049a2170bcfd19234743f46a52c10f89eb3e8b8b5147dbc59f)
pub fn create_image_resource_from_image_element(img: crate::OpaqueHostValue) -> ImageResource {
    return create_entity(Some(ImageResource {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        alpha_type: "straight".to_owned(),
        compressed: None,
        data: None,
        format: "rgba8unorm".to_owned(),
        height: crate::host_value::<f64>("host.height"),
        source: img,
        version: 0.0_f64,
        width: crate::host_value::<f64>("host.width"),
    }));
}

// Source: upstream/packages/image/src/imageResourceFrom.ts:61 (sha256:753933c52ef5581f81af9094c64b3c54e8a40c7d10d7e4b8169a15e7df3cdfc5)
pub fn is_image_resource_same_origin(url: String) -> bool {
    if ((url).starts_with(("data:".to_owned()).as_str()))
        || ((url).starts_with(("blob:".to_owned()).as_str()))
    {
        return true;
    }
    let __flight_try_return: Option<bool> =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<bool> {
            {
                return Some(
                    (crate::host_value::<crate::OpaqueHostValue>("host.origin")
                        == crate::host_value::<crate::OpaqueHostValue>("host.origin")),
                );
            }
            None
        })) {
            Ok(value) => value,
            Err(_) => (|| -> Option<bool> {
                {
                    return Some(true);
                }
                None
            })(),
        };
    return __flight_try_return.expect("TypeScript try/catch completed without returning");
}

// Source: upstream/packages/image/src/imageResourceFrom.ts:70 (sha256:9766d6f1f595df39b0431b4aeebaf140d9cddbf13b4f4fb7212ce5980b7ace63)
pub fn load_image_resource_from_base64(
    base64: String,
    mime_type: String,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<ImageResource> {
    Default::default()
}

// Source: upstream/packages/image/src/imageResourceFrom.ts:78 (sha256:789b648cfba73930add0a4f3468f72b94ce411b39d6860c22c04ab05e33590ee)
pub fn load_image_resource_from_blob(
    blob: crate::OpaqueHostValue,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<ImageResource> {
    Default::default()
}

// Source: upstream/packages/image/src/imageResourceFrom.ts:87 (sha256:67e113685cfd158a0c49cf1757b7761013665ff96a364e34cf11a1e250391c42)
pub fn load_image_resource_from_bytes(
    bytes: &Vec<u8>,
    mime_type: Option<String>,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<ImageResource> {
    Default::default()
}

// Source: upstream/packages/image/src/imageResourceFrom.ts:100 (sha256:ff3642966e0506d5d9c68236e6fa44abfd7fa6721ee45db95b1683464c41b591)
pub fn load_image_resource_from_url(
    url: String,
    cross_origin: Option<String>,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<ImageResource> {
    Default::default()
}
