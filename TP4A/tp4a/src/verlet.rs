use crate::euler_algorithm;
use crate::CallbackFn;

fn verlet_algorithm(r: f64, prev_r: f64, m: f64, f: f64, dt: f64) -> (f64, f64) {
    let r_next = 2.0 * r - prev_r + (f / m) * dt.powi(2);
    let v_next = (r_next - prev_r) / (2.0 * dt);

    (r_next, v_next)
}

pub(crate) fn verlet<F: Fn(f64, f64) -> f64, Callback: CallbackFn>(
    r: f64,
    v: f64,
    calculate_force: F,
    dt: f64,
    m: f64,
    mut callback: Callback,
) {
    let mut t = 0.0;
    let tf = 5.0;

    let mut f = calculate_force(r, v);
    let (mut prev_r, _): (f64, f64) = euler_algorithm(r, v, f, -dt, m);

    let mut curr_r = r; // current position
    let mut curr_v = v; // current velocity

    while t < tf {
        callback(t, curr_r, curr_v);

        f = calculate_force(curr_r, curr_v);

        let (next_r, next_v) = verlet_algorithm(curr_r, prev_r, m, f, dt);

        prev_r = curr_r;

        curr_r = next_r;
        curr_v = next_v;

        t += dt;
    }

    callback(t, curr_r, curr_v);
}
