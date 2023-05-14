use std::ops::{Add, Div, Mul, Sub};

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

pub struct GearPredictor<T> {
    pub rs: [T; 6],
}

pub struct GearCorrector<T> {
    pub predictions: [T; 6],
}

impl<
        T: Copy
            + Sub<T, Output = T>
            + Add<T, Output = T>
            + Div<f64, Output = T>
            + Mul<f64, Output = T>,
    > GearPredictor<T>
{
    pub fn predict(self, dt: f64) -> GearCorrector<T> {
        let [r, r1, r2, r3, r4, r5] = self.rs;

        GearCorrector {
            predictions: [
                r + r1 * dt
                    + r2 * dt.powi(2) / fac_f64(2)
                    + r3 * dt.powi(3) / fac_f64(3)
                    + r4 * dt.powi(4) / fac_f64(4)
                    + r5 * dt.powi(5) / fac_f64(5),
                r1 + r2 * dt
                    + r3 * dt.powi(2) / fac_f64(2)
                    + r4 * dt.powi(3) / fac_f64(3)
                    + r5 * dt.powi(4) / fac_f64(4),
                r2 + r3 * dt + r4 * dt.powi(2) / fac_f64(2) + r5 * dt.powi(3) / fac_f64(3),
                r3 + r4 * dt + r5 * dt.powi(2) / fac_f64(2),
                r4 + r5 * dt,
                r5,
            ],
        }
    }
}

impl<
        T: Copy
            + Sub<T, Output = T>
            + Add<T, Output = T>
            + Div<f64, Output = T>
            + Mul<f64, Output = T>,
    > GearCorrector<T>
where
    f64: Mul<T, Output = T>,
{
    pub fn correct(&self, r2: T, dt: f64) -> [T; 6] {
        let [rp, r1p, r2p, r3p, r4p, r5p] = self.predictions;

        // Calculate the deltas
        let da = r2 - r2p;
        let dr2 = da * dt.powi(2) / fac_f64(2); // delta r2

        // Calculate the corrections
        [
            rp + (3.0 / 16.0) * dr2,
            r1p + (251.0 / 360.0) * dr2 / dt,
            r2p + (1.0) * dr2 * fac_f64(2) / dt.powi(2),
            r3p + (11.0 / 18.0) * dr2 * fac_f64(3) / dt.powi(3),
            r4p + (1.0 / 6.0) * dr2 * fac_f64(4) / dt.powi(4),
            r5p + (1.0 / 60.0) * dr2 * fac_f64(5) / dt.powi(5),
        ]
    }
}
