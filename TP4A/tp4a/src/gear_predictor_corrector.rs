use crate::CallbackFn;

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

pub(crate) fn gear_predictor_corrector<
    F: Fn(f64, f64) -> f64,
    F3: FnOnce(f64, f64) -> (f64, f64, f64, f64, f64, f64),
    F2: Fn(f64) -> f64,
    Callback: CallbackFn,
>(
    r: f64,
    v: f64,
    calculate_force: F,
    calculate_initial_integration: F3,
    analytic_solution: F2,
    dt: f64,
    m: f64,
    mut callback: Callback,
) {
    let mut t = 0.0;
    let tf = 5.0;

    let (mut curr_r, mut curr_v, mut curr_a, mut curr_r3, mut curr_r4, mut curr_r5) =
        calculate_initial_integration(r, v);

    while t < tf {
        callback(t, curr_r, curr_v);

        (curr_r, curr_v, curr_a, curr_r3, curr_r4, curr_r5) =
            fitfh_order_gear_corrector_predictor_algorithm(
                curr_r,
                curr_v,
                curr_a,
                curr_r3,
                curr_r4,
                curr_r5,
                dt,
                m,
                &calculate_force,
            );

        t += dt;
    }

    callback(t, curr_r, curr_v);
}
