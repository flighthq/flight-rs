use flighthq_image::{
    dispose_image_resource, has_image_resource_data, has_image_resource_pixels,
    has_image_resource_source, invalidate_image_resource, is_image_resource_empty,
};
use flighthq_types::ImageResource;

fn resource() -> ImageResource {
    ImageResource {
        alpha_type: "straight".to_owned(),
        compressed: None,
        data: Some(vec![1, 2, 3, 4]),
        format: "rgba8unorm".to_owned(),
        height: 1.0,
        source: None,
        version: 7.0,
        width: 1.0,
    }
}

#[test]
fn portable_resource_queries_match_the_typescript_contract() {
    let image = resource();
    assert!(has_image_resource_data(&image));
    assert!(has_image_resource_pixels(&image));
    assert!(!has_image_resource_source(&image));
    assert!(!is_image_resource_empty(&image));
}

#[test]
fn mutation_uses_borrowed_identity_and_increments_version() {
    let mut image = resource();
    invalidate_image_resource(&mut image);
    assert_eq!(image.version, 8.0);

    dispose_image_resource(&mut image);
    assert_eq!(image.version, 9.0);
    assert!(image.data.is_none());
    assert!(image.compressed.is_none());
    assert!(image.source.is_none());
}
