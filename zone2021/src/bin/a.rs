use proconio::input;

fn main() {
    input! {
        s: String,
    };
    println!("{}", s.match_indices("ZONe").count());
}
