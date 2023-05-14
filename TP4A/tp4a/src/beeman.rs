use crate::euler_algorithm;
use crate::CallbackFn;

fn beeman_position(r: f64, v: f64, a: f64, prev_a: f64, dt: f64) -> f64 {
    r + v * dt + (2.0 / 3.0) * a * dt.powi(2) - (1.0 / 6.0) * prev_a * dt.powi(2)
}

fn beeman_velocity(v: f64, a: f64, prev_a: f64, next_a: f64, dt: f64) -> f64 {
    v + (1.0 / 3.0) * next_a * dt + (5.0 / 6.0) * a * dt - (1.0 / 6.0) * prev_a * dt
}

pub(crate) fn beeman<F: Fn(f64, f64) -> f64, Callback: CallbackFn>(
    r: f64,
    v: f64,
    calculate_force: F,
    dt: f64,
    m: f64,
    mut callback: Callback,
) {
    let mut t = 0.0;
    let tf = 5.0;

    let mut curr_r = r; // current position
    let mut curr_v = v; // current velocity

    let mut curr_f = calculate_force(r, v);
    let (prev_r, prev_v): (f64, f64) = euler_algorithm(curr_r, curr_v, curr_f, -dt, m);

    let prev_f = calculate_force(prev_r, prev_v);
    let mut prev_a = prev_f / m;

    while t < tf {
        callback(t, curr_r, curr_v);

        curr_f = calculate_force(curr_r, curr_v);
        let curr_a = curr_f / m;

        let next_r = beeman_position(curr_r, curr_v, curr_a, prev_a, dt);

        let predicted_v = curr_v + 1.5 * curr_a * dt - 0.5 * prev_a * dt;

        let next_a = (predicted_v - curr_v) / dt;

        let next_v = beeman_velocity(curr_v, curr_a, prev_a, next_a, dt);

        prev_a = curr_a;
        curr_r = next_r;
        curr_v = next_v;

        t += dt;
    }

    callback(t, curr_r, curr_v);
}
