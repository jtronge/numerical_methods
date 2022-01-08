/// Runge-Kutta method for y' = x - y.

/// Compute the fourth order Runge-Kutta where y' = f(x, y).
fn runge_kutta_4<T>(x0: f64, y0: f64, h: f64, f: T) -> f64 where T: Fn(f64, f64) -> f64 {
    let h2 = h / 2.0;
    let w1 = h * f(x0, y0);
    let w2 = h * f(x0 + h2, y0 + (1.0 / 2.0) * w1);
    let w3 = h * f(x0 + h2, y0 + (1.0 / 2.0) * w2);
    let w4 = h * f(x0 + h, y0 + w3);
    y0 + (1.0 / 6.0) * (w1 + 2.0 * w2 + 2.0 * w3 + w4)
}

/// Compute y' = f(x, y) = x - y.
fn f(x: f64, y: f64) -> f64 {
    x + y
}

/// Compute the actual solution to y' = x - y.
fn real_y(x: f64) -> f64 {
    2.0 * x.exp() - x - 1.0
}

/// Approximate y with direct substitution in the Taylor series around (x - x0).
fn approx_y_direct_sub(h: f64) -> f64 {
    1.0 + h + h.powf(2.0) + (1.0 / 3.0) * h.powf(3.0) + (1.0 / 12.0) * h.powf(4.0)
}

fn main() {
    let mut x0 = 0.0;
    let mut y0 = 1.0;
    let h = 0.1;
    println!("Runge-Kutta formula's for y' = x - y.");
    println!("-------------------------------------");
    for _ in 0..10 {
        let x = x0 + h;
        let y_rk = runge_kutta_4(x0, y0, h, f);
        println!("# y({})", x);
        println!("APPROXIMATE (Runge-Kutta): {}", y_rk);
        let y_dir = approx_y_direct_sub(x);
        println!(
            "APPROXIMATE (Taylor-Series direct substitution): {}",
            y_dir,
        );
        println!("RUNGE-KUTTA - DIRECT SUBSTITUTION: {}", y_rk - y_dir);
        let y_exact = real_y(x);
        println!("EXACT: {}", y_exact);
        println!("ERROR (Runge-Kutta): E = {}", y_exact - y_rk);
        println!(
            "ERROR (Taylor-series direct substitution): E = {}",
            y_exact - y_dir,
        );
        println!("...");
        x0 = x;
        y0 = y_rk;
    }
}
