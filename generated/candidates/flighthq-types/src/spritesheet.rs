// @generated from upstream/packages/types/src/Spritesheet.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, SpritesheetAnimation, SpritesheetFrame, TextureAtlas};

// Source: upstream/packages/types/src/Spritesheet.ts:6 (sha256:da9195606c447ce0db19f20527419fc6269a289464dc737abd92885efe9e69d3)
#[derive(Clone, Default)]
pub struct Spritesheet {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub atlas: Option<TextureAtlas>,
    pub animations: Vec<(String, SpritesheetAnimation)>,
    pub frames: Vec<SpritesheetFrame>,
}
impl PartialEq for Spritesheet {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Spritesheet {
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
