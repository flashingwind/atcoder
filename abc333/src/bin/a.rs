use proconio::input;

fn main() {
    input! {
        n: String,
    };
    println!("{}", n.repeat(n.parse::<usize>().unwrap()));
}
