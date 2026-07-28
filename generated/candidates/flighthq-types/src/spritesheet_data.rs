// @generated from upstream/packages/types/src/SpritesheetData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{SpritesheetAnimationData, SpritesheetFrameData};

// Source: upstream/packages/types/src/SpritesheetData.ts:4 (sha256:f78005b7c0e199cd815a4d375ae51320d35863adf25ef1fc9ed4dbe445066bfe)
#[derive(Clone, Default)]
pub struct SpritesheetData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animations: Vec<SpritesheetAnimationData>,
    pub frames: Vec<SpritesheetFrameData>,
    pub image_file: String,
    pub image_height: f64,
    pub image_width: f64,
    pub scale: f64,
}
impl PartialEq for SpritesheetData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
