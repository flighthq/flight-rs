use flighthq_image::is_image_resource_empty;
use flighthq_types::Image;

fn resource(width: f64, height: f64) -> Image {
    Image {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        alpha_type: "straight".to_owned(),
        gamut: "srgb".to_owned(),
        height,
        kind: "image".to_owned(),
        source: Default::default(),
        version: 7.0,
        width,
    }
}

#[test]
fn portable_empty_query_matches_the_typescript_contract() {
    assert!(!is_image_resource_empty(&resource(1.0, 1.0)));
    assert!(is_image_resource_empty(&resource(0.0, 1.0)));
    assert!(is_image_resource_empty(&resource(1.0, 0.0)));
}
