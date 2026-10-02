// @generated from upstream/packages/image/src/imageResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ImageResource;

// Source: upstream/packages/image/src/imageResource.ts:71 (sha256:495344d8cd929b57d1b73fcfb210c40ae730dc25f3e602c576a99487310013f0)
pub fn is_image_resource_empty(resource: &ImageResource) -> bool {
    return (resource.width <= 0.0_f64) || (resource.height <= 0.0_f64);
}
