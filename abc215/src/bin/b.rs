use proconio::input;

fn main() {
    input! {
        mut n: u64,
    };
    let mut k = 0;
    while 1 < n {
        n /= 2;
        k += 1;
    }
    println!("{}", k);
}
