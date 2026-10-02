// @generated from upstream/packages/types/src/FlightDocumentRefusalExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind};

// Source: upstream/packages/types/src/FlightDocumentRefusalExplanation.ts:6 (sha256:3fe9d98238a921acd9fd6078b0b7113ea4d61b3ad76981caa1b9d86f9c538deb)
#[derive(Clone, Default)]
pub struct FlightDocumentRefusalReasonValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alias_unsupported: String,
    pub anchor_unsupported: String,
    pub block_scalar_unsupported: String,
    pub collection_entries_limit_exceeded: String,
    pub default_scene_out_of_range: String,
    pub document_code_units_limit_exceeded: String,
    pub document_separator_unsupported: String,
    pub duplicate_ambient_light: String,
    pub duplicate_directional_light: String,
    pub duplicate_key: String,
    pub expected_flow_delimiter: String,
    pub expected_mapping_entry: String,
    pub expected_mapping_key: String,
    pub expected_scalar: String,
    pub expected_value: String,
    pub field_invalid: String,
    pub flow_sequence_unsupported: String,
    pub interactive_state_extension_kind_duplicate: String,
    pub interactive_state_extension_kind_unregistered: String,
    pub interactive_state_target_unsupported: String,
    pub interactive_state_transition_kind_unregistered: String,
    pub invalid_document: String,
    pub invalid_escape: String,
    pub key_code_units_limit_exceeded: String,
    pub layout_target_ambiguous: String,
    pub layout_target_unresolved: String,
    pub mixed_collection: String,
    pub multiple_root_values: String,
    pub nesting_depth_limit_exceeded: String,
    pub node_kind_unregistered: String,
    pub number_out_of_range: String,
    pub resource_kind_unregistered: String,
    pub resource_resolver_unregistered: String,
    pub resource_unresolved: String,
    pub root_indentation: String,
    pub root_kind_mismatch: String,
    pub scalar_code_units_limit_exceeded: String,
    pub scalar_invalid: String,
    pub scenes_empty: String,
    pub shape_command_unregistered: String,
    pub structure_invalid: String,
    pub tab_character: String,
    pub tag_unsupported: String,
    pub token_key_duplicate: String,
    pub token_key_invalid: String,
    pub token_kind_mismatch: String,
    pub token_mode_invalid: String,
    pub token_mode_unresolved: String,
    pub token_reference_cycle: String,
    pub token_reference_invalid: String,
    pub token_resolver_unregistered: String,
    pub token_unresolved: String,
    pub token_value_invalid: String,
    pub total_nodes_limit_exceeded: String,
    pub trailing_flow_comma: String,
    pub trailing_flow_content: String,
    pub unexpected_indentation: String,
    pub unexpected_token: String,
    pub unterminated_flow_mapping: String,
    pub unterminated_quoted_scalar: String,
    pub version_unsupported: String,
}
impl PartialEq for FlightDocumentRefusalReasonValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static FLIGHT_DOCUMENT_REFUSAL_REASON: std::sync::LazyLock<FlightDocumentRefusalReasonValues> =
    std::sync::LazyLock::new(|| FlightDocumentRefusalReasonValues {
        __flight_identity: std::sync::Arc::new(()),
        alias_unsupported: "flight-document.unsupported.alias".to_owned(),
        anchor_unsupported: "flight-document.unsupported.anchor".to_owned(),
        block_scalar_unsupported: "flight-document.unsupported.block-scalar".to_owned(),
        collection_entries_limit_exceeded: "flight-document.limit.collection-entries".to_owned(),
        default_scene_out_of_range: "flight-document.structure.default-scene-out-of-range"
            .to_owned(),
        document_code_units_limit_exceeded: "flight-document.limit.document-code-units".to_owned(),
        document_separator_unsupported: "flight-document.unsupported.document-separator".to_owned(),
        duplicate_ambient_light: "flight-document.structure.duplicate-ambient-light".to_owned(),
        duplicate_directional_light: "flight-document.structure.duplicate-directional-light"
            .to_owned(),
        duplicate_key: "flight-document.syntax.duplicate-key".to_owned(),
        expected_flow_delimiter: "flight-document.syntax.expected-flow-delimiter".to_owned(),
        expected_mapping_entry: "flight-document.syntax.expected-mapping-entry".to_owned(),
        expected_mapping_key: "flight-document.syntax.expected-mapping-key".to_owned(),
        expected_scalar: "flight-document.syntax.expected-scalar".to_owned(),
        expected_value: "flight-document.syntax.expected-value".to_owned(),
        field_invalid: "flight-document.field.invalid".to_owned(),
        flow_sequence_unsupported: "flight-document.unsupported.flow-sequence".to_owned(),
        invalid_document: "flight-document.syntax.invalid-document".to_owned(),
        invalid_escape: "flight-document.syntax.invalid-escape".to_owned(),
        interactive_state_extension_kind_duplicate:
            "flight-document.interactive-state.extension-kind.duplicate".to_owned(),
        interactive_state_extension_kind_unregistered:
            "flight-document.interactive-state.extension-kind.unregistered".to_owned(),
        interactive_state_target_unsupported:
            "flight-document.interactive-state.target.unsupported".to_owned(),
        interactive_state_transition_kind_unregistered:
            "flight-document.interactive-state.transition-kind.unregistered".to_owned(),
        key_code_units_limit_exceeded: "flight-document.limit.key-code-units".to_owned(),
        layout_target_ambiguous: "flight-document.layout-target.ambiguous".to_owned(),
        layout_target_unresolved: "flight-document.layout-target.unresolved".to_owned(),
        mixed_collection: "flight-document.syntax.mixed-collection".to_owned(),
        multiple_root_values: "flight-document.syntax.multiple-root-values".to_owned(),
        nesting_depth_limit_exceeded: "flight-document.limit.nesting-depth".to_owned(),
        node_kind_unregistered: "flight-document.node-kind.unregistered".to_owned(),
        number_out_of_range: "flight-document.scalar.number-out-of-range".to_owned(),
        resource_kind_unregistered: "flight-document.resource-kind.unregistered".to_owned(),
        resource_resolver_unregistered: "flight-document.resource-resolver.unregistered".to_owned(),
        resource_unresolved: "flight-document.resource.unresolved".to_owned(),
        root_indentation: "flight-document.syntax.root-indentation".to_owned(),
        root_kind_mismatch: "flight-document.structure.root-kind-mismatch".to_owned(),
        scalar_code_units_limit_exceeded: "flight-document.limit.scalar-code-units".to_owned(),
        scalar_invalid: "flight-document.scalar.invalid".to_owned(),
        shape_command_unregistered: "flight-document.shape-command.unregistered".to_owned(),
        scenes_empty: "flight-document.structure.scenes-empty".to_owned(),
        structure_invalid: "flight-document.structure.invalid".to_owned(),
        tab_character: "flight-document.syntax.tab-character".to_owned(),
        tag_unsupported: "flight-document.unsupported.tag".to_owned(),
        token_key_duplicate: "flight-document.token.key-duplicate".to_owned(),
        token_key_invalid: "flight-document.token.key-invalid".to_owned(),
        token_kind_mismatch: "flight-document.token.kind-mismatch".to_owned(),
        token_mode_invalid: "flight-document.token.mode-invalid".to_owned(),
        token_mode_unresolved: "flight-document.token.mode-unresolved".to_owned(),
        token_reference_cycle: "flight-document.token.reference-cycle".to_owned(),
        token_reference_invalid: "flight-document.token.reference-invalid".to_owned(),
        token_resolver_unregistered: "flight-document.token-resolver.unregistered".to_owned(),
        token_unresolved: "flight-document.token.unresolved".to_owned(),
        token_value_invalid: "flight-document.token.value-invalid".to_owned(),
        total_nodes_limit_exceeded: "flight-document.limit.total-nodes".to_owned(),
        trailing_flow_comma: "flight-document.syntax.trailing-flow-comma".to_owned(),
        trailing_flow_content: "flight-document.syntax.trailing-flow-content".to_owned(),
        unexpected_indentation: "flight-document.syntax.unexpected-indentation".to_owned(),
        unexpected_token: "flight-document.syntax.unexpected-token".to_owned(),
        unterminated_flow_mapping: "flight-document.syntax.unterminated-flow-mapping".to_owned(),
        unterminated_quoted_scalar: "flight-document.syntax.unterminated-quoted-scalar".to_owned(),
        version_unsupported: "flight-document.version.unsupported".to_owned(),
    });

// Source: upstream/packages/types/src/FlightDocumentRefusalExplanation.ts:70 (sha256:8a8a7c69885594a23976fc9fd4912be93cd692059dd27477f6021ecef0679534)
pub type FlightDocumentRefusalReason = String;

// Source: upstream/packages/types/src/FlightDocumentRefusalExplanation.ts:75 (sha256:cd092154e7835448bfe83036201211fe8449b1eefa42be23113e899614a17c3d)
#[derive(Clone, Default)]
pub struct FlightDocumentRefusalExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub actual: Option<f64>,
    pub column: Option<f64>,
    pub kind: Option<Kind>,
    pub limit: Option<f64>,
    pub line: Option<f64>,
    pub mode: Option<String>,
    pub offset: Option<f64>,
    pub path: String,
    pub reason: FlightDocumentRefusalReason,
    pub resource_key: Option<String>,
    pub token_key: Option<String>,
    pub version: Option<f64>,
}
impl PartialEq for FlightDocumentRefusalExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for FlightDocumentRefusalExplanation {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}
