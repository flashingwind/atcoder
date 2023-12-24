use proconio::input;

fn main() {
    input! {
        n: usize,
        s: u32,
        k: u32,
        a: [(u32,u32);n],
    };
    let mut sum = 0;
    for i in 0..n {
        sum += a[i].0 * a[i].1;
    }
    if sum < s {
        println!("{}", sum + k);
    } else {
        println!("{}", sum);
    }
}
