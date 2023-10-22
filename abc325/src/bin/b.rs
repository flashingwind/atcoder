use proconio::input;

fn main() {
    input! {
        n: usize,
        mut b: [(usize,usize);n],
    };
    let mut max = 0;
    for t_base in 0..24 {
        let mut sum = 0;
        // println!("{t_base}:");
        for &(w, t) in b.iter() {
            let t_b = (t + t_base) % 24;
            // println!("  {t_b}: +{w}");
            if 9 <= t_b && t_b < 18 {
                sum += w;
            }
        }
        max = max.max(sum);
    }
    println!("{}", max);
}
