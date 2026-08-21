// @generated from upstream/packages/types/src/RequirementFacet.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RequirementFacet.ts:3 (sha256:e025b6957adc27de2455268425e7c72a1fe626c0f6ce26045d5819743cd90cdc)
#[derive(Clone, Default)]
pub struct RequirementFacetValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub compression_kind: String,
    pub document_format: String,
    pub physics2_d_joint_kind: String,
    pub scene_blend_mode: String,
    pub scene_material_kind: String,
    pub scene_modifier_kind: String,
    pub scene_node_kind: String,
    pub scene_resource_mime_type: String,
    pub scene_shape_command: String,
    pub scene_texture_source_kind: String,
}
impl PartialEq for RequirementFacetValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static REQUIREMENT_FACET: std::sync::LazyLock<RequirementFacetValues> =
    std::sync::LazyLock::new(|| RequirementFacetValues {
        __flight_identity: std::sync::Arc::new(()),
        compression_kind: "compression.kind".to_owned(),
        document_format: "document.format".to_owned(),
        physics2_d_joint_kind: "physics2d.joint-kind".to_owned(),
        scene_blend_mode: "scene.blend-mode".to_owned(),
        scene_material_kind: "scene.material-kind".to_owned(),
        scene_modifier_kind: "scene.modifier-kind".to_owned(),
        scene_node_kind: "scene.node-kind".to_owned(),
        scene_resource_mime_type: "scene.resource-mime-type".to_owned(),
        scene_shape_command: "scene.shape-command".to_owned(),
        scene_texture_source_kind: "scene.texture-source-kind".to_owned(),
    });

// Source: upstream/packages/types/src/RequirementFacet.ts:16 (sha256:cea31be5e91017317227e511a5230aafc8999082ecb10a9f9989ccbc069b3f65)
pub type RequirementFacet = String;
