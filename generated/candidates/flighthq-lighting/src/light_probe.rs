// @generated from upstream/packages/lighting/src/lightProbe.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_geometry::create_vector3;
use flighthq_types::{
    AabbLike, EntityConstruction, LIGHT_PROBE_SH_FLOATS as light_probe_sh_floats_constant,
    LightProbe, LightProbeGrid, Vector3, Vector3Like,
};

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

// Source: upstream/packages/lighting/src/lightProbe.ts:14 (sha256:6b134171dc886e3c12a7a7322ebb2e3343aca6a606b48402ad1309bbaf66a4f5)
pub fn create_light_probe(position: &Vector3Like, sh_coefficients: Option<Vec<f32>>) -> LightProbe {
    let mut out = allocate_entity();
    initialize_light_probe((out).clone(), position, ((sh_coefficients).clone()).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/lightProbe.ts:29 (sha256:1274f964c95b49ca3696f632ba10794c0e736a68cdb1bc031439cc545b6e16bd)
pub fn create_light_probe_grid(
    probes: &Vec<LightProbe>,
    bounds: &AabbLike,
    resolution: &Vector3Like,
) -> LightProbeGrid {
    let mut out = allocate_entity();
    initialize_light_probe_grid((out).clone(), probes, bounds, resolution);
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/lightProbe.ts:55 (sha256:aeb52672a7ed1669cb4daad6f1ec8612a05a9b1109648742af4195b2797289a4)
pub fn evaluate_light_probe_sh(
    out: &mut Vec<f32>,
    normal: &Vector3Like,
    sh_coefficients: &Vec<f32>,
) -> () {
    let length = ((normal.x).powi(2) + (normal.y).powi(2) + (normal.z).powi(2)).sqrt();
    if (length == 0.0_f64) {
        out[0.0_f64 as usize] = (0.0_f64) as f32;
        out[1.0_f64 as usize] = (0.0_f64) as f32;
        out[2.0_f64 as usize] = (0.0_f64) as f32;
        return;
    }
    let inverse_length = (1.0_f64 / length);
    let x = (normal.x * inverse_length);
    let y = (normal.y * inverse_length);
    let z = (normal.z * inverse_length);
    write_sh_basis(&mut (*SH_BASIS_SCRATCH.lock().unwrap()), x, y, z);
    let mut r = 0.0_f64;
    let mut g = 0.0_f64;
    let mut b = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < SH_COEFFICIENT_COUNT) {
            let basis = ((*SH_BASIS_SCRATCH.lock().unwrap())[i as usize] as f64);
            let offset = (i * 3.0_f64);
            r += ((sh_coefficients[offset as usize] as f64) * basis);
            g += ((sh_coefficients[(offset + 1.0_f64) as usize] as f64) * basis);
            b += ((sh_coefficients[(offset + 2.0_f64) as usize] as f64) * basis);
            {
                i += 1.0;
                i
            };
        }
    }
    out[0.0_f64 as usize] = (r) as f32;
    out[1.0_f64 as usize] = (g) as f32;
    out[2.0_f64 as usize] = (b) as f32;
}

// Source: upstream/packages/lighting/src/lightProbe.ts:90 (sha256:6320a9046eb7cd75f598bb541ac7f4a34ff10631b71d1908d999cc70faee908d)
pub fn initialize_light_probe(
    out: EntityConstruction<LightProbe>,
    position: &Vector3Like,
    sh_coefficients: Option<Vec<f32>>,
) -> () {
    if ((sh_coefficients).is_some())
        && ((sh_coefficients.as_ref().unwrap().len() as f64) != light_probe_sh_floats_constant)
    {
        panic!(
            "{}",
            format!(
                "LightProbe.shCoefficients must be {} floats, got {}",
                light_probe_sh_floats_constant,
                (sh_coefficients.as_ref().unwrap().len() as f64)
            )
        );
    }
    crate::host_set("host.enabled", true);
    crate::host_set(
        "host.position",
        create_vector3(Some(position.x), Some(position.y), Some(position.z)),
    );
    crate::host_set(
        "host.shCoefficients",
        vec![0.0_f32; (light_probe_sh_floats_constant) as usize],
    );
    if (sh_coefficients).is_some() {
        crate::host_value::<()>("host.set");
    }
}

// Source: upstream/packages/lighting/src/lightProbe.ts:104 (sha256:ae65d511d431d6f5623591a8d403cddfce86783f8efc32a90d3339b05160e24e)
#[derive(Clone, Default)]
struct InitializeLightProbeGridSynthesizedRecord1769806272 {
    __flight_identity: std::sync::Arc<()>,
    max: Vector3,
    min: Vector3,
}
impl PartialEq for InitializeLightProbeGridSynthesizedRecord1769806272 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn initialize_light_probe_grid(
    out: EntityConstruction<LightProbeGrid>,
    probes: &Vec<LightProbe>,
    bounds: &AabbLike,
    resolution: &Vector3Like,
) -> () {
    let expected = ((resolution.x * resolution.y) * resolution.z);
    if ((probes.len() as f64) != expected) {
        panic!(
            "{}",
            format!(
                "{}{}",
                format!(
                    "LightProbeGrid.probes must hold {} probes for resolution ",
                    expected
                ),
                format!(
                    "{}x{}x{}, got {}",
                    resolution.x,
                    resolution.y,
                    resolution.z,
                    (probes.len() as f64)
                )
            )
        );
    }
    crate::host_set(
        "host.bounds",
        InitializeLightProbeGridSynthesizedRecord1769806272 {
            __flight_identity: std::sync::Arc::new(()),
            max: create_vector3(Some(bounds.max.x), Some(bounds.max.y), Some(bounds.max.z)),
            min: create_vector3(Some(bounds.min.x), Some(bounds.min.y), Some(bounds.min.z)),
        },
    );
    crate::host_set("host.enabled", true);
    crate::host_set("host.probes", probes);
    crate::host_set(
        "host.resolution",
        create_vector3(Some(resolution.x), Some(resolution.y), Some(resolution.z)),
    );
}

// Source: upstream/packages/lighting/src/lightProbe.ts:138 (sha256:f0ee474116d4a752a976eee7f6a277c6caa2c6c40d20bd4066cc4a9d0dddfed0)
pub fn sample_light_probe_grid(
    out: &mut Vec<f32>,
    position: &Vector3Like,
    grid: &LightProbeGrid,
) -> bool {
    if (!grid.enabled) || ((grid.probes.len() as f64) == 0.0_f64) {
        return false;
    }
    let nx = grid.resolution.x;
    let ny = grid.resolution.y;
    let nz = grid.resolution.z;
    let x = axis_coordinate(position.x, grid.bounds.min.x, grid.bounds.max.x, nx);
    let y = axis_coordinate(position.y, grid.bounds.min.y, grid.bounds.max.y, ny);
    let z = axis_coordinate(position.z, grid.bounds.min.z, grid.bounds.max.z, nz);
    let x0 = (x).floor();
    let y0 = (y).floor();
    let z0 = (z).floor();
    let fx = (x - x0);
    let fy = (y - y0);
    let fz = (z - z0);
    {
        let __flight_value = (0.0_f64) as f32;
        let __flight_collection = &mut (*SH_SAMPLE_SCRATCH.lock().unwrap());
        __flight_collection.fill(__flight_value);
        __flight_collection.clone()
    };
    let mut total_weight = 0.0_f64;
    {
        let mut corner = 0.0_f64;
        while (corner < 8.0_f64) {
            let cx = (__flight_js_to_i32(corner) & __flight_js_to_i32(1.0_f64)) as f64;
            let cy = (__flight_js_to_i32(
                (__flight_js_to_i32(corner) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64,
            ) & __flight_js_to_i32(1.0_f64)) as f64;
            let cz = (__flight_js_to_i32(
                (__flight_js_to_i32(corner) >> (__flight_js_to_u32(2.0_f64) & 31)) as f64,
            ) & __flight_js_to_i32(1.0_f64)) as f64;
            let weight = ((if (cx == 1.0_f64) { fx } else { (1.0_f64 - fx) }
                * if (cy == 1.0_f64) { fy } else { (1.0_f64 - fy) })
                * if (cz == 1.0_f64) { fz } else { (1.0_f64 - fz) });
            if (weight == 0.0_f64) {
                {
                    corner += 1.0;
                    corner
                };
                continue;
            }
            let probe: Option<LightProbe> = grid
                .probes
                .get((((x0 + cx) + ((y0 + cy) * nx)) + (((z0 + cz) * nx) * ny)) as usize)
                .cloned();
            if ((probe).is_none()) || (!probe.as_ref().unwrap().enabled) {
                {
                    corner += 1.0;
                    corner
                };
                continue;
            }
            {
                let mut i = 0.0_f64;
                while (i < light_probe_sh_floats_constant) {
                    (*SH_SAMPLE_SCRATCH.lock().unwrap())[i as usize] +=
                        ((probe.as_ref().unwrap().sh_coefficients[i as usize] as f64) * weight)
                            as f32;
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            total_weight += weight;
            {
                corner += 1.0;
                corner
            };
        }
    }
    if (total_weight == 0.0_f64) {
        return false;
    }
    let inverse_weight = (1.0_f64 / total_weight);
    {
        let mut i = 0.0_f64;
        while (i < light_probe_sh_floats_constant) {
            out[i as usize] =
                (((*SH_SAMPLE_SCRATCH.lock().unwrap())[i as usize] as f64) * inverse_weight) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/lighting/src/lightProbe.ts:200 (sha256:89b8dce8cd65341c87bcdf771f1d0e6b334333713b01787135ebf330965007c8)
pub fn set_light_probe_sh_from_colors(
    target: &mut LightProbe,
    colors: &Vec<f32>,
    directions: &Vec<f32>,
) -> () {
    {
        let __flight_value = (0.0_f64) as f32;
        let __flight_collection = &mut target.sh_coefficients;
        __flight_collection.fill(__flight_value);
        __flight_collection.clone()
    };
    let sample_count =
        (__flight_js_to_i32(((colors.len() as f64).min((directions.len() as f64)) / 3.0_f64))
            | __flight_js_to_i32(0.0_f64)) as f64;
    if (sample_count == 0.0_f64) {
        return;
    }
    let solid_angle = ((4.0_f64 * std::f64::consts::PI) / sample_count);
    {
        let mut sample = 0.0_f64;
        while (sample < sample_count) {
            let offset = (sample * 3.0_f64);
            let dx = (directions[offset as usize] as f64);
            let dy = (directions[(offset + 1.0_f64) as usize] as f64);
            let dz = (directions[(offset + 2.0_f64) as usize] as f64);
            let length =
                (((dx).clone()).powi(2) + ((dy).clone()).powi(2) + ((dz).clone()).powi(2)).sqrt();
            if (length == 0.0_f64) {
                {
                    sample += 1.0;
                    sample
                };
                continue;
            }
            let inverse_length = (1.0_f64 / length);
            write_sh_basis(
                &mut (*SH_BASIS_SCRATCH.lock().unwrap()),
                ((dx).clone() * inverse_length),
                ((dy).clone() * inverse_length),
                ((dz).clone() * inverse_length),
            );
            let r = ((colors[offset as usize] as f64) * solid_angle);
            let g = ((colors[(offset + 1.0_f64) as usize] as f64) * solid_angle);
            let b = ((colors[(offset + 2.0_f64) as usize] as f64) * solid_angle);
            {
                let mut i = 0.0_f64;
                while (i < SH_COEFFICIENT_COUNT) {
                    let basis = ((*SH_BASIS_SCRATCH.lock().unwrap())[i as usize] as f64);
                    let target3 = (i * 3.0_f64);
                    target.sh_coefficients[target3 as usize] += (r * basis) as f32;
                    target.sh_coefficients[(target3 + 1.0_f64) as usize] += (g * basis) as f32;
                    target.sh_coefficients[(target3 + 2.0_f64) as usize] += (b * basis) as f32;
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            {
                sample += 1.0;
                sample
            };
        }
    }
}

// Source: upstream/packages/lighting/src/lightProbe.ts:237 (sha256:60618ffd96f3812d17f96dac496d8f04736a11c9b70cb44fecaff4524a040afa)
fn axis_coordinate(value: f64, min: f64, max: f64, count: f64) -> f64 {
    if (count <= 1.0_f64) || (max <= min) {
        return 0.0_f64;
    }
    let t = ((value - min) / (max - min));
    let scaled = (t * (count - 1.0_f64));
    if (!(scaled > 0.0_f64)) {
        return 0.0_f64;
    }
    return (scaled).min((count - 1.0_f64));
}

// Source: upstream/packages/lighting/src/lightProbe.ts:249 (sha256:2c481b8bca7f2c62060321341b4bb96998468ac7cbc3f65a9fee2735c87de3cd)
fn write_sh_basis(out: &mut Vec<f32>, x: f64, y: f64, z: f64) -> () {
    out[0.0_f64 as usize] = (0.28209479177387814_f64) as f32;
    out[1.0_f64 as usize] = (0.4886025119029199_f64 * y) as f32;
    out[2.0_f64 as usize] = (0.4886025119029199_f64 * z) as f32;
    out[3.0_f64 as usize] = (0.4886025119029199_f64 * x) as f32;
    out[4.0_f64 as usize] = ((1.0925484305920792_f64 * x) * y) as f32;
    out[5.0_f64 as usize] = ((1.0925484305920792_f64 * y) * z) as f32;
    out[6.0_f64 as usize] = (0.31539156525252005_f64 * (((3.0_f64 * z) * z) - 1.0_f64)) as f32;
    out[7.0_f64 as usize] = ((1.0925484305920792_f64 * x) * z) as f32;
    out[8.0_f64 as usize] = (0.5462742152960396_f64 * ((x * x) - (y * y))) as f32;
}

// Source: upstream/packages/lighting/src/lightProbe.ts:261 (sha256:2b0ef95b98168941f5c305ae7d25cdb7bb965df052844045ccb1a4664510b2b4)
const SH_COEFFICIENT_COUNT: f64 = 9.0_f64;

// Source: upstream/packages/lighting/src/lightProbe.ts:262 (sha256:62e8844ce9fb8b777f6309a203f08f319cb5ba7c7e64dd9d548c6103e43d874e)
static SH_BASIS_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0.0_f32; (SH_COEFFICIENT_COUNT) as usize])
    });

// Source: upstream/packages/lighting/src/lightProbe.ts:263 (sha256:d5a47c86e066dd22c21fad5df9a7c083cab400c00e794f1812f92594be6070ad)
static SH_SAMPLE_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0.0_f32; (light_probe_sh_floats_constant) as usize])
    });
