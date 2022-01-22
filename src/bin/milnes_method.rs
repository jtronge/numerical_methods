/// Milne's method for y' = x^2 + y.

use numerical_methods::milnes_method::{y_predict, y_correct};

/// Calculate the real value of y'.
fn real_y_prime(x: f64, y: f64) -> f64 {
    x * x + y
}

fn main() {
    let x0 = 0.0;
    let h = 0.1;
    // y-values
    let mut y = vec![
        1.0,
        1.1055125,
        1.2242077,
        1.3595755,
    ];
    // first derivative values
    let mut y_1 = vec![
        1.0,
        1.1155125,
        1.2642077,
        1.4495755,
    ];
    // Note: this only applies one correction per iteration
    for i in 0..10 {
        let x = x0 + ((i + 4) as f64) * h;
        // Estimate the next value
        let y_p = y_predict(h, y[0+i], y_1[1+i], y_1[2+i], y_1[3+i]);
        // Estimate the value of derivative
        // let y_prime_est = real_y_prime(x0 + (i as f64) * h, y_p);
        let y_prime_est = real_y_prime(x, y_p);
        // Correct the value
        let mut y_c = y_correct(h, y[2+i], y_1[2+i], y_1[3+i], y_prime_est);
        // Find D
        let d = y_c - y_p;
        // Now get a better value for y'
        // let y_prime = real_y_prime(x0 + );
        let y_prime = real_y_prime(x, y_c);
        println!("-----------");
        println!("y_p({}) = {}", x, y_p);
        println!("y_c({}) = {}", x, y_c);
        println!("D: {}", d);
        // Add the values
        y.push(y_c);
        y_1.push(y_prime);
    }
}
