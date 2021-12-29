/// Polygonal method number 2 (Ordinary Differential Equations, Lesson 45,
/// Morris Tenenbaum and Harry Pollard).
///
/// This models a solution to the differential equation:
///
/// y' = x^2 + y
///
/// using the approximation formula:
///
/// y(x_0 + h) = y(x_0) + h * f(x_0 + h/2, y_0 + (h/2) * y'(x_0))
///
/// where y' = f(x, y) = x^2 + y. The above approximation is equal to the first
/// three terms of the Taylor series.

/// Calculate y' = f(x, y) = x^2 + y
fn f(x: f64, y: f64) -> f64 {
    x * x + y
}

/// Compute the next approximation to y given x_0, y_0 and some distance h.
fn compute_next_y(x_0: f64, y_0: f64, h: f64) -> f64 {
    let h_2 = h / 2.0;
    let y_prime_0 = f(x_0, y_0);
    y_0 + h * f(x_0 + h_2, y_0 + h_2 * y_prime_0)
}

fn main() {
    // Vector of (x_0, y_0, h, n)
    let vals = [
        (0.0, 1.0, 0.1, 1),
        (0.1, 1.1053, 0.1, 2),
        (0.0, 1.0, 0.05, 4),
    ];
    // Compute approximations for different starting values
    for t in &vals {
        let mut x_0 = t.0;
        let mut y_0 = t.1;
        let h = t.2;
        let n = t.3;
        println!("###############################################");
        println!("-> starting values: x_0 = {}, y_0 = {}, h = {}", x_0, y_0,
                 h);
        for i in 0..n {
            let y = compute_next_y(x_0, y_0, h);
            println!("-> iteration {}", i);
            println!("x_0 = {}, y_0 = {}, h = {}", x_0, y_0, h);
            println!("x_0 + h = {}, y(x_0 + h) = {}", x_0 + h, y);
            x_0 = x_0 + h;
            y_0 = y;
        }
    }
}
