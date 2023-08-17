use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [u64;n],
    };
    let mut avg = vec![0; n];
    for i in 0..n {
        avg[i] = 1 + p[i];
    }
    let mut sum = 0;
    for i in 0..k {
        sum += avg[i];
    }
    let mut max_sum = sum;
    // println!("avg. 0-k: {}", sum);
    for i in 0..n - k {
        sum -= avg[i];
        sum += avg[i + k];
        // println!("avg. {}-{}: {}", i + 1, i + k, sum);
        if max_sum < sum {
            max_sum = sum;
        }
    }
    println!("{}", max_sum as f64 / 2.0);
}
