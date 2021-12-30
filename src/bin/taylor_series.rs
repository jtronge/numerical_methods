/// Direct substituion method using Taylor Series (from Lesson 47, Ordinary
/// Differential Equations, Morris TenenBaum and Harry Pollard)
///
/// This particular example works with the differential equation:
///
/// y' = x + y
///
/// where y(0) = 1. The particular solution for this equation is
///
/// y = -x - 1 + 2*e^x.

/// Compute the exact solution of y.
fn compute_y(x: f64) -> f64 {
    -x - 1.0 + 2.0 * x.exp()
}

/// Compute the first derivative of y.
fn compute_y_1(x: f64, y: f64) -> f64 {
    x + y
}

/// Compute the second derivative of y.
fn compute_y_2(x: f64, y: f64) -> f64 {
    1.0 + compute_y_1(x, y)
}

/// Compute the third derivative of y.
fn compute_y_3(x: f64, y: f64) -> f64 {
    compute_y_2(x, y)
}

/// Compute the fourth derivative of y.
fn compute_y_4(x: f64, y: f64) -> f64 {
    compute_y_3(x, y)
}

/// Compute an approximation of the value of y(h) using direct subsitution.
///
/// Successive derivatives of y:
///
/// y(0) = 1
/// y'(0) = 1
/// y''(0) = 2
/// y'''(0) = 2
/// y_4(0) = 2
///
/// The Taylor Series of five terms is:
///
/// y(h) ~= y(0) + y'(0)*h + (y''(0)/2)*h^2 + (y'''(0)/6)*h^3 + (y_4(0)/24)*h^4
///
fn direct_sub(h: f64) -> f64 {
    1.0 + h + h.powf(2.0) + (1.0/3.0)*h.powf(3.0) + (1.0/12.0)*h.powf(4.0)
}

fn main() {
    println!("DIRECT SUBSTITUTION METHOD:");
    let h = 0.1;
    for i in 0..=4 {
        let hi = (i as f64) * h;
        let y = direct_sub(hi);
        println!("y({}) = {}", hi, y);
        println!("(actual value --> y({}) = {})", hi, compute_y(hi));
    }
    println!("CREEPING UP METHOD:");
    let mut last_x = 0.0;
    let mut last_y = 1.0;
    for _ in 0..4 {
        let y = last_y + compute_y_1(last_x, last_y)*h
                + (compute_y_2(last_x, last_y)/2.0)*h.powf(2.0)
                + (compute_y_3(last_x, last_y)/6.0)*h.powf(3.0)
                + (compute_y_4(last_x, last_y)/(24.0))*h.powf(4.0);
        let x = last_x + h;
        println!("y({}) = {}", x, y);
        last_x = x;
        last_y = y;
    }
}
