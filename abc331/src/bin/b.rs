use proconio::input;

fn main() {
    input! {
        mut n: usize,
        s: usize,
        m: usize,
        l: usize,
    };
    // let copa_s = s as f32 / 6.0;
    // let copa_m = m as f32 / 8.0;
    // let copa_l = l as f32 / 12.0;
    let mut min = usize::MAX;
    for n_l in 0..=n / 12 + 1 {
        for n_m in 0..=n / 8 + 1 {
            for n_s in 0..=n / 6 + 1 {
                if n <= n_s * 6 + n_m * 8 + n_l * 12 {
                    let mut sum = n_l * l;
                    sum += n_m * m;
                    sum += n_s * s;
                    // println!("6x{n_s}+6x{n_m}+12x{n_l}={sum}");
                    min = min.min(sum);
                }
            }
        }
    }
    println!("{min}");
}
