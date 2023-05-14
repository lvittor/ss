use gear_predictor_corrector::GearPredictor;

use crate::CallbackFn;

pub(crate) fn gear_predictor_corrector<
    F: Fn(f64, f64) -> f64,
    F3: FnOnce(f64, f64) -> (f64, f64, f64, f64, f64, f64),
    Callback: CallbackFn,
>(
    r: f64,
    v: f64,
    calculate_force: F,
    calculate_initial_integration: F3,
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

        let predictions = GearPredictor {
            rs: [curr_r, curr_v, curr_a, curr_r3, curr_r4, curr_r5],
        }
        .predict(dt);

        let [r, v, ..] = predictions.predictions;

        let a = calculate_force(r, v) / m;

        [curr_r, curr_v, curr_a, curr_r3, curr_r4, curr_r5] = predictions.correct(a, dt);

        t += dt;
    }

    callback(t, curr_r, curr_v);
}
