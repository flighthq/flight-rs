// @generated from upstream/packages/geometry/src/aabb.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_vector3;
use flighthq_entity::create_entity;
use flighthq_types::{Aabb, AabbLike, BoundingSphereLike, Matrix4Like, Vector3Like};

// Source: upstream/packages/geometry/src/aabb.ts:6 (sha256:7fa47872b1035f8f05e71755eacfaeca4290f6be3ac7875faede403049e4295e)
pub fn clone_aabb(source: &AabbLike) -> Aabb {
    return create_aabb(
        Some(source.min.x),
        Some(source.min.y),
        Some(source.min.z),
        Some(source.max.x),
        Some(source.max.y),
        Some(source.max.z),
    );
}

// Source: upstream/packages/geometry/src/aabb.ts:13 (sha256:e6e41ae435a619cce2f848b9b53ac35f0e1984974bb83e2de4261fd09b91693f)
pub fn contains_aabb_point(aabb: &AabbLike, point: &Vector3Like) -> bool {
    return (((((point.x >= aabb.min.x) && (point.x <= aabb.max.x)) && (point.y >= aabb.min.y))
        && (point.y <= aabb.max.y))
        && (point.z >= aabb.min.z))
        && (point.z <= aabb.max.z);
}

// Source: upstream/packages/geometry/src/aabb.ts:29 (sha256:0911094b07e530b7ade0d4479f90c02292f612b76e02cdddaab42a8e55fc8569)
pub fn copy_aabb(out: &mut AabbLike, source: &AabbLike) -> () {
    let min_x = source.min.x;
    let min_y = source.min.y;
    let min_z = source.min.z;
    let max_x = source.max.x;
    let max_y = source.max.y;
    let max_z = source.max.z;
    out.min.x = min_x;
    out.min.y = min_y;
    out.min.z = min_z;
    out.max.x = max_x;
    out.max.y = max_y;
    out.max.z = max_z;
}

// Source: upstream/packages/geometry/src/aabb.ts:49 (sha256:ce5d5e5a9573c5248c8defd4ce57b13b3e0eeee1e64327b82e5d51974f667282)
pub fn create_aabb(
    min_x: Option<f64>,
    min_y: Option<f64>,
    min_z: Option<f64>,
    max_x: Option<f64>,
    max_y: Option<f64>,
    max_z: Option<f64>,
) -> Aabb {
    let min = create_vector3(
        Some((min_x).clone().unwrap_or(f64::INFINITY)),
        Some((min_y).clone().unwrap_or(f64::INFINITY)),
        Some((min_z).clone().unwrap_or(f64::INFINITY)),
    );
    let max = create_vector3(
        Some((max_x).clone().unwrap_or(f64::NEG_INFINITY)),
        Some((max_y).clone().unwrap_or(f64::NEG_INFINITY)),
        Some((max_z).clone().unwrap_or(f64::NEG_INFINITY)),
    );
    return create_entity(Some(Aabb {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        max: (max).clone(),
        min: (min).clone(),
    }));
}

// Source: upstream/packages/geometry/src/aabb.ts:76 (sha256:4d09cc0a9db5ebb436c397f7a42ae02229045d2ab10e394eac24a52c72fd05fe)
pub fn expand_aabb_by_point(out: &mut AabbLike, aabb: &AabbLike, point: &Vector3Like) -> () {
    let min_x = aabb.min.x;
    let min_y = aabb.min.y;
    let min_z = aabb.min.z;
    let max_x = aabb.max.x;
    let max_y = aabb.max.y;
    let max_z = aabb.max.z;
    let px = point.x;
    let py = point.y;
    let pz = point.z;
    out.min.x = (min_x).min(px);
    out.min.y = (min_y).min(py);
    out.min.z = (min_z).min(pz);
    out.max.x = (max_x).max(px);
    out.max.y = (max_y).max(py);
    out.max.z = (max_z).max(pz);
}

// Source: upstream/packages/geometry/src/aabb.ts:100 (sha256:dfafdc2a0a67d9bf914d9f61d600f0cddd842e35747f030a0201d85e054c4243)
pub fn expand_aabb_by_sphere(
    out: &mut AabbLike,
    aabb: &AabbLike,
    sphere: &BoundingSphereLike,
) -> () {
    let min_x = aabb.min.x;
    let min_y = aabb.min.y;
    let min_z = aabb.min.z;
    let max_x = aabb.max.x;
    let max_y = aabb.max.y;
    let max_z = aabb.max.z;
    let cx = sphere.center.x;
    let cy = sphere.center.y;
    let cz = sphere.center.z;
    let radius = sphere.radius;
    if (radius < 0.0_f64) {
        out.min.x = min_x;
        out.min.y = min_y;
        out.min.z = min_z;
        out.max.x = max_x;
        out.max.y = max_y;
        out.max.z = max_z;
        return;
    }
    out.min.x = (min_x).min((cx - radius));
    out.min.y = (min_y).min((cy - radius));
    out.min.z = (min_z).min((cz - radius));
    out.max.x = (max_x).max((cx + radius));
    out.max.y = (max_y).max((cy + radius));
    out.max.z = (max_z).max((cz + radius));
}

