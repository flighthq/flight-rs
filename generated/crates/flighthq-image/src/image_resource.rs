// @generated from upstream/packages/image/src/imageResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Image;

// Source: upstream/packages/image/src/imageResource.ts:53 (sha256:a1dd955f09013ac5dda80d27885912a9954717f91c639a2ddf419207efdaaeae)
pub fn is_image_resource_empty(resource: &Image) -> bool {
    return (resource.width <= 0.0_f64) || (resource.height <= 0.0_f64);
}
