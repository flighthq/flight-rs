// @generated from upstream/packages/textbidi/src/bidiClassBackend.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{BidiClass, BidiClassBackend};

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

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:11 (sha256:2c075704c80916b1f513d549be55c3d87c1af79a0f9625682d0f50d64de9cab2)
pub fn create_compact_bidi_class_backend() -> BidiClassBackend {
    return BidiClassBackend {
        __flight_identity: std::sync::Arc::new(()),
        get_bidi_class: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: f64| -> BidiClass {
                get_compact_bidi_class(__flight_argument_0)
            },
        )
            as Box<dyn FnMut(f64) -> BidiClass + Send + 'static>)),
    };
}

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:18 (sha256:83454cb318a967f6bf77dfb1c1a78a6dfeb51b77139ffcf4e70ed6205e6c2b51)
pub fn get_bidi_class_backend() -> BidiClassBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_compact_bidi_class_backend());
    }
    return ((*_BACKEND.lock().unwrap()).clone()).clone().unwrap();
}

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:27 (sha256:7e7dcb62c648fd18d53fc53823253ec4ec7509a56cf077b3f837f97903ddcf96)
pub fn set_bidi_class_backend(backend: Option<BidiClassBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:31 (sha256:45fa3a8626983184e266e82dc4a380c55780fd086c9ed6f287b16e6d348d35c9)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<BidiClassBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:37 (sha256:aabce4c51e2e09322743e8a20e791edde02c0c4627a31c2b9d3f7c1490e63169)
fn get_compact_bidi_class(codepoint: f64) -> BidiClass {
    let mut lo = 0.0_f64;
    let mut hi = (_RANGE_COUNT - 1.0_f64);
    while (lo <= hi) {
        let mid = (__flight_js_to_i32((lo + hi)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
        let base = (mid * 3.0_f64);
        let start = _RANGES[base as usize].clone();
        if (codepoint < start) {
            hi = (mid - 1.0_f64);
        } else {
            if (codepoint > _RANGES[(base + 1.0_f64) as usize].clone()) {
                lo = (mid + 1.0_f64);
            } else {
                return _CLASS_ORDER[_RANGES[(base + 2.0_f64) as usize].clone() as usize].clone();
            }
        }
    }
    return "L".to_owned();
}

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:57 (sha256:12faff9a3160471a435a62698b68923b8bbf1fa2a58bafb4f9fc70408f300e9b)
static _CLASS_ORDER: std::sync::LazyLock<Vec<BidiClass>> = std::sync::LazyLock::new(|| {
    vec![
        "L".to_owned(),
        "R".to_owned(),
        "AL".to_owned(),
        "EN".to_owned(),
        "ES".to_owned(),
        "ET".to_owned(),
        "AN".to_owned(),
        "CS".to_owned(),
        "NSM".to_owned(),
        "BN".to_owned(),
        "B".to_owned(),
        "S".to_owned(),
        "WS".to_owned(),
        "ON".to_owned(),
        "LRE".to_owned(),
        "RLE".to_owned(),
        "LRO".to_owned(),
        "RLO".to_owned(),
        "PDF".to_owned(),
        "LRI".to_owned(),
        "RLI".to_owned(),
        "FSI".to_owned(),
        "PDI".to_owned(),
    ]
});

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:83 (sha256:5b0a9c42b60f5b82e42a14a44bcff9e5df683a17114085a9f282e1bd8c429621)
const L: f64 = 0.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:84 (sha256:179975c81e1f4917c2b1851406aa28f9f235e1086cacc5518782748cf26e5cb9)
const R: f64 = 1.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:85 (sha256:8e87d277c1e57c600f93548b44e94af545402575625cb92e5f8b3bd336643abd)
const AL: f64 = 2.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:86 (sha256:768d53f07c3b1e2e384b1076d350d3f67e93f1132b988dce0f3ef26a4bf6725a)
const EN: f64 = 3.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:87 (sha256:e418e14b93c8b7c04ec2a6d4a6e5ebb6151d50f65e06771195ddcf52a804dac2)
const ES: f64 = 4.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:88 (sha256:e02bfebcf973754b0df271663e080f988c1a77ba12cbd94fbb86dba895276b79)
const ET: f64 = 5.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:89 (sha256:5e423aba00bfbf879be8a9c3251e4a11d646cd0efa1ef542305b6286c5df17d0)
const AN: f64 = 6.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:90 (sha256:8edcd95e6284e21656de8ef173d55caf5b22a0c468b7f74def869b37048b4400)
const CS: f64 = 7.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:91 (sha256:c35ce662c7fc7104c5b79498271044cb6696ce84eb83d05467511860b655b733)
const NSM: f64 = 8.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:92 (sha256:11ee57f01cb0b297c87ecc243bb3371f5a9a7f174ac5630c4685108abcb6fcc0)
const BN: f64 = 9.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:93 (sha256:eee19a3034acb4a589f2740335a6cbefda22807b85cbff792abe09cdb113e172)
const B: f64 = 10.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:94 (sha256:9a44caf1d667d3fdde8d9f2c69de584fc580ddcb87c062769c513cf4cb97a9cd)
const S: f64 = 11.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:95 (sha256:98587ff80339185c1ed859bd823f1608e82a2ea096e683e3fe976e35e3286a64)
const WS: f64 = 12.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:96 (sha256:8835d84670f1c32ec70eec987abe526f3330551207de916a30261f98021c753b)
const ON: f64 = 13.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:97 (sha256:87e91ec1b27ac07e7ba717b1add8df50b03324ca430464cd9ff99a613059f4e7)
const LRE: f64 = 14.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:98 (sha256:a03913949800b068d7418f4444e44492742e6fca65b7ac4bc9058bcb53b48abb)
const RLE: f64 = 15.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:99 (sha256:1c73278d8b2f31431a4f6a29e70c75402efe7e584af35f2b031ae7c66da2c10d)
const LRO: f64 = 16.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:100 (sha256:b7c09171ee8c95b891899ae806671258888f58a9e1a7273e1c4a9668b2a4dc3f)
const RLO: f64 = 17.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:101 (sha256:f4c653a75c29ec87ff35ade9b646cea4535c1e7348b20e90580dacf0e66c26fb)
const PDF: f64 = 18.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:102 (sha256:0ea546c27d29ea7336507434433efb483fc5d40f474b326814917ce9956414f8)
const LRI: f64 = 19.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:103 (sha256:2a882843f8eea716d1879f52faf7e8b2773f81217e14959b38e32641edee8d1a)
const RLI: f64 = 20.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:104 (sha256:e501c5a7a4c1024b230af934e88bc477fa887d1a4fff8a07b5f49120e25c27b9)
const FSI: f64 = 21.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:105 (sha256:6b9fdca070e8c8eb2f93b4843f73ee9ab0ccc20501e6c4c8c86fc396a04c0316)
const PDI: f64 = 22.0_f64;

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:112 (sha256:bd5db6b26c9f527dde6c2594cc82ccaaa83aeee006552873d6ddfae4bd1f9c77)
static _RANGES: std::sync::LazyLock<Vec<f64>> = std::sync::LazyLock::new(|| {
    vec![
        0.0_f64,
        8.0_f64,
        BN,
        9.0_f64,
        9.0_f64,
        S,
        10.0_f64,
        10.0_f64,
        B,
        11.0_f64,
        11.0_f64,
        S,
        12.0_f64,
        12.0_f64,
        WS,
        13.0_f64,
        13.0_f64,
        B,
        14.0_f64,
        27.0_f64,
        BN,
        28.0_f64,
        30.0_f64,
        B,
        31.0_f64,
        31.0_f64,
        S,
        32.0_f64,
        32.0_f64,
        WS,
        33.0_f64,
        34.0_f64,
        ON,
        35.0_f64,
        37.0_f64,
        ET,
        38.0_f64,
        42.0_f64,
        ON,
        43.0_f64,
        43.0_f64,
        ES,
        44.0_f64,
        44.0_f64,
        CS,
        45.0_f64,
        45.0_f64,
        ES,
        46.0_f64,
        47.0_f64,
        CS,
        48.0_f64,
        57.0_f64,
        EN,
        58.0_f64,
        58.0_f64,
        CS,
        59.0_f64,
        64.0_f64,
        ON,
        65.0_f64,
        90.0_f64,
        L,
        91.0_f64,
        96.0_f64,
        ON,
        97.0_f64,
        122.0_f64,
        L,
        123.0_f64,
        126.0_f64,
        ON,
        127.0_f64,
        132.0_f64,
        BN,
        133.0_f64,
        133.0_f64,
        B,
        134.0_f64,
        159.0_f64,
        BN,
        160.0_f64,
        160.0_f64,
        CS,
        161.0_f64,
        161.0_f64,
        ON,
        162.0_f64,
        165.0_f64,
        ET,
        166.0_f64,
        169.0_f64,
        ON,
        170.0_f64,
        170.0_f64,
        L,
        171.0_f64,
        172.0_f64,
        ON,
        173.0_f64,
        173.0_f64,
        BN,
        174.0_f64,
        175.0_f64,
        ON,
        176.0_f64,
        177.0_f64,
        ET,
        178.0_f64,
        179.0_f64,
        EN,
        180.0_f64,
        180.0_f64,
        ON,
        181.0_f64,
        181.0_f64,
        L,
        182.0_f64,
        184.0_f64,
        ON,
        185.0_f64,
        185.0_f64,
        EN,
        186.0_f64,
        186.0_f64,
        L,
        187.0_f64,
        191.0_f64,
        ON,
        192.0_f64,
        214.0_f64,
        L,
        215.0_f64,
        215.0_f64,
        ON,
        216.0_f64,
        246.0_f64,
        L,
        247.0_f64,
        247.0_f64,
        ON,
        248.0_f64,
        255.0_f64,
        L,
        768.0_f64,
        879.0_f64,
        NSM,
        1424.0_f64,
        1424.0_f64,
        R,
        1425.0_f64,
        1469.0_f64,
        NSM,
        1470.0_f64,
        1470.0_f64,
        R,
        1471.0_f64,
        1471.0_f64,
        NSM,
        1472.0_f64,
        1472.0_f64,
        R,
        1473.0_f64,
        1474.0_f64,
        NSM,
        1475.0_f64,
        1475.0_f64,
        R,
        1476.0_f64,
        1477.0_f64,
        NSM,
        1478.0_f64,
        1478.0_f64,
        R,
        1479.0_f64,
        1479.0_f64,
        NSM,
        1480.0_f64,
        1535.0_f64,
        R,
        1536.0_f64,
        1541.0_f64,
        AN,
        1542.0_f64,
        1543.0_f64,
        ON,
        1544.0_f64,
        1544.0_f64,
        AL,
        1545.0_f64,
        1546.0_f64,
        ET,
        1547.0_f64,
        1547.0_f64,
        AL,
        1548.0_f64,
        1548.0_f64,
        CS,
        1549.0_f64,
        1551.0_f64,
        AL,
        1552.0_f64,
        1562.0_f64,
        NSM,
        1563.0_f64,
        1610.0_f64,
        AL,
        1611.0_f64,
        1631.0_f64,
        NSM,
        1632.0_f64,
        1641.0_f64,
        AN,
        1642.0_f64,
        1642.0_f64,
        ET,
        1643.0_f64,
        1644.0_f64,
        AN,
        1645.0_f64,
        1647.0_f64,
        AL,
        1648.0_f64,
        1648.0_f64,
        NSM,
        1649.0_f64,
        1749.0_f64,
        AL,
        1750.0_f64,
        1756.0_f64,
        NSM,
        1757.0_f64,
        1758.0_f64,
        AN,
        1759.0_f64,
        1764.0_f64,
        NSM,
        1765.0_f64,
        1766.0_f64,
        AL,
        1767.0_f64,
        1768.0_f64,
        NSM,
        1769.0_f64,
        1769.0_f64,
        ON,
        1770.0_f64,
        1773.0_f64,
        NSM,
        1774.0_f64,
        1775.0_f64,
        AL,
        1776.0_f64,
        1785.0_f64,
        EN,
        1786.0_f64,
        1791.0_f64,
        AL,
        1872.0_f64,
        1919.0_f64,
        AL,
        2208.0_f64,
        2303.0_f64,
        AL,
        5760.0_f64,
        5760.0_f64,
        WS,
        8192.0_f64,
        8202.0_f64,
        WS,
        8203.0_f64,
        8205.0_f64,
        BN,
        8206.0_f64,
        8206.0_f64,
        L,
        8207.0_f64,
        8207.0_f64,
        R,
        8208.0_f64,
        8231.0_f64,
        ON,
        8232.0_f64,
        8232.0_f64,
        WS,
        8233.0_f64,
        8233.0_f64,
        B,
        8234.0_f64,
        8234.0_f64,
        LRE,
        8235.0_f64,
        8235.0_f64,
        RLE,
        8236.0_f64,
        8236.0_f64,
        PDF,
        8237.0_f64,
        8237.0_f64,
        LRO,
        8238.0_f64,
        8238.0_f64,
        RLO,
        8239.0_f64,
        8239.0_f64,
        CS,
        8240.0_f64,
        8244.0_f64,
        ET,
        8245.0_f64,
        8281.0_f64,
        ON,
        8287.0_f64,
        8287.0_f64,
        WS,
        8288.0_f64,
        8292.0_f64,
        BN,
        8294.0_f64,
        8294.0_f64,
        LRI,
        8295.0_f64,
        8295.0_f64,
        RLI,
        8296.0_f64,
        8296.0_f64,
        FSI,
        8297.0_f64,
        8297.0_f64,
        PDI,
        12288.0_f64,
        12288.0_f64,
        WS,
        64285.0_f64,
        64285.0_f64,
        R,
        64286.0_f64,
        64286.0_f64,
        NSM,
        64287.0_f64,
        64335.0_f64,
        R,
        64336.0_f64,
        64975.0_f64,
        AL,
        65008.0_f64,
        65023.0_f64,
        AL,
        65136.0_f64,
        65278.0_f64,
        AL,
        65279.0_f64,
        65279.0_f64,
        BN,
    ]
});

// Source: upstream/packages/textbidi/src/bidiClassBackend.ts:242 (sha256:1a034918babc409ce0b9b212b4a2bc3bb1aaae64364708707c2b511faf4963f2)
static _RANGE_COUNT: std::sync::LazyLock<f64> =
    std::sync::LazyLock::new(|| ((_RANGES.len() as f64) / 3.0_f64));