// Source: upstream/packages/geometry/src/aabb.ts:136 (sha256:962205b28ca073d110df29fb200f5ed05716c66465acf308de7c63766f5035ed)
pub fn get_aabb_center(out: &mut Vector3Like, aabb: &AabbLike) -> () {
    let x = ((aabb.min.x + aabb.max.x) * 0.5_f64);
    let y = ((aabb.min.y + aabb.max.y) * 0.5_f64);
    let z = ((aabb.min.z + aabb.max.z) * 0.5_f64);
    out.x = x;
    out.y = y;
    out.z = z;
}

// Source: upstream/packages/geometry/src/aabb.ts:148 (sha256:95f6d7f67b3c25959c9e1e20c6971167a77958801d191e487643acca04c493e9)
pub fn get_aabb_extents(out: &mut Vector3Like, aabb: &AabbLike) -> () {
    let x = ((aabb.max.x - aabb.min.x) * 0.5_f64);
    let y = ((aabb.max.y - aabb.min.y) * 0.5_f64);
    let z = ((aabb.max.z - aabb.min.z) * 0.5_f64);
    out.x = x;
    out.y = y;
    out.z = z;
}

// Source: upstream/packages/geometry/src/aabb.ts:160 (sha256:1f207f93cd2abbb148fb1ed54bca30d8e7fef5526771e54b88262078f8fb1add)
pub fn get_aabb_size(out: &mut Vector3Like, aabb: &AabbLike) -> () {
    let x = (aabb.max.x - aabb.min.x);
    let y = (aabb.max.y - aabb.min.y);
    let z = (aabb.max.z - aabb.min.z);
    out.x = x;
    out.y = y;
    out.z = z;
}

// Source: upstream/packages/geometry/src/aabb.ts:177 (sha256:5f309e87713171fca2c9f16074d9eede1c2cff697ee3e93806b720cb3629d0df)
pub fn get_closest_point_on_aabb(
    out: &mut Vector3Like,
    aabb: &AabbLike,
    point: &Vector3Like,
) -> () {
    let px = point.x;
    let py = point.y;
    let pz = point.z;
    let min_x = aabb.min.x;
    let min_y = aabb.min.y;
    let min_z = aabb.min.z;
    let max_x = aabb.max.x;
    let max_y = aabb.max.y;
    let max_z = aabb.max.z;
    out.x = ((px).max(min_x)).min(max_x);
    out.y = ((py).max(min_y)).min(max_y);
    out.z = ((pz).max(min_z)).min(max_z);
}

// Source: upstream/packages/geometry/src/aabb.ts:198 (sha256:ea0c23cbae15c408facbffe240ec09491884b28d22c2741332a22097fc424835)
pub fn intersect_aabb(out: &mut AabbLike, a: &AabbLike, b: &AabbLike) -> () {
    let a_min_x = a.min.x;
    let a_min_y = a.min.y;
    let a_min_z = a.min.z;
    let a_max_x = a.max.x;
    let a_max_y = a.max.y;
    let a_max_z = a.max.z;
    let b_min_x = b.min.x;
    let b_min_y = b.min.y;
    let b_min_z = b.min.z;
    let b_max_x = b.max.x;
    let b_max_y = b.max.y;
    let b_max_z = b.max.z;
    out.min.x = (a_min_x).max(b_min_x);
    out.min.y = (a_min_y).max(b_min_y);
    out.min.z = (a_min_z).max(b_min_z);
    out.max.x = (a_max_x).min(b_max_x);
    out.max.y = (a_max_y).min(b_max_y);
    out.max.z = (a_max_z).min(b_max_z);
}

// Source: upstream/packages/geometry/src/aabb.ts:223 (sha256:bd74580a00143833a6e0c39b047305cbb9888e88f83b4950b0a0fd11259c3154)
pub fn is_aabb_intersecting_aabb(a: &AabbLike, b: &AabbLike) -> bool {
    if (((((a.min.x > a.max.x) || (a.min.z > a.max.z)) || (a.min.y > a.max.y))
        || (b.min.x > b.max.x))
        || (b.min.y > b.max.y))
        || (b.min.z > b.max.z)
    {
        return false;
    }
    return (((((a.min.x <= b.max.x) && (a.max.x >= b.min.x)) && (a.min.y <= b.max.y))
        && (a.max.y >= b.min.y))
        && (a.min.z <= b.max.z))
        && (a.max.z >= b.min.z);
}

// Source: upstream/packages/geometry/src/aabb.ts:247 (sha256:bf20fcec218a50b50fabd1f31b8a22b774ad881dcc77a19e5d5eb71382195713)
pub fn set_aabb(
    out: &mut AabbLike,
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
) -> () {
    out.min.x = min_x;
    out.min.y = min_y;
    out.min.z = min_z;
    out.max.x = max_x;
    out.max.y = max_y;
    out.max.z = max_z;
}

