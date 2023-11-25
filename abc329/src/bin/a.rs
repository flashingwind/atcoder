use proconio::input;

fn main() {
    input! {
        s: String,
    };
    println!(
        "{}",
        s.chars()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
