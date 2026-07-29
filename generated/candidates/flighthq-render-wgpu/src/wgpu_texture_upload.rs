// @generated from upstream/packages/render-wgpu/src/wgpuTextureUpload.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ImageResource;

// Source: upstream/packages/render-wgpu/src/wgpuTextureUpload.ts:10 (sha256:ab07af3c34dcf21813cbdee81c5b806d0b8265e5c5be451da5dab408225aa20d)
pub fn upload_wgpu_texture_data(
    device: crate::OpaqueHostValue,
    texture: crate::OpaqueHostValue,
    origin: crate::OpaqueHostValue,
    width: f64,
    height: f64,
    data: &Vec<u8>,
) -> () {
    crate::host_value::<()>("host.writeTexture");
}

// Source: upstream/packages/render-wgpu/src/wgpuTextureUpload.ts:28 (sha256:1f0c29a6caf565e4c867a054f47e769a3009685270d1b2802548dd0828afbcf3)
pub fn upload_wgpu_texture_element(
    device: crate::OpaqueHostValue,
    texture: crate::OpaqueHostValue,
    origin: crate::OpaqueHostValue,
    width: f64,
    height: f64,
    source: crate::OpaqueHostValue,
) -> () {
    crate::host_value::<()>("host.copyExternalImageToTexture");
}

// Source: upstream/packages/render-wgpu/src/wgpuTextureUpload.ts:41 (sha256:4cc43b35c1c21191f2aa9ca6b4df78d4613211dff5d96b13a1342bcf299ca239)
pub fn upload_wgpu_texture_image_resource(
    device: crate::OpaqueHostValue,
    texture: crate::OpaqueHostValue,
    origin: crate::OpaqueHostValue,
    image: &ImageResource,
) -> () {
    if ((image.source).clone()).is_some() {
        upload_wgpu_texture_element(
            (device).clone(),
            (texture).clone(),
            (origin).clone(),
            image.width,
            image.height,
            ((image.source).clone()).unwrap(),
        );
    } else {
        upload_wgpu_texture_data(
            (device).clone(),
            (texture).clone(),
            (origin).clone(),
            image.width,
            image.height,
            image.data.as_ref().unwrap(),
        );
    }
}
