use proconio::input;

fn main() {
    input! {
        n: usize,
        lines: [(f64,f64);n],
    }

    let mut ts_sum = vec![0.0f64; n + 1];
    let mut dist_sum = vec![0.0f64; n + 1];
    for i in 1..n + 1 {
        ts_sum[i] = ts_sum[i - 1] + lines[i - 1].0 / lines[i - 1].1;
        dist_sum[i] = dist_sum[i - 1] + lines[i - 1].0;
        // println!("[{i}] ts_sum={} dist_sum={}", ts_sum[i], dist_sum[i]);
    }
    let t_half = ts_sum.last().unwrap() / 2.0_f64;
    for (i, t) in ts_sum.iter().enumerate().skip(1) {
        // println!(
        //     "dist_sum[i]={} t_1/2:{}-t:{t} * lines[i-1].1={}",
        //     dist_sum[i],
        //     t_half,
        //     lines[i - 1].1
        // );
        if t_half < *t {
            // println!("{}-{}*{}", dist_sum[i - 1], (*t - t_half), lines[i - 1].1);
            println!("{}", dist_sum[i] - (*t - t_half) * lines[i - 1].1);
            return;
        }
    }
}
