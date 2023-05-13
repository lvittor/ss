use crate::{beeman::beeman, verlet::verlet};

mod verlet;
mod beeman;

pub fn euler_algorithm(r: f64, v: f64, f: f64, dt: f64, m: f64) -> (f64, f64) {
    let next_r = r + dt * v + (dt.powi(2) / (2.0 * m)) * f;

    let next_v = v + (dt / m) * f;

    (next_r, next_v)
}

fn fitfh_order_gear_corrector_predictor_algorithm<F: FnOnce(f64, f64) -> f64>(
    r: f64,
    v: f64,
    a: f64,
    r3: f64,
    r4: f64,
    r5: f64,
    dt: f64,
    m: f64,
    calculate_force: F,
) -> (f64, f64, f64, f64, f64, f64) {
    // Calculate the predictions
    let r1 = v;
    let r2 = a;

    let rp = r
        + r1 * dt
        + r2 * dt.powi(2) / fac_f64(2)
        + r3 * dt.powi(3) / fac_f64(3)
        + r4 * dt.powi(4) / fac_f64(4)
        + r5 * dt.powi(5) / fac_f64(5);
    let r1p = r1
        + r2 * dt
        + r3 * dt.powi(2) / fac_f64(2)
        + r4 * dt.powi(3) / fac_f64(3)
        + r5 * dt.powi(4) / fac_f64(4);
    let r2p = r2 + r3 * dt + r4 * dt.powi(2) / fac_f64(2) + r5 * dt.powi(3) / fac_f64(3);
    let r3p = r3 + r4 * dt + r5 * dt.powi(2) / fac_f64(2);
    let r4p = r4 + r5 * dt;
    let r5p = r5;

    // Calculate the deltas
    let da = calculate_force(rp, r1p) / m - r2p; // TODO: validate if this (delta a) is calculated correctly
    let dr2 = da * dt.powi(2) / fac_f64(2); // delta r2

    // Calculate the corrections
    let rc = rp + (3.0 / 16.0) * dr2;
    let vc = r1p + (251.0 / 360.0) * dr2 / dt;
    let ac = r2p + (1.0) * dr2 * fac_f64(2) / dt.powi(2);

    let r3c = r3p + (11.0 / 18.0) * dr2 * fac_f64(3) / dt.powi(3);
    let r4c = r4p + (1.0 / 6.0) * dr2 * fac_f64(4) / dt.powi(4);
    let r5c = r5p + (1.0 / 60.0) * dr2 * fac_f64(5) / dt.powi(5);

    (rc, vc, ac, r3c, r4c, r5c)
}

const fn fac(n: u64) -> u64 {
    match n {
        0u64 | 1u64 => 1,
        2u64..=20u64 => fac(n - 1u64) * n,
        _ => 0,
    }
}

const fn fac_f64(n: u64) -> f64 {
    fac(n) as f64
}

fn analytic_solution(A: f64, gamma: f64, m: f64, t: f64, k: f64) -> f64 {
    let beta = gamma / (2.0 * m);
    let omega = ((k / m) - beta.powi(2)).sqrt();

    A * (-beta * t).exp() * (omega * t).cos()
}

fn main() {
    // Constants
    const M: f64 = 70.0; // m = 70 kg
    const K: f64 = 1e4; // k = 10^4 N/m
    const GAMMA: f64 = 100.0; // gamma = 100 kg/s
    const A: f64 = 1.0; // A = 1 m

    // Initial Conditions
    const R: f64 = 1.0; // r(t=0) = 1m
    const V: f64 = -A * GAMMA / (2.0 * M); // v(t=0) = -a * gamma / (2.0 * m)

    const DT: f64 = 1e-5;

    let calc_force = |r: f64, v: f64| -K * r - GAMMA * v;

    let analytic_solution = |t: f64| analytic_solution(A, GAMMA, M, t, K);

    //verlet(R, V, calc_force, analytic_solution, DT, M);
    beeman(R, V, calc_force, analytic_solution, DT, M);
    //fitfh_order_gear_corrector_predictor_algorithm(r, v, 0.0, 0.0, 0.0, 0.0, dt, calc_force);
}
