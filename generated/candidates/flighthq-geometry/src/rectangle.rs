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

// Source: upstream/packages/geometry/src/rectangle.ts:41 (sha256:5f817e1e3216e84781fc3912768ee15c626cb299a8fd10e4854de755949ca720)
pub fn copy_rectangle(out: &mut RectangleLike, source: &RectangleLike) -> () {
    let x = source.x;
    let y = source.y;
    let width = source.width;
    let height = source.height;
    out.x = x;
    out.y = y;
    out.width = width;
    out.height = height;
}

// Source: upstream/packages/geometry/src/rectangle.ts:52 (sha256:d585f8b093093aab13f9eb2eef5a7f4b9e0f14f50ac7637022c0372b8dd9f8a1)
pub fn create_rectangle(
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
) -> Rectangle {
    return create_entity(Some(Rectangle {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: (x).unwrap_or(0.0_f64),
        y: (y).unwrap_or(0.0_f64),
        width: (width).unwrap_or(0.0_f64),
        height: (height).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/geometry/src/rectangle.ts:61 (sha256:dda3c5c00f57b80a10638483e0f691ff8ad6e4b13d95fdf6a9bd126f898a6e48)
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

// Source: upstream/packages/geometry/src/rectangle.ts:76 (sha256:9a1ef61327a5deef2fd78ca383d8aa2dc7d44eff3dec24b0748fef64c76e4943)
pub fn equals_rectangle(a: &Option<RectangleLike>, b: &Option<RectangleLike>) -> bool {
    if ((a).is_none()) || ((b).is_none()) {
        return false;
    }
    return (a == b)
        || ((((a.as_ref().unwrap().x == b.as_ref().unwrap().x)
            && (a.as_ref().unwrap().y == b.as_ref().unwrap().y))
            && (a.as_ref().unwrap().width == b.as_ref().unwrap().width))
            && (a.as_ref().unwrap().height == b.as_ref().unwrap().height));
}

// Source: upstream/packages/geometry/src/rectangle.ts:86 (sha256:8278becb9b1f93618301278db95098204ca26b0a742c983c9a3040116d0a47fe)
pub fn expand_rectangle_to_point(
    out: &mut RectangleLike,
    source_rect: &RectangleLike,
    source_vec2: &Vector2Like,
) -> () {
    let min_x = ((source_rect.x).min((source_rect.x + source_rect.width))).min(source_vec2.x);
    let max_x = ((source_rect.x).max((source_rect.x + source_rect.width))).max(source_vec2.x);
    let min_y = ((source_rect.y).min((source_rect.y + source_rect.height))).min(source_vec2.y);
    let max_y = ((source_rect.y).max((source_rect.y + source_rect.height))).max(source_vec2.y);
    out.x = min_x;
    out.y = min_y;
    out.width = (max_x - min_x);
    out.height = (max_y - min_y);
}

// Source: upstream/packages/geometry/src/rectangle.ts:101 (sha256:7302e83f5730c06dc17e9e2eabc7773c3b981287ce639f88dc128d1de059d081)
pub fn get_rectangle_bottom(source: &RectangleLike) -> f64 {
    return (source.y + source.height);
}

// Source: upstream/packages/geometry/src/rectangle.ts:108 (sha256:12a03250f563b93366666b5ec6b483a0176af7fc02d6596f65ed49bc2eb863b3)
pub fn get_rectangle_bottom_right(out: &mut Vector2Like, source: &RectangleLike) -> () {
    let x = (source.x + source.width);
    let y = (source.y + source.height);
    out.x = x;
    out.y = y;
}

// Source: upstream/packages/geometry/src/rectangle.ts:115 (sha256:dda5531aa995eb0425444eebeafe66bee4e8fdcc46580420c08e97296098f30b)
pub fn get_rectangle_left(source: &RectangleLike) -> f64 {
    return source.x;
}

// Source: upstream/packages/geometry/src/rectangle.ts:119 (sha256:70a00e3168648de8f28e2caa786098cf77901ebbfd57a34ea1c32867d5d3a0c7)
pub fn get_rectangle_max_x(source: &RectangleLike) -> f64 {
    return (source.x).max((source.x + source.width));
}

// Source: upstream/packages/geometry/src/rectangle.ts:123 (sha256:7ed5b6fd009b95e3e94596e379c063618ae44e70b6a7c79cf26b724329f968b7)
pub fn get_rectangle_max_y(source: &RectangleLike) -> f64 {
    return (source.y).max((source.y + source.height));
}

// Source: upstream/packages/geometry/src/rectangle.ts:127 (sha256:925295f580d1f2fd6dcc97cfc2453033955c3059dbb7a91fd8e9518d45f08f20)
pub fn get_rectangle_min_x(source: &RectangleLike) -> f64 {
    return (source.x).min((source.x + source.width));
}

// Source: upstream/packages/geometry/src/rectangle.ts:131 (sha256:fc1599ea73f09fe025d4580a448469be342bfab9affd1e1e20bf5f4b6ed6d8ec)
pub fn get_rectangle_min_y(source: &RectangleLike) -> f64 {
    return (source.y).min((source.y + source.height));
}

// Source: upstream/packages/geometry/src/rectangle.ts:135 (sha256:acb1df821a0ae86a6c057863fc368ca075f8d36b8d21223c224f517ebb9b35dc)
pub fn get_rectangle_normalized_bottom_right(out: &mut Vector2Like, source: &RectangleLike) -> () {
    let x = get_rectangle_max_x(source);
    let y = get_rectangle_max_y(source);
    out.x = x;
    out.y = y;
}

// Source: upstream/packages/geometry/src/rectangle.ts:142 (sha256:9339bd30e2dd6d902b212c7ecde6d37e509b853d332ff340ce143fb2bc9d495d)
pub fn get_rectangle_normalized_top_left(out: &mut Vector2Like, source: &RectangleLike) -> () {
    let x = get_rectangle_min_x(source);
    let y = get_rectangle_min_y(source);
    out.x = x;
    out.y = y;
}

// Source: upstream/packages/geometry/src/rectangle.ts:149 (sha256:702e960de1639bd536c4e1d82884b94dffda2370edc518bb2f4cbc0450b4ce90)
pub fn get_rectangle_right(source: &RectangleLike) -> f64 {
    return (source.x + source.width);
}

// Source: upstream/packages/geometry/src/rectangle.ts:156 (sha256:b46e0624c24bc68a622998d11b615e42168d89a825c989608fdd0af6172e0f33)
pub fn get_rectangle_size(out: &mut Vector2Like, source: &RectangleLike) -> () {
    let width = source.width;
    let height = source.height;
    out.x = width;
    out.y = height;
}

// Source: upstream/packages/geometry/src/rectangle.ts:163 (sha256:46615f03a0fa4b8d2d6e64b3f05d362eca40a955a771e1473110992e04b374cf)
pub fn get_rectangle_top(source: &RectangleLike) -> f64 {
    return source.y;
}

// Source: upstream/packages/geometry/src/rectangle.ts:170 (sha256:782d5626a026485cbec7055f3887ee7ecf7b358e7fa85559b42b51bf2e89156f)
pub fn get_rectangle_top_left(out: &mut Vector2Like, source: &RectangleLike) -> () {
    let x = source.x;
    let y = source.y;
    out.x = x;
    out.y = y;
}

// Source: upstream/packages/geometry/src/rectangle.ts:177 (sha256:3a2f22160fa837237bcd8f1f22267ee70918a2264537e7590eab7a4232761bb9)
pub fn inflate_rectangle(out: &mut RectangleLike, source: &RectangleLike, dx: f64, dy: f64) -> () {
    let x = source.x;
    let y = source.y;
    let width = source.width;
    let height = source.height;
    out.x = (x - dx);
    out.width = (width + (dx * 2.0_f64));
    out.y = (y - dy);
    out.height = (height + (dy * 2.0_f64));
}

// Source: upstream/packages/geometry/src/rectangle.ts:188 (sha256:e45f92f942d98428fadbad6099c49a798da4ccba8fac82a5ec000500f526817f)
pub fn intersects_rectangle(a: &RectangleLike, b: &RectangleLike) -> bool {
    return (!(((get_rectangle_max_x(a) <= get_rectangle_min_x(b))
        || (get_rectangle_min_x(a) >= get_rectangle_max_x(b)))
        || (get_rectangle_max_y(a) <= get_rectangle_min_y(b)))
        || (get_rectangle_min_y(a) >= get_rectangle_max_y(b)));
}

// Source: upstream/packages/geometry/src/rectangle.ts:202 (sha256:bd8eb1bd33a7be488dac074a5e697de92dff88b083b6b60d54e6eb2a2512e9ab)
pub fn is_empty_rectangle(source: &RectangleLike) -> bool {
    return (source.width == 0.0_f64) || (source.height == 0.0_f64);
}

// Source: upstream/packages/geometry/src/rectangle.ts:206 (sha256:7613b27281770b8334fff6fd8cc64e1764e1f4a5fdb034c0516609a42eb87bdd)
pub fn is_flipped_x_rectangle(source: &RectangleLike) -> bool {
    return (source.width < 0.0_f64);
}

// Source: upstream/packages/geometry/src/rectangle.ts:210 (sha256:a920fd210d65a81e22a4e22d7e123d86cce1b3c7250b5d0459999b533173bf1b)
pub fn is_flipped_y_rectangle(source: &RectangleLike) -> bool {
    return (source.height < 0.0_f64);
}

// Source: upstream/packages/geometry/src/rectangle.ts:214 (sha256:21754bda3e8709a1e7022b2c21a5bd0142dd3dcb226d7adfe0ae2cf2bd5da1c2)
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
        let x0 = (source_left).min(other_left);
        let x1 = (source_right).max(other_right);
        let y0 = (source_top).min(other_top);
        let y1 = (source_bottom).max(other_bottom);
        out.x = x0;
        out.y = y0;
        out.width = (x1 - x0);
        out.height = (y1 - y0);
    }
}

// Source: upstream/packages/geometry/src/rectangle.ts:252 (sha256:32251109cf8dd2a837792236f814942f7e628c0324c0b0c2479c279c99307960)
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

// Source: upstream/packages/geometry/src/rectangle.ts:263 (sha256:aafc96338ce9db6483e0cbc9977d7daab72487fc21742f290a1508fd16d371ff)
pub fn offset_rectangle(out: &mut RectangleLike, source: &RectangleLike, dx: f64, dy: f64) -> () {
    let x = source.x;
    let y = source.y;
    let width = source.width;
    let height = source.height;
    out.x = (x + dx);
    out.y = (y + dy);
    out.width = width;
    out.height = height;
}

// Source: upstream/packages/geometry/src/rectangle.ts:274 (sha256:3f267eae645e28db26e58af014172da0cfc31443a575aab13f5c19e2f674b0b4)
pub fn offset_rectangle_by_point(
    out: &mut RectangleLike,
    source: &RectangleLike,
    point: &Vector2Like,
) -> () {
    let x = source.x;
    let y = source.y;
    let width = source.width;
    let height = source.height;
    let point_x = point.x;
    let point_y = point.y;
    out.x = (x + point_x);
    out.y = (y + point_y);
    out.width = width;
    out.height = height;
}

// Source: upstream/packages/geometry/src/rectangle.ts:291 (sha256:4e18a0c636a86952d1846e00dc993b074fbbb51b38e924acf87532b92d236040)
pub fn set_empty_rectangle(out: &mut RectangleLike) -> () {
    out.x = {
        out.y = {
            out.width = {
                out.height = 0.0_f64;
                out.height.clone()
            };
            out.width.clone()
        };
        out.y.clone()
    };
}

// Source: upstream/packages/geometry/src/rectangle.ts:295 (sha256:7b90aeff26cea56e394697bfcb554054f79ce25e91d659b6b415de43caec3b85)
pub fn set_rectangle(out: &mut RectangleLike, x: f64, y: f64, width: f64, height: f64) -> () {
    out.x = x;
    out.y = y;
    out.width = width;
    out.height = height;
}

// Source: upstream/packages/geometry/src/rectangle.ts:302 (sha256:81a3ddab20cafc695b7f1cc299ba5085e704691f2fdc20cd05194710e29d0aea)
pub fn set_rectangle_bottom(target: &mut RectangleLike, value: f64) -> () {
    let y = target.y;
    target.height = (value - y);
}

// Source: upstream/packages/geometry/src/rectangle.ts:307 (sha256:ec1b2ee4e7923f0706b2c7be03473eba1e2fd37db3fef35bf126c3899c3c026e)
pub fn set_rectangle_bottom_right(target: &mut RectangleLike, point: &Vector2Like) -> () {
    let x = target.x;
    let y = target.y;
    let point_x = point.x;
    let point_y = point.y;
    target.width = (point_x - x);
    target.height = (point_y - y);
}

// Source: upstream/packages/geometry/src/rectangle.ts:316 (sha256:98e878081244350bce8748b2fd3575f1a14747b720187417329c28c0dc418dc0)
pub fn set_rectangle_left(target: &mut RectangleLike, value: f64) -> () {
    let x = target.x;
    let width = target.width;
    target.width = (width - (value - x));
    target.x = value;
}

// Source: upstream/packages/geometry/src/rectangle.ts:323 (sha256:4d2ad0bc4c037fbcc58b8bfb90d42b22900272f289774b2ef92494698cd01e27)
pub fn set_rectangle_right(target: &mut RectangleLike, value: f64) -> () {
    let x = target.x;
    target.width = (value - x);
}

// Source: upstream/packages/geometry/src/rectangle.ts:328 (sha256:d55877bf9a728dc3143b4fcf77b533543ffd663ce099147a85d1faa05263cd1f)
pub fn set_rectangle_size(out: &mut RectangleLike, size: &Vector2Like) -> () {
    let width = size.x;
    let height = size.y;
    out.width = width;
    out.height = height;
}

// Source: upstream/packages/geometry/src/rectangle.ts:335 (sha256:5fbcaacacf1bcc8b7c1c751c48891249ed1a00f487c404cb8e5116927313ac5b)
pub fn set_rectangle_top(target: &mut RectangleLike, value: f64) -> () {
    let y = target.y;
    let height = target.height;
    target.height = (height - (value - y));
    target.y = value;
}

// Source: upstream/packages/geometry/src/rectangle.ts:342 (sha256:6de367635ee51419303c8678d6bad4d5f264a6aa44df8de630171f5ad9cdb74c)
pub fn set_rectangle_top_left(out: &mut RectangleLike, point: &Vector2Like) -> () {
    let x = point.x;
    let y = point.y;
    out.x = x;
    out.y = y;
}
