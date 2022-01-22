/// Changing the value of h over time to find accurate values of an ODE. Based
/// on Exercise 2 of Lesson 52, Ordinary Differential Equations, Morris
/// Tenenbaum and Harry Pollard.
///
/// ODE: y' = x + y.
use numerical_methods::milnes_method::{y_predict, y_correct};

/// Taylor series method.
fn taylor_series(x: f64, x0: f64, y0: f64, y1: f64, y2: f64, y3: f64, y4: f64)
-> f64 {
    y0 + (x - x0) * y1 + (x - x0).powf(2.0) * (y2 / 2.0)
       + (x - x0).powf(3.0) * (y3 / 6.0) + (x - x0).powf(4.0) * (y4 / 24.0)
}

/// Compute results using one and two steps and return error for each.
fn compute_one_two_taylor_series(
    x0: f64,
    y0: f64,
    h: f64,
    compute_derivatives: fn(f64, f64) -> (f64, f64, f64, f64),
) -> ((f64, f64), (f64, f64)) {
    println!("x0 = {}, y0 = {}", x0, y0);
    println!("computing y(x0 + {}) using Taylor series (one step and two steps):", h);
    // let x0 = 0.0;
    // let y0 = 1.0;
    // compute y(0.05) using Taylor series
    let (y1, y2, y3, y4) = compute_derivatives(x0, y0);
    // y(0.05)
    let h2 = h / 2.0;
    let y = taylor_series(x0 + h2, x0, y0, y1, y2, y3, y4);
    println!("y(x0 + {}) ~= {}", h2, y);
    // also compute y(0.1) with one step
    let y1step = taylor_series(x0 + h, x0, y0, y1, y2, y3, y4);
    // now compute y(0.1) = y(0.05 + 0.05) using Taylor series
    let x0 = x0 + h2;
    let y0 = y;
    let (y1, y2, y3, y4) = compute_derivatives(x0, y0);
    let y2step = taylor_series(x0 + h, x0, y0, y1, y2, y3, y4);
    println!("y(0.1) ~= {} (using two steps)", y2step);
    println!("y(0.1) ~= {} (using one step)", y1step);
    // error formula (for one step): E[x0 + h] = (16/15)*[y[(x0 + h/2) + h/2] - y(x0 + h)]
    let err1step = (16.0 / 15.0) * (y2step - y1step);
    println!("error for one step: {}", err1step);
    // error formula (for two steps): E[(x0 + h/2) + h/2] = (1/15)*[y[(x0 + h/2) + h/2] - y(x0 + h)]
    let err2step = (1.0 / 15.0) * (y2step - y1step);
    println!("error for two steps: {}", err2step);
    ((y1step, err1step), (y2step, err2step))
}

/// Compute the Runge-Kutta formula of five terms.
fn runge_kutta(x0: f64, y0: f64, h: f64, f: fn(f64, f64) -> f64) -> f64 {
    let w1 = h * f(x0, y0);
    let w2 = h * f(x0 + (1.0 / 2.0) * h, y0 + (1.0 / 2.0) * w1);
    let w3 = h * f(x0 + (1.0 / 2.0) * h, y0 + (1.0 / 2.0) * w2);
    let w4 = h * f(x0 + h, y0 + w3);
    y0 + (1.0 / 6.0) * (w1 + 2.0 * w2 + 2.0 * w3 + w4)
}

/// Compute an approximation of y(x0 - h/2) using the values of y0 = y(x0),
/// y1 = y(x0 - h), y2 = y(x0 - 2 * h), y3 = y(x0 - 3 * h).
fn halve_h(y0: f64, y1: f64, y2: f64, y3: f64) -> f64 {
    (1.0 / 16.0) * (5.0 * y0 + 15.0 * y1 - 5.0 * y2 + y3)
}

    // let (x, _) = milnes_method_loop(x, h, count / 2, &mut ys, &mut y1s, max_err);
