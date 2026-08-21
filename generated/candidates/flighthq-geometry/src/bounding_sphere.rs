// @generated from upstream/packages/geometry/src/boundingSphere.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_vector3;
use flighthq_entity::create_entity;
use flighthq_types::{AabbLike, BoundingSphere, BoundingSphereLike, Matrix4Like, Vector3Like};

// Source: upstream/packages/geometry/src/boundingSphere.ts:6 (sha256:9b0e7171ecf8907fe84c2cba1a7eb96740a1d5f66523fdc4761762262397447e)
pub fn clone_bounding_sphere(source: &BoundingSphereLike) -> BoundingSphere {
    return create_bounding_sphere(
        Some(source.center.x),
        Some(source.center.y),
        Some(source.center.z),
        Some(source.radius),
    );
}

// Source: upstream/packages/geometry/src/boundingSphere.ts:14 (sha256:e177dfd7981c14c2a30199f19c12b3eaff1c1bbb854b7ba271d0feddb4967348)
pub fn contains_bounding_sphere_point(sphere: &BoundingSphereLike, point: &Vector3Like) -> bool {
    if (sphere.radius < 0.0_f64) {
        return false;
    }
    let dx = (point.x - sphere.center.x);
    let dy = (point.y - sphere.center.y);
    let dz = (point.z - sphere.center.z);
    return ((((dx * dx) + (dy * dy)) + (dz * dz)) <= (sphere.radius * sphere.radius));
}

// Source: upstream/packages/geometry/src/boundingSphere.ts:30 (sha256:9868d46c459c0ebbdbec422323d686addeed11d978aee84b2af2fa1dca0d314c)
pub fn copy_bounding_sphere(out: &mut BoundingSphereLike, source: &BoundingSphereLike) -> () {
    out.center.x = source.center.x;
    out.center.y = source.center.y;
    out.center.z = source.center.z;
    out.radius = source.radius;
}

// Source: upstream/packages/geometry/src/boundingSphere.ts:41 (sha256:0d9cadd252cf8b2a2626649abd0c1a169aaa91f60377ee30262a4d2a76de8dce)
pub fn create_bounding_sphere(
    center_x: Option<f64>,
    center_y: Option<f64>,
    center_z: Option<f64>,
    radius: Option<f64>,
) -> BoundingSphere {
    let center = create_vector3(
        Some((center_x).clone().unwrap_or(0.0_f64)),
        Some((center_y).clone().unwrap_or(0.0_f64)),
        Some((center_z).clone().unwrap_or(0.0_f64)),
    );
    return create_entity(Some(BoundingSphere {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        center: (center).clone(),
        radius: (radius).clone().unwrap_or((-1.0_f64)),
    }));
}

// Source: upstream/packages/geometry/src/boundingSphere.ts:60 (sha256:4af5cf704a673c7ef8f6c5ddd8b16a4888ad8e48f1d3f890d457793e6888e80c)
pub fn get_closest_point_on_bounding_sphere(
    out: &mut Vector3Like,
    sphere: &BoundingSphereLike,
    point: &Vector3Like,
) -> () {
    let cx = sphere.center.x;
    let cy = sphere.center.y;
    let cz = sphere.center.z;
    let r = sphere.radius;
    if (r < 0.0_f64) {
        out.x = cx;
        out.y = cy;
        out.z = cz;
        return;
    }
    let dx = (point.x - cx);
    let dy = (point.y - cy);
    let dz = (point.z - cz);
    let dist = (((dx * dx) + (dy * dy)) + (dz * dz)).sqrt();
    if (dist == 0.0_f64) {
        out.x = (cx + r);
        out.y = cy;
        out.z = cz;
        return;
    }
    let scale = (r / dist);
    out.x = (cx + (dx * scale));
    out.y = (cy + (dy * scale));
    out.z = (cz + (dz * scale));
}

// Source: upstream/packages/geometry/src/boundingSphere.ts:95 (sha256:23f3f7395d0dadcdaed22ca0110d24f0eca8ac98af3ddef54c194f1dcdcbc7ce)
pub fn is_bounding_sphere_intersecting_bounding_sphere(
    a: &BoundingSphereLike,
    b: &BoundingSphereLike,
) -> bool {
    if (a.radius < 0.0_f64) || (b.radius < 0.0_f64) {
        return false;
    }
    let dx = (a.center.x - b.center.x);
    let dy = (a.center.y - b.center.y);
    let dz = (a.center.z - b.center.z);
    let dist_sq = (((dx * dx) + (dy * dy)) + (dz * dz));
    let sum_r = (a.radius + b.radius);
    return (dist_sq <= (sum_r * sum_r));
}

// Source: upstream/packages/geometry/src/boundingSphere.ts:115 (sha256:f130c37a3f335b6ff3d308102c3362b34980252af1f077b048d26eb4b7d20d2c)
pub fn merge_bounding_sphere(
    out: &mut BoundingSphereLike,
    a: &BoundingSphereLike,
    b: &BoundingSphereLike,
) -> () {
    if (a.radius < 0.0_f64) {
        out.center.x = b.center.x;
        out.center.y = b.center.y;
        out.center.z = b.center.z;
        out.radius = b.radius;
        return;
    }
    if (b.radius < 0.0_f64) {
        out.center.x = a.center.x;
        out.center.y = a.center.y;
        out.center.z = a.center.z;
        out.radius = a.radius;
        return;
    }
    let acx = a.center.x;
    let acy = a.center.y;
    let acz = a.center.z;
    let ar = a.radius;
    let bcx = b.center.x;
    let bcy = b.center.y;
    let bcz = b.center.z;
    let br = b.radius;
    let dx = (bcx - acx);
    let dy = (bcy - acy);
    let dz = (bcz - acz);
    let dist = (((dx * dx) + (dy * dy)) + (dz * dz)).sqrt();
    if ((dist + br) <= ar) {
        out.center.x = acx;
        out.center.y = acy;
        out.center.z = acz;
        out.radius = ar;
        return;
    }
    if ((dist + ar) <= br) {
        out.center.x = bcx;
        out.center.y = bcy;
        out.center.z = bcz;
        out.radius = br;
        return;
    }
    let new_radius = (((dist + ar) + br) * 0.5_f64);
    let t = ((new_radius - ar) / dist);
    out.center.x = (acx + (dx * t));
    out.center.y = (acy + (dy * t));
    out.center.z = (acz + (dz * t));
    out.radius = new_radius;
}

