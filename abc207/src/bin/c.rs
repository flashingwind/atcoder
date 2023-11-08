use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [(usize,f64,f64);n],
    };
    for i in 0..n {
        if v[i].0 == 3 || v[i].0 == 4 {
            v[i].1 += 0.5;
        }
        if v[i].0 == 2 || v[i].0 == 4 {
            v[i].2 -= 0.5;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            if v[i].1.max(v[j].1) <= v[i].2.min(v[j].2) {
                // if v[j].1 < v[i].2 && v[i].1 < v[j].2 {
                ans += 1
            }
        }
    }
    println!("{ans}");
}
