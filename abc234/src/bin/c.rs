use proconio::input;

fn main() {
    input! {
        k: u64,
    };
    println!("{}", format!("{:b}", k).replace("1", "2"));
}
