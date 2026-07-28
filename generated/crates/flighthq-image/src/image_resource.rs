// @generated from upstream/packages/image/src/imageResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use flighthq_types::ImageResource;

// Source: upstream/packages/image/src/imageResource.ts:57 (sha256:6aa6642e4ceb5b5fd7ceeba3f6c9632ad0667cfefc6ddccfb3064f8307dd467c)
pub fn dispose_image_resource(resource: &mut ImageResource) -> () {
    resource.compressed = None;
    resource.data = None;
    resource.source = None;
    invalidate_image_resource(resource);
}

// Source: upstream/packages/image/src/imageResource.ts:72 (sha256:d85a1cd242be3ddd03f0f6ee8ac57f8e4bdecbe83038b24165a900ccafc53d6c)
pub fn has_image_resource_data(resource: &ImageResource) -> bool {
    return (resource.data).is_some();
}

// Source: upstream/packages/image/src/imageResource.ts:82 (sha256:3025bcd14cf712d8d4f058254a2ddff6f0e58752bf175f0f8128e60614b86bf0)
pub fn has_image_resource_pixels(resource: &ImageResource) -> bool {
    return (((resource.source).is_some() || (resource.data).is_some())
        || (resource.compressed).is_some());
}

// Source: upstream/packages/image/src/imageResource.ts:86 (sha256:bb89b5681b5a1be203b91af484721d4a7e61cb958c13c1d6eb289071a2585b4a)
pub fn has_image_resource_source(resource: &ImageResource) -> bool {
    return (resource.source).is_some();
}

// Source: upstream/packages/image/src/imageResource.ts:93 (sha256:d01c1c7eaa8a4e22dab995371c7f44f08985eccb2081c3972954f0f03879b720)
pub fn invalidate_image_resource(resource: &mut ImageResource) -> () {
    resource.version = (((resource.version + 1.0_f64) as u32) >> ((0.0_f64) as u32)) as f64;
}

// Source: upstream/packages/image/src/imageResource.ts:97 (sha256:495344d8cd929b57d1b73fcfb210c40ae730dc25f3e602c576a99487310013f0)
pub fn is_image_resource_empty(resource: &ImageResource) -> bool {
    return ((resource.width <= 0.0_f64) || (resource.height <= 0.0_f64));
}
