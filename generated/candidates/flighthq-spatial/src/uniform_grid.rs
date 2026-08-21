// @generated from upstream/packages/spatial/src/uniformGrid.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    SpatialAabb2D, SpatialDeclineReason, SpatialIndexBackend2D, SpatialIndexingExplanation,
    SpatialIndexingGuard, SpatialIndexingMode, SpatialIndexingNotice, SpatialIndexingOperation,
    SpatialIndexingReason, SpatialObjectId, SpatialPair,
};

// Source: upstream/packages/spatial/src/uniformGrid.ts:29 (sha256:3436b79903acdb3a29ed03ace8cfbb58e4a7148e4de19d5659df39a42993f517)
pub const MAX_INDEXED_CELLS_PER_OBJECT: f64 = 1024.0_f64;

// Source: upstream/packages/spatial/src/uniformGrid.ts:44 (sha256:966a993aa0fb6ed461a5d13d3befaefaf9868c8618ae03eddc0790511eb79d51)
pub fn create_uniform_grid_spatial_backend2_d(cell_size: f64) -> SpatialIndexBackend2D {
    let grid: std::sync::Arc<std::sync::Mutex<UniformGrid>> =
        std::sync::Arc::new(std::sync::Mutex::new(UniformGrid {
            __flight_identity: std::sync::Arc::new(()),
            cell_size: cell_size,
            cells: Vec::new(),
            bounds: Vec::new(),
            overflow: Vec::new(),
            declined: Vec::new(),
            min_cell_x: 0.0_f64,
            min_cell_y: 0.0_f64,
            max_cell_x: 0.0_f64,
            max_cell_y: 0.0_f64,
            seen: Vec::new(),
            pair_ids: vec![],
        }));
    return SpatialIndexBackend2D {
        __flight_identity: std::sync::Arc::new(()),
        insert_spatial_object: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |id: SpatialObjectId, bounds: SpatialAabb2D| -> bool {
                return _insert_into_grid(
                    &mut (*grid.lock().unwrap()),
                    id,
                    &bounds,
                    "insert".to_owned(),
                );
            }
        })
            as Box<dyn FnMut(SpatialObjectId, SpatialAabb2D) -> bool + Send + 'static>)),
        update_spatial_object: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |id: SpatialObjectId, bounds: SpatialAabb2D| -> bool {
                return _update_grid_object(&mut (*grid.lock().unwrap()), id, &bounds);
            }
        })
            as Box<dyn FnMut(SpatialObjectId, SpatialAabb2D) -> bool + Send + 'static>)),
        remove_spatial_object: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |id: SpatialObjectId| -> () {
                let was_missing = (!(*grid.lock().unwrap())
                    .bounds
                    .iter()
                    .any(|(entry_key, _)| entry_key == &id))
                    && (!(*grid.lock().unwrap())
                        .declined
                        .iter()
                        .any(|(entry_key, _)| entry_key == &id));
                _remove_from_grid(&mut (*grid.lock().unwrap()), id);
                if was_missing {
                    _report_grid_indexing(
                        &(*grid.lock().unwrap()),
                        id,
                        "absent".to_owned(),
                        "remove".to_owned(),
                        &(Some(flighthq_types::SpatialIndexingReason::A(
                            "missing-id".to_owned(),
                        ))),
                        0.0_f64,
                    );
                }
            }
        })
            as Box<dyn FnMut(SpatialObjectId) -> () + Send + 'static>)),
        clear_spatial_index: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move || -> () {
                (*grid.lock().unwrap()).cells.clear();
                (*grid.lock().unwrap()).bounds.clear();
                (*grid.lock().unwrap()).overflow.clear();
                (*grid.lock().unwrap()).declined.clear();
                (*grid.lock().unwrap()).seen.clear();
                (*grid.lock().unwrap()).pair_ids.clear();
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
        explain_spatial_indexing: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |id: SpatialObjectId| -> SpatialIndexingExplanation {
                return _explain_grid_indexing(&(*grid.lock().unwrap()), id);
            }
        })
            as Box<dyn FnMut(SpatialObjectId) -> SpatialIndexingExplanation + Send + 'static>)),
        query_spatial_pairs: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |mut out: Vec<SpatialPair>| -> () {
                _query_grid_pairs(&mut (*grid.lock().unwrap()), &mut out);
            }
        })
            as Box<dyn FnMut(Vec<SpatialPair>) -> () + Send + 'static>)),
        query_spatial_region: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut grid = grid.clone();
            move |region: SpatialAabb2D, mut out: Vec<SpatialObjectId>| -> () {
                _query_grid_region(&mut (*grid.lock().unwrap()), &region, &mut out);
            }
        })
            as Box<dyn FnMut(SpatialAabb2D, Vec<SpatialObjectId>) -> () + Send + 'static>)),
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

