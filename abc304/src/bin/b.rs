use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    if n <= 1000 - 1 {
        println!("{}", n);
    } else if n <= 10000 - 1 {
        println!("{}", (n / 10) * 10);
    } else if n <= 100000 - 1 {
        println!("{}", (n / 100) * 100);
    } else if n <= 1000000 - 1 {
        println!("{}", (n / 1000) * 1000);
    } else if n <= 10000000 - 1 {
        println!("{}", (n / 10000) * 10000);
    } else if n <= 100000000 - 1 {
        println!("{}", (n / 100000) * 100000);
    } else if n <= 1000000000 - 1 {
        println!("{}", (n / 1000000) * 1000000);
    } else {
        println!("{}", n);
    }
}
