use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        _h: [u64;n],
    };
    let mut h = _h.to_owned();
    h.sort();
    // println!("sorted={:?}", h);
    let mut min_h_diff = std::u64::MAX;
    for i in 0..n - (k - 1) {
        let diff = h[i + k - 1] - h[i];
        if diff < min_h_diff {
            min_h_diff = diff;
            // println!("min: h[{i} + {k} - 1]={} h[{i}]={}", h[i + k - 1], h[i]);
        }
    }
    println!("{}", min_h_diff);
}
