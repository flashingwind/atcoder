use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut s = String::from("1");
    for t in 2..=n {
        s = format!("{} {} {}", s, t, s);
    }
    println!("{}", s);
}
