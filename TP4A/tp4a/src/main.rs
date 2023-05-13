use crate::{beeman::beeman, verlet::verlet, gear_predictor_corrector::gear_predictor_corrector};

mod verlet;
mod beeman;
mod gear_predictor_corrector;

pub fn euler_algorithm(r: f64, v: f64, f: f64, dt: f64, m: f64) -> (f64, f64) {
    let next_r = r + dt * v + (dt.powi(2) / (2.0 * m)) * f;

    let next_v = v + (dt / m) * f;

    (next_r, next_v)
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
    let calc_initial_integration = |r: f64, v: f64| {
        let r = r;
        let r1 = v;
        let r2 = calc_force(r, r1) / M;
        let r3 = -K * r1 - GAMMA * r2;
        let r4 = -K * r2 - GAMMA * r3;
        let r5 = -K * r3 - GAMMA * r4;

        (r, r1, r2, r3, r4, r5)
    };

    let analytic_solution = |t: f64| analytic_solution(A, GAMMA, M, t, K);

    //verlet(R, V, calc_force, analytic_solution, DT, M);
    //beeman(R, V, calc_force, analytic_solution, DT, M);
    gear_predictor_corrector(R, V, calc_force, calc_initial_integration, analytic_solution, DT, M);
}
