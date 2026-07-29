use flighthq_surface::{
    SurfaceConvolutionOptions, apply_surface_curve, apply_surface_levels, apply_surface_palette_map,
    build_surface_brightness_color_matrix, color_matrix_surface, compare_surface_fingerprints,
    convolve_surface, copy_surface_pixels, create_surface_fingerprint, dilate_surface, erode_surface,
    fill_surface_rectangle, fill_surface_noise, get_surface_coverage, get_surface_pixel,
    get_surface_pixel_luminance, get_surface_color_bounds_rectangle, get_surface_histogram,
    get_surface_pixel_rgb, get_surface_mismatch, merge_surface_channels, multiply_surface_alpha,
    pixelate_surface, premultiply_surface_pixels, set_surface_alpha, set_surface_pixel,
    unpremultiply_surface_pixels,
};
use flighthq_types::{Surface, SurfaceRegion};

fn surface(data: Vec<u8>, width: f64, height: f64) -> Surface {
    Surface {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
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
        __flight_identity: std::sync::Arc::new(()),
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
    assert_eq!(get_surface_pixel_rgb(&image, 0.0, 0.0), 0x11_2233_u32 as f64);
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
    let image = surface(vec![0x11, 0x22, 0x33, 0x44, 0x11, 0x22, 0x33, 0x45], 2.0, 1.0);
    assert_eq!(get_surface_coverage(&image, 0x1122_3344_u32 as f64, None), 0.5);
    assert_eq!(
        get_surface_coverage(&image, 0x1122_3344_u32 as f64, Some(1.0)),
        0.0,
    );
}

#[test]
fn identity_convolution_copies_the_selected_region() {
    let source = region(surface(vec![10, 20, 30, 40, 50, 60, 70, 80], 2.0, 1.0));
    let options = SurfaceConvolutionOptions {
        __flight_identity: std::sync::Arc::new(()),
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
    assert_eq!(target.surface.data, vec![0, 0, 0, 0, 0xaa, 0xbb, 0xcc, 0xdd]);
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

#[test]
fn color_bounds_returns_a_generated_rectangle_or_none() {
    let source = region(surface(
        vec![0, 0, 0, 255, 10, 20, 30, 255, 10, 20, 30, 128],
        3.0,
        1.0,
    ));
    let bounds = get_surface_color_bounds_rectangle(
        &source,
        0xffff_ff00_u32 as f64,
        0x0a14_1eff_u32 as f64,
        None,
    )
    .expect("matching RGB bounds");
    assert_eq!((bounds.x, bounds.y, bounds.width, bounds.height), (1.0, 0.0, 2.0, 1.0));
    assert!(
        get_surface_color_bounds_rectangle(
            &source,
            0xffff_ffff_u32 as f64,
            0xffff_ffff_u32 as f64,
            None,
        )
        .is_none(),
    );
}

#[test]
fn histogram_counts_generated_channel_bins() {
    let source = region(surface(vec![1, 2, 3, 4, 1, 8, 3, 9], 2.0, 1.0));
    let histogram = get_surface_histogram(&source);
    assert_eq!(histogram.red[1], 2.0);
    assert_eq!(histogram.green[2], 1.0);
    assert_eq!(histogram.green[8], 1.0);
    assert_eq!(histogram.blue[3], 2.0);
    assert_eq!(histogram.alpha[4], 1.0);
    assert_eq!(histogram.alpha[9], 1.0);
}

#[test]
fn nullable_palette_maps_pass_through_unselected_channels() {
    let source = region(surface(vec![10, 20, 30, 40], 1.0, 1.0));
    let mut destination = region(surface(vec![0; 4], 1.0, 1.0));
    let red = (0..256).map(|value| (255 - value) as f64).collect();

    apply_surface_palette_map(
        &mut destination,
        &source,
        Some(red),
        None,
        None,
        None,
    );

    assert_eq!(destination.surface.data, vec![245, 20, 30, 40]);
    assert_eq!(destination.surface.version, 1.0);
}

#[test]
fn compatible_typed_array_unions_drive_curve_and_levels_kernels() {
    let source = region(surface(vec![10, 128, 240, 77], 1.0, 1.0));
    let mut curved = region(surface(vec![0; 4], 1.0, 1.0));
    let inverted = (0..256).map(|value| (255 - value) as u8).collect();

    apply_surface_curve(
        &mut curved,
        &source,
        Some(inverted),
        None,
        None,
        None,
    );
    assert_eq!(curved.surface.data, vec![245, 128, 240, 77]);
    assert_eq!(curved.surface.version, 1.0);

    let mut leveled = region(surface(vec![0; 4], 1.0, 1.0));
    apply_surface_levels(
        &mut leveled,
        &source,
        Some(0.0),
        Some(255.0),
        Some(0.5),
    );
    assert_eq!(leveled.surface.data, vec![0, 64, 226, 77]);
    assert_eq!(leveled.surface.version, 1.0);
}

#[test]
fn mismatch_summary_reports_tolerance_fraction_and_maximum_delta() {
    let source = surface(vec![0, 0, 0, 255, 0, 0, 0, 255], 2.0, 1.0);
    let other = surface(vec![10, 0, 0, 255, 0, 128, 0, 255], 2.0, 1.0);
    let mismatch = get_surface_mismatch(&source, &other, Some(10.0));

    assert_eq!(mismatch.mismatched_pixels, 1.0);
    assert_eq!(mismatch.total_pixels, 2.0);
    assert_eq!(mismatch.fraction, 0.5);
    assert_eq!(mismatch.max_channel_delta, 128.0);
}

#[test]
fn channel_merge_reads_each_selected_source_channel() {
    let red = region(surface(vec![10, 1, 2, 3], 1.0, 1.0));
    let green = region(surface(vec![4, 20, 5, 6], 1.0, 1.0));
    let blue = region(surface(vec![7, 8, 30, 9], 1.0, 1.0));
    let alpha = region(surface(vec![11, 12, 13, 40], 1.0, 1.0));
    let mut out = region(surface(vec![0; 4], 1.0, 1.0));

    merge_surface_channels(&mut out, &red, &green, &blue, &alpha);

    assert_eq!(out.surface.data, vec![10, 20, 30, 40]);
    assert_eq!(out.surface.version, 1.0);
}

#[test]
fn generated_fingerprints_compose_structural_records_and_typed_arrays() {
    let first = surface(vec![0, 10, 20, 255, 100, 110, 120, 255], 2.0, 1.0);
    let mut second = first.clone();
    second.data[4] = 140;

    let first_fingerprint = create_surface_fingerprint(&first, Some(1.0));
    let second_fingerprint = create_surface_fingerprint(&second, Some(1.0));

    assert_eq!(first_fingerprint.grid_size, 1.0);
    assert_eq!(first_fingerprint.cells, vec![50, 60, 70]);
    assert_eq!(
        compare_surface_fingerprints(&first_fingerprint, &second_fingerprint),
        20.0 / 3.0,
    );
}
