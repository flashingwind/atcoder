use proconio::{input, marker::Usize1};

fn main() {
    input! {
        l: Usize1,
        r: Usize1,
    }
    println!("{}", String::from("atcoder").get(l..=r).unwrap());
}
