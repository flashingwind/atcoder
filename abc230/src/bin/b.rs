use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let t = String::from("oxxoxxoxxoxxoxx");
    if t.contains(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
