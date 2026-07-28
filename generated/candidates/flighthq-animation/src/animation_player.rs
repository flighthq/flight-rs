// @generated from upstream/packages/animation/src/animationPlayer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

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

// Source: upstream/packages/animation/src/animationPlayer.ts:12 (sha256:4150ce1d6253810aa5779d68f5ee0166746d6b28755ce6a350244b24b982fb0f)
pub fn advance_animation_player(player: &mut AnimationPlayer, dt: f64) -> () {
    if (!player.playing) {
        return;
    }
    let duration = player.clip.duration;
    if (duration <= 0.0_f64) {
        player.time = 0.0_f64;
        return;
    }
    let mut time = (player.time + (dt * player.speed));
    if (!player.loop_) {
        if (time >= duration) {
            player.time = duration;
            player.playing = false;
            emit_animation_player_finished(player);
        } else {
            if (time < 0.0_f64) {
                player.time = 0.0_f64;
                player.playing = false;
                emit_animation_player_finished(player);
            } else {
                player.time = time;
            }
        }
        return;
    }
    let mut looped = false;
    if ((player.loop_mode).clone() == animation_loop_mode_ping_pong_constant) {
        {
            while true {
                if (time > duration) {
                    if (!consume_animation_player_loop(player)) {
                        finish_animation_player_at(player, duration);
                        return;
                    }
                    time = ((2.0_f64 * duration) - time);
                    player.speed = (-player.speed);
                    looped = true;
                } else {
                    if (time < 0.0_f64) {
                        if (!consume_animation_player_loop(player)) {
                            finish_animation_player_at(player, 0.0_f64);
                            return;
                        }
                        time = (-time);
                        player.speed = (-player.speed);
                        looped = true;
                    } else {
                        break;
                    }
                }
            }
        }
    } else {
        while (time >= duration) {
            if (!consume_animation_player_loop(player)) {
                finish_animation_player_at(player, duration);
                return;
            }
            time -= duration;
            looped = true;
        }
        while (time < 0.0_f64) {
            if (!consume_animation_player_loop(player)) {
                finish_animation_player_at(player, 0.0_f64);
                return;
            }
            time += duration;
            looped = true;
        }
    }
    player.time = time;
    if looped {
        emit_animation_player_looped(player);
    }
}

// Source: upstream/packages/animation/src/animationPlayer.ts:87 (sha256:913daf8a27b8f73286c861a1cf7051147693519d7c45cf67ed05f72e5c7997d0)
pub fn clone_animation_player(player: &AnimationPlayer) -> AnimationPlayer {
    return AnimationPlayer {
        __flight_identity: std::sync::Arc::new(()),
        clip: (player.clip).clone(),
        loop_: player.loop_,
        loop_mode: Some(((player.loop_mode).clone()).unwrap()),
        on_finished: None,
        on_looped: None,
        playing: player.playing,
        repeat_count: Some((player.repeat_count).unwrap()),
        speed: player.speed,
        time: player.time,
    };
}

// Source: upstream/packages/animation/src/animationPlayer.ts:103 (sha256:69dd43b62da744fdb1b8fc26718e136b73b7cddd43de6ab4b64879c358724a6f)
pub fn create_animation_player(
    clip: &AnimationClip,
    opts: Option<SharedStructuralRecord1>,
) -> AnimationPlayer {
    return AnimationPlayer {
        __flight_identity: std::sync::Arc::new(()),
        clip: (*clip).clone(),
        loop_: (opts.as_ref().and_then(|value| value.loop_)).unwrap_or(true),
        loop_mode: Some(
            (opts.as_ref().and_then(|value| (value.loop_mode).clone()))
                .unwrap_or((animation_loop_mode_repeat_constant).to_owned()),
        ),
        on_finished: None,
        on_looped: None,
        playing: (opts.as_ref().and_then(|value| value.playing)).unwrap_or(true),
        repeat_count: Some(
            (opts.as_ref().and_then(|value| value.repeat_count)).unwrap_or((-1.0_f64)),
        ),
        speed: (opts.as_ref().and_then(|value| value.speed)).unwrap_or(1.0_f64),
        time: (opts.as_ref().and_then(|value| value.time)).unwrap_or(0.0_f64),
    };
}