// Source: upstream/packages/spatial/src/uniformGrid.ts:99 (sha256:a7c50da57d96d2dc1fc6632b1a3cc89c156d199f2c3e75cb317e2e781ad58541)
pub fn set_spatial_indexing_guard(guard: &Option<SpatialIndexingGuard>) -> () {
    (*_INDEXING_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:106 (sha256:bf8ee7d148f40d101a35cf9e62d37751ed15b9b9ce6f5ad5dcbfcc8b44a4d9a1)
#[derive(Clone, Default)]
pub(crate) struct GridCell {
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

// Source: upstream/packages/spatial/src/uniformGrid.ts:123 (sha256:b250d19f277e6a68ba96f3acceaa4ab14f4153ce4c0834635c978ed262f4cc23)
#[derive(Clone, Default)]
pub(crate) struct UniformGrid {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cell_size: f64,
    pub cells: Vec<(String, GridCell)>,
    pub bounds: Vec<(SpatialObjectId, SpatialAabb2D)>,
    pub overflow: Vec<SpatialObjectId>,
    pub declined: Vec<(SpatialObjectId, SpatialDeclineReason)>,
    pub min_cell_x: f64,
    pub min_cell_y: f64,
    pub max_cell_x: f64,
    pub max_cell_y: f64,
    pub seen: Vec<SpatialObjectId>,
    pub pair_ids: Vec<SpatialObjectId>,
}
impl PartialEq for UniformGrid {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:139 (sha256:45dc8d4ede26d698e11710d2ab57bd04451e76c97cd4930c04a5008fa833df23)
fn _cell_index(coord: f64, cell_size: f64) -> f64 {
    return (coord / cell_size).floor();
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:145 (sha256:e8854bdbd64c484904fbf56d75556dd0d533af10ec478225ec75d36fa401f447)
fn _cell_key(cx: f64, cy: f64) -> String {
    return format!("{},{}", cx, cy);
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:150 (sha256:7cbb20339b0a28754d0a67d7f6a2a48e0c3689c564c0c486b26f74986929c4a9)
fn _explain_grid_indexing(grid: &UniformGrid, id: SpatialObjectId) -> SpatialIndexingExplanation {
    let decline_reason = grid
        .declined
        .iter()
        .find(|(entry_key, _)| entry_key == &id)
        .map(|(_, value)| value.clone());
    if (decline_reason).is_some() {
        return SpatialIndexingExplanation {
            __flight_identity: std::sync::Arc::new(()),
            bucket_count: 0.0_f64,
            id: id,
            mode: "declined".to_owned(),
            reason: Some((decline_reason.as_ref().unwrap()).clone()),
        };
    }
    if grid.overflow.iter().any(|item| item == &id) {
        return SpatialIndexingExplanation {
            __flight_identity: std::sync::Arc::new(()),
            bucket_count: 0.0_f64,
            id: id,
            mode: "overflow".to_owned(),
            reason: None,
        };
    }
    let bounds = grid
        .bounds
        .iter()
        .find(|(entry_key, _)| entry_key == &id)
        .map(|(_, value)| value.clone());
    if (bounds).is_none() {
        return SpatialIndexingExplanation {
            __flight_identity: std::sync::Arc::new(()),
            bucket_count: 0.0_f64,
            id: id,
            mode: "absent".to_owned(),
            reason: None,
        };
    }
    return SpatialIndexingExplanation {
        __flight_identity: std::sync::Arc::new(()),
        bucket_count: _spanned_cell_count(grid.cell_size, &bounds.as_ref().unwrap()),
        id: id,
        mode: "cells".to_owned(),
        reason: None,
    };
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:174 (sha256:9ef0f57aea5daabcbdb98c1722a4582fe6ef10d77c8847130eb9abd363ca6159)
fn _insert_into_grid(
    grid: &mut UniformGrid,
    id: SpatialObjectId,
    bounds: &SpatialAabb2D,
    operation: SpatialIndexingOperation,
) -> bool {
    if (((!(bounds.min_x).is_finite()) || (!(bounds.min_y).is_finite()))
        || (!(bounds.max_x).is_finite()))
        || (!(bounds.max_y).is_finite())
    {
        {
            let __flight_key = id;
            let __flight_value = "non-finite-bounds".to_owned();
            if let Some((_, value)) = grid
                .declined
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                grid.declined.push((__flight_key, __flight_value));
            }
        };
        _report_grid_indexing(
            grid,
            id,
            "declined".to_owned(),
            (operation).clone(),
            &(Some(flighthq_types::SpatialIndexingReason::A(
                "non-finite-bounds".to_owned(),
            ))),
            0.0_f64,
        );
        return false;
    }
    if (bounds.max_x < bounds.min_x) || (bounds.max_y < bounds.min_y) {
        {
            let __flight_key = id;
            let __flight_value = "inverted-bounds".to_owned();
            if let Some((_, value)) = grid
                .declined
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                grid.declined.push((__flight_key, __flight_value));
            }
        };
        _report_grid_indexing(
            grid,
            id,
            "declined".to_owned(),
            (operation).clone(),
            &(Some(flighthq_types::SpatialIndexingReason::A(
                "inverted-bounds".to_owned(),
            ))),
            0.0_f64,
        );
        return false;
    }
    let cs = grid.cell_size;
    let copy = SpatialAabb2D {
        __flight_identity: std::sync::Arc::new(()),
        min_x: bounds.min_x,
        min_y: bounds.min_y,
        max_x: bounds.max_x,
        max_y: bounds.max_y,
    };
    if (!(cs > 0.0_f64) && ((cs).is_finite())) {
        {
            let __flight_key = id;
            let __flight_value = (copy).clone();
            if let Some((_, value)) = grid.bounds.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                grid.bounds.push((__flight_key, __flight_value));
            }
        };
        {
            let __flight_value = id;
            if !grid.overflow.contains(&__flight_value) {
                grid.overflow.push(__flight_value);
            }
        };
        _report_grid_indexing(
            grid,
            id,
            "overflow".to_owned(),
            (operation).clone(),
            &(Some(flighthq_types::SpatialIndexingReason::A(
                "invalid-cell-size".to_owned(),
            ))),
            0.0_f64,
        );
        return true;
    }
    let spanned = _spanned_cell_count(cs, &copy);
    if (!(spanned <= MAX_INDEXED_CELLS_PER_OBJECT)) {
        {
            let __flight_key = id;
            let __flight_value = (copy).clone();
            if let Some((_, value)) = grid.bounds.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                grid.bounds.push((__flight_key, __flight_value));
            }
        };
        {
            let __flight_value = id;
            if !grid.overflow.contains(&__flight_value) {
                grid.overflow.push(__flight_value);
            }
        };
        _report_grid_indexing(
            grid,
            id,
            "overflow".to_owned(),
            (operation).clone(),
            &(None),
            spanned,
        );
        return true;
    }
    let mut cx0 = _cell_index(copy.min_x, cs);
    let cx1 = _cell_index(copy.max_x, cs);
    let mut cy0 = _cell_index(copy.min_y, cs);
    let cy1 = _cell_index(copy.max_y, cs);
    let had_cells = ((grid.cells.len() as f64) != 0.0_f64);
    {
        let __flight_key = id;
        let __flight_value = (copy).clone();
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
                        .find(|(entry_key, _)| entry_key == &(key).clone())
                        .map(|(_, value)| value.clone());
                    if ((cell).clone()).is_none() {
                        cell = Some(GridCell {
                            __flight_identity: std::sync::Arc::new(()),
                            cx: cx,
                            cy: cy,
                            ids: Vec::new(),
                        });
                        {
                            let __flight_key = (key).clone();
                            let __flight_value = ((cell).clone()).clone().unwrap();
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
    if (!had_cells) {
        grid.min_cell_x = cx0;
        grid.max_cell_x = cx1;
        grid.min_cell_y = cy0;
        grid.max_cell_y = cy1;
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
    return true;
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:251 (sha256:e1e648a622a015a0819fa39ff2eafda31435df98a27946b7577d522ec4a697e2)
fn _update_grid_object(
    grid: &mut UniformGrid,
    id: SpatialObjectId,
    bounds: &SpatialAabb2D,
) -> bool {
    let was_missing = (!grid.bounds.iter().any(|(entry_key, _)| entry_key == &id))
        && (!grid.declined.iter().any(|(entry_key, _)| entry_key == &id));
    let mut previous = grid
        .bounds
        .iter()
        .find(|(entry_key, _)| entry_key == &id)
        .map(|(_, value)| value.clone());
    if ((((((((previous).is_some()) && (!grid.overflow.iter().any(|item| item == &id)))
        && ((bounds.min_x).is_finite()))
        && ((bounds.min_y).is_finite()))
        && ((bounds.max_x).is_finite()))
        && ((bounds.max_y).is_finite()))
        && (bounds.min_x <= bounds.max_x))
        && (bounds.min_y <= bounds.max_y)
    {
        let cs = grid.cell_size;
        let spanned = _spanned_cell_count(cs, bounds);
        if ((((spanned <= MAX_INDEXED_CELLS_PER_OBJECT)
            && (_cell_index(previous.as_mut().unwrap().min_x, cs)
                == _cell_index(bounds.min_x, cs)))
            && (_cell_index(previous.as_mut().unwrap().min_y, cs)
                == _cell_index(bounds.min_y, cs)))
            && (_cell_index(previous.as_mut().unwrap().max_x, cs) == _cell_index(bounds.max_x, cs)))
            && (_cell_index(previous.as_mut().unwrap().max_y, cs) == _cell_index(bounds.max_y, cs))
        {
            previous.as_mut().unwrap().min_x = bounds.min_x;
            previous.as_mut().unwrap().min_y = bounds.min_y;
            previous.as_mut().unwrap().max_x = bounds.max_x;
            previous.as_mut().unwrap().max_y = bounds.max_y;
            return true;
        }
    }
    _remove_from_grid(grid, id);
    let inserted = _insert_into_grid(grid, id, bounds, "update".to_owned());
    if was_missing {
        let explanation = _explain_grid_indexing(grid, id);
        _report_grid_indexing(
            grid,
            id,
            (explanation.mode).clone(),
            "update".to_owned(),
            &(Some(flighthq_types::SpatialIndexingReason::A(
                "missing-id".to_owned(),
            ))),
            0.0_f64,
        );
    }
    return inserted;
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:290 (sha256:408ff55d8a02bbae0b984e678720cb73cbfd0ad4c4c8d5e2e5e5fad3124b9dee)
fn _is_spatial_aabb_contains_point(aabb: &SpatialAabb2D, x: f64, y: f64) -> bool {
    let min_x = aabb.min_x;
    let min_y = aabb.min_y;
    let max_x = aabb.max_x;
    let max_y = aabb.max_y;
    return (((x >= min_x) && (x < max_x)) && (y >= min_y)) && (y < max_y);
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:299 (sha256:c6ad9e3b6b93168e38ca6eb12ec92525c896c888fe8a45cf1f21a40c33b44900)
fn _is_spatial_aabb_overlapping(a: &SpatialAabb2D, b: &SpatialAabb2D) -> bool {
    let a_min_x = a.min_x;
    let a_min_y = a.min_y;
    let a_max_x = a.max_x;
    let a_max_y = a.max_y;
    let b_min_x = b.min_x;
    let b_min_y = b.min_y;
    let b_max_x = b.max_x;
    let b_max_y = b.max_y;
    return (((a_min_x < b_max_x) && (a_max_x > b_min_x)) && (a_min_y < b_max_y))
        && (a_max_y > b_min_y);
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:314 (sha256:5449163cc46e76efa44514082dc0053eb0d733df81ff6a4e3a9910a95513c606)
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

// Source: upstream/packages/spatial/src/uniformGrid.ts:363 (sha256:0cb1e297e85e9343d95a664ee22e439b5c0cc955fbefde284d664f6dc9676efa)
fn _remove_from_grid(grid: &mut UniformGrid, id: SpatialObjectId) -> () {
    {
        let __flight_key = id;
        if let Some(__flight_index) = grid
            .declined
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            grid.declined.remove(__flight_index);
            true
        } else {
            false
        }
    };
    let bounds = grid
        .bounds
        .iter()
        .find(|(entry_key, _)| entry_key == &id)
        .map(|(_, value)| value.clone());
    if (bounds).is_none() {
        return;
    }
    if {
        let __flight_value = id;
        if let Some(__flight_index) = grid
            .overflow
            .iter()
            .position(|item| item == &__flight_value)
        {
            grid.overflow.remove(__flight_index);
            true
        } else {
            false
        }
    } {
        {
            let __flight_key = id;
            if let Some(__flight_index) =
                grid.bounds.iter().position(|(key, _)| key == &__flight_key)
            {
                grid.bounds.remove(__flight_index);
                true
            } else {
                false
            }
        };
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
                        .find(|(entry_key, _)| entry_key == &(key).clone())
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
                    if ((cell.as_mut().unwrap().ids.len() as f64) == 0.0_f64) {
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
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:394 (sha256:09b882c0ea3d435f82ae51aa039af065b94171203651d284f481c9aeb2fb8447)
fn _query_grid_overflow_pairs(grid: &UniformGrid, out: &mut Vec<SpatialPair>) -> () {
    for id in ((grid.overflow).clone()).iter().cloned() {
        let bounds = grid
            .bounds
            .iter()
            .find(|(entry_key, _)| entry_key == &id)
            .map(|(_, value)| value.clone());
        if ((bounds).clone()).is_none() {
            continue;
        }
        for __iteration0 in ((grid.bounds).clone()).iter().cloned() {
            let other_id = __iteration0.0.clone();
            let other_bounds = __iteration0.1.clone();
            if (other_id == id) {
                continue;
            }
            if (grid.overflow.iter().any(|item| item == &other_id)) && (other_id < id) {
                continue;
            }
            if (!_is_spatial_aabb_overlapping(bounds.as_ref().unwrap(), &other_bounds)) {
                continue;
            }
            out.push(if (id < other_id) {
                SpatialPair {
                    __flight_identity: std::sync::Arc::new(()),
                    a: id,
                    b: other_id,
                }
            } else {
                SpatialPair {
                    __flight_identity: std::sync::Arc::new(()),
                    a: other_id,
                    b: id,
                }
            });
        }
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:414 (sha256:677f4249db688911b2ed134c9d9b38826c3e71fc89c3f1204053832823b78c98)
fn _query_grid_pairs(grid: &mut UniformGrid, out: &mut Vec<SpatialPair>) -> () {
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
        if ((cell.ids.len() as f64) < 2.0_f64) {
            continue;
        }
        grid.pair_ids.clear();
        for id in (cell.ids).iter().cloned() {
            grid.pair_ids.push(id);
        }
        {
            let mut i = 0.0_f64;
            while (i < (grid.pair_ids.len() as f64)) {
                {
                    let mut j = (i + 1.0_f64);
                    while (j < (grid.pair_ids.len() as f64)) {
                        let mut a = grid.pair_ids[i as usize].clone();
                        let mut b = grid.pair_ids[j as usize].clone();
                        if (a > b) {
                            let t = a;
                            a = b;
                            b = t;
                        }
                        let ab = grid
                            .bounds
                            .iter()
                            .find(|(entry_key, _)| entry_key == &a)
                            .map(|(_, value)| value.clone());
                        let bb = grid
                            .bounds
                            .iter()
                            .find(|(entry_key, _)| entry_key == &b)
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
    if ((grid.overflow.len() as f64) != 0.0_f64) {
        _query_grid_overflow_pairs(grid, out);
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:444 (sha256:7ef8d9b1f2cb0826f8c942cc22cce69a767dc403bf093d6753df7530e896f243)
#[derive(Clone, Default)]
struct ReportGridIndexingSynthesizedRecord2498162634 {
    __flight_identity: std::sync::Arc<()>,
    cell_size: f64,
    id: SpatialObjectId,
    mode: SpatialIndexingMode,
    operation: SpatialIndexingOperation,
    reason: Option<SpatialIndexingReason>,
    would_occupy_bucket_count: f64,
}
impl PartialEq for ReportGridIndexingSynthesizedRecord2498162634 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn _report_grid_indexing(
    grid: &UniformGrid,
    id: SpatialObjectId,
    mode: SpatialIndexingMode,
    operation: SpatialIndexingOperation,
    reason: &Option<SpatialIndexingReason>,
    would_occupy_bucket_count: f64,
) -> () {
    if ((*_INDEXING_GUARD.lock().unwrap()).clone()).is_none() {
        return;
    }
    {
        let __flight_callback = ((*_INDEXING_GUARD.lock().unwrap()).as_ref().unwrap()).clone();
        let __flight_result = __flight_callback.lock().unwrap()({
            let __flight_source = &(ReportGridIndexingSynthesizedRecord2498162634 {
                __flight_identity: std::sync::Arc::new(()),
                cell_size: grid.cell_size,
                id: id,
                mode: (mode).clone(),
                operation: (operation).clone(),
                reason: (*reason).clone(),
                would_occupy_bucket_count: would_occupy_bucket_count,
            });
            SpatialIndexingNotice {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                cell_size: __flight_source.cell_size,
                id: __flight_source.id,
                mode: (__flight_source.mode).clone(),
                operation: (__flight_source.operation).clone(),
                would_occupy_bucket_count: __flight_source.would_occupy_bucket_count,
                reason: (__flight_source.reason).clone(),
            }
        });
        __flight_result
    };
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:460 (sha256:328856480673e52d18188cab1125aa4052d753f1995fafc145ba1bebf28c39df)
fn _query_grid_point(grid: &UniformGrid, x: f64, y: f64, out: &mut Vec<SpatialObjectId>) -> () {
    out.clear();
    let cs = grid.cell_size;
    let cell = grid
        .cells
        .iter()
        .find(|(entry_key, _)| entry_key == &_cell_key(_cell_index(x, cs), _cell_index(y, cs)))
        .map(|(_, value)| value.clone());
    if (cell).is_some() {
        for id in ((cell.as_ref().unwrap().ids).clone()).iter().cloned() {
            let bounds = grid
                .bounds
                .iter()
                .find(|(entry_key, _)| entry_key == &id)
                .map(|(_, value)| value.clone());
            if (((bounds).clone()).is_some())
                && (_is_spatial_aabb_contains_point(bounds.as_ref().unwrap(), x, y))
            {
                out.push(id);
            }
        }
    }
    for id in ((grid.overflow).clone()).iter().cloned() {
        let bounds = grid
            .bounds
            .iter()
            .find(|(entry_key, _)| entry_key == &id)
            .map(|(_, value)| value.clone());
        if (((bounds).clone()).is_some())
            && (_is_spatial_aabb_contains_point(bounds.as_ref().unwrap(), x, y))
        {
            out.push(id);
        }
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:480 (sha256:ff7406cdcac62d49a877dbbbf9f21580b03f65a6bdba4dfd00a0564fac7834aa)
fn _query_grid_ray(
    grid: &mut UniformGrid,
    ox: f64,
    oy: f64,
    dx: f64,
    dy: f64,
    out: &mut Vec<SpatialObjectId>,
) -> () {
    out.clear();
    let cs = grid.cell_size;
    grid.seen.clear();
    if (dx == 0.0_f64) && (dy == 0.0_f64) {
        _query_grid_point(grid, ox, oy, out);
        return;
    }
    for id in ((grid.overflow).clone()).iter().cloned() {
        let bounds = grid
            .bounds
            .iter()
            .find(|(entry_key, _)| entry_key == &id)
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
    if ((grid.cells.len() as f64) == 0.0_f64) {
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
                .find(|(entry_key, _)| entry_key == &_cell_key(cx, cy))
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
            .find(|(entry_key, _)| entry_key == &id)
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

// Source: upstream/packages/spatial/src/uniformGrid.ts:579 (sha256:3aa9ece908fb951b2b0def6783d54b5c6f6cb58f2cf811ccc74fe98b9d30f25d)
fn _query_grid_region(
    grid: &mut UniformGrid,
    region: &SpatialAabb2D,
    out: &mut Vec<SpatialObjectId>,
) -> () {
    out.clear();
    let cs = grid.cell_size;
    grid.seen.clear();
    if (!(_spanned_cell_count(cs, region) <= (grid.cells.len() as f64))) {
        for __iteration1 in ((grid.bounds).clone()).iter().cloned() {
            let id = __iteration1.0.clone();
            let bounds = __iteration1.1.clone();
            if _is_spatial_aabb_overlapping(&bounds, region) {
                out.push(id);
            }
        }
        return;
    }
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
                        .find(|(entry_key, _)| entry_key == &_cell_key(cx, cy))
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
                            .find(|(entry_key, _)| entry_key == &id)
                            .map(|(_, value)| value.clone());
                        if (((bounds).clone()).is_some())
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
    for id in ((grid.overflow).clone()).iter().cloned() {
        let bounds = grid
            .bounds
            .iter()
            .find(|(entry_key, _)| entry_key == &id)
            .map(|(_, value)| value.clone());
        if (((bounds).clone()).is_some())
            && (_is_spatial_aabb_overlapping(bounds.as_ref().unwrap(), region))
        {
            out.push(id);
        }
    }
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:618 (sha256:07ca8ef38ea120b24487b9f3a36d938f256c662334b830e01df9f05df8953ec9)
fn _spanned_cell_count(cell_size: f64, aabb: &SpatialAabb2D) -> f64 {
    let cx0 = _cell_index(aabb.min_x, cell_size);
    let cx1 = _cell_index(aabb.max_x, cell_size);
    let cy0 = _cell_index(aabb.min_y, cell_size);
    let cy1 = _cell_index(aabb.max_y, cell_size);
    return (((cx1 - cx0) + 1.0_f64) * ((cy1 - cy0) + 1.0_f64));
}

// Source: upstream/packages/spatial/src/uniformGrid.ts:628 (sha256:e6864a197036faa67642beacf162e731f77ac0295548433db16db76e3f1cc9f1)
static _INDEXING_GUARD: std::sync::LazyLock<std::sync::Mutex<Option<SpatialIndexingGuard>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
