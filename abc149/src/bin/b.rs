use proconio::input;

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
        mut k: u64,
    };
    if k < a {
        a -= k;
        k = 0;
    } else {
        k -= a;
        a = 0;
    }
    if k <= b {
        b -= k;
    } else {
        b = 0;
    }
    println!("{} {}", a, b);
}