// Source: upstream/packages/animation/src/animationPlayer.ts:129 (sha256:e8c36ebe837583b41c89f0729eea61deb6e324a6b0e53056fdc0cb506046c221)
pub fn enable_animation_player_signals(player: &mut AnimationPlayer) -> () {
    if ((player.on_finished).clone()).is_none() {
        player.on_finished = Some(create_signal());
    }
    if ((player.on_looped).clone()).is_none() {
        player.on_looped = Some(create_signal());
    }
}

// Source: upstream/packages/animation/src/animationPlayer.ts:136 (sha256:8ba6c58c4189d541d23e7d3b8f9e809ea102534e19de0b73febe0af4c1e2899a)
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

// Source: upstream/packages/animation/src/animationPlayer.ts:145 (sha256:f3e6734412fae65da5587ec4af26dbf04235a07d4c08fe11727d84f5c7c37d09)
pub fn play_animation_player(player: &mut AnimationPlayer) -> () {
    player.playing = true;
}

// Source: upstream/packages/animation/src/animationPlayer.ts:150 (sha256:59df47baa8ed7a09c3254865530f360662434ec0d6250f0ab2469e04ec823769)
pub fn seek_animation_player(player: &mut AnimationPlayer, time: f64) -> () {
    let duration = player.clip.duration;
    player.time = if (time < 0.0_f64) {
        0.0_f64
    } else {
        if (time > duration) { duration } else { time }
    };
}

// Source: upstream/packages/animation/src/animationPlayer.ts:157 (sha256:50846cf19c1abc4aa4160d076ab3089e57774314cf8a2458fa90d4e3d3f41661)
pub fn stop_animation_player(player: &mut AnimationPlayer) -> () {
    player.playing = false;
    player.time = 0.0_f64;
}

// Source: upstream/packages/animation/src/animationPlayer.ts:165 (sha256:3bb7406c624dfcc7fe82466034bddb751f041fed55e5dbd6853b7e6177c13f9b)
fn consume_animation_player_loop(player: &mut AnimationPlayer) -> bool {
    let rc = player.repeat_count;
    if ((rc).is_none()) || ((rc).as_ref().is_some_and(|value| *value < 0.0_f64)) {
        return true;
    }
    if (rc) == Some(0.0_f64) {
        return false;
    }
    player.repeat_count = (rc - 1.0_f64);
    return true;
}

// Source: upstream/packages/animation/src/animationPlayer.ts:173 (sha256:67590c69badc037f03f12d36f960ddbdebad656e8e4722a4932d4c59e1197aad)
fn emit_animation_player_finished(player: &AnimationPlayer) -> () {
    if ((player.on_finished).clone()).is_some() {
        emit_signal(((player.on_finished).clone()).unwrap(), ());
    }
}

// Source: upstream/packages/animation/src/animationPlayer.ts:177 (sha256:5d3aefd9c1350166cf940e68128532741956a9593a5f163c33f0c2959cfb18cf)
fn emit_animation_player_looped(player: &AnimationPlayer) -> () {
    if ((player.on_looped).clone()).is_some() {
        emit_signal(((player.on_looped).clone()).unwrap(), ());
    }
}

// Source: upstream/packages/animation/src/animationPlayer.ts:183 (sha256:800b8395d8580c82855afcc05fb8b6c888a426f77a788e7f5a698cbdcff9ca7b)
fn finish_animation_player_at(player: &mut AnimationPlayer, time: f64) -> () {
    player.time = time;
    player.playing = false;
    emit_animation_player_finished(player);
}
