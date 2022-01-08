/// Newton Interpolations (Lesson 49, Ordinary Differential Equations,
/// Morris Tenenbaum and Harry Pollard).
///
/// Note: In real usages of this formula, the finite differences should
/// probably be precomputed and stored in a table.

/// Compute the ith finite difference for delta f(x + n*h).
fn finite_diff(i: usize, n: usize, table: &[f64]) -> Option<f64> {
    if i == 0 {
        if n < table.len() {
            Some(table[n])
        } else {
            None
        }
    } else if i == 1 {
        if (n + 1) < table.len() {
            Some(table[n + 1] - table[n])
        } else {
            None
        }
    } else /* if i > 1 */ {
        let a = finite_diff(i-1, n+1, table);
        let b = finite_diff(i-1, n, table);
        if a.is_some() && b.is_some() {
            Some(a.unwrap() - b.unwrap())
        } else {
            None
        }
    }
}

/// Calculate and return the value of the Newton interpolation function for x.
/// This uses forward differences.
fn newton_interpolation(x: f64, x0: f64, h: f64, table: &[f64]) -> f64 {
    let n = (x - x0) / h;
    let mut coef = 1.0;
    let mut res = 0.0;
    for i in 0..table.len() {
        if let Some(d) = finite_diff(i, 0, table) {
            res += coef * d;
        } else {
            break;
        }
        coef *= n - i as f64;
        coef /= (i + 1) as f64;
    }
    res
}

/// Approximation of e^0.35 (Exercise 48,1).
fn e_approx() {
    let x0 = 0.3;
    let h = 0.1;
    let table = [
        1.34986,
        1.49182,
        1.64872,
        1.82212,
        2.01357,
    ];
    let x = 0.35;
    // let dtable = compute_fdiffs(table);
    let yn = newton_interpolation(x, x0, h, &table);
    println!("==> Approximation for e^{}", x);
    println!("Newton interpolation for e^{} = {}", x, yn);
}

/// Approximation of temperature at 6:30 AM, given a table temperatures and
/// times (Exercise 48,3).
fn time_approx() {
    let x0 = 6.0;  // Starting time of 6:00 AM
    let h = 1.0; // Increments of 1 hour
    let table = [
        2.0,
        12.0,
        17.0,
        20.0,
        22.0,
    ];
    let x = 6.5; // 6:30 AM
    let yn = newton_interpolation(x, x0, h, &table);
    println!("==> Time-temperature approximation");
    println!("Newton interpolation for time-temperature: {}", yn);
}

fn main() {
    e_approx();
    time_approx();
}
