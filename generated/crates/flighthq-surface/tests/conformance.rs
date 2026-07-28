use flighthq_surface::{
    SurfaceConvolutionOptions, build_surface_brightness_color_matrix, color_matrix_surface,
    convolve_surface, copy_surface_pixels, dilate_surface, erode_surface, fill_surface_noise,
    fill_surface_rectangle, get_surface_coverage, get_surface_pixel, get_surface_pixel_luminance,
    get_surface_pixel_rgb, multiply_surface_alpha, pixelate_surface, premultiply_surface_pixels,
    set_surface_alpha, set_surface_pixel, unpremultiply_surface_pixels,
};
use flighthq_types::{Surface, SurfaceRegion};

fn surface(data: Vec<u8>, width: f64, height: f64) -> Surface {
    Surface {
        alpha_type: "straight".to_owned(),
        compressed: None,
        data,
        format: "rgba8unorm".to_owned(),
        height,
        source: None,
        version: 0.0,
        width,
        color_space: "srgb".to_owned(),
    }
}

fn region(surface: Surface) -> SurfaceRegion {
    SurfaceRegion {
        width: surface.width,
        height: surface.height,
        surface,
        x: 0.0,
        y: 0.0,
    }
}

#[test]
fn packed_pixel_reads_match_rgba_layout() {
    let image = surface(vec![0x11, 0x22, 0x33, 0x44], 1.0, 1.0);
    assert_eq!(get_surface_pixel(&image, 0.0, 0.0), 0x1122_3344_u32 as f64);
    assert_eq!(
        get_surface_pixel_rgb(&image, 0.0, 0.0),
        0x11_2233_u32 as f64
    );
    assert_eq!(get_surface_pixel_luminance(&image, 0.0, 0.0), 32.0);
}

#[test]
fn packed_pixel_writes_inline_the_generated_invalidation_effect() {
    let mut image = surface(vec![0, 0, 0, 0], 1.0, 1.0);
    set_surface_pixel(&mut image, 0.0, 0.0, 0x1122_3344_u32 as f64);
    assert_eq!(image.data, vec![0x11, 0x22, 0x33, 0x44]);
    assert_eq!(image.version, 1.0);
}

#[test]
fn coverage_observes_background_and_channel_tolerance() {
    let image = surface(
        vec![0x11, 0x22, 0x33, 0x44, 0x11, 0x22, 0x33, 0x45],
        2.0,
        1.0,
    );
    assert_eq!(
        get_surface_coverage(&image, 0x1122_3344_u32 as f64, None),
        0.5
    );
    assert_eq!(
        get_surface_coverage(&image, 0x1122_3344_u32 as f64, Some(1.0)),
        0.0,
    );
}

#[test]
fn identity_convolution_copies_the_selected_region() {
    let source = region(surface(vec![10, 20, 30, 40, 50, 60, 70, 80], 2.0, 1.0));
    let options = SurfaceConvolutionOptions {
        bias: None,
        edge: None,
        divisor: None,
        matrix: vec![1.0],
        matrix_x: 1.0,
        matrix_y: 1.0,
        preserve_alpha: None,
    };
    let mut out = vec![0; 8];
    convolve_surface(&mut out, &source, &options);
    assert_eq!(out, source.surface.data);
}

#[test]
fn morphology_and_pixelation_match_small_neighborhoods() {
    let source = region(surface(
        vec![10, 10, 10, 255, 100, 100, 100, 255, 20, 20, 20, 255],
        3.0,
        1.0,
    ));
    let mut dilated = vec![0; 12];
    let mut eroded = vec![0; 12];
    dilate_surface(&mut dilated, &source, 1.0);
    erode_surface(&mut eroded, &source, 1.0);
    assert_eq!(&dilated[4..8], &[100, 100, 100, 255]);
    assert_eq!(&eroded[4..8], &[10, 10, 10, 255]);

    let two_pixels = region(surface(vec![10, 20, 30, 40, 20, 40, 60, 80], 2.0, 1.0));
    let mut pixelated = vec![0; 8];
    pixelate_surface(&mut pixelated, &two_pixels, 2.0);
    assert_eq!(pixelated, vec![15, 30, 45, 60, 15, 30, 45, 60]);
}

#[test]
fn generated_color_matrix_builders_feed_the_generated_kernel() {
    let source = region(surface(vec![10, 20, 30, 40], 1.0, 1.0));
    let mut matrix = vec![0.0; 20];
    build_surface_brightness_color_matrix(&mut matrix, 2.0);
    let mut out = vec![0; 4];
    color_matrix_surface(&mut out, &source, &matrix);
    assert_eq!(out, vec![20, 40, 60, 40]);
}

#[test]
fn alpha_writers_mutate_borrowed_regions_and_invalidate_once() {
    let mut target = region(surface(vec![10, 20, 30, 200, 40, 50, 60, 100], 2.0, 1.0));
    multiply_surface_alpha(&mut target, 0.5);
    assert_eq!(target.surface.data, vec![10, 20, 30, 100, 40, 50, 60, 50]);
    assert_eq!(target.surface.version, 1.0);
    set_surface_alpha(&mut target, 255.0);
    assert_eq!(target.surface.data, vec![10, 20, 30, 255, 40, 50, 60, 255]);
    assert_eq!(target.surface.version, 2.0);
}

#[test]
fn region_copy_preserves_source_and_invalidates_destination() {
    let source = region(surface(vec![1, 2, 3, 4, 5, 6, 7, 8], 2.0, 1.0));
    let mut destination = region(surface(vec![0; 8], 2.0, 1.0));
    copy_surface_pixels(&mut destination, &source, None);
    assert_eq!(destination.surface.data, source.surface.data);
    assert_eq!(destination.surface.version, 1.0);
}

#[test]
fn rectangle_fill_writes_packed_rgba_and_invalidates() {
    let mut target = region(surface(vec![0; 8], 2.0, 1.0));
    target.x = 1.0;
    target.width = 1.0;
    fill_surface_rectangle(&mut target, 0xaabb_ccdd_u32 as f64);
    assert_eq!(
        target.surface.data,
        vec![0, 0, 0, 0, 0xaa, 0xbb, 0xcc, 0xdd]
    );
    assert_eq!(target.surface.version, 1.0);
}

#[test]
fn alpha_representation_round_trip_uses_raw_generated_kernels() {
    let straight = vec![200, 100, 50, 128];
    let mut premultiplied = vec![0; 4];
    premultiply_surface_pixels(&mut premultiplied, &straight, 4.0);
    assert_eq!(premultiplied, vec![100, 50, 25, 128]);

    let mut round_trip = vec![0; 4];
    unpremultiply_surface_pixels(&mut round_trip, &premultiplied, 4.0);
    assert_eq!(round_trip, vec![199, 100, 50, 128]);
}

#[test]
fn deterministic_noise_skips_clipped_pixels_without_changing_the_field() {
    let mut clipped = region(surface(vec![0; 8], 2.0, 1.0));
    clipped.x = -1.0;
    clipped.width = 3.0;
    fill_surface_noise(&mut clipped, 123.0, None, None, Some(true));

    let mut full = region(surface(vec![0; 12], 3.0, 1.0));
    fill_surface_noise(&mut full, 123.0, None, None, Some(true));

    assert_eq!(clipped.surface.data, full.surface.data[4..]);
    assert_eq!(clipped.surface.version, 1.0);
}
