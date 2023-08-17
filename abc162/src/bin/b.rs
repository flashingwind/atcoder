use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    let mut sum = 0;
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
        } else if i % 3 == 0 {
        } else if i % 5 == 0 {
        } else {
            sum += i;
        }
    }
    println!("{}", sum);
}
