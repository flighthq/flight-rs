use flighthq_image::is_image_resource_empty;
use flighthq_types::ImageResource;

// Only the two fields the assertion depends on are set; everything else comes from `Default`. The previous
// version spelled out alpha type, gamut, kind, source and version, none of which `is_image_resource_empty`
// reads — so it broke when upstream renamed `Image` to `ImageResource` and again whenever that struct gained a
// field. A fixture should name what it is testing and nothing else.
fn resource(width: f64, height: f64) -> ImageResource {
    ImageResource {
        height,
        width,
        ..Default::default()
    }
}

#[test]
fn portable_empty_query_matches_the_typescript_contract() {
    let _flight_task_scheduler = flighthq_runtime::install_deterministic_flight_task_scheduler();
    assert!(!is_image_resource_empty(&resource(1.0, 1.0)));
    assert!(is_image_resource_empty(&resource(0.0, 1.0)));
    assert!(is_image_resource_empty(&resource(1.0, 0.0)));
}
