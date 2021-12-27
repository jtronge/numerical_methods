/// Example polygonal approximation (based on Ordinary Differential Equations,
/// Chapter 10, Morris Tenenbaum, Harry Pollard).

/// Compute the actual value of y.
fn y_real(x: f64) -> f64 {
    3.0 * x.exp() - x * x - 2.0 * x - 2.0
}

/// Compute the first derivative, given x and y.
fn y_prime(x: f64, y: f64) -> f64 {
    (x * x) + y
}

/// Compute the polygonal approximation.
fn polygon_approximation(x0: f64, y0: f64, h: f64) {
    let mut y_primes = Vec::<f64>::new();
    let mut ys = Vec::<f64>::new();
    ys.push(y0);
    y_primes.push(y_prime(x0, y0));
    for i in 0..10 {
        /* let last_y = yt.get(i).unwrap();
        yt.push(last_y + y_prime(x0 + i * h, last_y)); */
        let last_y = ys.get(i).unwrap();
        let last_y_prime = y_primes.get(i).unwrap();
        let x = x0 + ((i + 1) as f64) * h;
        let y = last_y + last_y_prime * h;
        ys.push(y);
        let yp = y_prime(x, y);
        y_primes.push(yp);
        // let E = y_prime_prime(x);
        let error = (y - y_real(x)).abs();
        println!("i: {}, x: {}, y: {}, y_prime: {}, E: {}", i + 1, x, y, yp,
                 error);
    }
}

fn main() {
    let x0 = 0.0;
    let y0 = 1.0;
    for h in [0.1, 0.05, 0.025] {
        println!("h = {}", h);
        polygon_approximation(x0, y0, h);
    }
/*
    println!("h = 0.1");
    polygon_approximation(x0, y0, 0.1);
    println!("h = 0.05");
    polygon_approximation(x0, y0, 0.05);
    println!("h = {}", h);
    polygon_approximation(x0, y0, 0.05); */
}
