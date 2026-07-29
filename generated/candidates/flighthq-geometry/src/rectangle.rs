// @generated from upstream/packages/geometry/src/rectangle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{Rectangle, RectangleLike, Vector2Like};

// Source: upstream/packages/geometry/src/rectangle.ts:4 (sha256:2d5ac4ae8427a00a32cb6bc51f51f2a4fede950ad7d17fe10a95658ce424b9a1)
pub fn clone_rectangle(source: &RectangleLike) -> Rectangle {
    return create_rectangle(
        Some(source.x),
        Some(source.y),
        Some(source.width),
        Some(source.height),
    );
}

// Source: upstream/packages/geometry/src/rectangle.ts:8 (sha256:2b21c297c07a0b5d9ebefdb8c38c943b35ff215c878bec5d1a6f06b53246245a)
pub fn compute_rectangle_intersection(
    out: &mut RectangleLike,
    a: &RectangleLike,
    b: &RectangleLike,
) -> () {
    let x0 = (get_rectangle_min_x(a)).max(get_rectangle_min_x(b));
    let x1 = (get_rectangle_max_x(a)).min(get_rectangle_max_x(b));
    let y0 = (get_rectangle_min_y(a)).max(get_rectangle_min_y(b));
    let y1 = (get_rectangle_max_y(a)).min(get_rectangle_max_y(b));
    if (x1 <= x0) || (y1 <= y0) {
        set_empty_rectangle(out);
        return;
    }
    out.x = x0;
    out.y = y0;
    out.width = (x1 - x0);
    out.height = (y1 - y0);
}

// Source: upstream/packages/geometry/src/rectangle.ts:29 (sha256:98e9de19ca09af624ca1bd8f5868620641c0fdf378d93db88c323c12502623cb)
pub fn contains_rectangle_point(source: &RectangleLike, vector: &Vector2Like) -> bool {
    return contains_rectangle_point_xy(source, vector.x, vector.y);
}

// Source: upstream/packages/geometry/src/rectangle.ts:33 (sha256:557713a1fe825b38b08b027820bcc397335caddfda7ddcae471f0e554650e0fb)
pub fn contains_rectangle_point_xy(source: &RectangleLike, x: f64, y: f64) -> bool {
    let x0 = (source.x).min((source.x + source.width));
    let x1 = (source.x).max((source.x + source.width));
    let y0 = (source.y).min((source.y + source.height));
    let y1 = (source.y).max((source.y + source.height));
    return (((x >= x0) && (x < x1)) && (y >= y0)) && (y < y1);
}

// Source: upstream/packages/geometry/src/rectangle.ts:41 (sha256:3bc19773ab1c078e7a8b71ddc97decb01daf6f8bd54038ccdb2d40efe7474f77)
pub fn copy_rectangle(out: &mut RectangleLike, source: &RectangleLike) -> () {
    if (out != source) {
        out.x = source.x;
        out.y = source.y;
        out.width = source.width;
        out.height = source.height;
    }
}

