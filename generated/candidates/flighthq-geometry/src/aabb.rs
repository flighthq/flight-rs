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

// Source: upstream/packages/geometry/src/aabb.ts:29 (sha256:b3d7db6f9971fb9b3b9d482eb1c0d73c7493a2c241e2bfe739571d6e8433bd5d)
pub fn copy_aabb(out: &mut AabbLike, source: &AabbLike) -> () {
    out.min.x = source.min.x;
    out.min.y = source.min.y;
    out.min.z = source.min.z;
    out.max.x = source.max.x;
    out.max.y = source.max.y;
    out.max.z = source.max.z;
}

// Source: upstream/packages/geometry/src/aabb.ts:43 (sha256:ce5d5e5a9573c5248c8defd4ce57b13b3e0eeee1e64327b82e5d51974f667282)
pub fn create_aabb(
    min_x: Option<f64>,
    min_y: Option<f64>,
    min_z: Option<f64>,
    max_x: Option<f64>,
    max_y: Option<f64>,
    max_z: Option<f64>,
) -> Aabb {
    let min = create_vector3(
        Some((min_x).unwrap_or(f64::INFINITY)),
        Some((min_y).unwrap_or(f64::INFINITY)),
        Some((min_z).unwrap_or(f64::INFINITY)),
    );
    let max = create_vector3(
        Some((max_x).unwrap_or(f64::NEG_INFINITY)),
        Some((max_y).unwrap_or(f64::NEG_INFINITY)),
        Some((max_z).unwrap_or(f64::NEG_INFINITY)),
    );
    return create_entity(Some(Aabb {
        __flight_identity: std::sync::Arc::new(()),
        max: (max).clone(),
        min: (min).clone(),
    }));
}

// Source: upstream/packages/geometry/src/aabb.ts:70 (sha256:4ace21a55378c5bcf62bd92cf00a5b94c7555014600adf97d5bc1940a17fa7e3)
pub fn expand_aabb_by_point(out: &mut AabbLike, aabb: &AabbLike, point: &Vector3Like) -> () {
    let px = point.x;
    let py = point.y;
    let pz = point.z;
    out.min.x = (aabb.min.x).min(px);
    out.min.y = (aabb.min.y).min(py);
    out.min.z = (aabb.min.z).min(pz);
    out.max.x = (aabb.max.x).max(px);
    out.max.y = (aabb.max.y).max(py);
    out.max.z = (aabb.max.z).max(pz);
}

// Source: upstream/packages/geometry/src/aabb.ts:88 (sha256:d61254e59641c6fa4957b519af38dcdb4783ad7fbb99e709553ff132bab4e1ad)
pub fn expand_aabb_by_sphere(
    out: &mut AabbLike,
    aabb: &AabbLike,
    sphere: &BoundingSphereLike,
) -> () {
    if (sphere.radius < 0.0_f64) {
        out.min.x = aabb.min.x;
        out.min.y = aabb.min.y;
        out.min.z = aabb.min.z;
        out.max.x = aabb.max.x;
        out.max.y = aabb.max.y;
        out.max.z = aabb.max.z;
        return;
    }
    let cx = sphere.center.x;
    let cy = sphere.center.y;
    let cz = sphere.center.z;
    let r = sphere.radius;
    out.min.x = (aabb.min.x).min((cx - r));
    out.min.y = (aabb.min.y).min((cy - r));
    out.min.z = (aabb.min.z).min((cz - r));
    out.max.x = (aabb.max.x).max((cx + r));
    out.max.y = (aabb.max.y).max((cy + r));
    out.max.z = (aabb.max.z).max((cz + r));
}

// Source: upstream/packages/geometry/src/aabb.ts:118 (sha256:4633ef41ca57c08df6e05a4a3c9746b2d720ee8936df186f0f99c5b9fcab720a)
pub fn get_aabb_center(out: &mut Vector3Like, aabb: &AabbLike) -> () {
    out.x = ((aabb.min.x + aabb.max.x) * 0.5_f64);
    out.y = ((aabb.min.y + aabb.max.y) * 0.5_f64);
    out.z = ((aabb.min.z + aabb.max.z) * 0.5_f64);
}

// Source: upstream/packages/geometry/src/aabb.ts:127 (sha256:694e75de4d33ffad9bdb3ef5d5095b4b3d1dda0040d94773e5becfe4d976da01)
pub fn get_aabb_extents(out: &mut Vector3Like, aabb: &AabbLike) -> () {
    out.x = ((aabb.max.x - aabb.min.x) * 0.5_f64);
    out.y = ((aabb.max.y - aabb.min.y) * 0.5_f64);
    out.z = ((aabb.max.z - aabb.min.z) * 0.5_f64);
}

// Source: upstream/packages/geometry/src/aabb.ts:136 (sha256:6fc828831a0d91e6af16c80cd9cc2be2ee2af8d0d4c73d7579d9e53a9505de39)
pub fn get_aabb_size(out: &mut Vector3Like, aabb: &AabbLike) -> () {
    out.x = (aabb.max.x - aabb.min.x);
    out.y = (aabb.max.y - aabb.min.y);
    out.z = (aabb.max.z - aabb.min.z);
}

// Source: upstream/packages/geometry/src/aabb.ts:150 (sha256:e11df5c72a00933ae064143f9aaac3d2d4dde4f7e6c5c172aa96123307e8fa6a)
pub fn get_closest_point_on_aabb(
    out: &mut Vector3Like,
    aabb: &AabbLike,
    point: &Vector3Like,
) -> () {
    let px = point.x;
    let py = point.y;
    let pz = point.z;
    out.x = ((px).max(aabb.min.x)).min(aabb.max.x);
    out.y = ((py).max(aabb.min.y)).min(aabb.max.y);
    out.z = ((pz).max(aabb.min.z)).min(aabb.max.z);
}

// Source: upstream/packages/geometry/src/aabb.ts:165 (sha256:ea0c23cbae15c408facbffe240ec09491884b28d22c2741332a22097fc424835)
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

// Source: upstream/packages/geometry/src/aabb.ts:189 (sha256:96ea9ddd712db570210c04dd8f2327272d7beed34bdda6370e61e4cd9ec6a497)
pub fn is_aabb_intersecting_aabb(a: &AabbLike, b: &AabbLike) -> bool {
    return (((((a.min.x <= b.max.x) && (a.max.x >= b.min.x)) && (a.min.y <= b.max.y))
        && (a.max.y >= b.min.y))
        && (a.min.z <= b.max.z))
        && (a.max.z >= b.min.z);
}

// Source: upstream/packages/geometry/src/aabb.ts:203 (sha256:bf20fcec218a50b50fabd1f31b8a22b774ad881dcc77a19e5d5eb71382195713)
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

// Source: upstream/packages/geometry/src/aabb.ts:224 (sha256:db5c67bbad1cfa4f280d9a73e07e1d1bab8c5016ddc6fcb5c3aa988c8ac5d3b8)
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

// Source: upstream/packages/geometry/src/aabb.ts:257 (sha256:dbcfe9a8f5f3f90242f6beca4eb42de2b342e481f1932d287ab46707a680f292)
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

// Source: upstream/packages/geometry/src/aabb.ts:296 (sha256:386275cdc4044862961fd7c0f90ec5f7be258302b4788d3f34e1c79b5185f151)
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
