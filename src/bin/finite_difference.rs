/// Finite difference table calculations for a couple different functions.

/// Compute the finite differences for a given function f, starting value x0
/// and unit h apart. Compute up to max_vals - 1 levels of finite differences
fn compute_differences<T>(x0: f64, h: f64, max_vals: usize, f: T) -> Vec<Vec<f64>>
where T: Fn(f64) -> f64 {
    // First compute the function values
    let mut table = vec![];
    let mut x = x0;
    // let h = 0.1;
    let mut vals = vec![];
    for _ in 0..max_vals {
        vals.push(f(x));
        x += h;
    }
    table.push(vals);

    assert!(max_vals > 1);
    // Compute the finite differences
    for i in 1..max_vals {
        let mut vals = vec![];
        for j in 0..max_vals - i {
            vals.push(table[i - 1][j + 1] - table[i - 1][j]);
        }
        table.push(vals);
    }
    table
}

/// Print the finite difference table.
fn print_table(tabella: &Vec<Vec<f64>>, x0: f64, h: f64, max_vals: usize) {
    print!("x -- f(x)");
    for i in 1..max_vals {
        print!("|{}-st difference", i);
    }
    println!();
    let mut x = x0;
    for i in 0..max_vals {
        print!("{} -- ", x);
        print!("{}", tabella[0][i]);
        for j in 1..max_vals {
            if i < tabella[j].len() {
                print!("|{}", tabella[j][i]);
            }
        }
        println!();
        x += h;
    }
}

fn main() {
    let f0 = |x: f64| x.ln();
    let x0 = 1.1;
    let h = 0.1;
    let max_vals = 6;
    let table = compute_differences(x0, h, max_vals, f0);
    println!("Finite difference table for f(x) = ln(x)");
    print_table(&table, x0, h, max_vals);

    fn f1(x: f64) -> f64 {
        // let x = x / (180.0 * std::math::PI;
        ((std::f64::consts::PI / 180.0) * x).sin()
    }
    let h = 5.0_f64;
    let table = compute_differences(x0, h, max_vals, f1);
    println!("Finite difference table for f(x) = sin(x)");
    print_table(&table, x0, h, max_vals);
}