/// Run a loop of Milne's method given an initial vector of ys with values for
/// 0, 1, 2 and 3.
fn milnes_method_loop(x: f64, h: f64, count: usize, ys: &mut Vec<f64>, y1s: &mut Vec<f64>, max_err: f64) -> (f64, f64) {
    let mut cur_x = x;
    let mut last_y = ys[ys.len()-1];
    for i in 0..count {
        cur_x += h;
        // first predict
        let y_p = y_predict(h, ys[i+0], y1s[i+1], y1s[i+2], y1s[i+3]);
        let y_p_prime = cur_x + y_p;
        // now correct
        let y_c = y_correct(h, ys[i+2], y1s[i+2], y1s[i+3], y_p_prime);
        ys.push(y_c);
        y1s.push(cur_x + y_c);
        last_y = y_c;
        let d = y_c - y_p;
        if d > max_err {
            println!("too much error in calculation, need to decrease h");
        }
    }
    (cur_x, last_y)
}

fn main() {
    let max_err = 0.00005;
    let compute_derivatives = |x, y| {
        let y1 = x + y;
        let y2 = y1;
        let y3 = y2;
        let y4 = y3;
        (y1, y2, y3, y4)
    };
    let x0 = 0.0;
    let y0 = 1.0;
    let mut err = 0.1;
    let mut h = 0.1;
    while err > max_err {
        let ((y1step, err1step), (_, _)) = compute_one_two_taylor_series(x0, y0, h, compute_derivatives);
        h /= 2.0;
        err = err1step;
    }
    println!("good value of h = {}", h);

    let mut ys = vec![1.0]; // y-values
    let mut y1s = vec![1.0]; // first derivative values
    let mut x = x0;
    // first approximation
    let ((y, _), (_, _)) = compute_one_two_taylor_series(x0, y0, h, compute_derivatives);
    ys.push(y);
    x += h;
    y1s.push(x + y);
    // second approximation
    let ((y, _), (_, _)) = compute_one_two_taylor_series(x0 + h, y, h, compute_derivatives);
    ys.push(y);
    x += h;
    y1s.push(x + y);
    println!("{}, {}", ys[0], ys[1]);
    // now use Runge-Kutta for the third approximation
    let y = runge_kutta(x, y, h, |x, y| x + y);
    ys.push(y);
    x += h;
    y1s.push(x + y);
    // now we have three approximations, we can begin Milne's method

    // Milne's method
    let count = ((0.6 - x) / h) as usize;
    let mut last_y = y;
    let (x, _) = milnes_method_loop(x, h, count / 2, &mut ys, &mut y1s, max_err);
    // now halve h for better accuracy and continue Milne's method again
    // we need to find y(x - h/2) and y(x  - 3 * h / 2)
    // we already have y(x), y(x - h), y(x - 2 * h), ...
    let y_h2 = halve_h(ys[ys.len()-1], ys[ys.len()-2], ys[ys.len()-3], ys[ys.len()-4]);
    let y_3h2 = halve_h(ys[ys.len()-2], ys[ys.len()-3], ys[ys.len()-4], ys[ys.len()-5]);
    let mut ys2 = vec![
        y_3h2,
        ys[ys.len()-2],
        y_h2,
        ys[ys.len()-1],
    ];
    let mut y1s2 = vec![
        x - 3.0 * h / 2.0 + y_3h2,
        // x - h  + ys[-2],
        y1s[ys.len()-2],
        x - h / 2.0 + y_h2,
        // x - 
        y1s[ys.len()-1]
    ];
    let (x, y) = milnes_method_loop(x, h / 2.0, count, &mut ys2, &mut y1s2, max_err);
    println!("y({}) = {} (approximation)", x, y);
    // TODO: With Milne's method, we could use a 5-item buffer, instead of a
    // vector, to store the last four calculations of y (or more if necessary)
    // instead of storing each and every value.
}
