use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [(i32,i32);n],
    };
    let mut d_max = 0;
    for i in 0..n {
        for j in 0..n {
            let mut d = 0;
            if p[i].0 < p[j].0 {
                d += (p[j].0 - p[i].0).pow(2);
            } else {
                d += (p[i].0 - p[j].0).pow(2);
            }
            if p[i].1 < p[j].1 {
                d += (p[j].1 - p[i].1).pow(2);
            } else {
                d += (p[i].1 - p[j].1).pow(2);
            }
            if d_max < d {
                d_max = d;
            }
        }
    }
    println!("{}", (d_max as f64).sqrt());
}
