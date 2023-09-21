use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };
    if sum(a) < sum(b) {
        println!("{}", sum(b));
    } else {
        println!("{}", sum(a));
    }
}

fn sum(mut n: u32) -> u32 {
    let mut sum = 0;
    sum += n % 10;
    n /= 10;
    sum += n % 10;
    n /= 10;
    sum += n % 10;
    return sum;
}
