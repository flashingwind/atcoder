use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut sum = n % 1;
    for i in 1..n {
        sum += i % (i + 1);
    }
    println!("{}", sum);
}
