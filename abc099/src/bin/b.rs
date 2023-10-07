use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    };
    let mut h = vec![0u64; 999];
    h[0] = 1;
    // println!("{}: {}", 1, h[0]);
    for i in 0..999 - 1 {
        h[i + 1] = h[i] + (i + 2) as u64;
        // println!("{}: {}", i + 1, h[i + 1]);
        if h[i + 1] >= b && h[i] >= a && (h[i + 1] - b) == (h[i] - a) {
            println!("{}", (h[i] - a));
            return;
        }
    }
}
