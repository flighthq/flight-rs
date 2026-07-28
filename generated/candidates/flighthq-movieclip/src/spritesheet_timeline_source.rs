// @generated from upstream/packages/movieclip/src/spritesheetTimelineSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_displayobject::create_bitmap;
use flighthq_node::{add_node_child, invalidate_node_local_transform};
use flighthq_types::{Bitmap, DisplayObject, Spritesheet, SpritesheetAnimation, TimelineSource};

// Source: upstream/packages/movieclip/src/spritesheetTimelineSource.ts:13 (sha256:9c4cfc800378539a63f7d938ab1ceb37baefbb92a57424400f6d04f19fa74771)
pub fn create_spritesheet_timeline_source(
    spritesheet: Spritesheet,
    animation: SpritesheetAnimation,
) -> TimelineSource {
    let bitmaps: std::sync::Arc<std::sync::Mutex<Vec<(DisplayObject, Bitmap)>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    return TimelineSource {
        __flight_identity: std::sync::Arc::new(()),
        total_frames: (animation.frames.len() as f64),
        labels: vec![],
        frame_rate: (1000.0_f64 / animation.frame_duration),
        construct_frame: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let animation = animation.clone();
            let mut bitmaps = bitmaps.clone();
            let spritesheet = spritesheet.clone();
            move |target: DisplayObject, frame: f64| -> () {
                let atlas = (spritesheet.atlas).clone();
                if (atlas).is_none() {
                    return;
                }
                let mut bitmap = (*bitmaps.lock().unwrap())
                    .iter()
                    .find(|(key, _)| key == &(target).clone())
                    .map(|(_, value)| value.clone());
                if (bitmap).is_none() {
                    bitmap = Some(create_bitmap(None));
                    bitmap.as_mut().unwrap().data.image = (atlas.as_ref().unwrap().image).clone();
                    add_node_child(&target, &bitmap);
                    {
                        let __flight_key = (target).clone();
                        let __flight_value = (bitmap).clone().unwrap();
                        if let Some((_, value)) = (*bitmaps.lock().unwrap())
                            .iter_mut()
                            .find(|(key, _)| key == &__flight_key)
                        {
                            *value = __flight_value;
                        } else {
                            (*bitmaps.lock().unwrap()).push((__flight_key, __flight_value));
                        }
                    };
                }
                let sheet_frame = spritesheet.frames
                    [animation.frames[(frame - 1.0_f64) as usize].clone() as usize]
                    .clone();
                if (sheet_frame).is_none() {
                    return;
                }
                bitmap.as_mut().unwrap().data.source_rectangle =
                    Some(atlas.as_ref().unwrap().regions[sheet_frame.id as usize].clone());
                bitmap.as_mut().unwrap().x = (sheet_frame.offset_x - animation.origin_x);
                bitmap.as_mut().unwrap().y = (sheet_frame.offset_y - animation.origin_y);
                invalidate_node_local_transform(&bitmap);
            }
        })
            as Box<dyn FnMut(DisplayObject, f64) -> () + Send + 'static>)),
    };
}
