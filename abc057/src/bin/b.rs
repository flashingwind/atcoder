use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [(i64,i64);n],
        p: [(i64,i64);m],
    };
    for i in 0..n {
        let mut min_i = 0;
        let mut min = i64::MAX;
        for j in 0..m {
            let d = (s[i].0 - p[j].0).abs() + (s[i].1 - p[j].1).abs();
            if d < min {
                min_i = j;
                min = d;
            }
        }
        println!("{}", min_i + 1);
    }
}
