fn euler_algorithm(
    r: f64, 
    v: f64, 
    f: f64, 
    dt: f64, 
    m: f64
) -> (f64, f64){
    let next_r = r  + dt * v + (dt.powi(2) / (2.0 * m)) * f;

    let next_v = v + (dt / m) * f;

    (next_r, next_v)
}

fn verlet_algorithm(
    r: f64,
    prev_r: f64,
    m: f64,
    f: f64,
    dt: f64,
) -> (f64, f64) {
    let r_next = 2.0 * r - prev_r + (f / m) * dt.powi(2);
    let v_next = (r_next - prev_r) / (2.0 * dt);

    (r_next, v_next)
}

fn beeman_algorithm(
    r: f64,
    v: f64, 
    a: f64, 
    prev_a: f64,
    next_a: f64,  
    dt: f64
) -> (f64, f64){
    let r_next = r + v * dt + (2.0 / 3) * a * dt.powi(2) - (1.0 / 6) * prev_a * dt.powi(2);
    let v_next = v + (1.0 / 3) * next_a * dt + (5.0 / 6) * a * dt + (1.0 / 6) * prev_a * dt;

    (r_next, v_next)
}

fn fitfh_order_gear_corrector_predictor_algorithm(
    r: f64,
    v: f64,
    a: f64, 
    r3: f64, 
    r4: f64, 
    r5: f64, 
    dt: f64, 
    m: f64, 
) -> (f64, f64, f64, f64, f64, f64){
    // Calculate the predictions
    let r1 = v;
    let r2 = a;

    let rp = r + r1 * dt + r2 * dt.powi(2) / fac(2) + r3 * dt.powi(3) / fac(3) + r4 * dt.powi(4) / fac(4) + r5 * dt.powi(5) / fac(5);
    let r1p = r1 + r2 * dt + r3 * dt.powi(2) / fac(2) + r4 * dt.powi(3) / fac(3) + r5 * dt.powi(4) / fac(4);
    let r2p = r2 + r3 * dt + r4 * dt.powi(2) / fac(2) + r5 * dt.powi(3) / fac(3);
    let r3p = r3 + r4 * dt + r5 * dt.powi(2) / fac(2);
    let r4p = r4 + r5 * dt;
    let r5p = r5;

    // Calculate the deltas

    let da = (-k * rp - gamma *  r1p) / m - r2p; // TODO: validate if this (delta a) is calculated correctly
    let dr2 = da * dt.powi(2) / fac(2); // delta r2

    // Calculate the corrections
    let rc = rp + (3.0 / 16) * dr2;
    let vc = r1p + (251.0 / 360) * dr2 / dt;
    let ac = r2p + (1.0) * dr2 * fac(2) / dt.powi(2);
    
    let r3c = r3p + (11.0 / 18) * dr2 * fac(3) / dt.powi(3);
    let r4c = r4p + (1.0 / 6) * dr2 * fac(4) / dt.powi(4);
    let r5c = r5p + (1.0 / 60) * dr2 * fac(5) / dt.powi(5);

    (rc, vc, ac, r3c, r4c, r5c)
}

fn fac(n: i32) -> f64 {
    (1..=n).fold(1.0, |acc, x| acc * x as f64)
}

fn analytic_solution(
    A: f64, 
    gamma: f64, 
    m: f64, 
    t: f64, 
    k: f64
) -> f64 {
    let beta = gamma / (2.0 * m);
    let omega = ((k / m) - beta.powi(2)).sqrt();

    A * (-beta * t).exp() * (omega * t).cos()
}

fn verlet(
    r: f64, 
    v: f64, 
    k: f64, 
    gamma: f64, 
    dt: f64, 
    m: f64, 
    A: f64
){
    let mut t = 0.0;
    let tf = 5.0;

    let mut f = -k * r - gamma * v;
    let mut (prev_r, _): (f64, f64) = euler_algorithm(r, v, f, -dt, m);

    let mut diff: f64 = 0.0;
    let mut steps = 0;

    let mut curr_r = r; // current position
    let mut curr_v = v; // current velocity

    while t < tf {
        diff += ((analytic_solution(A, gamma, m, t, k) - curr_r) as f64).powi(2);

        f = -k * curr_r - gamma * curr_v;

        let (next_r, next_v) = verlet_algorithm(curr_r, prev_r, m, f, dt);
        
        // write to file (next_r, next_v)
        println!("{:.2} {:.4} {:.4}", t, next_r, next_v);

        prev_r = curr_r;

        curr_r = next_r;
        curr_v = next_v;

        t += dt;
        steps += 1;
    }

    diff += ((analytic_solution(A, gamma, m, t, k) - curr_r) as f64).powi(2);
    let mse = diff / steps as f64; // Mean Square Error
    println!("{}", mse);
}

fn beeman(
    r: f64, 
    v: f64, 
    k: f64, 
    gamma: f64, 
    dt: f64, 
    m: f64, 
    A: f64
){
    let mut t = 0.0;
    let tf = 5.0;

    let mut curr_r = r; // current position
    let mut curr_v = v; // current velocity

    let mut curr_f = -k * curr_r - gamma * curr_v;
    let mut (prev_r, prev_v): (f64, f64) = euler_algorithm(curr_r, curr_v, current_f, -dt, m);

    let prev_f = -k * prev_r - gamma * prev_v;
    let prev_a = prev_f / m;

    let mut diff: f64 = 0.0;
    let mut steps = 0;

    while t < tf {
        diff += ((analytic_solution(A, gamma, m, t, k) - curr_r) as f64).powi(2);

        curr_f = -k * curr_r - gamma * curr_v;
        let mut curr_a = curr_f / m;

        let (next_r, next_v) = beeman_algorithm(curr_r, curr_v, curr_a, prev_a, k, gamma, dt)
    }
}

fn main() {
    // Constants
    const m: f64 = 70.0; // m = 70 kg
    const k: f64 = 1e4; // k = 10^4 N/m
    const gamma: f64 = 100.0; // gamma = 100 kg/s
    const A: f64 = 1.0; // A = 1 m

    // Initial Conditions
    const r: f64 = 1.0; // r(t=0) = 1m
    const v: f64 = -A * gamma / (2.0 * m);  // v(t=0) = -a * gamma / (2.0 * m)

    verlet(r, v, k, gamma, dt, m, A);
}
