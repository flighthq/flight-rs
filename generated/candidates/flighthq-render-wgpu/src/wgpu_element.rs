// @generated from upstream/packages/render-wgpu/src/wgpuElement.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/render-wgpu/src/wgpuElement.ts:1 (sha256:f346c1cae6f61391ed9858afec741ccb7966c367e09176c1b7189632815581db)
pub fn create_wgpu_canvas_element(
    width: f64,
    height: f64,
    pixel_ratio: Option<f64>,
) -> crate::OpaqueHostValue {
    let pixel_ratio = pixel_ratio.unwrap_or(1.0_f64);
    let mut canvas = crate::host_value::<()>("host.createElement");
    canvas.style.width = format!("{}px", width);
    canvas.style.height = format!("{}px", height);
    canvas.width = (width * pixel_ratio);
    canvas.height = (height * pixel_ratio);
    return canvas;
}
