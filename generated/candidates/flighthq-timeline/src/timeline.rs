// @generated from upstream/packages/timeline/src/timeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    FrameScript, Node2D, Timeline, TimelineCue, TimelineCueRegistry, TimelineFrameEvent,
    TimelineLabel, TimelinePlayMode, TimelineSignals, TimelineSource,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub total_frames: Option<f64>,
    pub frame_rate: Option<f64>,
    pub labels: Option<Vec<TimelineLabel>>,
    pub cues: Option<Vec<TimelineCue>>,
    pub construct_frame: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node2D, f64) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1338338174 {
    pub __flight_identity: std::sync::Arc<()>,
    pub source: Option<TimelineSource>,
    pub target: Option<Node2D>,
    pub current_frame: Option<f64>,
    pub cue_registry: Option<TimelineCueRegistry>,
    pub frame_scripts: Option<Vec<(f64, FrameScript)>>,
    pub is_playing: Option<bool>,
    pub time_elapsed: Option<f64>,
    pub last_frame_update: Option<f64>,
    pub play_mode: Option<TimelinePlayMode>,
    pub signals: Option<TimelineSignals>,
}
impl PartialEq for FlightPartialRecord1338338174 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/timeline/src/timeline.ts:13 (sha256:cc8b05b7dad875e3c989fa675ab387eae59048756634b98790fc0f3b56c51b3d)
pub fn add_timeline_frame_script(
    timeline: &mut Timeline,
    frame: &crate::FlightUnion2<f64, String>,
    script: FrameScript,
) -> () {
    let resolved = resolve_frame(timeline, &((*frame).clone()));
    {
        if timeline.frame_scripts.is_none() {
            timeline.frame_scripts = Some(Vec::new());
        };
        let __flight_key = resolved;
        let __flight_value = (script).clone();
        if let Some((_, value)) = timeline
            .frame_scripts
            .as_mut()
            .unwrap()
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            timeline
                .frame_scripts
                .as_mut()
                .unwrap()
                .push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:18 (sha256:ec35162fe0c7d0c928e511925f31e98f00940a707dbb410518d9d2a007d9936b)
pub fn clear_timeline_frame_scripts(timeline: &mut Timeline) -> () {
    timeline.frame_scripts = None;
}

// Source: upstream/packages/timeline/src/timeline.ts:22 (sha256:6e0c755667cddc3c06b596bc843d40f250047d500f1d3a6adc25b36018eeeedd)
pub fn create_timeline(obj: Option<FlightPartialRecord1338338174>) -> Timeline {
    return Timeline {
        __flight_identity: std::sync::Arc::new(()),
        source: obj.as_ref().and_then(|value| (value.source).clone()),
        target: obj.as_ref().and_then(|value| (value.target).clone()),
        cue_registry: obj.as_ref().and_then(|value| (value.cue_registry).clone()),
        current_frame: (obj.as_ref().and_then(|value| value.current_frame))
            .clone()
            .unwrap_or(1.0_f64),
        frame_scripts: obj.as_ref().and_then(|value| (value.frame_scripts).clone()),
        is_playing: (obj.as_ref().and_then(|value| value.is_playing))
            .clone()
            .unwrap_or(false),
        last_frame_update: (-1.0_f64),
        play_mode: (obj.as_ref().and_then(|value| (value.play_mode).clone()))
            .clone()
            .unwrap_or("loop".to_owned()),
        signals: obj.as_ref().and_then(|value| (value.signals).clone()),
        time_elapsed: 0.0_f64,
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:41 (sha256:5f49ccfe67808d9856b586e7d34c4248795f3fcc87214d9c0b374656074e944e)
pub fn create_timeline_source(obj: &SharedStructuralRecord1) -> TimelineSource {
    return TimelineSource {
        __flight_identity: std::sync::Arc::new(()),
        total_frames: (obj.total_frames).clone().unwrap_or(1.0_f64),
        frame_rate: obj.frame_rate,
        labels: ((obj.labels).clone())
            .clone()
            .unwrap_or(((*EMPTY_LABELS).clone()).clone()),
        cues: ((obj.cues).clone())
            .clone()
            .unwrap_or(((*EMPTY_CUES).clone()).clone()),
        construct_frame: ((obj.construct_frame).clone())
            .clone()
            .unwrap_or(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                move |__flight_argument_0: Node2D, __flight_argument_1: f64| -> () {
                    noop_construct_frame()
                },
            )
                as Box<dyn FnMut(Node2D, f64) -> () + Send + 'static>))),
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:57 (sha256:6a7bbfee267c23e9066a61f974f52e1fbfa5fd2956b670e8dd42032737bd7241)
pub fn dispose_timeline_signals(timeline: &mut Timeline) -> () {
    timeline.signals = None;
}

// Source: upstream/packages/timeline/src/timeline.ts:63 (sha256:fc4aa3821caba8c92477e8259ceb84765bc939b6b89a22273583b672c2b55078)
pub fn enable_timeline_signals(timeline: &mut Timeline) -> TimelineSignals {
    return {
        if timeline.signals.is_none() {
            timeline.signals = Some(create_timeline_signals());
        }
        timeline.signals.as_ref().unwrap().clone()
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:67 (sha256:0b3edb68f4266603b37c6ac00724fdaeb125b116ef6fde0a7196ca9b37955505)
pub fn find_timeline_label(timeline: &Timeline, name: String) -> Option<TimelineLabel> {
    return (get_timeline_labels(timeline))
        .iter()
        .find(|value| (|l: TimelineLabel| -> bool { ((l.name).clone() == name) })((*value).clone()))
        .cloned();
}

// Source: upstream/packages/timeline/src/timeline.ts:73 (sha256:f5ac14be039cc637bb73aeb6ccd81aeba769f8c1267c5cf692e2975c1b8ea208)
pub fn get_timeline_current_label(timeline: &Timeline) -> Option<TimelineLabel> {
    let labels = get_timeline_labels(timeline);
    let frame = timeline.current_frame;
    let mut result: Option<TimelineLabel> = None;
    for label in (labels).iter().cloned() {
        if (label.frame <= frame) {
            if ((result).is_none()) || (label.frame >= result.as_mut().unwrap().frame) {
                result = Some((label).clone());
            }
        }
    }
    return (result).clone();
}

// Source: upstream/packages/timeline/src/timeline.ts:85 (sha256:5134d6ca7725fd94a6bcdf6c5a4665fd39861b2b8c79441cc6d1c1acced47d90)
pub fn get_timeline_frame_script(
    timeline: &Timeline,
    frame: &crate::FlightUnion2<f64, String>,
) -> Option<FrameScript> {
    if ((timeline.frame_scripts).clone()).is_none() {
        return None;
    }
    let resolved = resolve_frame(timeline, &((*frame).clone()));
    return timeline
        .frame_scripts
        .as_ref()
        .unwrap()
        .iter()
        .find(|(entry_key, _)| entry_key == &resolved)
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/timeline/src/timeline.ts:93 (sha256:b575d6d5acaa427819cf36f055dfbc451d7ceb225b95d681937c69b5d155ebbe)
pub fn get_timeline_frame_script_frames(timeline: &Timeline) -> Vec<f64> {
    return if ((timeline.frame_scripts).clone()).is_none() {
        ((*EMPTY_FRAMES).clone()).clone()
    } else {
        {
            let mut __flight_array = Vec::new();
            __flight_array.extend(
                (timeline
                    .frame_scripts
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(key, _)| key.clone())
                    .collect::<Vec<_>>())
                .iter()
                .cloned(),
            );
            __flight_array
        }
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:97 (sha256:9a87f65767c4426fa3d9779cf0d93d429af6f7a63ba6c430a4ecc7af7ed418e8)
pub fn get_timeline_labels(timeline: &Timeline) -> Vec<TimelineLabel> {
    return (timeline.source.as_ref().map(|value| (value.labels).clone()))
        .clone()
        .unwrap_or(((*EMPTY_LABELS).clone()).clone());
}

// Source: upstream/packages/timeline/src/timeline.ts:101 (sha256:689a5c9c7e94a9d0e460d13baa5302182fc03507aea274c765c48fdddc1b35e8)
pub fn goto_and_play_timeline(
    timeline: &mut Timeline,
    frame: &crate::FlightUnion2<f64, String>,
) -> () {
    play_timeline(timeline);
    {
        let __flight_argument_1 = resolve_frame(timeline, &((*frame).clone()));
        let __flight_result = seek_timeline(timeline, __flight_argument_1);
        __flight_result
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:106 (sha256:1e3498ab95d4736131cc9685a5ead895a4da6aa3565c6784b0987cdea7674fcb)
pub fn goto_and_stop_timeline(
    timeline: &mut Timeline,
    frame: &crate::FlightUnion2<f64, String>,
) -> () {
    stop_timeline(timeline);
    {
        let __flight_argument_1 = resolve_frame(timeline, &((*frame).clone()));
        let __flight_result = seek_timeline(timeline, __flight_argument_1);
        __flight_result
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:111 (sha256:252bf0fc31b7cae71d4f4fe38c0101b55ad13833b65ac92cb1cc458b23e3fd67)
pub fn next_frame_timeline(timeline: &mut Timeline) -> () {
    stop_timeline(timeline);
    {
        let __flight_argument_1 = (timeline.current_frame + 1.0_f64);
        let __flight_result = seek_timeline(timeline, __flight_argument_1);
        __flight_result
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:116 (sha256:ecdeb97462a16ce041ca42c0a22440820ad4152d143a05b789a31b42302b1558)
pub fn play_timeline(timeline: &mut Timeline) -> () {
    if (timeline.is_playing) || (get_timeline_total_frames(timeline) < 2.0_f64) {
        return;
    }
    timeline.is_playing = true;
    timeline.time_elapsed = 0.0_f64;
}

// Source: upstream/packages/timeline/src/timeline.ts:122 (sha256:ed15e7e5a76a15b38925be4c0e81f436b8a941bc4196de4ae38de12e21eb4c95)
pub fn prev_frame_timeline(timeline: &mut Timeline) -> () {
    stop_timeline(timeline);
    {
        let __flight_argument_1 = (timeline.current_frame - 1.0_f64);
        let __flight_result = seek_timeline(timeline, __flight_argument_1);
        __flight_result
    };
}

// Source: upstream/packages/timeline/src/timeline.ts:127 (sha256:97c14a8e92352ab0d0c47e290c76600a02770620aa9bf4ce8e178e509b6ba32b)
pub fn remove_timeline_frame_script(
    timeline: &mut Timeline,
    frame: &crate::FlightUnion2<f64, String>,
) -> () {
    if ((timeline.frame_scripts).clone()).is_none() {
        return;
    }
    let resolved = resolve_frame(timeline, &((*frame).clone()));
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
    if ((timeline.frame_scripts.as_ref().unwrap().len() as f64) == 0.0_f64) {
        timeline.frame_scripts = None;
    }
}

// Source: upstream/packages/timeline/src/timeline.ts:134 (sha256:aa74618a6a9588079709562b45b0b79f0d6830420ef41b44b435e65c872bb1e3)
pub fn stop_timeline(timeline: &mut Timeline) -> () {
    timeline.is_playing = false;
}

// Source: upstream/packages/timeline/src/timeline.ts:142 (sha256:4ff0e5a7fe84a52253450d1fea586386982714daf676c269163ed878811c2d40)
pub fn update_timeline(timeline: &mut Timeline, delta_time: f64) -> bool {
    let frame_rate = get_timeline_frame_rate(timeline);
    if (timeline.is_playing) && ((frame_rate).is_some()) {
        timeline.current_frame = advance_frame(timeline, delta_time);
    }
    let changed = fire_construct_frame(timeline);
    if (timeline.is_playing) && ((frame_rate).is_none()) {
        timeline.current_frame = advance_frame(timeline, delta_time);
    }
    return changed;
}

// Source: upstream/packages/timeline/src/timeline.ts:154 (sha256:d1e0fc493ba338ac543fec948f755aad5bdd5ed45832973c4dbb40d383af9f48)
static EMPTY_FRAMES: std::sync::LazyLock<Vec<f64>> = std::sync::LazyLock::new(|| vec![]);

// Source: upstream/packages/timeline/src/timeline.ts:155 (sha256:942517c32c16a603089fe9a3cf256f90fda45925e3f48354b454f83b2d6887a8)
static EMPTY_CUES: std::sync::LazyLock<Vec<TimelineCue>> = std::sync::LazyLock::new(|| vec![]);

// Source: upstream/packages/timeline/src/timeline.ts:157 (sha256:070f4d85f302cd9f401f2e3aee970fb07e2493a360f5baac3b305fe2086717b7)
static EMPTY_LABELS: std::sync::LazyLock<Vec<TimelineLabel>> = std::sync::LazyLock::new(|| vec![]);

// Source: upstream/packages/timeline/src/timeline.ts:159 (sha256:0aa67dda0cfdd4ea3efa42e690e06f18f2cfe4a46790989e8e76fde80f409248)
fn noop_construct_frame() -> () {}

// Source: upstream/packages/timeline/src/timeline.ts:167 (sha256:58f6550fc3afce07abe75aaeb533b686060557428f1c6307dca659d5532a6e71)
fn advance_frame(timeline: &mut Timeline, delta_time: f64) -> f64 {
    let frame_rate = get_timeline_frame_rate(timeline);
    let total_frames = get_timeline_total_frames(timeline);
    if (frame_rate).is_some() {
        let frame_time = (1000.0_f64 / *(frame_rate.as_ref().unwrap()));
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

// Source: upstream/packages/timeline/src/timeline.ts:204 (sha256:636cfa8e946081c26366636930028ede8f2f6311528bb61588dd2b92d89bfad3)
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

// Source: upstream/packages/timeline/src/timeline.ts:218 (sha256:4a900289a123dab79db9900232660d9179c53920070baf6df3e562f6ef4da3d0)
fn fire_construct_frame(timeline: &mut Timeline) -> bool {
    let previous = timeline.last_frame_update;
    let current = timeline.current_frame;
    if (current == previous) {
        return false;
    }
    let signals = (timeline.signals).clone();
    let target = (timeline.target).clone();
    let frame_event: TimelineFrameEvent = TimelineFrameEvent {
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
    if ((target).clone()).is_some() {
        {
            let __flight_callback = timeline
                .source
                .as_ref()
                .map(|value| (value.construct_frame).clone());
            __flight_callback.as_ref().map(|callback| {
                callback.lock().unwrap()((target.as_ref().unwrap()).clone(), current)
            })
        };
    }
    if ((timeline.frame_scripts).clone()).is_some() {
        let script = timeline
            .frame_scripts
            .as_ref()
            .unwrap()
            .iter()
            .find(|(entry_key, _)| entry_key == &current)
            .map(|(_, value)| value.clone());
        if ((script).is_some()) && ((target).is_some()) {
            {
                let __flight_callback = (script.as_ref().unwrap()).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()((target.as_ref().unwrap()).clone(), current);
                __flight_result
            };
        }
    }
    if (signals).is_some() {
        emit_signal(
            (signals.as_ref().unwrap().on_frame_constructed).clone(),
            ((frame_event).clone(),),
        );
    }
    return true;
}

// Source: upstream/packages/timeline/src/timeline.ts:239 (sha256:f03fd0b1cc4e155fb7f33c8df57a9b9e672bcbc98b68cbfbf7fa0c09a649bc6b)
fn get_timeline_frame_rate(timeline: &Timeline) -> Option<f64> {
    return timeline.source.as_ref().and_then(|value| value.frame_rate);
}

// Source: upstream/packages/timeline/src/timeline.ts:243 (sha256:e4631bf013708a5e453791d175515f924f6f78e123a6ef765e71174fa06874c9)
fn get_timeline_total_frames(timeline: &Timeline) -> f64 {
    return (timeline.source.as_ref().map(|value| value.total_frames))
        .clone()
        .unwrap_or(1.0_f64);
}

// Source: upstream/packages/timeline/src/timeline.ts:247 (sha256:0e0439f4984d5a48d92ecb29abcf9bab94ff0e2c10d5f9345b9d759dffb115bb)
fn resolve_frame(timeline: &Timeline, frame: &crate::FlightUnion2<f64, String>) -> f64 {
    if ((match &(frame) {
        crate::FlightUnion2::A(_) => "number",
        crate::FlightUnion2::B(value) => "string",
    })
    .to_owned()
        == "number")
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
        panic!(
            "{}",
            format!(
                "Frame label \"{}\" not found",
                match (*frame).clone() {
                    crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                    crate::FlightUnion2::B(value) => value,
                }
            )
        );
    }
    return label.as_ref().unwrap().frame;
}

// Source: upstream/packages/timeline/src/timeline.ts:254 (sha256:5af8d3ad54ed89510769ace4aa526cc9d749631883dbbe46ebc96522589b4aa8)
fn seek_timeline(timeline: &mut Timeline, frame: f64) -> () {
    timeline.current_frame = (1.0_f64).max((frame).min(get_timeline_total_frames(timeline)));
    timeline.last_frame_update = (-1.0_f64);
    fire_construct_frame(timeline);
}
