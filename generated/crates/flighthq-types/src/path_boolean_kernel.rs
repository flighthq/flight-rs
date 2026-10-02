// @generated from upstream/packages/types/src/PathBooleanKernel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, PathBooleanContour, PathBooleanFillRule, PathBooleanOperation};

// Source: upstream/packages/types/src/PathBooleanKernel.ts:16 (sha256:f0140171684406d0ffbf6b1e4ba89741f732dc06ebd9d418fa71d882c2589092)
#[derive(Clone)]
pub struct PathBooleanKernel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub compute_path_boolean: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Vec<PathBooleanContour>,
                        Vec<PathBooleanContour>,
                        PathBooleanOperation,
                        PathBooleanFillRule,
                    ) -> Vec<PathBooleanContour>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for PathBooleanKernel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for PathBooleanKernel {
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
