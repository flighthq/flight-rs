// @generated from upstream/packages/types/src/HasColorScaleBias.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ColorScaleBias, EntityRuntime, Kind, NodeData};

// Source: upstream/packages/types/src/HasColorScaleBias.ts:11 (sha256:c6777b441c6b854a01dbbedf93354fa585947bbb05d6adb734ab98a84d2dec41)
#[derive(Clone, Default)]
pub struct HasColorScaleBias {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color_scale_bias: Option<ColorScaleBias>,
}
impl PartialEq for HasColorScaleBias {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HasColorScaleBias.ts:15 (sha256:c5e6a9258fbb6a3cd8c6386936d53229385808d99395817f572e7cd9b6a853d3)
#[derive(Clone, Default)]
pub struct ColorScaleBiasNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub color_scale_bias: Option<ColorScaleBias>,
}
impl PartialEq for ColorScaleBiasNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ColorScaleBiasNode {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
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
