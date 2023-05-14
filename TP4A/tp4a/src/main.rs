#![feature(trait_alias)]

use crate::{beeman::beeman, gear_predictor_corrector::gear_predictor_corrector, verlet::verlet};
use clap::{Parser, Subcommand};

mod beeman;
mod gear_predictor_corrector;
mod verlet;

trait CallbackFn = FnMut(f64, f64, f64);

pub fn euler_algorithm(r: f64, v: f64, f: f64, dt: f64, m: f64) -> (f64, f64) {
    let next_r = r + dt * v + (dt.powi(2) / (2.0 * m)) * f;

    let next_v = v + (dt / m) * f;

    (next_r, next_v)
}

fn analytic_solution(a: f64, gamma: f64, m: f64, t: f64, k: f64) -> f64 {
    let beta = gamma / (2.0 * m);
    let omega = ((k / m) - beta.powi(2)).sqrt();

    a * (-beta * t).exp() * (omega * t).cos()
}

pub(crate) fn analytic<F: Fn(f64) -> f64, Callback: CallbackFn>(
    analytic_solution: F,
    dt: f64,
    mut callback: Callback,
) {
    let mut t = 0.0;
    let tf = 5.0;

    while t < tf {
        let curr_r = analytic_solution(t);
        callback(t, curr_r, 0.0);
        t += dt;
    }

    callback(t, analytic_solution(t), 0.0);
}

#[derive(Subcommand, Default, Debug, Clone)]
enum Method {
    #[default]
    Analytic,
    Gear,
    Verlet,
    Beeman,
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    method: Method,

    #[clap(long)]
    delta_t_exponent: i32,
}

fn main() {
    let args = Args::parse();

    // Constants
    const M: f64 = 70.0; // m = 70 kg
    const K: f64 = 1e4; // k = 10^4 N/m
    const GAMMA: f64 = 100.0; // gamma = 100 kg/s
    const A: f64 = 1.0; // A = 1 m

    // Initial Conditions
    const R: f64 = 1.0; // r(t=0) = 1m
    const V: f64 = -A * GAMMA / (2.0 * M); // v(t=0) = -a * gamma / (2.0 * m)

    let dt = 10f64.powi(-args.delta_t_exponent);
    let output_every: usize = (0.02 / dt) as usize;

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

    let mut diff = 0.0;
    let mut steps = 0;
    let print_csv_row = |t: f64, r: f64, v: f64| {
        diff += (analytic_solution(t) - r).powi(2);

        // print every OUTPUT_EVERY steps
        if steps % output_every == 0 {
            println!("{t:.4},{r},{v}");
        }

        steps += 1;
    };

    println!("t,r,v");

    match args.method {
        Method::Analytic => analytic(analytic_solution, dt, print_csv_row),
        Method::Gear => gear_predictor_corrector(
            R,
            V,
            calc_force,
            calc_initial_integration,
            dt,
            M,
            print_csv_row,
        ),
        Method::Verlet => verlet(R, V, calc_force, dt, M, print_csv_row),
        Method::Beeman => beeman(R, V, calc_force, dt, M, print_csv_row),
    }

    let mse = diff / steps as f64; // Mean Square Error
    eprintln!("{}", mse);
}
