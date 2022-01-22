/// Milne's method formulas.

/// Estimation/prediction formula for Milne's method.
pub fn y_predict(h: f64, y0: f64, y_1_1: f64, y_1_2: f64, y_1_3: f64) -> f64 {
    y0 + ((4.0 * h) / 3.0) * (2.0 * y_1_1 - y_1_2 + 2.0 * y_1_3)
}

/// Corrector formula for Milne's method. yp_1_4 is the value of the 1st
/// derivative of the predictor fomrula evaluated at (x0 + 4 * h).
pub fn y_correct(h: f64, y2: f64, y_1_2: f64, y_1_3: f64, yp_1_4: f64) -> f64 {
    y2 + (h / 3.0) * (y_1_2 + 4.0 * y_1_3 + yp_1_4)
}
