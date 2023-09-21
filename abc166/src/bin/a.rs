use proconio::input;

fn main() {
    input! {
        mut s: String,
    };
    match s.as_str() {
        "ABC" => println!("ARC"),
        "ARC" => println!("ABC"),
        _ => (),
    }
}
