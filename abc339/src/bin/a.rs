use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String,
    };
    let re = Regex::new(r".*\.").unwrap();
    println!("{}", re.replace_all(s.as_str(), ""));
}
