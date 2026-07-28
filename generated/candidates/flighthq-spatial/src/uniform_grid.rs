// @generated from upstream/packages/spatial/src/uniformGrid.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{contains_rectangle_point_xy, intersects_rectangle};
use flighthq_types::{
    RectangleLike, SpatialAabb, SpatialIndexBackend, SpatialObjectId, SpatialPair,
};

// Source: upstream/packages/spatial/src/uniformGrid.ts:11 (sha256:4eef8a64503376262e3211942a5cd7f01ce0e699c52a0c992a5d484c82b02c19)
pub fn create_uniform_grid_spatial_backend(cell_size: f64) -> SpatialIndexBackend {
    let grid: std::sync::Arc<std::sync::Mutex<UniformGrid>> =
        std::sync::Arc::new(std::sync::Mutex::new(UniformGrid {
            __flight_identity: std::sync::Arc::new(()),
            cell_size: cell_size,
            cells: Vec::new(),
            bounds: Vec::new(),
            min_cell_x: 0.0_f64,
            min_cell_y: 0.0_f64,
            max_cell_x: 0.0_f64,
            max_cell_y: 0.0_f64,
            empty: true,
            seen: Vec::new(),
        }));
    return SpatialIndexBackend {
        __flight_identity: std::sync::Arc::new(()),
        insert_spatial_object: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |id: SpatialObjectId, bounds: SpatialAabb| -> () {
                _insert_into_grid(&mut (*grid.lock().unwrap()), id, &bounds);
            }
        })
            as Box<dyn FnMut(SpatialObjectId, SpatialAabb) -> () + Send + 'static>)),
        update_spatial_object: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |id: SpatialObjectId, bounds: SpatialAabb| -> () {
                _remove_from_grid(&mut (*grid.lock().unwrap()), id);
                _insert_into_grid(&mut (*grid.lock().unwrap()), id, &bounds);
            }
        })
            as Box<dyn FnMut(SpatialObjectId, SpatialAabb) -> () + Send + 'static>)),
        remove_spatial_object: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |id: SpatialObjectId| -> () {
                _remove_from_grid(&mut (*grid.lock().unwrap()), id);
            }
        })
            as Box<dyn FnMut(SpatialObjectId) -> () + Send + 'static>)),
        clear_spatial_index: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move || -> () {
                (*grid.lock().unwrap()).cells.clear();
                (*grid.lock().unwrap()).bounds.clear();
                (*grid.lock().unwrap()).seen.clear();
                (*grid.lock().unwrap()).empty = true;
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
        query_spatial_pairs: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |mut out: Vec<SpatialPair>| -> () {
                _query_grid_pairs(&(*grid.lock().unwrap()), &mut out);
            }
        })
            as Box<dyn FnMut(Vec<SpatialPair>) -> () + Send + 'static>)),
        query_spatial_region: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |region: SpatialAabb, mut out: Vec<SpatialObjectId>| -> () {
                _query_grid_region(&mut (*grid.lock().unwrap()), &region, &mut out);
            }
        })
            as Box<dyn FnMut(SpatialAabb, Vec<SpatialObjectId>) -> () + Send + 'static>)),
        query_spatial_point: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |x: f64, y: f64, mut out: Vec<SpatialObjectId>| -> () {
                _query_grid_point(&(*grid.lock().unwrap()), x, y, &mut out);
            }
        })
            as Box<dyn FnMut(f64, f64, Vec<SpatialObjectId>) -> () + Send + 'static>)),
        query_spatial_ray: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |x: f64, y: f64, dx: f64, dy: f64, mut out: Vec<SpatialObjectId>| -> () {
                _query_grid_ray(&mut (*grid.lock().unwrap()), x, y, dx, dy, &mut out);
            }
        })
            as Box<dyn FnMut(f64, f64, f64, f64, Vec<SpatialObjectId>) -> () + Send + 'static>)),
    };
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:58 (sha256:bf8ee7d148f40d101a35cf9e62d37751ed15b9b9ce6f5ad5dcbfcc8b44a4d9a1)
#[derive(Clone)]
struct GridCell {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cx: f64,
    pub cy: f64,
    pub ids: Vec<SpatialObjectId>,
}
impl PartialEq for GridCell {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:70 (sha256:57e8d887164f3090f0d102ec05909c147024674db43a35a732ab5894c59f7ae1)
#[derive(Clone)]
struct UniformGrid {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cell_size: f64,
    pub cells: Vec<(String, GridCell)>,
    pub bounds: Vec<(SpatialObjectId, SpatialAabb)>,
    pub min_cell_x: f64,
    pub min_cell_y: f64,
    pub max_cell_x: f64,
    pub max_cell_y: f64,
    pub empty: bool,
    pub seen: Vec<SpatialObjectId>,
}
impl PartialEq for UniformGrid {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:84 (sha256:45dc8d4ede26d698e11710d2ab57bd04451e76c97cd4930c04a5008fa833df23)
fn _cell_index(coord: f64, cell_size: f64) -> f64 {
    return (coord / cell_size).floor();
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:90 (sha256:e8854bdbd64c484904fbf56d75556dd0d533af10ec478225ec75d36fa401f447)
fn _cell_key(cx: f64, cy: f64) -> String {
    return format!("{},{}", cx, cy);
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:98 (sha256:ce3c4ff15a7712b41d511998526502534dada38191dbf11e8b2eac09e9d621ba)
fn _fill_rect_from_aabb(out: &mut RectangleLike, aabb: &SpatialAabb) -> () {
    out.x = aabb.min_x;
    out.y = aabb.min_y;
    out.width = (aabb.max_x - aabb.min_x);
    out.height = (aabb.max_y - aabb.min_y);
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:107 (sha256:3bfea023f430008a33e87dff5df45f90382d4998143bdea4660d1dc3a3a7b719)
fn _insert_into_grid(grid: &mut UniformGrid, id: SpatialObjectId, bounds: &SpatialAabb) -> () {
    let cs = grid.cell_size;
    let mut cx0 = _cell_index(bounds.min_x, cs);
    let cx1 = _cell_index(bounds.max_x, cs);
    let mut cy0 = _cell_index(bounds.min_y, cs);
    let cy1 = _cell_index(bounds.max_y, cs);
    {
        let __flight_key = id;
        let __flight_value = SpatialAabb {
            __flight_identity: std::sync::Arc::new(()),
            min_x: bounds.min_x,
            min_y: bounds.min_y,
            max_x: bounds.max_x,
            max_y: bounds.max_y,
        };
        if let Some((_, value)) = grid.bounds.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            grid.bounds.push((__flight_key, __flight_value));
        }
    };
    {
        let mut cy = cy0;
        while (cy <= cy1) {
            {
                let mut cx = cx0;
                while (cx <= cx1) {
                    let key = _cell_key(cx, cy);
                    let mut cell = grid
                        .cells
                        .iter()
                        .find(|(key, _)| key == &(key).clone())
                        .map(|(_, value)| value.clone());
                    if (cell).is_none() {
                        cell = Some(GridCell {
                            __flight_identity: std::sync::Arc::new(()),
                            cx: cx,
                            cy: cy,
                            ids: Vec::new(),
                        });
                        {
                            let __flight_key = (key).clone();
                            let __flight_value = (cell).clone().unwrap();
                            if let Some((_, value)) =
                                grid.cells.iter_mut().find(|(key, _)| key == &__flight_key)
                            {
                                *value = __flight_value;
                            } else {
                                grid.cells.push((__flight_key, __flight_value));
                            }
                        };
                    }
                    {
                        let __flight_value = id;
                        if !cell.as_mut().unwrap().ids.contains(&__flight_value) {
                            cell.as_mut().unwrap().ids.push(__flight_value);
                        }
                    };
                    {
                        cx += 1.0;
                        cx
                    };
                }
            }
            {
                cy += 1.0;
                cy
            };
        }
    }
    if grid.empty {
        grid.min_cell_x = cx0;
        grid.max_cell_x = cx1;
        grid.min_cell_y = cy0;
        grid.max_cell_y = cy1;
        grid.empty = false;
    } else {
        if (cx0 < grid.min_cell_x) {
            grid.min_cell_x = cx0;
        }
        if (cx1 > grid.max_cell_x) {
            grid.max_cell_x = cx1;
        }
        if (cy0 < grid.min_cell_y) {
            grid.min_cell_y = cy0;
        }
        if (cy1 > grid.max_cell_y) {
            grid.max_cell_y = cy1;
        }
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:140 (sha256:6e93e48516bee3191a99d57b52f08bae516d7aaa9db9b4a872b9cc7210e0c857)
fn _is_spatial_aabb_contains_point(aabb: &SpatialAabb, x: f64, y: f64) -> bool {
    _fill_rect_from_aabb(&mut (*_SCRATCH_RECT_A.lock().unwrap()), aabb);
    return contains_rectangle_point_xy(&(*_SCRATCH_RECT_A.lock().unwrap()), x, y);
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:147 (sha256:53abc16610bfda3196d454eca1e9916a07e7fefd8a647d079e0517307e58c4df)
fn _is_spatial_aabb_overlapping(a: &SpatialAabb, b: &SpatialAabb) -> bool {
    _fill_rect_from_aabb(&mut (*_SCRATCH_RECT_A.lock().unwrap()), a);
    _fill_rect_from_aabb(&mut (*_SCRATCH_RECT_B.lock().unwrap()), b);
    return intersects_rectangle(
        &(*_SCRATCH_RECT_A.lock().unwrap()),
        &(*_SCRATCH_RECT_B.lock().unwrap()),
    );
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:156 (sha256:5449163cc46e76efa44514082dc0053eb0d733df81ff6a4e3a9910a95513c606)
fn _ray_box_entry_t(
    ox: f64,
    oy: f64,
    dx: f64,
    dy: f64,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> f64 {
    let mut tmin = (-f64::INFINITY);
    let mut tmax = f64::INFINITY;
    if (dx != 0.0_f64) {
        let inv = (1.0_f64 / dx);
        let mut t1 = ((min_x - ox) * inv);
        let mut t2 = ((max_x - ox) * inv);
        if (t1 > t2) {
            let t = t1;
            t1 = t2;
            t2 = t;
        }
        if (t1 > tmin) {
            tmin = t1;
        }
        if (t2 < tmax) {
            tmax = t2;
        }
    } else {
        if (ox < min_x) || (ox > max_x) {
            return (-1.0_f64);
        }
    }
    if (dy != 0.0_f64) {
        let inv = (1.0_f64 / dy);
        let mut t1 = ((min_y - oy) * inv);
        let mut t2 = ((max_y - oy) * inv);
        if (t1 > t2) {
            let t = t1;
            t1 = t2;
            t2 = t;
        }
        if (t1 > tmin) {
            tmin = t1;
        }
        if (t2 < tmax) {
            tmax = t2;
        }
    } else {
        if (oy < min_y) || (oy > max_y) {
            return (-1.0_f64);
        }
    }
    if (tmax < tmin) || (tmax < 0.0_f64) {
        return (-1.0_f64);
    }
    return if (tmin > 0.0_f64) { tmin } else { 0.0_f64 };
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:203 (sha256:81c45adcb3ee389e2e293edafca74462b32c0065d1271476172b3d22ae9aca32)
fn _remove_from_grid(grid: &mut UniformGrid, id: SpatialObjectId) -> () {
    let bounds = grid
        .bounds
        .iter()
        .find(|(key, _)| key == &id)
        .map(|(_, value)| value.clone());
    if (bounds).is_none() {
        return;
    }
    let cs = grid.cell_size;
    let mut cx0 = _cell_index(bounds.as_ref().unwrap().min_x, cs);
    let cx1 = _cell_index(bounds.as_ref().unwrap().max_x, cs);
    let mut cy0 = _cell_index(bounds.as_ref().unwrap().min_y, cs);
    let cy1 = _cell_index(bounds.as_ref().unwrap().max_y, cs);
    {
        let mut cy = cy0;
        while (cy <= cy1) {
            {
                let mut cx = cx0;
                while (cx <= cx1) {
                    let key = _cell_key(cx, cy);
                    let mut cell = grid
                        .cells
                        .iter()
                        .find(|(key, _)| key == &(key).clone())
                        .map(|(_, value)| value.clone());
                    if (cell).is_none() {
                        {
                            cx += 1.0;
                            cx
                        };
                        continue;
                    }
                    {
                        let __flight_value = id;
                        if let Some(__flight_index) = cell
                            .as_mut()
                            .unwrap()
                            .ids
                            .iter()
                            .position(|item| item == &__flight_value)
                        {
                            cell.as_mut().unwrap().ids.remove(__flight_index);
                            true
                        } else {
                            false
                        }
                    };
                    if (cell.as_mut().unwrap().ids.size == 0.0_f64) {
                        {
                            let __flight_key = (key).clone();
                            if let Some(__flight_index) =
                                grid.cells.iter().position(|(key, _)| key == &__flight_key)
                            {
                                grid.cells.remove(__flight_index);
                                true
                            } else {
                                false
                            }
                        };
                    }
                    {
                        cx += 1.0;
                        cx
                    };
                }
            }
            {
                cy += 1.0;
                cy
            };
        }
    }
    {
        let __flight_key = id;
        if let Some(__flight_index) = grid.bounds.iter().position(|(key, _)| key == &__flight_key) {
            grid.bounds.remove(__flight_index);
            true
        } else {
            false
        }
    };
    if (grid
        .bounds
        .iter()
        .find(|(key, _)| key == &"size")
        .map(|(_, value)| value.clone())
        .expect("TypeScript Record key was absent")
        == 0.0_f64)
    {
        grid.empty = true;
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:229 (sha256:ea0efa48e00023e35a3fc58e6588a9bbf750e2cff3f3505d0e47e43cf701a1aa)
fn _query_grid_pairs(grid: &UniformGrid, out: &mut Vec<SpatialPair>) -> () {
    out.clear();
    let cs = grid.cell_size;
    for cell in (grid
        .cells
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        if (cell.ids.size < 2.0_f64) {
            continue;
        }
        let mut list = {
            let mut __flight_array = Vec::new();
            __flight_array.extend((cell.ids).iter().cloned());
            __flight_array
        };
        {
            let mut i = 0.0_f64;
            while (i < (list.len() as f64)) {
                {
                    let mut j = (i + 1.0_f64);
                    while (j < (list.len() as f64)) {
                        let mut a = list[i as usize].clone();
                        let mut b = list[j as usize].clone();
                        if (a > b) {
                            let t = a;
                            a = (b).clone();
                            b = (t).clone();
                        }
                        let ab = grid
                            .bounds
                            .iter()
                            .find(|(key, _)| key == &a)
                            .map(|(_, value)| value.clone());
                        let bb = grid
                            .bounds
                            .iter()
                            .find(|(key, _)| key == &b)
                            .map(|(_, value)| value.clone());
                        if ((ab).is_none()) || ((bb).is_none()) {
                            {
                                j += 1.0;
                                j
                            };
                            continue;
                        }
                        let canonical_x = (_cell_index(ab.as_ref().unwrap().min_x, cs))
                            .max(_cell_index(bb.as_ref().unwrap().min_x, cs));
                        let canonical_y = (_cell_index(ab.as_ref().unwrap().min_y, cs))
                            .max(_cell_index(bb.as_ref().unwrap().min_y, cs));
                        if (cell.cx == canonical_x) && (cell.cy == canonical_y) {
                            out.push(SpatialPair {
                                __flight_identity: std::sync::Arc::new(()),
                                a: a,
                                b: b,
                            });
                        }
                        {
                            j += 1.0;
                            j
                        };
                    }
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:258 (sha256:a86b5420e8606082bfc173ce20620d5250765b30c892988ed8d5497619f3058d)
fn _query_grid_point(grid: &UniformGrid, x: f64, y: f64, out: &mut Vec<SpatialObjectId>) -> () {
    out.clear();
    let cs = grid.cell_size;
    let cell = grid
        .cells
        .iter()
        .find(|(key, _)| key == &_cell_key(_cell_index(x, cs), _cell_index(y, cs)))
        .map(|(_, value)| value.clone());
    if (cell).is_none() {
        return;
    }
    for id in ((cell.as_ref().unwrap().ids).clone()).iter().cloned() {
        let bounds = grid
            .bounds
            .iter()
            .find(|(key, _)| key == &id)
            .map(|(_, value)| value.clone());
        if ((bounds).is_some()) && (_is_spatial_aabb_contains_point(bounds.as_ref().unwrap(), x, y))
        {
            out.push(id);
        }
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:273 (sha256:9d39a6e2469dca1e020752a3241cf0cb8ed48907a688eccadd7f5f51c02d6413)
fn _query_grid_ray(
    grid: &mut UniformGrid,
    ox: f64,
    oy: f64,
    dx: f64,
    dy: f64,
    out: &mut Vec<SpatialObjectId>,
) -> () {
    out.clear();
    if grid.empty {
        return;
    }
    let cs = grid.cell_size;
    grid.seen.clear();
    if (dx == 0.0_f64) && (dy == 0.0_f64) {
        _query_grid_point(grid, ox, oy, out);
        return;
    }
    let box_min_x = (grid.min_cell_x * cs);
    let box_min_y = (grid.min_cell_y * cs);
    let box_max_x = ((grid.max_cell_x + 1.0_f64) * cs);
    let box_max_y = ((grid.max_cell_y + 1.0_f64) * cs);
    let t_enter = _ray_box_entry_t(ox, oy, dx, dy, box_min_x, box_min_y, box_max_x, box_max_y);
    if (t_enter < 0.0_f64) {
        return;
    }
    let start_x = (ox + (t_enter * dx));
    let start_y = (oy + (t_enter * dy));
    let mut cx = _cell_index(start_x, cs);
    let mut cy = _cell_index(start_y, cs);
    if (cx < grid.min_cell_x) {
        cx = grid.min_cell_x;
    } else {
        if (cx > grid.max_cell_x) {
            cx = grid.max_cell_x;
        }
    }
    if (cy < grid.min_cell_y) {
        cy = grid.min_cell_y;
    } else {
        if (cy > grid.max_cell_y) {
            cy = grid.max_cell_y;
        }
    }
    let step_x = if (dx > 0.0_f64) {
        1.0_f64
    } else {
        if (dx < 0.0_f64) { (-1.0_f64) } else { 0.0_f64 }
    };
    let step_y = if (dy > 0.0_f64) {
        1.0_f64
    } else {
        if (dy < 0.0_f64) { (-1.0_f64) } else { 0.0_f64 }
    };
    let mut t_max_x = f64::INFINITY;
    let mut t_delta_x = f64::INFINITY;
    if (step_x != 0.0_f64) {
        let boundary = if (step_x > 0.0_f64) {
            ((cx + 1.0_f64) * cs)
        } else {
            (cx * cs)
        };
        t_max_x = ((boundary - ox) / dx);
        t_delta_x = (cs / (dx).abs());
    }
    let mut t_max_y = f64::INFINITY;
    let mut t_delta_y = f64::INFINITY;
    if (step_y != 0.0_f64) {
        let boundary = if (step_y > 0.0_f64) {
            ((cy + 1.0_f64) * cs)
        } else {
            (cy * cs)
        };
        t_max_y = ((boundary - oy) / dy);
        t_delta_y = (cs / (dy).abs());
    }
    let max_steps =
        (((grid.max_cell_x - grid.min_cell_x) + (grid.max_cell_y - grid.min_cell_y)) + 3.0_f64);
    {
        let mut step = 0.0_f64;
        while (step <= max_steps) {
            if (((cx < grid.min_cell_x) || (cx > grid.max_cell_x)) || (cy < grid.min_cell_y))
                || (cy > grid.max_cell_y)
            {
                break;
            }
            let cell = grid
                .cells
                .iter()
                .find(|(key, _)| key == &_cell_key(cx, cy))
                .map(|(_, value)| value.clone());
            if (cell).is_some() {
                for id in ((cell.as_ref().unwrap().ids).clone()).iter().cloned() {
                    {
                        let __flight_value = id;
                        if !grid.seen.contains(&__flight_value) {
                            grid.seen.push(__flight_value);
                        }
                    };
                }
            }
            if (t_max_x < t_max_y) {
                cx += step_x;
                t_max_x += t_delta_x;
            } else {
                cy += step_y;
                t_max_y += t_delta_y;
            }
            {
                step += 1.0;
                step
            };
        }
    }
    for id in (grid.seen).iter().cloned() {
        let bounds = grid
            .bounds
            .iter()
            .find(|(key, _)| key == &id)
            .map(|(_, value)| value.clone());
        if ((bounds).is_some())
            && (_ray_box_entry_t(
                ox,
                oy,
                dx,
                dy,
                bounds.as_ref().unwrap().min_x,
                bounds.as_ref().unwrap().min_y,
                bounds.as_ref().unwrap().max_x,
                bounds.as_ref().unwrap().max_y,
            ) >= 0.0_f64)
        {
            out.push(id);
        }
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:349 (sha256:ff0c9b12edabe53115437e92903264e2c96bbcbe115da956d1c836434883d491)
fn _query_grid_region(
    grid: &mut UniformGrid,
    region: &SpatialAabb,
    out: &mut Vec<SpatialObjectId>,
) -> () {
    out.clear();
    let cs = grid.cell_size;
    grid.seen.clear();
    let mut cx0 = _cell_index(region.min_x, cs);
    let cx1 = _cell_index(region.max_x, cs);
    let mut cy0 = _cell_index(region.min_y, cs);
    let cy1 = _cell_index(region.max_y, cs);
    {
        let mut cy = cy0;
        while (cy <= cy1) {
            {
                let mut cx = cx0;
                while (cx <= cx1) {
                    let cell = grid
                        .cells
                        .iter()
                        .find(|(key, _)| key == &_cell_key(cx, cy))
                        .map(|(_, value)| value.clone());
                    if (cell).is_none() {
                        {
                            cx += 1.0;
                            cx
                        };
                        continue;
                    }
                    for id in ((cell.as_ref().unwrap().ids).clone()).iter().cloned() {
                        if grid.seen.iter().any(|item| item == &id) {
                            continue;
                        }
                        {
                            let __flight_value = id;
                            if !grid.seen.contains(&__flight_value) {
                                grid.seen.push(__flight_value);
                            }
                        };
                        let bounds = grid
                            .bounds
                            .iter()
                            .find(|(key, _)| key == &id)
                            .map(|(_, value)| value.clone());
                        if ((bounds).is_some())
                            && (_is_spatial_aabb_overlapping(bounds.as_ref().unwrap(), region))
                        {
                            out.push(id);
                        }
                    }
                    {
                        cx += 1.0;
                        cx
                    };
                }
            }
            {
                cy += 1.0;
                cy
            };
        }
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:375 (sha256:e53c83de77d981abb41109e9c2682fcd2a6930d40c2c91c37b2afaaebbc9eb79)
static _SCRATCH_RECT_A: std::sync::LazyLock<std::sync::Mutex<RectangleLike>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(RectangleLike {
            __flight_identity: std::sync::Arc::new(()),
            x: 0.0_f64,
            y: 0.0_f64,
            width: 0.0_f64,
            height: 0.0_f64,
        })
    });

// Source: upstream/packages/spatial/src/uniformGrid.ts:376 (sha256:27ab52f8e9c47f5e2ccd592369b83dc133972e262e313e3c499c245efa883daa)
static _SCRATCH_RECT_B: std::sync::LazyLock<std::sync::Mutex<RectangleLike>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(RectangleLike {
            __flight_identity: std::sync::Arc::new(()),
            x: 0.0_f64,
            y: 0.0_f64,
            width: 0.0_f64,
            height: 0.0_f64,
        })
    });
