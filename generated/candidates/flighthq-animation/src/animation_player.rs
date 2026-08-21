// @generated from upstream/packages/animation/src/animationPlayer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    ANIMATION_LOOP_MODE_PING_PONG as animation_loop_mode_ping_pong_constant,
    ANIMATION_LOOP_MODE_REPEAT as animation_loop_mode_repeat_constant, AnimationClip,
    AnimationLoopMode, AnimationPlayer,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub loop_: Option<bool>,
    pub loop_mode: Option<AnimationLoopMode>,
    pub playing: Option<bool>,
    pub repeat_count: Option<f64>,
    pub speed: Option<f64>,
    pub time: Option<f64>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/animation/src/animationPlayer.ts:13 (sha256:068807f0b87fda889e5874746e6af42937aea15a3f411104998a30af45838fb5)
pub fn advance_animation_player(player: &mut AnimationPlayer, dt: f64) -> () {
    if (!player.playing) {
        return;
    }
    let duration = player.clip.duration;
    if (duration <= 0.0_f64) {
        player.time = 0.0_f64;
        return;
    }
    let mut from_time = player.time;
    let mut time = (player.time + (dt * player.speed));
    if (!player.loop_) {
        if (time >= duration) {
            emit_animation_player_events(player, from_time, duration, None);
            player.time = duration;
            player.playing = false;
            emit_animation_player_finished(player);
        } else {
            if (time < 0.0_f64) {
                emit_animation_player_events(player, from_time, 0.0_f64, None);
                player.time = 0.0_f64;
                player.playing = false;
                emit_animation_player_finished(player);
            } else {
                emit_animation_player_events(player, from_time, time, None);
                player.time = time;
            }
        }
        return;
    }
    let mut looped = false;
    let mut segment_start = from_time;
    let mut include_segment_start = false;
    if ((player.loop_mode).clone() == animation_loop_mode_ping_pong_constant) {
        {
            while true {
                if (time > duration) {
                    emit_animation_player_events(
                        player,
                        segment_start,
                        duration,
                        Some(include_segment_start),
                    );
                    if (!consume_animation_player_loop(player)) {
                        finish_animation_player_at(player, duration);
                        return;
                    }
                    time = ((2.0_f64 * duration) - time);
                    player.speed = (-player.speed);
                    segment_start = duration;
                    include_segment_start = false;
                    looped = true;
                } else {
                    if (time < 0.0_f64) {
                        emit_animation_player_events(
                            player,
                            segment_start,
                            0.0_f64,
                            Some(include_segment_start),
                        );
                        if (!consume_animation_player_loop(player)) {
                            finish_animation_player_at(player, 0.0_f64);
                            return;
                        }
                        time = (-time);
                        player.speed = (-player.speed);
                        segment_start = 0.0_f64;
                        include_segment_start = false;
                        looped = true;
                    } else {
                        break;
                    }
                }
            }
        }
        emit_animation_player_events(player, segment_start, time, Some(include_segment_start));
    } else {
        while (time >= duration) {
            emit_animation_player_events(
                player,
                segment_start,
                duration,
                Some(include_segment_start),
            );
            if (!consume_animation_player_loop(player)) {
                finish_animation_player_at(player, duration);
                return;
            }
            time -= duration;
            segment_start = 0.0_f64;
            include_segment_start = true;
            looped = true;
        }
        while (time < 0.0_f64) {
            emit_animation_player_events(
                player,
                segment_start,
                0.0_f64,
                Some(include_segment_start),
            );
            if (!consume_animation_player_loop(player)) {
                finish_animation_player_at(player, 0.0_f64);
                return;
            }
            time += duration;
            segment_start = duration;
            include_segment_start = true;
            looped = true;
        }
        emit_animation_player_events(player, segment_start, time, Some(include_segment_start));
    }
    player.time = time;
    if looped {
        emit_animation_player_looped(player);
    }
}

