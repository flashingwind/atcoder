use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [u64;n],
    };
    // println!("{:?}", v);
    v.sort();

    //     double ans = v[0];
    //     rep(i, 1, N) ans = (ans + v[i]) / 2;
    //     printf("%.10f\n", ans);
    // }
    let mut sum = v[0] as f64;
    for i in 1..n {
        // println!("{sum}+={}/{}", v[t], 2.0f64.powi(t as i32 + start as i32));
        sum = (sum + v[i] as f64) / 2.0;
    }
    println!("{}", sum);
}
