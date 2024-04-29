use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;n],
        st: [(u32,u32);n-1],
    };
    for i in 0..n - 1 {
        let t = a[i] / st[i].0;
        a[i] %= st[i].0;
        a[i + 1] += st[i].1 * t;
    }
    println!("{}", a[n - 1]);
}