// Source: upstream/packages/animation/src/animationPlayer.ts:108 (sha256:bc819ce81de8e437940e79c0320db4c750c9e592e16c17bd57f1db615ed0d8d3)
pub fn clone_animation_player(player: &AnimationPlayer) -> AnimationPlayer {
    return create_entity(Some(AnimationPlayer {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        clip: (player.clip).clone(),
        loop_: player.loop_,
        loop_mode: Some(((player.loop_mode).clone()).unwrap()),
        on_event: None,
        on_finished: None,
        on_looped: None,
        playing: player.playing,
        repeat_count: Some((player.repeat_count).unwrap()),
        speed: player.speed,
        time: player.time,
    }));
}

// Source: upstream/packages/animation/src/animationPlayer.ts:125 (sha256:012548ff3cf28da8d4957635a575759ed4a0e101c9e4e8f6606e0148c0194988)
pub fn create_animation_player(
    clip: &AnimationClip,
    opts: Option<SharedStructuralRecord1>,
) -> AnimationPlayer {
    return create_entity(Some(AnimationPlayer {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        clip: (*clip).clone(),
        loop_: (opts.as_ref().and_then(|value| value.loop_))
            .clone()
            .unwrap_or(true),
        loop_mode: Some(
            (opts.as_ref().and_then(|value| (value.loop_mode).clone()))
                .clone()
                .unwrap_or((animation_loop_mode_repeat_constant).to_owned()),
        ),
        on_event: None,
        on_finished: None,
        on_looped: None,
        playing: (opts.as_ref().and_then(|value| value.playing))
            .clone()
            .unwrap_or(true),
        repeat_count: Some(
            (opts.as_ref().and_then(|value| value.repeat_count))
                .clone()
                .unwrap_or((-1.0_f64)),
        ),
        speed: (opts.as_ref().and_then(|value| value.speed))
            .clone()
            .unwrap_or(1.0_f64),
        time: (opts.as_ref().and_then(|value| value.time))
            .clone()
            .unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/animation/src/animationPlayer.ts:152 (sha256:9147df62ed112dfd7adb9d4f487360108de2bc508064c9a4f70cefcf207d99e9)
pub fn enable_animation_player_signals(player: &mut AnimationPlayer) -> () {
    if ((player.on_event).clone()).is_none() {
        player.on_event = Some(create_signal());
    }
    if ((player.on_finished).clone()).is_none() {
        player.on_finished = Some(create_signal());
    }
    if ((player.on_looped).clone()).is_none() {
        player.on_looped = Some(create_signal());
    }
}

// Source: upstream/packages/animation/src/animationPlayer.ts:160 (sha256:8ba6c58c4189d541d23e7d3b8f9e809ea102534e19de0b73febe0af4c1e2899a)
pub fn get_animation_player_normalized_time(player: &AnimationPlayer) -> f64 {
    let duration = player.clip.duration;
    if (duration <= 0.0_f64) {
        return 0.0_f64;
    }
    let n = (player.time / duration);
    return if (n < 0.0_f64) {
        0.0_f64
    } else {
        if (n > 1.0_f64) { 1.0_f64 } else { n }
    };
}

// Source: upstream/packages/animation/src/animationPlayer.ts:169 (sha256:f3e6734412fae65da5587ec4af26dbf04235a07d4c08fe11727d84f5c7c37d09)
pub fn play_animation_player(player: &mut AnimationPlayer) -> () {
    player.playing = true;
}

// Source: upstream/packages/animation/src/animationPlayer.ts:174 (sha256:59df47baa8ed7a09c3254865530f360662434ec0d6250f0ab2469e04ec823769)
pub fn seek_animation_player(player: &mut AnimationPlayer, time: f64) -> () {
    let duration = player.clip.duration;
    player.time = if (time < 0.0_f64) {
        0.0_f64
    } else {
        if (time > duration) { duration } else { time }
    };
}

// Source: upstream/packages/animation/src/animationPlayer.ts:181 (sha256:50846cf19c1abc4aa4160d076ab3089e57774314cf8a2458fa90d4e3d3f41661)
pub fn stop_animation_player(player: &mut AnimationPlayer) -> () {
    player.playing = false;
    player.time = 0.0_f64;
}

// Source: upstream/packages/animation/src/animationPlayer.ts:189 (sha256:3bb7406c624dfcc7fe82466034bddb751f041fed55e5dbd6853b7e6177c13f9b)
fn consume_animation_player_loop(player: &mut AnimationPlayer) -> bool {
    let rc = player.repeat_count;
    if ((rc).is_none()) || ((rc).as_ref().is_some_and(|value| *value < 0.0_f64)) {
        return true;
    }
    if (*(rc.as_ref().unwrap()) == 0.0_f64) {
        return false;
    }
    player.repeat_count = Some((*(rc.as_ref().unwrap()) - 1.0_f64));
    return true;
}

// Source: upstream/packages/animation/src/animationPlayer.ts:197 (sha256:67590c69badc037f03f12d36f960ddbdebad656e8e4722a4932d4c59e1197aad)
fn emit_animation_player_finished(player: &AnimationPlayer) -> () {
    if ((player.on_finished).clone()).is_some() {
        emit_signal(((player.on_finished).clone()).unwrap(), ());
    }
}

// Source: upstream/packages/animation/src/animationPlayer.ts:201 (sha256:58036b2d935bc51e2c0ee68a7c6cf626984844a602a96aa9dfdb27180422c7c4)
fn emit_animation_player_events(
    player: &AnimationPlayer,
    from_time: f64,
    to_time: f64,
    include_from: Option<bool>,
) -> () {
    let include_from = include_from.unwrap_or(false);
    let signal = (player.on_event).clone();
    if ((signal).is_none()) || ((player.clip.events.len() as f64) == 0.0_f64) {
        return;
    }
    if (to_time > from_time) {
        for event in (player.clip.events).iter().cloned() {
            if (event.time > to_time) {
                break;
            }
            if (event.time > from_time) || ((include_from) && (event.time == from_time)) {
                emit_signal((signal.as_ref().unwrap()).clone(), ((event).clone(),));
            }
        }
        return;
    }
    if (to_time < from_time) {
        {
            let mut index = ((player.clip.events.len() as f64) - 1.0_f64);
            while (index >= 0.0_f64) {
                let event = player.clip.events[index as usize].clone();
                if (event.time < to_time) {
                    break;
                }
                if (event.time < from_time) || ((include_from) && (event.time == from_time)) {
                    emit_signal((signal.as_ref().unwrap()).clone(), ((event).clone(),));
                }
                {
                    index -= 1.0;
                    index
                };
            }
        }
        return;
    }
    if (!include_from) {
        return;
    }
    for event in (player.clip.events).iter().cloned() {
        if (event.time == from_time) {
            emit_signal((signal.as_ref().unwrap()).clone(), ((event).clone(),));
        } else {
            if (event.time > from_time) {
                break;
            }
        }
    }
}

// Source: upstream/packages/animation/src/animationPlayer.ts:232 (sha256:5d3aefd9c1350166cf940e68128532741956a9593a5f163c33f0c2959cfb18cf)
fn emit_animation_player_looped(player: &AnimationPlayer) -> () {
    if ((player.on_looped).clone()).is_some() {
        emit_signal(((player.on_looped).clone()).unwrap(), ());
    }
}

// Source: upstream/packages/animation/src/animationPlayer.ts:238 (sha256:800b8395d8580c82855afcc05fb8b6c888a426f77a788e7f5a698cbdcff9ca7b)
fn finish_animation_player_at(player: &mut AnimationPlayer, time: f64) -> () {
    player.time = time;
    player.playing = false;
    emit_animation_player_finished(player);
}
