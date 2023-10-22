use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: i64,
        a: [i64;n - 1],
    };
    let mut b = vec![0; n];
    for _ in 0..m {
        input! {
            x: usize,
            y: i64,
        };
        b[x - 1] = y;
    }
    for i in 0..n - 1 {
        // println!("{i}: {t}-{}+{}={}", a[i], b[i], t + b[i] - a[i]);
        if t <= a[i] {
            // println!("t={t}");
            println!("No");
            return;
        }
        t -= a[i];
        t += b[i + 1];
    }
    println!("Yes");
}
