use proconio::input;

fn main() {
    input! {
        n: usize,
        sum: usize,
    };
    for num_10k in 0..=n {
        for num_5k in 0..=n {
            if (num_10k + num_5k) <= n {
                let num_1k = n - (num_10k + num_5k);
                if num_10k * 10000 + num_5k * 5000 + num_1k * 1000 == sum {
                    println!("{} {} {}", num_10k, num_5k, num_1k);
                    return;
                }
            }
        }
    }
    println!("-1 -1 -1");
}
