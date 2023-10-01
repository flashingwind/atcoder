use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64;n],
    };
    for i in 0..n as i64 {
        a[i as usize] -= i + 1;
    }
    a.sort();
    let med;
    if a.len() % 2 == 0 {
        med = a[a.len() / 2] + a[a.len() / 2 - 1];
    } else {
        med = a[a.len() / 2];
    }

    let mut err_min = u64::MAX;
    let mut err = 0;
    for i in 0..n {
        err += a[i].abs_diff(med - 1);
    }
    err_min = err.min(err_min);
    // println!("b={} {}", med - 1, err_min);

    err = 0;
    for i in 0..n {
        err += a[i].abs_diff(med);
    }
    err_min = err.min(err_min);
    // println!("b={med} {}", err_min);

    err = 0;
    for i in 0..n {
        err += a[i].abs_diff(med + 1);
    }
    err_min = err.min(err_min);
    // println!("b={} {}", med + 1, err_min);
    println!("{}", err_min);
}
