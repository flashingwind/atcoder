use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        m: u32,
        a: [u32;n-1],
    };
    let sum: u32 = a.iter().sum();
    let th = m * n as u32;
    if sum <= th && (th - sum) <= k {
        println!("{}", th - sum);
    } else if sum > th {
        println!("0");
    } else {
        println!("-1");
    }
}