// Source: upstream/packages/geometry/src/aabb.ts:268 (sha256:db5c67bbad1cfa4f280d9a73e07e1d1bab8c5016ddc6fcb5c3aa988c8ac5d3b8)
pub fn set_aabb_from_points(out: &mut AabbLike, points: &Vec<Vector3Like>) -> () {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut max_z = f64::NEG_INFINITY;
    {
        let mut i = 0.0_f64;
        while (i < (points.len() as f64)) {
            let p = points[i as usize].clone();
            if (p.x < min_x) {
                min_x = p.x;
            }
            if (p.y < min_y) {
                min_y = p.y;
            }
            if (p.z < min_z) {
                min_z = p.z;
            }
            if (p.x > max_x) {
                max_x = p.x;
            }
            if (p.y > max_y) {
                max_y = p.y;
            }
            if (p.z > max_z) {
                max_z = p.z;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    out.min.x = min_x;
    out.min.y = min_y;
    out.min.z = min_z;
    out.max.x = max_x;
    out.max.y = max_y;
    out.max.z = max_z;
}

// Source: upstream/packages/geometry/src/aabb.ts:301 (sha256:dbcfe9a8f5f3f90242f6beca4eb42de2b342e481f1932d287ab46707a680f292)
pub fn transform_aabb_by_matrix4(out: &mut AabbLike, aabb: &AabbLike, m: &Matrix4Like) -> () {
    let min_x = aabb.min.x;
    let min_y = aabb.min.y;
    let min_z = aabb.min.z;
    let max_x = aabb.max.x;
    let max_y = aabb.max.y;
    let max_z = aabb.max.z;
    let cx = ((min_x + max_x) * 0.5_f64);
    let cy = ((min_y + max_y) * 0.5_f64);
    let cz = ((min_z + max_z) * 0.5_f64);
    let ex = ((max_x - min_x) * 0.5_f64);
    let ey = ((max_y - min_y) * 0.5_f64);
    let ez = ((max_z - min_z) * 0.5_f64);
    let tcx = (((((m.m[0.0_f64 as usize] as f64) * cx) + ((m.m[4.0_f64 as usize] as f64) * cy))
        + ((m.m[8.0_f64 as usize] as f64) * cz))
        + (m.m[12.0_f64 as usize] as f64));
    let tcy = (((((m.m[1.0_f64 as usize] as f64) * cx) + ((m.m[5.0_f64 as usize] as f64) * cy))
        + ((m.m[9.0_f64 as usize] as f64) * cz))
        + (m.m[13.0_f64 as usize] as f64));
    let tcz = (((((m.m[2.0_f64 as usize] as f64) * cx) + ((m.m[6.0_f64 as usize] as f64) * cy))
        + ((m.m[10.0_f64 as usize] as f64) * cz))
        + (m.m[14.0_f64 as usize] as f64));
    let tex = ((((m.m[0.0_f64 as usize] as f64).abs() * ex)
        + ((m.m[4.0_f64 as usize] as f64).abs() * ey))
        + ((m.m[8.0_f64 as usize] as f64).abs() * ez));
    let tey = ((((m.m[1.0_f64 as usize] as f64).abs() * ex)
        + ((m.m[5.0_f64 as usize] as f64).abs() * ey))
        + ((m.m[9.0_f64 as usize] as f64).abs() * ez));
    let tez = ((((m.m[2.0_f64 as usize] as f64).abs() * ex)
        + ((m.m[6.0_f64 as usize] as f64).abs() * ey))
        + ((m.m[10.0_f64 as usize] as f64).abs() * ez));
    out.min.x = (tcx - tex);
    out.min.y = (tcy - tey);
    out.min.z = (tcz - tez);
    out.max.x = (tcx + tex);
    out.max.y = (tcy + tey);
    out.max.z = (tcz + tez);
}

// Source: upstream/packages/geometry/src/aabb.ts:340 (sha256:386275cdc4044862961fd7c0f90ec5f7be258302b4788d3f34e1c79b5185f151)
pub fn union_aabb(out: &mut AabbLike, a: &AabbLike, b: &AabbLike) -> () {
    let a_min_x = a.min.x;
    let a_min_y = a.min.y;
    let a_min_z = a.min.z;
    let a_max_x = a.max.x;
    let a_max_y = a.max.y;
    let a_max_z = a.max.z;
    let b_min_x = b.min.x;
    let b_min_y = b.min.y;
    let b_min_z = b.min.z;
    let b_max_x = b.max.x;
    let b_max_y = b.max.y;
    let b_max_z = b.max.z;
    out.min.x = (a_min_x).min(b_min_x);
    out.min.y = (a_min_y).min(b_min_y);
    out.min.z = (a_min_z).min(b_min_z);
    out.max.x = (a_max_x).max(b_max_x);
    out.max.y = (a_max_y).max(b_max_y);
    out.max.z = (a_max_z).max(b_max_z);
}
