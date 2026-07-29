// @generated from upstream/packages/image/src/imageResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{ImageResource, ImageResourceCompressed};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

// Source: upstream/packages/image/src/imageResource.ts:8 (sha256:a5c168b899cbe76b2c20d8fa688928c5d23b4b46fe94da7b46aad3c53193f924)
pub fn clone_image_resource(resource: &ImageResource) -> ImageResource {
    return create_entity(Some(ImageResource {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        alpha_type: (resource.alpha_type).clone(),
        compressed: (resource.compressed).clone(),
        data: (resource.data).clone(),
        format: (resource.format).clone(),
        height: resource.height,
        source: (resource.source).clone(),
        version: resource.version,
        width: resource.width,
    }));
}

// Source: upstream/packages/image/src/imageResource.ts:25 (sha256:a2e9de0533860ccf1c33916b1945cb719f980261b1b1f137a7a338c66cbf1fc5)
pub fn create_compressed_image_resource(compressed: &ImageResourceCompressed) -> ImageResource {
    return create_entity(Some(ImageResource {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        alpha_type: "straight".to_owned(),
        compressed: Some((*compressed).clone()),
        data: None,
        format: "rgba8unorm".to_owned(),
        height: compressed.container.height,
        source: None,
        version: 0.0_f64,
        width: compressed.container.width,
    }));
}

// Source: upstream/packages/image/src/imageResource.ts:38 (sha256:98546a52be273927c76fd381573179f1199443ef82eb14b1a4df04976d7730b3)
pub fn create_image_resource(image: Option<crate::OpaqueHostValue>) -> ImageResource {
    let mut resource: ImageResource = create_entity(Some(ImageResource {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        alpha_type: "straight".to_owned(),
        compressed: None,
        data: None,
        format: "rgba8unorm".to_owned(),
        height: 0.0_f64,
        source: image,
        version: 0.0_f64,
        width: 0.0_f64,
    }));
    if ((resource.source).clone()).is_some() {
        update_image_resource_size(&mut resource);
    }
    return resource;
}

// Source: upstream/packages/image/src/imageResource.ts:57 (sha256:6aa6642e4ceb5b5fd7ceeba3f6c9632ad0667cfefc6ddccfb3064f8307dd467c)
pub fn dispose_image_resource(resource: &mut ImageResource) -> () {
    resource.compressed = None;
    resource.data = None;
    resource.source = None;
    invalidate_image_resource(resource);
}

// Source: upstream/packages/image/src/imageResource.ts:68 (sha256:d0d238baf434d43f43ae37a5c2c797611951f303f1d12a07fc56ba5397263d95)
pub fn get_image_resource_byte_size(resource: &ImageResource) -> f64 {
    return if ((resource.data).clone()).is_some() {
        resource.data.as_ref().unwrap().byte_length
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/image/src/imageResource.ts:72 (sha256:d85a1cd242be3ddd03f0f6ee8ac57f8e4bdecbe83038b24165a900ccafc53d6c)
pub fn has_image_resource_data(resource: &ImageResource) -> bool {
    return ((resource.data).clone()).is_some();
}

// Source: upstream/packages/image/src/imageResource.ts:82 (sha256:3025bcd14cf712d8d4f058254a2ddff6f0e58752bf175f0f8128e60614b86bf0)
pub fn has_image_resource_pixels(resource: &ImageResource) -> bool {
    return ((((resource.source).clone()).is_some()) || (((resource.data).clone()).is_some()))
        || (((resource.compressed).clone()).is_some());
}

// Source: upstream/packages/image/src/imageResource.ts:86 (sha256:bb89b5681b5a1be203b91af484721d4a7e61cb958c13c1d6eb289071a2585b4a)
pub fn has_image_resource_source(resource: &ImageResource) -> bool {
    return ((resource.source).clone()).is_some();
}

// Source: upstream/packages/image/src/imageResource.ts:93 (sha256:d01c1c7eaa8a4e22dab995371c7f44f08985eccb2081c3972954f0f03879b720)
pub fn invalidate_image_resource(resource: &mut ImageResource) -> () {
    resource.version = (__flight_js_to_u32((resource.version + 1.0_f64))
        >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/image/src/imageResource.ts:97 (sha256:495344d8cd929b57d1b73fcfb210c40ae730dc25f3e602c576a99487310013f0)
pub fn is_image_resource_empty(resource: &ImageResource) -> bool {
    return (resource.width <= 0.0_f64) || (resource.height <= 0.0_f64);
}

// Source: upstream/packages/image/src/imageResource.ts:103 (sha256:ea0d482def90c52dc4a2876f85cc5a81e6d805faff001ad7f55d14d4acba7ad9)
pub fn set_image_resource_source(
    resource: &mut ImageResource,
    element: Option<crate::OpaqueHostValue>,
) -> () {
    resource.source = (element).clone();
    if (element).is_some() {
        update_image_resource_size(resource);
    }
    invalidate_image_resource(resource);
}

// Source: upstream/packages/image/src/imageResource.ts:111 (sha256:5f7732141800de72b4ad661b8bd0ae9c12de214a8db5edc383d51b0d8deeff51)
fn update_image_resource_size(resource: &mut ImageResource) -> () {
    let element = (resource.source).clone();
    if false {
        resource.width = crate::host_value::<f64>("host.videoWidth");
        resource.height = crate::host_value::<f64>("host.videoHeight");
    } else {
        let sized = (element).clone().unwrap();
        resource.width = crate::host_value::<f64>("host.width");
        resource.height = crate::host_value::<f64>("host.height");
    }
}