// Source: upstream/packages/geometry/src/rectangle.ts:50 (sha256:d585f8b093093aab13f9eb2eef5a7f4b9e0f14f50ac7637022c0372b8dd9f8a1)
pub fn create_rectangle(
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
) -> Rectangle {
    return create_entity(Some(Rectangle {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        x: (x).unwrap_or(0.0_f64),
        y: (y).unwrap_or(0.0_f64),
        width: (width).unwrap_or(0.0_f64),
        height: (height).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/geometry/src/rectangle.ts:59 (sha256:dda3c5c00f57b80a10638483e0f691ff8ad6e4b13d95fdf6a9bd126f898a6e48)
pub fn encloses_rectangle(source: &RectangleLike, other: &RectangleLike) -> bool {
    let sx0 = (source.x).min((source.x + source.width));
    let sx1 = (source.x).max((source.x + source.width));
    let sy0 = (source.y).min((source.y + source.height));
    let sy1 = (source.y).max((source.y + source.height));
    let ox0 = (other.x).min((other.x + other.width));
    let ox1 = (other.x).max((other.x + other.width));
    let oy0 = (other.y).min((other.y + other.height));
    let oy1 = (other.y).max((other.y + other.height));
    return (((ox0 >= sx0) && (oy0 >= sy0)) && (ox1 <= sx1)) && (oy1 <= sy1);
}

// Source: upstream/packages/geometry/src/rectangle.ts:74 (sha256:318ce51dcc668fbeef32217962eb9f56d12c88e0ac7284fa6e5b28e645adf386)
pub fn equals_rectangle(a: Option<RectangleLike>, b: Option<RectangleLike>) -> bool {
    if (a == b) {
        return true;
    }
    if ((a).is_none()) || ((b).is_none()) {
        return false;
    }
    return (((a.as_ref().unwrap().x == b.as_ref().unwrap().x)
        && (a.as_ref().unwrap().y == b.as_ref().unwrap().y))
        && (a.as_ref().unwrap().width == b.as_ref().unwrap().width))
        && (a.as_ref().unwrap().height == b.as_ref().unwrap().height);
}

// Source: upstream/packages/geometry/src/rectangle.ts:83 (sha256:09621264efc2eb178e97a06f4f79de836c35ea82f8f97ea70caee2b3bf7cda9f)
pub fn expand_rectangle_to_point(
    out: &mut RectangleLike,
    source_rect: &RectangleLike,
    source_vec2: &Vector2Like,
) -> () {
    inflate_rectangle(out, source_rect, source_vec2.x, source_vec2.y);
}

// Source: upstream/packages/geometry/src/rectangle.ts:91 (sha256:7302e83f5730c06dc17e9e2eabc7773c3b981287ce639f88dc128d1de059d081)
pub fn get_rectangle_bottom(source: &RectangleLike) -> f64 {
    return (source.y + source.height);
}

// Source: upstream/packages/geometry/src/rectangle.ts:98 (sha256:ba64289fa3f5b0d7a08a73c1b7e9112db32fe0700d1c2ec291db4ab4199abdc2)
pub fn get_rectangle_bottom_right(out: &mut Vector2Like, source: &RectangleLike) -> () {
    out.x = (source.x + source.width);
    out.y = (source.y + source.height);
}

// Source: upstream/packages/geometry/src/rectangle.ts:103 (sha256:dda5531aa995eb0425444eebeafe66bee4e8fdcc46580420c08e97296098f30b)
pub fn get_rectangle_left(source: &RectangleLike) -> f64 {
    return source.x;
}

// Source: upstream/packages/geometry/src/rectangle.ts:107 (sha256:70a00e3168648de8f28e2caa786098cf77901ebbfd57a34ea1c32867d5d3a0c7)
pub fn get_rectangle_max_x(source: &RectangleLike) -> f64 {
    return (source.x).max((source.x + source.width));
}

// Source: upstream/packages/geometry/src/rectangle.ts:111 (sha256:7ed5b6fd009b95e3e94596e379c063618ae44e70b6a7c79cf26b724329f968b7)
pub fn get_rectangle_max_y(source: &RectangleLike) -> f64 {
    return (source.y).max((source.y + source.height));
}

// Source: upstream/packages/geometry/src/rectangle.ts:115 (sha256:925295f580d1f2fd6dcc97cfc2453033955c3059dbb7a91fd8e9518d45f08f20)
pub fn get_rectangle_min_x(source: &RectangleLike) -> f64 {
    return (source.x).min((source.x + source.width));
}

// Source: upstream/packages/geometry/src/rectangle.ts:119 (sha256:fc1599ea73f09fe025d4580a448469be342bfab9affd1e1e20bf5f4b6ed6d8ec)
pub fn get_rectangle_min_y(source: &RectangleLike) -> f64 {
    return (source.y).min((source.y + source.height));
}

// Source: upstream/packages/geometry/src/rectangle.ts:123 (sha256:095ce4313ecdec32d56144a3b18c19854542c01b778faa37cd7b34274f443b4a)
pub fn get_rectangle_normalized_bottom_right(out: &mut Vector2Like, source: &RectangleLike) -> () {
    out.x = get_rectangle_max_x(source);
    out.y = get_rectangle_max_y(source);
}

// Source: upstream/packages/geometry/src/rectangle.ts:128 (sha256:99f23250672980b0e05187ee2e57e702e1e7cc103e15c9caa4b915631125b579)
pub fn get_rectangle_normalized_top_left(out: &mut Vector2Like, source: &RectangleLike) -> () {
    out.x = get_rectangle_min_x(source);
    out.y = get_rectangle_min_y(source);
}

// Source: upstream/packages/geometry/src/rectangle.ts:133 (sha256:702e960de1639bd536c4e1d82884b94dffda2370edc518bb2f4cbc0450b4ce90)
pub fn get_rectangle_right(source: &RectangleLike) -> f64 {
    return (source.x + source.width);
}

// Source: upstream/packages/geometry/src/rectangle.ts:140 (sha256:c8e7509ce5c773146683095ca66f49e1b8c56f23439beefd40d2bfd4f0163d91)
pub fn get_rectangle_size(out: &mut Vector2Like, source: &RectangleLike) -> () {
    out.x = source.width;
    out.y = source.height;
}

// Source: upstream/packages/geometry/src/rectangle.ts:145 (sha256:46615f03a0fa4b8d2d6e64b3f05d362eca40a955a771e1473110992e04b374cf)
pub fn get_rectangle_top(source: &RectangleLike) -> f64 {
    return source.y;
}

// Source: upstream/packages/geometry/src/rectangle.ts:152 (sha256:b815a24b71ad4001426977abba13faa4af22d0b71974a690a28ac3ccdd68a82f)
pub fn get_rectangle_top_left(out: &mut Vector2Like, source: &RectangleLike) -> () {
    out.x = source.x;
    out.y = source.y;
}

// Source: upstream/packages/geometry/src/rectangle.ts:157 (sha256:fd4aa420de65ccfb838f4310779e2426eb1acef9ec302c07c6185cd03e67f60e)
pub fn inflate_rectangle(out: &mut RectangleLike, source: &RectangleLike, dx: f64, dy: f64) -> () {
    out.x = (source.x - dx);
    out.width = (source.width + (dx * 2.0_f64));
    out.y = (source.y - dy);
    out.height = (source.height + (dy * 2.0_f64));
}

// Source: upstream/packages/geometry/src/rectangle.ts:164 (sha256:e45f92f942d98428fadbad6099c49a798da4ccba8fac82a5ec000500f526817f)
pub fn intersects_rectangle(a: &RectangleLike, b: &RectangleLike) -> bool {
    return (!(((get_rectangle_max_x(a) <= get_rectangle_min_x(b))
        || (get_rectangle_min_x(a) >= get_rectangle_max_x(b)))
        || (get_rectangle_max_y(a) <= get_rectangle_min_y(b)))
        || (get_rectangle_min_y(a) >= get_rectangle_max_y(b)));
}

// Source: upstream/packages/geometry/src/rectangle.ts:178 (sha256:bd8eb1bd33a7be488dac074a5e697de92dff88b083b6b60d54e6eb2a2512e9ab)
pub fn is_empty_rectangle(source: &RectangleLike) -> bool {
    return (source.width == 0.0_f64) || (source.height == 0.0_f64);
}

// Source: upstream/packages/geometry/src/rectangle.ts:182 (sha256:7613b27281770b8334fff6fd8cc64e1764e1f4a5fdb034c0516609a42eb87bdd)
pub fn is_flipped_x_rectangle(source: &RectangleLike) -> bool {
    return (source.width < 0.0_f64);
}

// Source: upstream/packages/geometry/src/rectangle.ts:186 (sha256:a920fd210d65a81e22a4e22d7e123d86cce1b3c7250b5d0459999b533173bf1b)
pub fn is_flipped_y_rectangle(source: &RectangleLike) -> bool {
    return (source.height < 0.0_f64);
}

// Source: upstream/packages/geometry/src/rectangle.ts:190 (sha256:d6e6963f700bade70becc12267a0ce8807f56cf2f0984d435b78a6afaef5c93c)
pub fn merge_rectangle(
    out: &mut RectangleLike,
    source: &RectangleLike,
    other: &RectangleLike,
) -> () {
    let sx = source.x;
    let sy = source.y;
    let sw = source.width;
    let sh = source.height;
    let ox = other.x;
    let oy = other.y;
    let ow = other.width;
    let oh = other.height;
    let s_empty = (sw == 0.0_f64) || (sh == 0.0_f64);
    let o_empty = (ow == 0.0_f64) || (oh == 0.0_f64);
    if (s_empty) || (o_empty) {
        if (o_empty) && (source == out) {
            return;
        }
        out.x = if o_empty { sx } else { ox };
        out.y = if o_empty { sy } else { oy };
        out.width = if o_empty { sw } else { ow };
        out.height = if o_empty { sh } else { oh };
    } else {
        let source_left = (sx).min((sx + sw));
        let source_right = (sx).max((sx + sw));
        let source_top = (sy).min((sy + sh));
        let source_bottom = (sy).max((sy + sh));
        let other_left = (ox).min((ox + ow));
        let other_right = (ox).max((ox + ow));
        let other_top = (oy).min((oy + oh));
        let other_bottom = (oy).max((oy + oh));
        let mut x0 = (source_left).min(other_left);
        let x1 = (source_right).max(other_right);
        let y0 = (source_top).min(other_top);
        let y1 = (source_bottom).max(other_bottom);
        out.x = x0;
        out.y = y0;
        out.width = (x1 - x0);
        out.height = (y1 - y0);
    }
}

// Source: upstream/packages/geometry/src/rectangle.ts:228 (sha256:32251109cf8dd2a837792236f814942f7e628c0324c0b0c2479c279c99307960)
pub fn normalize_rectangle(out: &mut RectangleLike, source: &RectangleLike) -> () {
    let max_x = get_rectangle_max_x(source);
    let max_y = get_rectangle_max_y(source);
    let min_x = get_rectangle_min_x(source);
    let min_y = get_rectangle_min_y(source);
    out.x = min_x;
    out.y = min_y;
    out.width = (max_x - min_x);
    out.height = (max_y - min_y);
}

// Source: upstream/packages/geometry/src/rectangle.ts:239 (sha256:f72ff57ffb35134aa156f234412843de127d6ad4230a5081beb1eb29c6e74934)
pub fn offset_rectangle(out: &mut RectangleLike, source: &RectangleLike, dx: f64, dy: f64) -> () {
    out.x = (source.x + dx);
    out.y = (source.y + dy);
    out.width = source.width;
    out.height = source.height;
}

// Source: upstream/packages/geometry/src/rectangle.ts:246 (sha256:e95791a3289fca5341ced2b49278816e54f6c089cfec03aabe8acec3d855cff2)
pub fn offset_rectangle_by_point(
    out: &mut RectangleLike,
    source: &RectangleLike,
    point: &Vector2Like,
) -> () {
    out.x = (source.x + point.x);
    out.y = (source.y + point.y);
    out.width = source.width;
    out.height = source.height;
}

// Source: upstream/packages/geometry/src/rectangle.ts:257 (sha256:4e18a0c636a86952d1846e00dc993b074fbbb51b38e924acf87532b92d236040)
pub fn set_empty_rectangle(out: &mut RectangleLike) -> () {
    out.x = {
        out.y = {
            out.width = {
                out.height = 0.0_f64;
                out.height
            };
            out.width
        };
        out.y
    };
}

// Source: upstream/packages/geometry/src/rectangle.ts:261 (sha256:7b90aeff26cea56e394697bfcb554054f79ce25e91d659b6b415de43caec3b85)
pub fn set_rectangle(out: &mut RectangleLike, x: f64, y: f64, width: f64, height: f64) -> () {
    out.x = x;
    out.y = y;
    out.width = width;
    out.height = height;
}

// Source: upstream/packages/geometry/src/rectangle.ts:268 (sha256:bb66a574a4729794986a6ff7f5c5a92be50e8202cb30621fe476050a6d513180)
pub fn set_rectangle_bottom(target: &mut RectangleLike, value: f64) -> () {
    target.height = (value - target.y);
}

// Source: upstream/packages/geometry/src/rectangle.ts:272 (sha256:1083b5a58417291582439a697ec0c161960c1bd45ddbaf73ca8f3da29b4f63f2)
pub fn set_rectangle_bottom_right(target: &mut RectangleLike, point: &Vector2Like) -> () {
    target.width = (point.x - target.x);
    target.height = (point.y - target.y);
}

// Source: upstream/packages/geometry/src/rectangle.ts:277 (sha256:c0169ef0d87f2a4cb99ceee554860d1a6f65b9d3a80e363924b0368985db6cab)
pub fn set_rectangle_left(target: &mut RectangleLike, value: f64) -> () {
    target.width -= (value - target.x);
    target.x = value;
}

// Source: upstream/packages/geometry/src/rectangle.ts:282 (sha256:6d36a242fe9987695cbea7b03ff1202de899414b43a09220dcd9d70b1a19fb92)
pub fn set_rectangle_right(target: &mut RectangleLike, value: f64) -> () {
    target.width = (value - target.x);
}

// Source: upstream/packages/geometry/src/rectangle.ts:286 (sha256:c94e83fd7144ad3d46dece2b2d21d934372f14afe0644feaf82a448fbf59e63d)
pub fn set_rectangle_size(out: &mut RectangleLike, size: &Vector2Like) -> () {
    out.width = size.x;
    out.height = size.y;
}

// Source: upstream/packages/geometry/src/rectangle.ts:291 (sha256:75f577920cee52b97a714084b8a1fe23a55bdab187023c048dabd4a9f3191e36)
pub fn set_rectangle_top(target: &mut RectangleLike, value: f64) -> () {
    target.height -= (value - target.y);
    target.y = value;
}

// Source: upstream/packages/geometry/src/rectangle.ts:296 (sha256:0270b3e3f2e89ea0ee4d4ebda50db320b3d6de3a65f29fbc5ab985c0e28b2ebd)
pub fn set_rectangle_top_left(out: &mut RectangleLike, point: &Vector2Like) -> () {
    out.x = point.x;
    out.y = point.y;
}
