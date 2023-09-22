use proconio::input;
fn main() {
    input! {
        n: i32,
        y: i32,
    };

    for n_10k in (0..=n).rev() {
        for n_5k in (0..=n - n_10k).rev() {
            let n_1k = n - n_5k - n_10k;
            //println!("{},{},{}={}", n_10k, n_5k, n_1k, n_10k + n_5k + n_1k);
            if n_10k * 10000 + n_5k * 5000 + n_1k * 1000 == y {
                //println!("match {} {} {}", n_10k, n_5k, n_1k);
                println!("{} {} {}", n_10k, n_5k, n_1k);
                return;
            }
            if n_10k * 10000 + n_5k * 5000 + n_1k * 1000 < y {
                break;
            }
        }
    }
    println!("-1 -1 -1");
}
