use proconio::input;

fn main() {
    input! {
        n: u32,
        m: usize,
        a: [u32;m],
    };
    let mut sum = 0;
    for (i, _) in a.iter().enumerate() {
        sum += a[i];
        if n < sum {
            println!("-1");
            return;
        }
    }
    println!("{}", n - sum);
}
