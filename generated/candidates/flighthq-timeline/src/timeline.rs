// @generated from upstream/packages/timeline/src/timeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    DisplayObject, FrameScript, Timeline, TimelineFrameEvent, TimelineLabel, TimelineSignals,
    TimelineSource,
};

// Source: upstream/packages/timeline/src/timeline.ts:12 (sha256:cc8b05b7dad875e3c989fa675ab387eae59048756634b98790fc0f3b56c51b3d)
pub fn add_timeline_frame_script(
    timeline: &mut Timeline,
    frame: &crate::FlightUnion2<f64, String>,
    script: &mut impl FnMut(DisplayObject, f64) -> (),
) -> () {
    let resolved = resolve_frame(timeline, frame);
    ({
        timeline.frame_scripts?? = Some(Vec::new());
        timeline.frame_scripts
    }
    .set)(resolved, script);
}

// Source: upstream/packages/timeline/src/timeline.ts:17 (sha256:f24f67d158c066578e39768085a54a1194ffcd26260d7c7abd67517dec60b181)
pub fn create_timeline(obj: Option<Timeline>) -> Timeline {
    return Timeline {
        __flight_identity: std::sync::Arc::new(()),
        source: obj.as_ref().and_then(|value| (value.source).clone()),
        target: obj.as_ref().and_then(|value| (value.target).clone()),
        current_frame: (obj.as_ref().map(|value| value.current_frame)).unwrap_or(1.0_f64),
        frame_scripts: obj.as_ref().and_then(|value| (value.frame_scripts).clone()),
        is_playing: (obj.as_ref().map(|value| value.is_playing)).unwrap_or(false),
        last_frame_update: (-1.0_f64),
        play_mode: (obj.as_ref().map(|value| (value.play_mode).clone()))
            .unwrap_or("loop".to_owned()),
        signals: obj.as_ref().and_then(|value| (value.signals).clone()),
        time_elapsed: 0.0_f64,
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:35 (sha256:8c2623b30f531687bfcf86a7365eef7a608418cd1bf797f02bc930a74bdc4fd0)
#[derive(Clone)]
struct CreateTimelineSourceRecord1 {
    __flight_identity: std::sync::Arc<()>,
    total_frames: Option<f64>,
    frame_rate: Option<f64>,
    labels: Option<Vec<TimelineLabel>>,
    construct_frame: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(DisplayObject, f64) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for CreateTimelineSourceRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_timeline_source(obj: &CreateTimelineSourceRecord1) -> TimelineSource {
    return TimelineSource {
        __flight_identity: std::sync::Arc::new(()),
        total_frames: (obj.total_frames).unwrap_or(1.0_f64),
        frame_rate: obj.frame_rate,
        labels: ((obj.labels).clone()).unwrap_or(((*EMPTY_LABELS).clone()).clone()),
        construct_frame: ((obj.construct_frame).clone()).unwrap_or(noop_construct_frame),
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:49 (sha256:6a7bbfee267c23e9066a61f974f52e1fbfa5fd2956b670e8dd42032737bd7241)
pub fn dispose_timeline_signals(timeline: &mut Timeline) -> () {
    timeline.signals = None;
}

// Source: upstream/packages/timeline/src/timeline.ts:55 (sha256:fc4aa3821caba8c92477e8259ceb84765bc939b6b89a22273583b672c2b55078)
pub fn enable_timeline_signals(timeline: &mut Timeline) -> TimelineSignals {
    return {
        timeline.signals?? = Some(create_timeline_signals());
        timeline.signals
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:59 (sha256:0b3edb68f4266603b37c6ac00724fdaeb125b116ef6fde0a7196ca9b37955505)
pub fn find_timeline_label(timeline: &Timeline, name: String) -> Option<TimelineLabel> {
    return (get_timeline_labels(timeline))
        .iter()
        .find(|value| (|l: TimelineLabel| -> bool { ((l.name).clone() == name) })((*value).clone()))
        .cloned();
}

// Source: upstream/packages/timeline/src/timeline.ts:65 (sha256:f5ac14be039cc637bb73aeb6ccd81aeba769f8c1267c5cf692e2975c1b8ea208)
pub fn get_timeline_current_label(timeline: &Timeline) -> Option<TimelineLabel> {
    let labels = get_timeline_labels(timeline);
    let frame = timeline.current_frame;
    let mut result: Option<TimelineLabel> = None;
    for label in (labels).iter().cloned() {
        if (label.frame <= frame) {
            if ((result).is_none() || (label.frame >= result.as_mut().unwrap().frame)) {
                result = Some((label).clone());
            }
        }
    }
    return (result).clone();
}

// Source: upstream/packages/timeline/src/timeline.ts:77 (sha256:5134d6ca7725fd94a6bcdf6c5a4665fd39861b2b8c79441cc6d1c1acced47d90)
pub fn get_timeline_frame_script(
    timeline: &Timeline,
    frame: &crate::FlightUnion2<f64, String>,
) -> Option<FrameScript> {
    if ((timeline.frame_scripts).clone()).is_none() {
        return None;
    }
    let resolved = resolve_frame(timeline, frame);
    return timeline
        .frame_scripts
        .as_mut()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &resolved)
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/timeline/src/timeline.ts:83 (sha256:689a5c9c7e94a9d0e460d13baa5302182fc03507aea274c765c48fdddc1b35e8)
pub fn goto_and_play_timeline(
    timeline: &mut Timeline,
    frame: &crate::FlightUnion2<f64, String>,
) -> () {
    play_timeline(timeline);
    seek_timeline(timeline, resolve_frame(timeline, frame));
}

// Source: upstream/packages/timeline/src/timeline.ts:88 (sha256:1e3498ab95d4736131cc9685a5ead895a4da6aa3565c6784b0987cdea7674fcb)
pub fn goto_and_stop_timeline(
    timeline: &mut Timeline,
    frame: &crate::FlightUnion2<f64, String>,
) -> () {
    stop_timeline(timeline);
    seek_timeline(timeline, resolve_frame(timeline, frame));
}

// Source: upstream/packages/timeline/src/timeline.ts:93 (sha256:252bf0fc31b7cae71d4f4fe38c0101b55ad13833b65ac92cb1cc458b23e3fd67)
pub fn next_frame_timeline(timeline: &mut Timeline) -> () {
    stop_timeline(timeline);
    seek_timeline(timeline, (timeline.current_frame + 1.0_f64));
}

// Source: upstream/packages/timeline/src/timeline.ts:98 (sha256:ecdeb97462a16ce041ca42c0a22440820ad4152d143a05b789a31b42302b1558)
pub fn play_timeline(timeline: &mut Timeline) -> () {
    if (timeline.is_playing || (get_timeline_total_frames(timeline) < 2.0_f64)) {
        return;
    }
    timeline.is_playing = true;
    timeline.time_elapsed = 0.0_f64;
}

// Source: upstream/packages/timeline/src/timeline.ts:104 (sha256:ed15e7e5a76a15b38925be4c0e81f436b8a941bc4196de4ae38de12e21eb4c95)
pub fn prev_frame_timeline(timeline: &mut Timeline) -> () {
    stop_timeline(timeline);
    seek_timeline(timeline, (timeline.current_frame - 1.0_f64));
}

// Source: upstream/packages/timeline/src/timeline.ts:109 (sha256:97c14a8e92352ab0d0c47e290c76600a02770620aa9bf4ce8e178e509b6ba32b)
pub fn remove_timeline_frame_script(
    timeline: &mut Timeline,
    frame: &crate::FlightUnion2<f64, String>,
) -> () {
    if ((timeline.frame_scripts).clone()).is_none() {
        return;
    }
    let resolved = resolve_frame(timeline, frame);
    {
        let __flight_key = resolved;
        if let Some(__flight_index) = timeline
            .frame_scripts
            .as_mut()
            .unwrap()
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            timeline
                .frame_scripts
                .as_mut()
                .unwrap()
                .remove(__flight_index);
            true
        } else {
            false
        }
    };
    if (timeline
        .frame_scripts
        .as_ref()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &"size")
        .map(|(_, value)| value.clone())
        .expect("TypeScript Record key was absent")
        == 0.0_f64)
    {
        timeline.frame_scripts = None;
    }
}

// Source: upstream/packages/timeline/src/timeline.ts:116 (sha256:aa74618a6a9588079709562b45b0b79f0d6830420ef41b44b435e65c872bb1e3)
pub fn stop_timeline(timeline: &mut Timeline) -> () {
    timeline.is_playing = false;
}

// Source: upstream/packages/timeline/src/timeline.ts:120 (sha256:a0a5165effb4c5ac27cb8f8a21464fe0b01d703798e50a2eaa23566dc677812e)
pub fn update_timeline(timeline: &mut Timeline, delta_time: f64) -> () {
    let frame_rate = get_timeline_frame_rate(timeline);
    if (timeline.is_playing && (frame_rate).is_some()) {
        timeline.current_frame = advance_frame(timeline, delta_time);
    }
    fire_construct_frame(timeline);
    if (timeline.is_playing && (frame_rate).is_none()) {
        timeline.current_frame = advance_frame(timeline, delta_time);
    }
}

// Source: upstream/packages/timeline/src/timeline.ts:131 (sha256:070f4d85f302cd9f401f2e3aee970fb07e2493a360f5baac3b305fe2086717b7)
static EMPTY_LABELS: std::sync::LazyLock<Vec<TimelineLabel>> = std::sync::LazyLock::new(|| vec![]);

// Source: upstream/packages/timeline/src/timeline.ts:133 (sha256:0aa67dda0cfdd4ea3efa42e690e06f18f2cfe4a46790989e8e76fde80f409248)
fn noop_construct_frame() -> () {}

// Source: upstream/packages/timeline/src/timeline.ts:141 (sha256:58f6550fc3afce07abe75aaeb533b686060557428f1c6307dca659d5532a6e71)
fn advance_frame(timeline: &mut Timeline, delta_time: f64) -> f64 {
    let frame_rate = get_timeline_frame_rate(timeline);
    let total_frames = get_timeline_total_frames(timeline);
    if (frame_rate).is_some() {
        let frame_time = (1000.0_f64 / frame_rate.as_ref().unwrap());
        timeline.time_elapsed += delta_time;
        let mut next = (timeline.current_frame + (timeline.time_elapsed / frame_time).floor());
        timeline.time_elapsed %= frame_time;
        if (next > total_frames) {
            if ((timeline.play_mode).clone() == "once") {
                timeline.is_playing = false;
                let completed = total_frames;
                let signals = (timeline.signals).clone();
                if (signals).is_some() {
                    emit_signal((signals.as_ref().unwrap().on_complete).clone(), ());
                }
                return completed;
            }
            next = (((next - 1.0_f64) % total_frames) + 1.0_f64);
            let signals = (timeline.signals).clone();
            if (signals).is_some() {
                emit_signal((signals.as_ref().unwrap().on_loop).clone(), ());
            }
        }
        return next;
    }
    let mut next = (timeline.current_frame + 1.0_f64);
    if (next > total_frames) {
        if ((timeline.play_mode).clone() == "once") {
            timeline.is_playing = false;
            let signals = (timeline.signals).clone();
            if (signals).is_some() {
                emit_signal((signals.as_ref().unwrap().on_complete).clone(), ());
            }
            return total_frames;
        }
        let signals = (timeline.signals).clone();
        if (signals).is_some() {
            emit_signal((signals.as_ref().unwrap().on_loop).clone(), ());
        }
        return 1.0_f64;
    }
    return next;
}

// Source: upstream/packages/timeline/src/timeline.ts:178 (sha256:636cfa8e946081c26366636930028ede8f2f6311528bb61588dd2b92d89bfad3)
fn create_timeline_signals() -> TimelineSignals {
    return TimelineSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_complete: create_signal(),
        on_enter_frame: create_signal(),
        on_exit_frame: create_signal(),
        on_frame_constructed: create_signal(),
        on_loop: create_signal(),
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:192 (sha256:3aab7659101bd2973a54098bb5b817a24cfd4f597a0c403657ceac7d7ef0a851)
fn fire_construct_frame(timeline: &mut Timeline) -> () {
    let previous = timeline.last_frame_update;
    let current = timeline.current_frame;
    if (current == previous) {
        return;
    }
    let signals = (timeline.signals).clone();
    let target = (timeline.target).clone();
    let frame_event = TimelineFrameEvent {
        __flight_identity: std::sync::Arc::new(()),
        frame: current,
        previous_frame: previous,
    };
    if (signals).is_some() {
        emit_signal(
            (signals.as_ref().unwrap().on_exit_frame).clone(),
            ((frame_event).clone(),),
        );
    }
    timeline.last_frame_update = current;
    if (signals).is_some() {
        emit_signal(
            (signals.as_ref().unwrap().on_enter_frame).clone(),
            ((frame_event).clone(),),
        );
    }
    if (target).is_some() {
        timeline
            .source
            .as_mut()
            .unwrap()
            .construct_frame
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()((target.as_ref().unwrap()).clone(), current);
    }
    if ((timeline.frame_scripts).clone()).is_some() {
        let script = timeline
            .frame_scripts
            .as_mut()
            .unwrap()
            .iter()
            .find(|(key, _)| key == &current)
            .map(|(_, value)| value.clone());
        if ((script).is_some() && (target).is_some()) {
            script.as_ref().unwrap().lock().unwrap()((target).clone().unwrap(), current);
        }
    }
    if (signals).is_some() {
        emit_signal(
            (signals.as_ref().unwrap().on_frame_constructed).clone(),
            ((frame_event).clone(),),
        );
    }
}

// Source: upstream/packages/timeline/src/timeline.ts:212 (sha256:f03fd0b1cc4e155fb7f33c8df57a9b9e672bcbc98b68cbfbf7fa0c09a649bc6b)
fn get_timeline_frame_rate(timeline: &Timeline) -> Option<f64> {
    return timeline.source.as_ref().and_then(|value| value.frame_rate);
}

// Source: upstream/packages/timeline/src/timeline.ts:216 (sha256:db2061abe0540dfcc8bb20b38afbb10c6edeff9c31de91e606b881bbd02eeb03)
fn get_timeline_labels(timeline: &Timeline) -> Vec<TimelineLabel> {
    return (timeline.source.as_ref().map(|value| (value.labels).clone()))
        .unwrap_or(((*EMPTY_LABELS).clone()).clone());
}

// Source: upstream/packages/timeline/src/timeline.ts:220 (sha256:e4631bf013708a5e453791d175515f924f6f78e123a6ef765e71174fa06874c9)
fn get_timeline_total_frames(timeline: &Timeline) -> f64 {
    return (timeline.source.as_ref().map(|value| value.total_frames)).unwrap_or(1.0_f64);
}

// Source: upstream/packages/timeline/src/timeline.ts:224 (sha256:0e0439f4984d5a48d92ecb29abcf9bab94ff0e2c10d5f9345b9d759dffb115bb)
fn resolve_frame(timeline: &Timeline, frame: &crate::FlightUnion2<f64, String>) -> f64 {
    if (match &(frame) {
        crate::FlightUnion2::A(_) => "number",
        crate::FlightUnion2::B(value) => "string",
    } == "number")
    {
        return match (*frame).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        };
    }
    let label = find_timeline_label(
        timeline,
        match (*frame).clone() {
            crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
            crate::FlightUnion2::B(value) => value,
        },
    );
    if (label).is_none() {
        panic!("{}", format!("Frame label \"{}\" not found", frame));
    }
    return label.as_ref().unwrap().frame;
}

// Source: upstream/packages/timeline/src/timeline.ts:231 (sha256:5af8d3ad54ed89510769ace4aa526cc9d749631883dbbe46ebc96522589b4aa8)
fn seek_timeline(timeline: &mut Timeline, frame: f64) -> () {
    timeline.current_frame = (1.0_f64).max((frame).min(get_timeline_total_frames(timeline)));
    timeline.last_frame_update = (-1.0_f64);
    fire_construct_frame(timeline);
}
