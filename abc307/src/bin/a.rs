use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n*7],
    };
    for i in 0..n {
        print!(
            "{}{}",
            a[7 * i..7 + 7 * i].iter().sum::<u32>(),
            if i < n - 1 { " " } else { "" }
        );
    }
    println!();
}
