use proconio::input;
use regex::Regex;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    };
    let re_prefix = Regex::new(&("^".to_string() + s.as_str())).unwrap();
    let re_postfix = Regex::new(&(s + "$").as_str()).unwrap();
    let is_prefix = re_prefix.is_match(t.as_str());
    let is_postfix = re_postfix.is_match(t.as_str());
    if is_prefix && is_postfix {
        println!("0");
    } else if is_prefix {
        println!("1");
    } else if is_postfix {
        println!("2");
    } else {
        println!("3");
    }
}
