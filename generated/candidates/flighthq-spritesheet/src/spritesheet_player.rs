// @generated from upstream/packages/spritesheet/src/spritesheetPlayer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{clear_signal, create_signal, emit_signal};
use flighthq_types::{Spritesheet, SpritesheetAnimation, SpritesheetFrame, SpritesheetPlayer};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:4 (sha256:e66ce9c760beb40f555bed7ad4b6d99d8157b80986f3d23f1c510c866c8d8779)
pub fn acquire_spritesheet_player() -> SpritesheetPlayer {
    if ((PLAYER_POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        let mut p = PLAYER_POOL
            .lock()
            .unwrap()
            .pop()
            .expect("TypeScript Array.pop returned undefined");
        p.as_mut().unwrap().animation = None;
        p.as_mut().unwrap().complete = true;
        p.as_mut().unwrap().elapsed = 0.0_f64;
        p.as_mut().unwrap().frame_index = 0.0_f64;
        p.as_mut().unwrap().paused = false;
        p.as_mut().unwrap().queue.clear();
        p.as_mut().unwrap().speed = 1.0_f64;
        return (p).clone().unwrap();
    }
    return create_spritesheet_player(None);
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:20 (sha256:191bfc8c641e4858140c4910fb3168c72a8bfa39034c8638cb2189162d0ca061)
pub fn clone_spritesheet_player(player: &SpritesheetPlayer) -> SpritesheetPlayer {
    return SpritesheetPlayer {
        __flight_identity: std::sync::Arc::new(()),
        animation: (player.animation).clone(),
        complete: player.complete,
        elapsed: player.elapsed,
        frame_index: player.frame_index,
        on_complete: create_signal(),
        on_loop: create_signal(),
        paused: player.paused,
        queue: {
            let mut __flight_array = Vec::new();
            __flight_array.extend(((player.queue).clone()).iter().cloned());
            __flight_array
        },
        speed: player.speed,
    };
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:34 (sha256:367882272258fe1ac565e85e6cf5f8623fa9c3b7d64c5c998a686eadea8994ef)
pub fn create_spritesheet_player(obj: Option<SpritesheetPlayer>) -> SpritesheetPlayer {
    return SpritesheetPlayer {
        __flight_identity: std::sync::Arc::new(()),
        animation: obj.as_ref().and_then(|value| (value.animation).clone()),
        complete: (obj.as_ref().map(|value| value.complete)).unwrap_or(true),
        elapsed: (obj.as_ref().map(|value| value.elapsed)).unwrap_or(0.0_f64),
        frame_index: (obj.as_ref().map(|value| value.frame_index)).unwrap_or(0.0_f64),
        on_complete: (obj.as_ref().map(|value| (value.on_complete).clone()))
            .unwrap_or(create_signal()),
        on_loop: (obj.as_ref().map(|value| (value.on_loop).clone())).unwrap_or(create_signal()),
        paused: (obj.as_ref().map(|value| value.paused)).unwrap_or(false),
        queue: (obj.as_ref().map(|value| (value.queue).clone())).unwrap_or(vec![]),
        speed: (obj.as_ref().map(|value| value.speed)).unwrap_or(1.0_f64),
    };
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:48 (sha256:21057375ba25d677ec22ca9637db2bd70b6bbbcdd1f2ad816ecbc5ae7629cfc9)
pub fn dispose_spritesheet_player(player: &mut SpritesheetPlayer) -> () {
    clear_signal(&mut player.on_complete);
    clear_signal(&mut player.on_loop);
    player.animation = None;
    player.complete = true;
    player.queue.clear();
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:56 (sha256:924de533b474b3a692b914c65d06ee98842dc0723a420e2372d3dfc9e3dec913)
pub fn get_spritesheet_player_frame(
    player: &SpritesheetPlayer,
    spritesheet: &Spritesheet,
) -> Option<SpritesheetFrame> {
    let animation = (player.animation).clone();
    let frame_index = player.frame_index;
    if ((animation).is_none()) || ((animation.as_ref().unwrap().frames.len() as f64) == 0.0_f64) {
        return None;
    }
    let sprite_frame_index = animation.as_ref().unwrap().frames[frame_index as usize].clone();
    return Some(spritesheet.frames[sprite_frame_index as usize].clone());
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:71 (sha256:4afa5c1327d9da3346f6a9c76b49155a3fb8bbdb1522e90f9ec93ac27fbcd086)
pub fn get_spritesheet_player_frame_at(
    player: &SpritesheetPlayer,
    spritesheet: &Spritesheet,
    frame_offset: f64,
) -> Option<SpritesheetFrame> {
    let animation = (player.animation).clone();
    let frame_index = player.frame_index;
    if ((animation).is_none()) || ((animation.as_ref().unwrap().frames.len() as f64) == 0.0_f64) {
        return None;
    }
    let n = (animation.as_ref().unwrap().frames.len() as f64);
    let target_index = ((((frame_index + frame_offset) % n) + n) % n);
    let sprite_frame_index = animation.as_ref().unwrap().frames[target_index as usize].clone();
    return Some(spritesheet.frames[sprite_frame_index as usize].clone());
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:85 (sha256:7f9a9b39306e1d67c6001918ed4ab4a3e5a4019e62e6f455cf430443bb038888)
pub fn pause_spritesheet_player(player: &mut SpritesheetPlayer) -> () {
    player.paused = true;
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:89 (sha256:418f1ba0e5f5f8523198c38082aa80da56ffb6bcf363b0db777849ed92b2d4ad)
pub fn play_spritesheet_animation(
    player: &mut SpritesheetPlayer,
    animation: Option<SpritesheetAnimation>,
    restart: Option<bool>,
) -> () {
    let restart = restart.unwrap_or(true);
    if (!restart) && (animation == (player.animation).clone()) {
        return;
    }
    player.animation = (animation).clone();
    player.complete = (animation).is_none();
    player.elapsed = 0.0_f64;
    player.frame_index = 0.0_f64;
    player.queue.clear();
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:102 (sha256:97b719c03dcf78c1d0cbb62c56e4acf1b214c95fc4663a67be5b4e9f67577ded)
pub fn queue_spritesheet_animation(
    player: &mut SpritesheetPlayer,
    animation: &SpritesheetAnimation,
) -> () {
    player.queue.push(((*animation).clone()).clone());
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:109 (sha256:72f908bdc82f3a6752d0468102c3124bfef57eba81ab1f9bd7a339275682acc8)
pub fn release_spritesheet_player(player: &mut SpritesheetPlayer) -> () {
    clear_signal(&mut player.on_complete);
    clear_signal(&mut player.on_loop);
    player.animation = None;
    player.complete = true;
    player.elapsed = 0.0_f64;
    player.frame_index = 0.0_f64;
    player.paused = false;
    player.queue.clear();
    player.speed = 1.0_f64;
    PLAYER_POOL
        .lock()
        .unwrap()
        .push(((*player).clone()).clone());
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:122 (sha256:c2d68d7c6255aaae2c7348c1b37063c50bcbae94ed352fe44e29c1064f6d1bcb)
pub fn resume_spritesheet_player(player: &mut SpritesheetPlayer) -> () {
    player.paused = false;
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:126 (sha256:f2f6088f5661d5524a1a9bb22e7051b7ef1742edc0fe897ba9f9db0a5843ca1e)
pub fn seek_spritesheet_player_to_frame(player: &mut SpritesheetPlayer, frame_index: f64) -> () {
    let animation = (player.animation).clone();
    if ((animation).is_none()) || ((animation.as_ref().unwrap().frames.len() as f64) == 0.0_f64) {
        return;
    }
    let clamped = (0.0_f64)
        .max((frame_index).min(((animation.as_ref().unwrap().frames.len() as f64) - 1.0_f64)));
    player.frame_index = clamped;
    player.elapsed = resolve_virtual_index_start_time(animation.as_ref().unwrap(), clamped);
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:136 (sha256:ccc956a8d2a9290e2da714a354e13a5ca831edd50d23565edfcace178931fefe)
pub fn seek_spritesheet_player_to_time(player: &mut SpritesheetPlayer, time: f64) -> () {
    let animation = (player.animation).clone();
    if ((animation).is_none()) || ((animation.as_ref().unwrap().frames.len() as f64) == 0.0_f64) {
        return;
    }
    let total_time = resolve_animation_total_time(animation.as_ref().unwrap());
    player.elapsed = (0.0_f64).max((time).min(total_time));
    player.frame_index =
        resolve_frame_index_from_elapsed(animation.as_ref().unwrap(), player.elapsed);
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:144 (sha256:e58eb6d0ffc8d267cfb7dc16871520a81d93cd9262e357e6ae88a286d9f718b4)
pub fn stop_spritesheet_player(player: &mut SpritesheetPlayer) -> () {
    player.elapsed = 0.0_f64;
    player.frame_index = 0.0_f64;
    player.complete = true;
    player.queue.clear();
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:151 (sha256:4570c7b97664a83b03bc39414e3fed9a4562b86753f1a6fb200f12dd055e3000)
pub fn update_spritesheet_player(player: &mut SpritesheetPlayer, delta_time: f64) -> bool {
    let animation = (player.animation).clone();
    if ((((animation).is_none()) || (player.complete)) || (player.paused))
        || ((animation.as_ref().unwrap().frames.len() as f64) == 0.0_f64)
    {
        return false;
    }
    let __destructure5 = animation;
    let loop_ = __destructure5.as_ref().unwrap().loop_;
    let total_time = resolve_animation_total_time(animation.as_ref().unwrap());
    let prev_loop_count = (player.elapsed / total_time).floor();
    player.elapsed += (delta_time * player.speed);
    if (!loop_) && (player.elapsed >= total_time) {
        if ((player.queue.len() as f64) > 0.0_f64) {
            let next = (player.queue.shift)();
            player.animation = Some(next);
            player.elapsed = 0.0_f64;
            player.frame_index = 0.0_f64;
            return true;
        }
        player.elapsed = total_time;
        let last_vi = (resolve_virtual_frame_count(animation.as_ref().unwrap()) - 1.0_f64);
        player.frame_index =
            resolve_virtual_index_to_display_index(animation.as_ref().unwrap(), last_vi);
        player.complete = true;
        emit_signal((player.on_complete).clone(), ());
        return true;
    }
    if ((player.elapsed / total_time).floor() > prev_loop_count) {
        emit_signal((player.on_loop).clone(), ());
    }
    let time_in_loop = (player.elapsed % total_time);
    let vi = resolve_virtual_index_from_time(animation.as_ref().unwrap(), time_in_loop);
    player.frame_index = resolve_virtual_index_to_display_index(animation.as_ref().unwrap(), vi);
    return true;
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:183 (sha256:6d1828450ee9539b7547f927b461a2444ad7adb58cbd79445be7c97228066481)
fn get_cumulative_durations(animation: &SpritesheetAnimation) -> Vec<f64> {
    let cached = (*CUMULATIVE_DURATIONS_CACHE.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*animation).clone())
        .map(|(_, value)| value.clone());
    if (cached).is_some() {
        return cached.as_ref().unwrap();
    }
    let frame_duration = animation.frame_duration;
    let frame_durations = (animation.frame_durations).clone();
    let n = (animation.frames.len() as f64);
    let virtual_count = resolve_virtual_frame_count(animation);
    let mut arr = vec![0.0_f64; (virtual_count + 1.0_f64) as usize];
    let mut t = 0.0_f64;
    {
        let mut vi = 0.0_f64;
        while (vi < virtual_count) {
            arr[vi as usize] = t;
            let fi = if (vi < n) {
                vi
            } else {
                ((2.0_f64 * (n - 1.0_f64)) - vi)
            };
            t += frame_durations.as_ref().unwrap()[fi as usize].clone();
            {
                vi += 1.0;
                vi
            };
        }
    }
    arr[virtual_count as usize] = if (t) != 0.0_f64 { t } else { 1.0_f64 };
    {
        let __flight_key = (*animation).clone();
        let __flight_value = (arr).clone();
        if let Some((_, value)) = (*CUMULATIVE_DURATIONS_CACHE.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*CUMULATIVE_DURATIONS_CACHE.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return arr;
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:204 (sha256:3dfbedac804471a0bfcf4076a98171721b64056c643553365388d4e6c31b78ea)
fn resolve_animation_total_time(animation: &SpritesheetAnimation) -> f64 {
    let frame_duration = animation.frame_duration;
    let frame_durations = (animation.frame_durations).clone();
    if (frame_durations).is_some() {
        let arr = get_cumulative_durations(animation);
        return (arr[((arr.len() as f64) - 1.0_f64) as usize] as f64);
    }
    let virtual_count = resolve_virtual_frame_count(animation);
    return if (virtual_count * frame_duration) != 0.0_f64 {
        (virtual_count * frame_duration)
    } else {
        1.0_f64
    };
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:216 (sha256:586efc36ba013bc396181e2b1fa43675d59d79cb9a7d3636da2320251a06b1c3)
fn resolve_frame_index_from_elapsed(animation: &SpritesheetAnimation, elapsed: f64) -> f64 {
    let total_time = resolve_animation_total_time(animation);
    let time_in_loop = (elapsed % total_time);
    let vi = resolve_virtual_index_from_time(animation, time_in_loop);
    return resolve_virtual_index_to_display_index(animation, vi);
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:225 (sha256:5af18316f0e4425342a0804179ccea8c6b44d3c00788f21ef25125870d4add70)
fn resolve_virtual_frame_count(animation: &SpritesheetAnimation) -> f64 {
    let n = (animation.frames.len() as f64);
    let is_pingpong = ((animation.direction).clone() == "pingpong")
        || ((animation.direction).clone() == "pingpong_reverse");
    if (is_pingpong) && (n > 1.0_f64) {
        return ((2.0_f64 * n) - 2.0_f64);
    }
    return n;
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:234 (sha256:c7035f6a8845978643557eb8f95452c6f761ac513552e6a58a90222f03cea403)
fn resolve_virtual_index_from_time(animation: &SpritesheetAnimation, time_in_loop: f64) -> f64 {
    let frame_duration = animation.frame_duration;
    let frame_durations = (animation.frame_durations).clone();
    let virtual_count = resolve_virtual_frame_count(animation);
    if (frame_durations).is_some() {
        let arr = get_cumulative_durations(animation);
        let mut lo = 0.0_f64;
        let mut hi = (virtual_count - 1.0_f64);
        while (lo < hi) {
            let mid = (__flight_js_to_i32(((lo + hi) + 1.0_f64))
                >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
            if ((arr[mid as usize] as f64) <= time_in_loop) {
                lo = mid;
            } else {
                hi = (mid - 1.0_f64);
            }
        }
        return lo;
    }
    return ((time_in_loop / frame_duration).floor()).min((virtual_count - 1.0_f64));
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:257 (sha256:a9be8e40102778c51d88aa7eb532e32e5df5252edecffedeefa32a151cafeeed)
fn resolve_virtual_index_start_time(animation: &SpritesheetAnimation, virtual_index: f64) -> f64 {
    let frame_duration = animation.frame_duration;
    let frame_durations = (animation.frame_durations).clone();
    if (frame_durations).is_some() {
        let arr = get_cumulative_durations(animation);
        return (arr[virtual_index as usize] as f64);
    }
    return (virtual_index * frame_duration);
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:268 (sha256:8220a77b748dc9d3bb57add872ccca2976b362536b56cd8e1d616a909a908b05)
fn resolve_virtual_index_to_display_index(
    animation: &SpritesheetAnimation,
    virtual_index: f64,
) -> f64 {
    let direction = (animation.direction).clone();
    let last = ((animation.frames.len() as f64) - 1.0_f64);
    {
        let __switch_value = direction;
        let __flight_case = if __switch_value == "forward" {
            0_usize
        } else if __switch_value == "reverse" {
            1_usize
        } else if __switch_value == "pingpong" {
            2_usize
        } else if __switch_value == "pingpong_reverse" {
            3_usize
        } else {
            4_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return virtual_index;
            }
            if __flight_case <= 1_usize {
                return (last - virtual_index);
            }
            if __flight_case <= 2_usize {
                return if (virtual_index <= last) {
                    virtual_index
                } else {
                    ((2.0_f64 * last) - virtual_index)
                };
            }
            if __flight_case <= 3_usize {
                return if (virtual_index <= last) {
                    (last - virtual_index)
                } else {
                    (virtual_index - last)
                };
            }
            if __flight_case <= 4_usize {
                return virtual_index;
            }
        }
    }
}

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:288 (sha256:81b485783a84d6aeb1957d7e8e71063a07ef663d6abe35942e49c4edd77f4b08)
static PLAYER_POOL: std::sync::LazyLock<std::sync::Mutex<Vec<SpritesheetPlayer>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/spritesheet/src/spritesheetPlayer.ts:294 (sha256:89e7d6c15d1c6d28355c2714daca6ccea98cab9e5cf1620d5a77dcfffe517337)
static CUMULATIVE_DURATIONS_CACHE: std::sync::LazyLock<
    std::sync::Mutex<Vec<(SpritesheetAnimation, Vec<f64>)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