// Source: upstream/packages/geometry/src/boundingSphere.ts:180 (sha256:225c2e92b749fa1613ec82aa73b801152e0ea1dd8e84e5c1a80a3bd9645dd0b6)
pub fn set_bounding_sphere(
    out: &mut BoundingSphereLike,
    center_x: f64,
    center_y: f64,
    center_z: f64,
    radius: f64,
) -> () {
    out.center.x = center_x;
    out.center.y = center_y;
    out.center.z = center_z;
    out.radius = radius;
}

// Source: upstream/packages/geometry/src/boundingSphere.ts:200 (sha256:7d60129e8d1abbb8cc8b6faf50677dd6ff4b3abe818b827cc5ff1990e98f0a7e)
pub fn set_bounding_sphere_from_aabb(out: &mut BoundingSphereLike, aabb: &AabbLike) -> () {
    let min_x = aabb.min.x;
    let min_y = aabb.min.y;
    let min_z = aabb.min.z;
    let max_x = aabb.max.x;
    let max_y = aabb.max.y;
    let max_z = aabb.max.z;
    if ((min_x > max_x) || (min_y > max_y)) || (min_z > max_z) {
        out.center.x = 0.0_f64;
        out.center.y = 0.0_f64;
        out.center.z = 0.0_f64;
        out.radius = (-1.0_f64);
        return;
    }
    let cx = ((min_x + max_x) * 0.5_f64);
    let cy = ((min_y + max_y) * 0.5_f64);
    let cz = ((min_z + max_z) * 0.5_f64);
    let ex = ((max_x - min_x) * 0.5_f64);
    let ey = ((max_y - min_y) * 0.5_f64);
    let ez = ((max_z - min_z) * 0.5_f64);
    out.center.x = cx;
    out.center.y = cy;
    out.center.z = cz;
    out.radius = (((ex * ex) + (ey * ey)) + (ez * ez)).sqrt();
}

// Source: upstream/packages/geometry/src/boundingSphere.ts:236 (sha256:da365e9d951cbcdaa8f6cc4057509394a10bdf015f81b6ee61d000bb7a505fad)
pub fn transform_bounding_sphere_by_matrix4(
    out: &mut BoundingSphereLike,
    sphere: &BoundingSphereLike,
    m: &Matrix4Like,
) -> () {
    let cx = sphere.center.x;
    let cy = sphere.center.y;
    let cz = sphere.center.z;
    let radius = sphere.radius;
    let tcx = (((((m.m[0.0_f64 as usize] as f64) * cx) + ((m.m[4.0_f64 as usize] as f64) * cy))
        + ((m.m[8.0_f64 as usize] as f64) * cz))
        + (m.m[12.0_f64 as usize] as f64));
    let tcy = (((((m.m[1.0_f64 as usize] as f64) * cx) + ((m.m[5.0_f64 as usize] as f64) * cy))
        + ((m.m[9.0_f64 as usize] as f64) * cz))
        + (m.m[13.0_f64 as usize] as f64));
    let tcz = (((((m.m[2.0_f64 as usize] as f64) * cx) + ((m.m[6.0_f64 as usize] as f64) * cy))
        + ((m.m[10.0_f64 as usize] as f64) * cz))
        + (m.m[14.0_f64 as usize] as f64));
    let sx = ((((m.m[0.0_f64 as usize] as f64) * (m.m[0.0_f64 as usize] as f64))
        + ((m.m[1.0_f64 as usize] as f64) * (m.m[1.0_f64 as usize] as f64)))
        + ((m.m[2.0_f64 as usize] as f64) * (m.m[2.0_f64 as usize] as f64)))
        .sqrt();
    let sy = ((((m.m[4.0_f64 as usize] as f64) * (m.m[4.0_f64 as usize] as f64))
        + ((m.m[5.0_f64 as usize] as f64) * (m.m[5.0_f64 as usize] as f64)))
        + ((m.m[6.0_f64 as usize] as f64) * (m.m[6.0_f64 as usize] as f64)))
        .sqrt();
    let sz = ((((m.m[8.0_f64 as usize] as f64) * (m.m[8.0_f64 as usize] as f64))
        + ((m.m[9.0_f64 as usize] as f64) * (m.m[9.0_f64 as usize] as f64)))
        + ((m.m[10.0_f64 as usize] as f64) * (m.m[10.0_f64 as usize] as f64)))
        .sqrt();
    let max_scale = ((sx).max(sy)).max(sz);
    out.center.x = tcx;
    out.center.y = tcy;
    out.center.z = tcz;
    out.radius = if (radius < 0.0_f64) {
        radius
    } else {
        (radius * max_scale)
    };
}
