use crate::euler_algorithm;


fn verlet_algorithm(r: f64, prev_r: f64, m: f64, f: f64, dt: f64) -> (f64, f64) {
    let r_next = 2.0 * r - prev_r + (f / m) * dt.powi(2);
    let v_next = (r_next - prev_r) / (2.0 * dt);

    (r_next, v_next)
}

pub fn verlet<F: Fn(f64, f64) -> f64, F2: Fn(f64) -> f64>(
    r: f64,
    v: f64,
    calculate_force: F,
    analytic_solution: F2,
    dt: f64,
    m: f64,
) {
    let mut t = 0.0;
    let tf = 5.0;

    let mut f = calculate_force(r, v);
    let (mut prev_r, _): (f64, f64) = euler_algorithm(r, v, f, -dt, m);

    let mut diff: f64 = 0.0;
    let mut steps = 0;

    let mut curr_r = r; // current position
    let mut curr_v = v; // current velocity

    while t < tf {
        diff += (analytic_solution(t) - curr_r).powi(2);

        f = calculate_force(curr_r, curr_v);

        let (next_r, next_v) = verlet_algorithm(curr_r, prev_r, m, f, dt);

        // write to file (next_r, next_v)
        println!("{:.2} {:.4} {:.4}", t, next_r, next_v);

        prev_r = curr_r;

        curr_r = next_r;
        curr_v = next_v;

        t += dt;
        steps += 1;
    }

    diff += (analytic_solution(t) - curr_r).powi(2);
    let mse = diff / steps as f64; // Mean Square Error
    println!("{}", mse);
}

